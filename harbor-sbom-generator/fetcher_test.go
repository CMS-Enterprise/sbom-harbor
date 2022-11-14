package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func newTestFetcher(t *testing.T) *Fetcher {
	fetcher := NewFetcher()
	require.NotNil(t, fetcher)

	return fetcher
}

func Test_FetchRepos(t *testing.T) {
	fetcher := newTestFetcher(t)
	fetcher.opts.Org = "anchore"

	fetcher.FetchRepos()

	require.NotNil(t, fetcher.results)
	require.Greater(t, len(fetcher.results), 0)
}

func Test_Stats(t *testing.T) {
	fetcher := newTestFetcher(t)

	fetcher.opts.Org = DEFAULT_GITHUB_ORG
	fetcher.opts.Repo = ""

	require.NotNil(t, fetcher.results)

	err := fetcher.FetchRepos()
	require.NoError(t, err)
	stats := fetcher.Stats()
	require.NotNil(t, stats)

	for language, count := range stats.Languages {
		t.Logf("language %s has count %d", language, count)
		require.Greater(t, count, 0, language)
	}
}

// Test_FetchSyftAndUpload is a full e2e test for manual
// debugging purposes. It should be skipped in CI.
func Test_FetchSyftAndUpload(t *testing.T) {
	t.Skip()

	opts := testOpts(t)
	opts.Org = "anchore"

	harbor := NewHarbor()

	err := harbor.Login()
	require.NoError(t, err)

	err = harbor.CreateTeam()
	require.NoError(t, err)

	// Get list of repos
	fetcher := NewFetcher()
	fetcher.opts = opts
	err = fetcher.FetchRepos()
	require.NoError(t, err)

	results := fetcher.results
	require.Greater(t, fetcher.results, 0)

	for _, result := range results {
		result.TeamID = harbor.team.ID
	}

	err = fetcher.CloneSyftAndUpload(harbor.token, harbor.sbomToken, harbor.team, results)
	require.NoError(t, err)
}

func Test_Fetch_Single_Repo(t *testing.T) {
	fetcher := newTestFetcher(t)
	fetcher.opts.Org = "anchore"
	fetcher.opts.Repo = "syft"

	t.Logf("cloning repos to %s", fetcher.opts.WorkingDir)

	err := fetcher.FetchRepos()
	require.NoError(t, err, err)
	require.Equal(t, len(fetcher.results), 1)
}

// Test_ListRepos retieves the list of repos for an organization.
// Created to fulfill client request.
func Test_ListRepos(t *testing.T) {
	fetcher := newTestFetcher(t)
	fetcher.opts.Org = DEFAULT_GITHUB_ORG

	err := fetcher.FetchRepos()
	require.NoError(t, err, err)
	require.Greater(t, len(fetcher.results), 0)

	repoList := ""

	for _, result := range fetcher.results {
		result.IsEmpty = fetcher.IsEmpty(result)
		require.NoError(t, result.Err)

		if !result.IsEmpty {
			repoList = repoList + result.Name + "\n"
		}
	}

	t.Log(repoList)
}
