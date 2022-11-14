package main

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
	"strings"
	"time"

	"github.com/hashicorp/go-hclog"
)

const JSON_CONTENT_TYPE = "application/json; charset=UTF-8"
const LOGIN_URL = "/login"
const CREATE_TEAM_URL = "/team?children=true"
const CREATE_PROJECT_URL = "/project?teamId=%s&children=true"
const UPLOAD_URL = "/%s/%s/%s/sbom" // /{teamID}/{projectID}/{codebaseID}/sbom

type Harbor struct {
	client     *http.Client
	logger     hclog.Logger
	opts       *Opts
	token      string
	team       *Team
	sbomToken  string
	ResultsMap map[string]*HarborResult
}

func NewHarbor() *Harbor {
	opts := NewOptsFromEnv("harbor")
	client := &Harbor{
		client: &http.Client{
			Timeout: 60 * time.Second,
		},
		logger:     opts.Logger,
		opts:       opts,
		ResultsMap: map[string]*HarborResult{},
	}

	return client
}

func (h *Harbor) String() string {
	var builder strings.Builder

	teamName := "Not set"
	if h.team != nil && h.team.Name != "" {
		teamName = h.team.Name
	}

	builder.WriteString(fmt.Sprintf(HARBOR_STRINGER_TEMPLATE, teamName))

	for _, result := range h.ResultsMap {
		builder.WriteString(result.String())
	}

	return builder.String()
}

// Post posts the byte slice to the provided url. If a result is provided,
// the result of the operation is captured in the result fields.
func (h *Harbor) Post(token, url string, body []byte, result *HarborResult) (*http.Response, []byte, error) {
	req, err := http.NewRequest(http.MethodPost, url, bytes.NewBuffer(body))
	if err != nil {
		result.Err = err
		return nil, nil, result.Err
	}

	req.Header.Add("Content-Type", JSON_CONTENT_TYPE)

	if token != "" {
		req.Header.Add("Authorization", token)
	}

	resp, err := h.client.Do(req)
	if err != nil {
		if result != nil {
			result.Err = err
			return nil, nil, result.Err
		}
		return nil, nil, err
	}

	defer resp.Body.Close()

	if result != nil {
		result.StatusCode = resp.StatusCode
		result.Status = resp.Status
	}

	respBody, err := io.ReadAll(resp.Body)
	if err != nil {
		err = fmt.Errorf("error reading response body from url %s: %s", url, err)
		if result != nil {
			result.Err = err
			return nil, nil, result.Err
		}
		return nil, nil, err
	}

	return resp, respBody, nil
}

func (h *Harbor) LoginURL() string {
	loginURL := fmt.Sprintf("%s%s", h.opts.APIBaseURL, LOGIN_URL)
	h.logger.Info("attempting to login to api", "login_url", loginURL)
	return loginURL
}

func (h *Harbor) Login() error {
	loginReq := &LoginRequest{
		Username: h.opts.HarborUser,
		Password: h.opts.HarborPassword,
	}

	reqBody, err := json.Marshal(loginReq)
	if err != nil {
		return fmt.Errorf("error marshalling login request: %s", err)
	}

	resp, respBody, err := h.Post("", h.LoginURL(), reqBody, nil)
	if err != nil {
		return err
	}

	if resp.StatusCode != http.StatusOK {
		return fmt.Errorf("error logging in: %s", resp.Status)
	}

	loginResp := &LoginResponse{}
	err = json.Unmarshal(respBody, &loginResp)
	if err != nil {
		return fmt.Errorf("error unmarshalling login response: %s", err)
	}

	h.token = loginResp.Token
	return nil
}

// CreateTeam creates the team entity
func (h *Harbor) CreateTeam() error {
	err := h.Login()
	if err != nil {
		h.logger.Error("error logging into harbor", "error", err)
		return err
	}

	team := &Team{
		Name:     fmt.Sprintf("%s-%s", h.opts.Org, time.Now().String()),
		Members:  []*Member{},
		Projects: []*Project{},
		Tokens:   []*Token{},
	}

	reqBody, err := json.Marshal(team)
	if err != nil {
		return fmt.Errorf("error marshalling create team request: %s", err)
	}

	resp, respBody, err := h.Post(h.token, h.CreateTeamURL(), reqBody, nil)
	if err != nil {
		return err
	}

	if resp.StatusCode != http.StatusOK {
		return fmt.Errorf("error creating team: %s", resp.Status)
	}

	err = json.Unmarshal(respBody, &team)
	if err != nil {
		return fmt.Errorf("error unmarshalling create team response: %s", err)
	}

	tokensLen := len(team.Tokens)
	if tokensLen < 1 {
		return fmt.Errorf("unexpected create team response: tokens length %d", tokensLen)
	}

	h.team = team

	for _, token := range team.Tokens {
		h.sbomToken = token.Token
		break
	}

	if h.sbomToken == "" {
		return errors.New("unexpected create team response: sbom token unset")
	}

	return nil
}

// BuildTeam is builds a team, project, and codebases for
// a GitHub Organization. This method is being left in the
// code for now in the hope that we can revert to building
// teams/projects/codebases in one operation.
func (h *Harbor) BuildTeam(results []*HarborResult) error {
	team := &Team{
		Name:     fmt.Sprintf("%s-%s", h.opts.Org, time.Now().String()),
		Members:  []*Member{},
		Projects: []*Project{},
		Tokens:   []*Token{},
	}

	for _, result := range results {
		project := &Project{
			Name:  result.Name,
			Fisma: "",
			Codebases: []*Codebase{
				{
					Name:     result.Name,
					Language: result.Language,
				},
			},
		}

		team.Projects = append(team.Projects, project)
	}

	reqBody, err := json.Marshal(team)
	if err != nil {
		return fmt.Errorf("error marshalling create team request: %s", err)
	}

	resp, respBody, err := h.Post(h.token, h.CreateTeamURL(), reqBody, nil)
	if err != nil {
		return err
	}

	if resp.StatusCode != http.StatusOK {
		return fmt.Errorf("error creating team: %s", resp.Status)
	}

	err = json.Unmarshal(respBody, &team)
	if err != nil {
		return fmt.Errorf("error unmarshalling create team response: %s", err)
	}

	tokensLen := len(team.Tokens)
	if tokensLen < 1 {
		return fmt.Errorf("unexpected create team response: tokens length %d", tokensLen)
	}

	h.team = team

	for _, token := range team.Tokens {
		h.sbomToken = token.Token
		break
	}

	if h.sbomToken == "" {
		return errors.New("unexpected create team response: sbom token unset")
	}

	projectsLen := len(team.Projects)
	if projectsLen != 1 {
		return fmt.Errorf("unexpected create team response: projects length %d", projectsLen)
	}

	codebasesLen := len(team.Projects[0].Codebases)
	if codebasesLen != 1 {
		return fmt.Errorf("unexpected create team response: codebases length %d", codebasesLen)
	}

	for _, project := range team.Projects {
		result, ok := h.ResultsMap[project.Name]
		if ok {
			result.Project = project
			result.Codebase = project.Codebases[0]
			break
		} else {
			h.logger.Trace("error setting project: key not found", "project_name", project.Name)
		}
	}

	return nil
}

func (h *Harbor) CreateTeamURL() string {
	return fmt.Sprintf("%s%s", h.opts.APIBaseURL, CREATE_TEAM_URL)
}

func (h *Harbor) BuildProject(token, sbomToken string, team *Team, result *HarborResult) {
	h.token = token
	h.sbomToken = sbomToken
	h.team = team

	// TODO: Find a way to get the build tool from the GitHub API.
	project := &Project{
		Name:  result.Name,
		Fisma: "",
		Codebases: []*Codebase{
			{
				Name:      result.Name,
				Language:  result.Language,
				BuildTool: "",
			},
		},
	}

	reqBody, err := json.Marshal(project)
	if err != nil {
		result.Err = fmt.Errorf("error marshalling create project request: %s", err)
		return
	}

	resp, respBody, err := h.Post(h.token, h.CreateProjectURL(result), reqBody, nil)
	if err != nil {
		return
	}

	if resp.StatusCode != http.StatusOK {
		result.Err = fmt.Errorf("error creating project: %s", resp.Status)
		return
	}

	err = json.Unmarshal(respBody, &project)
	if err != nil {
		result.Err = fmt.Errorf("error unmarshalling create project response: %s", err)
		return
	}

	if project.ID == "" {
		result.Err = fmt.Errorf("error creating project: invalid project_id")
		return
	}

	codebasesLen := len(project.Codebases)
	if codebasesLen != 1 {
		result.Err = fmt.Errorf("unexpected create project response: codebases length %d", codebasesLen)
		return
	}

	result.Project = project
	result.Codebase = project.Codebases[0]

	if result.Codebase.ID == "" {
		result.Err = fmt.Errorf("error creating project: invalid codebase_id")
		return
	}

	h.logger.Info("project created", "name", result.Project.Name, "project_id", result.Project.ID, "codebase_name", result.Codebase.Name, "codebase_id", result.Codebase.ID)
}

func (h *Harbor) CreateProjectURL(result *HarborResult) string {
	return fmt.Sprintf("%s%s", h.opts.APIBaseURL, fmt.Sprintf(CREATE_PROJECT_URL, result.TeamID))
}

// UploadSBOM uploads an SBOM to Harbor and tracks the status/status code of the operation.
func (h *Harbor) UploadSBOM(result *HarborResult) {
	h.logger.Info("attempting to upload", "url", h.UploadURL(result))
	_, respBody, err := h.Post(h.sbomToken, h.UploadURL(result), []byte(result.SBOM), result)
	if err != nil {
		result.Err = fmt.Errorf("error posting SMOB: %s", err)
		return
	}

	if result.StatusCode != http.StatusOK {
		result.Err = fmt.Errorf("unsuccessful SBOM upload %s: status_code %d - status %s", result.Name, result.StatusCode, result.Status)
		return
	}

	result.UploadResponse = &SBOMUploadResponse{}
	err = json.Unmarshal(respBody, result.UploadResponse)
	if err != nil {
		result.Err = fmt.Errorf("error unmarshalling SBOM upload response: %s", err)
		result.Err = err
	}
}

// UploadURL formats the urls to post SBOMs to by injecting the necessary path variables.
func (h *Harbor) UploadURL(result *HarborResult) string {
	return h.opts.APIBaseURL + fmt.Sprintf(UPLOAD_URL, h.team.ID, result.Project.ID, result.Codebase.ID)
}

func (h *Harbor) SuccessCount() int {
	count := 0

	for _, result := range h.ResultsMap {
		if result.StatusCode == http.StatusOK {
			count = count + 1
		}
	}

	return count
}

func (h *Harbor) Failures() []*HarborResult {
	var failures []*HarborResult

	for _, result := range h.ResultsMap {
		if result.StatusCode != http.StatusOK {
			failures = append(failures, result)
		}
	}

	return failures
}

type HarborResult struct {
	TeamID          string
	Name            string `json:"name"`
	CreatedAt       time.Time
	Err             error
	Project         *Project
	Codebase        *Codebase
	ContributorsURL string `json:"contributors_url"`
	GitSHA          string
	HtmlURL         string `json:"html_url"`
	IsEmpty         bool
	Language        string `json:"language"`
	SBOM            string
	Status          string
	StatusCode      int
	UploadResponse  *SBOMUploadResponse
}

func (r *HarborResult) String() string {
	projectName := "Not set"
	codebaseName := "Not set"

	if r.Project != nil && r.Project.Name != "" {
		projectName = r.Project.Name
	}

	if r.Codebase != nil && r.Codebase.Name != "" {
		codebaseName = r.Codebase.Name
	}

	return fmt.Sprintf(RESULT_STRINGER_TEMPLATE, projectName, codebaseName, r.HtmlURL, r.Language, r.StatusCode, r.Status, r.UploadResponse, r.Err)
}

// GoGetterURL generates a URL in the format that go-getter
// requires to clone repositories.
func (r *HarborResult) GoGetterURL() string {
	if r.HtmlURL == "" {
		return ""
	}

	// TODO: Verify this very naive parsing works in all cases. Depends on API result consistency.
	if strings.HasSuffix(r.HtmlURL, ".git") {
		return fmt.Sprintf("git::%s", r.HtmlURL)
	}

	return fmt.Sprintf("git::%s.git", r.HtmlURL)
}

type LoginRequest struct {
	Username string `json:"username"`
	Password string `json:"password"`
}

type LoginResponse struct {
	Token string `json:"token"`
}

type Team struct {
	ID       string     `json:"id"`
	Name     string     `json:"name"`
	Members  []*Member  `json:"members"`
	Projects []*Project `json:"projects"`
	Tokens   []*Token   `json:"tokens"`
}

type Member struct {
	ID         string `json:"id"`
	Name       string `json:"name"`
	Email      string `json:"email"`
	IsTeamLead bool   `json:"isTeamLead"`
}

type Project struct {
	ID        string      `json:"id"`
	Name      string      `json:"name"`
	Fisma     string      `json:"fisma"`
	Codebases []*Codebase `json:"codebases"`
}

type Codebase struct {
	ID        string `json:"id"`
	Name      string `json:"name"`
	Language  string `json:"language"`
	BuildTool string `json:"buildTool"`
}

type Token struct {
	ID    string `json:"id"`
	Name  string `json:"name"`
	Token string `json:"token"`
}

type SBOMUploadResponse struct {
	Valid        bool   `json:"valid"`
	S3BucketName string `json:"s3BucketName"`
	S3Objectkey  string `json:"s3ObjectKey"`
}

const HARBOR_STRINGER_TEMPLATE = `
"====================Harbor========================"
Team: 			%s\n
`

const RESULT_STRINGER_TEMPLATE = `
"====================Result========================"
Project:  		%s\n
Codebase: 		%s\n
GitURL: 		%s\n
Language:		%s\n
StatusCode:		%d\n
Status:			%s\n
UploadResponse:	%#v\n
Error: 			%s\n
`
