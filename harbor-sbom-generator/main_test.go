package main

import (
	"os"
	"testing"

	hclog "github.com/hashicorp/go-hclog"
	"github.com/joho/godotenv"
	"github.com/stretchr/testify/require"
)

func init() {
	godotenv.Load("../.env")
}

func Test_Main(t *testing.T) {
	require.NotEmpty(t, os.Getenv("GH_FETCH_TOKEN"))
	require.NotEmpty(t, os.Getenv("CF_DOMAIN")+"/api/v1")
	require.NotEmpty(t, os.Getenv("HARBOR_USERNAME"))
	require.NotEmpty(t, os.Getenv("HARBOR_PASSWORD"))
}

func testOpts(t *testing.T) *Opts {
	opts := NewOptsFromEnv("test")

	workingDir := t.TempDir()
	if os.Getenv("SBOM_PATH") != "" {
		workingDir = os.Getenv("SBOM_PATH")
	}

	opts.WorkingDir = workingDir
	opts.Logger.SetLevel(hclog.Debug)

	return opts
}
