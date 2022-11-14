package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func Test_Syfter_Encode(t *testing.T) {
	workingDir := t.TempDir()

	fetcher := NewFetcher()
	fetcher.opts.Org = "anchore"
	fetcher.opts.Repo = "syft"
	fetcher.opts.WorkingDir = workingDir

	err := fetcher.FetchRepos()
	require.NoError(t, err)
	require.GreaterOrEqual(t, len(fetcher.results), 1)

	fetcher.CloneRepo(fetcher.results[0])

	result := fetcher.results[0]
	syfter := NewSyfter()
	syfter.opts.Org = "anchore"
	syfter.opts.Repo = "syft"
	syfter.opts.WorkingDir = workingDir

	syfter.Syft(result)
	require.NoError(t, result.Err)

	require.NotEmpty(t, result.SBOM)
}
