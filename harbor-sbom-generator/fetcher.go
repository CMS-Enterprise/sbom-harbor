package main

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"
	"sync"
	"time"

	getter "github.com/hashicorp/go-getter"
	hclog "github.com/hashicorp/go-hclog"
	"github.com/hashicorp/go-multierror"
)

const DEFAULT_PAGE = 1
const DEFAULT_PAGE_SIZE = 100
const DEFAULT_DOWNLOAD_PATH = "/tmp"
const REPO_URL_TEMPLATE = "https://api.github.com/repos/%s/%s"
const REPOS_URL_TEMPLATE = "https://api.github.com/users/%s/repos?per_page=%d&page=%d"

// Stats encapsulates metadata about a GitHub repository.
type Stats struct {
	Total     int
	Empty     int
	Err       int
	Languages map[string]int
}

// Fetcher fetches and transforms repo data from the GitHub API.
type Fetcher struct {
	client          *http.Client
	opts            *Opts
	logger          hclog.Logger
	stats           *Stats
	results         []*HarborResult
	targetLanguages map[string]struct{}
}

// NewFetcher is the factory method for a Fetcher instance.
func NewFetcher() *Fetcher {
	opts := NewOptsFromEnv("fetcher")

	return &Fetcher{
		client: &http.Client{
			Timeout: 10 * time.Second,
		},
		opts:    opts,
		logger:  opts.Logger,
		stats:   nil,
		results: []*HarborResult{},
		targetLanguages: map[string]struct{}{
			"Java":       {},
			"Python":     {},
			"Jinja":      {},
			"XSLT":       {},
			"Ruby":       {},
			"SAS":        {},
			"Groovy":     {},
			"JavaScript": {},
			"Vue":        {},
			"Shell":      {},
			"Go":         {},
			"TypeScript": {},
			"HCL":        {},
		},
	}
}

// ReposURL generates a GitHub API repo URL request by org, page size, and page number.
func (f *Fetcher) ReposURL(page uint16) string {
	// If fetching only a single repo, build a single result and return.
	if f.opts.Repo != "" {
		return fmt.Sprintf(REPO_URL_TEMPLATE, f.opts.Org, f.opts.Repo)
	}

	pageSize := f.opts.PageSize

	if pageSize == 0 {
		pageSize = DEFAULT_PAGE_SIZE
	}

	if page == 0 {
		page = DEFAULT_PAGE
	}

	return fmt.Sprintf(REPOS_URL_TEMPLATE, f.opts.Org, pageSize, page)
}

// Get is a utility method for HTTP GET operations using
// the http.Client instance.
func (f *Fetcher) Get(url string) (*http.Response, error) {
	req, err := http.NewRequest(http.MethodGet, url, nil)
	if err != nil {
		fmt.Printf("client: could not create request: %s\n", err)
		os.Exit(1)
	}

	req.Header.Set("Authorization", fmt.Sprintf("Bearer %s", f.opts.FetchToken))

	res, err := f.client.Do(req)
	if err != nil {
		err = fmt.Errorf("error making http request: %s", err)
		f.logger.Error(err.Error())
		return nil, err
	}

	return res, nil
}

// FetchRepos makes several HTTP request to the GitHub API and
// builds the set of repositories for the organization.
func (f *Fetcher) FetchRepos() error {
	currentPage := f.opts.StartPage
	currentPageSize := f.opts.PageSize
	createdAt := time.Now()

	// Page through the results until all repositories have been discovered.
	for currentPageSize == f.opts.PageSize {
		res, err := f.Get(f.ReposURL(currentPage))
		if err != nil {
			err = fmt.Errorf("error making http request: %s", err)
			f.logger.Error(err.Error())
			return err
		}

		f.logger.Info("fetch_repos", "status_code", res.StatusCode)

		bytes, err := io.ReadAll(res.Body)
		if err != nil {
			err = fmt.Errorf("error reading response body: %s", err)
			f.logger.Error(err.Error())
			return err
		}

		if res.StatusCode != http.StatusOK {
			body := string(bytes)
			f.logger.Error("fetch_repos failed", "body", body)
			return fmt.Errorf("error in fetch_repos: %s", body)
		}

		var results []*HarborResult
		if f.opts.Repo == "" {
			err = json.Unmarshal(bytes, &results)
		} else {
			result := &HarborResult{}
			err = json.Unmarshal(bytes, result)
			results = append(results, result)
		}

		if err != nil {
			err = fmt.Errorf("error unmarshalling response body: %s", err)
			f.logger.Error(err.Error())
			return err
		}

		currentPageSize = uint8(len(results))

		for _, result := range results {
			result.CreatedAt = createdAt
		}

		f.results = append(f.results, results...)

		currentPage = currentPage + 1
	}

	return nil
}

// Stats returns statistics about the fetch results such as
// the number of repos that are tagged with a specific language.
func (f *Fetcher) Stats() *Stats {
	if f.stats == nil {
		f.stats = &Stats{
			Total:     len(f.results),
			Languages: map[string]int{},
		}
		for _, result := range f.results {
			if result.IsEmpty {
				f.stats.Empty = f.stats.Empty + 1
			}

			if result.Err != nil {
				f.stats.Err = f.stats.Err + 1
			}

			if result.Language == "" {
				result.Language = "Not specified"
			}

			languageCount, ok := f.stats.Languages[result.Language]
			if !ok {
				f.stats.Languages[result.Language] = 1
			} else {
				f.stats.Languages[result.Language] = languageCount + 1
			}
		}
	}

	// Return a copy so it can't be mutated externally.
	result := f.stats
	return result
}

// CloneSyftAndUpload clones the repos for the team, generates
// an SBOM, and then uploads to the harbor.
func (f *Fetcher) CloneSyftAndUpload(token, sbomToken string, team *Team, results []*HarborResult) error {
	var waitGroup sync.WaitGroup
	workerCount := 10
	processCh := make(chan *HarborResult, workerCount)
	doneCh := make(chan int, len(results))
	waitGroup.Add(len(results))

	// Start workers
	for index, result := range results {
		processCh <- result
		go func(index int, result *HarborResult) {
			defer func() { <-processCh }()
			defer waitGroup.Done()

			logger := buildLogger(fmt.Sprintf("worker-%d", index))

			fetcher := NewFetcher()
			if _, ok := fetcher.targetLanguages[result.Language]; !ok {
				logger.Info("skipping invalid target language", "index", index, "language", result.Language)
				doneCh <- index
				return
			}

			logger.Info("processing result", "index", index, "language", result.Language)

			fetcher.CloneRepo(result)
			if result.Err != nil {
				logger.Error("error cloning repo", "error", result.Err)
				doneCh <- index
				return
			}

			syfter := NewSyfter()
			syfter.Syft(result)
			if result.Err != nil {
				logger.Error("error generating SBOMs", "error", result.Err)
				doneCh <- index
				return
			}

			if result.SBOM == "" {
				logger.Error("skipping empty SBOM", "name", result.Name)
				doneCh <- index
				return
			}

			harbor := NewHarbor()
			harbor.BuildProject(token, sbomToken, team, result)
			if result.Err != nil {
				logger.Error("error creating project", "error", result.Err)
				doneCh <- index
				return
			}

			harbor.UploadSBOM(result)
			if result.Err != nil {
				logger.Error("error uploading SBOM", "name", result.Name, "error", result.Err)
				logger.Trace(result.SBOM)
				doneCh <- index
				return
			}

			doneCh <- index
		}(index, result)
	}

	go func() {
		waitGroup.Wait()
		close(doneCh)
	}()

	var mErr *multierror.Error

	for index := range doneCh {
		f.logger.Info("finished processing result", "index", index)
		if results[index].Err != nil {
			mErr = multierror.Append(mErr, results[index].Err)
		}
	}

	return mErr.ErrorOrNil()
}

// CloneRepo clones a single repo and decrements the WaitGroup
func (f *Fetcher) CloneRepo(result *HarborResult) {
	var err error

	result.IsEmpty = f.IsEmpty(result)
	if result.Err != nil {
		return
	}

	if result.IsEmpty {
		f.logger.Trace("skipping empty repo", "repo_name", result.Name, "repo_url", result.HtmlURL)
		return
	}

	goGetterURL := result.GoGetterURL()
	if goGetterURL == "" {
		result.Err = fmt.Errorf("go getter url cannot be blank: %s", result.Name)
		return
	}

	err = getter.Get(clonePath(f.opts.WorkingDir, result.Name), goGetterURL)
	if err != nil {
		result.Err = fmt.Errorf("error cloning repo %s from %s: %s", result.Name, goGetterURL, err)
		return
	}

	f.logger.Info("finished cloning repo", "repo_name", result.Name, "repo_url", result.HtmlURL)
}

// IsEmpty determines whether a repo has content.
func (f *Fetcher) IsEmpty(result *HarborResult) bool {
	// TODO: Verify this is the best way to determine emptiness
	res, err := f.Get(result.ContributorsURL)
	if err != nil {
		result.Err = fmt.Errorf("error retrieving contributors: %s", err)
		return true
	}

	if res.StatusCode != http.StatusOK && res.StatusCode != http.StatusNoContent {
		bytes, ioErr := io.ReadAll(res.Body)
		if ioErr != nil {
			result.Err = fmt.Errorf("error reading contributors response body: %s", ioErr)
			return true
		}

		body := string(bytes)
		result.Err = fmt.Errorf("unexpected response in is_empty: status_code: %d - %s", res.StatusCode, body)
	}

	return res.StatusCode == http.StatusNoContent
}
