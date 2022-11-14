package main

import (
	"bytes"
	"fmt"
	"net/http"
	"os/exec"
	"time"

	hclog "github.com/hashicorp/go-hclog"
)

// TODO: Refactor to support configurable package formats once we support them.
// We can use other formats such as:
// syftjson
// cyclonedxxml
// cyclonedxjson
// github
// spdx22tagvalue
// spdx22json
// table
// text
// template

const CYCLONEDX_JSON_FORMAT = "cyclonedx-json"

type Syfter struct {
	client *http.Client
	logger hclog.Logger
	opts   *Opts
}

// NewSyfter is the factory method for a Syfter instance.
func NewSyfter() *Syfter {
	opts := NewOptsFromEnv("syfter")
	return &Syfter{
		client: &http.Client{
			Timeout: 30 * time.Second,
		},
		logger: opts.Logger,
		opts:   opts,
	}
}

// TODO: Code Review this thoroughly for accuracy of implementation and ouput.
// TODO: Replace with CLI call

// Syft creates an SBOM from a cloned repository on the filesystem.
func (s *Syfter) Syft(result *HarborResult) {
	if result.IsEmpty {
		s.logger.Info("skipping SBOM generation for empty repo", "repo_name", result.Name)
		return
	}

	if result.Err != nil {
		s.logger.Info("skipping SBOM generation for repo with clone err", "repo_name", result.Name)
		return
	}

	sourcePath := clonePath(s.opts.WorkingDir, result.Name)

	cmd := exec.Command("syft", "--output", CYCLONEDX_JSON_FORMAT, sourcePath)

	var bytes bytes.Buffer
	cmd.Stdout = &bytes

	err := cmd.Run()
	if err != nil {
		result.Err = fmt.Errorf("error encoding syft output: %s", err)
		return
	}

	result.SBOM = bytes.String()

	s.logger.Trace("syft complete", "repo_name", result.Name)

	return
}
