package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func Test_Login(t *testing.T) {
	harbor := NewHarbor()
	require.NotNil(t, harbor)
	require.NotNil(t, harbor.opts)

	err := harbor.Login()
	require.NoError(t, err)
	require.NotEmpty(t, harbor.token)
}

func Test_CreateTeam(t *testing.T) {
	opts := testOpts(t)
	opts.Org = "anchore"
	opts.Repo = "syft"

	harbor := NewHarbor()
	require.NotNil(t, harbor)

	harbor.opts = opts

	err := harbor.CreateTeam()
	require.NoError(t, err)

	require.NotNil(t, harbor.team)
	require.NotEmpty(t, harbor.team.ID)
	require.NotNil(t, harbor.team.Tokens)
	require.NotEmpty(t, harbor.sbomToken)
}
