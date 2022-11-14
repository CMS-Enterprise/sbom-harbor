package main

import (
	"context"
	"errors"
	"fmt"
	"os"
	"path"

	"github.com/aws/aws-lambda-go/lambda"
	hclog "github.com/hashicorp/go-hclog"
)

const DEFAULT_GITHUB_ORG = "cmsgov"

func main() {
	lambda.Start(HandleRequest)
}

type FetchEvent struct {
	OrgName string `json:"org_name"`
}

// Opts encapsulates settings for the pipeline.
type Opts struct {
	Logger         hclog.Logger
	APIBaseURL     string
	HarborUser     string
	HarborPassword string
	FetchToken     string
	Org            string
	Repo           string
	WorkingDir     string
	PageSize       uint8
	StartPage      uint16
}

func HandleRequest(ctx context.Context, name FetchEvent) error {
	opts := NewOptsFromEnv("generator")
	if opts.FetchToken == "" {
		opts.Logger.Error("github fetch token not found")
		return fmt.Errorf("error retrieving github fetch token")
	}

	harbor := NewHarbor()
	err := harbor.CreateTeam()
	if err != nil {
		opts.Logger.Error("error building harbor", "error", err)
		return err
	}

	// Get list of repos
	fetcher := NewFetcher()
	err = fetcher.FetchRepos()
	if err != nil {
		opts.Logger.Error("error fetching repos", "error", err)
		return err
	}

	results := fetcher.results
	if len(results) < 1 {
		opts.Logger.Error("no results")
		return errors.New("no results")
	}

	for _, result := range results {
		result.TeamID = harbor.team.ID
	}

	err = fetcher.CloneSyftAndUpload(harbor.token, harbor.sbomToken, harbor.team, results)
	if err != nil {
		opts.Logger.Error("generator complete with errors.", "error", err)

		// If processing a single repo, and there is an error, fail.
		if len(results) == 1 {
			return err
		}

		return nil
	}

	opts.Logger.Info("generator complete")

	return nil
}

// NewOptsFromEnv is a constructor function that builds options
// from environment variables, and sets allowable defaults.
func NewOptsFromEnv(loggerName string) *Opts {
	logger := buildLogger(loggerName)
	opts := &Opts{
		Logger:         logger,
		FetchToken:     os.Getenv("GH_FETCH_TOKEN"),
		Org:            os.Getenv("GITHUB_ORG"),
		Repo:           os.Getenv("GITHUB_REPO"),
		APIBaseURL:     os.Getenv("CF_DOMAIN") + "/api/v1",
		HarborUser:     os.Getenv("HARBOR_USERNAME"),
		HarborPassword: os.Getenv("HARBOR_PASSWORD"),
	}

	if opts.Org == "" {
		opts.Org = DEFAULT_GITHUB_ORG
	}

	if opts.PageSize < 1 {
		opts.PageSize = DEFAULT_PAGE_SIZE
	}

	if opts.StartPage < 1 {
		opts.StartPage = DEFAULT_PAGE
	}

	if opts.WorkingDir == "" {
		opts.WorkingDir = DEFAULT_DOWNLOAD_PATH
	}

	return opts
}

func buildLogger(name string) hclog.Logger {
	opts := hclog.DefaultOptions
	opts.Name = name

	level := os.Getenv("LOG_LEVEL")
	if level == "" {
		opts.Level = hclog.Debug
	} else {
		opts.Level = hclog.LevelFromString(level)
	}

	return hclog.New(opts)
}

// clonePath builds the directory path that a repository should be clone to.
// The directory must not exist or go-getter will fail.
func clonePath(downloadPath, name string) string {
	return path.Join(downloadPath, name)
}
