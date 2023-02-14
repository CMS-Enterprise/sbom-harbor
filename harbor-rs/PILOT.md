# Overview

Harbor Pilot is an APIGateway fronted Lambda that generates an SBOM for a repository. It is intended
to be invoked by automation such as a GitHub Action. The following payload body is expected. All
fields are required.

```json
{
  "teamId": "",
  "projectId": "",
  "codebaseId": "",
  "gitHubUrl": "https://github.com/cmsgov/ab2d-lambdas",
  "cloudFrontDomain": "",
  "apiGatewayUrl": "",
  "harborToken": ""
}
```

Any GitHub URL can be specified so long as the repository is public and not empty. The preceding example
contains a known good test repo.

## Setting up the GitHub Action

The GitHub Action is located at `harbor-rs/src/bin/pilot/harbor-pilot.yml`. To apply it to a repo, follow
these steps.

### Install the action

- Checkout the target repo and create a new branch.
- Ensure that the repo has a `.github/workflows` directory off the root directory.
- Paste a copy of `harbor-rs/src/bin/pilot/harbor-pilot.yml` in the `.github/workflows` directory.
- Commit your changes and merge the branch to `main`.
- Optionally, you could modify the branches that trigger the action by editing the following `yaml`.

```yaml
on:
  push:
    branches:
      - 'main'
```

## Add configuration to GitHub

GitHub is able to store encrypted configuration values. This feature is called [_GitHub Secrets_](https://docs.github.com/en/actions/security-guides/encrypted-secrets).
The Harbor Pilot GitHub Action relies on a secret named `HARBOR_PILOT_REQUEST` to be registered for the repo.
The Secret is a base64 encoded version of the json message discussed previously.

To navigate directly to the secrets configuration screen, update the following url and place it
in your browser address bar.

```shell
https://github.com/<your-org>/<your-repo>/settings/secrets/actions
```

### Generate a GitHub Secret Manually

Copy the JSON below and fill in the values for your environment.

```json
{
  "teamId": "<your-id>",
  "projectId": "<your-id>",
  "codebaseId": "<your-id>",
  "gitHubUrl": "<your-github-https-clone-url>",
  "cloudFrontDomain": "<your-harbor-instance-cloudfront-domain>",
  "apiGatewayUrl": "<your-harbor-instance-api-gateway-url>/api/v1/pilot",
  "harborToken": "<your-valid-sbom-upload-token>"
}
```

You should save this value as a file somewhere that is not checked into source control. You
will use this in the next step.

#### Base64 encode the secret

Since, secrets can be referenced by actions, and actions have their own special syntax that includes curly
braces, it's necessary to base64 encode the secret.

Change directory to the place you saved your request file and run the following command.
_Note_ that it assumes you named your file `pilot.json`.

```shell
cat pilot.json  | base64
```

Copy the output of the command so that you can save it in GitHub.

#### Add the secret to GitHub

Click the "New repository secret" button at the top. Name the secret `HARBOR_PILOT_REQUEST` and paste
the base64 encoded string in as the value. Then click save.

### Generate a GitHub secret automatically

For developer testing, there is a function in `tests/common/mod.rs` called `save_test_fixtures`. If you run
this function, it will create a team, project, and codebase in Harbor for you in your sandbox. It then saves
a base64 encoded secret for you in at `tests/fixtures/HARBOR_PILOT_REQUEST`. You can use the value in this
file for E2E testing your sandbox. The `save_test_fixtures` function relies on the following environment
variables being set.

- `ADMIN_USERNAME` - Used by tests to create test team, project, and codebase. Not required by GitHub Action.
- `ADMIN_PASSWORD` - Used by tests to create test team, project, and codebase. Not required by GitHub Action.
- `CF_DOMAIN` - The CloudFront Domain of the Harbor instance.
- `API_GW_URL` - The public API GW URL of the Harbor instance.
- `HARBOR_TOKEN` - Valid, not expired Harbor SBOM token.
- `HARBOR_TEST_GH_URL` - URL to a public repo that can be cloned.

Add `tests/fixtures/HARBOR_PILOT_REQUEST` as a GitHub Repository secret following the steps outlined
in the preceding section.

## Building the lambda

First, run the build script to cross-compile the binary for Amazon Linux. Details of the script can
be found at the end of this document.

```shell
./pilot-build.sh
```

Next, build a Lambda image that has the compiled binary loaded.

```shell
docker build -t harbor/pilot:latest -f Dockerfile.pilot .
```

## Deploying to Sandbox

Deployment to the sandbox is incorporated with the Ingress Stack with one key exception.
You _must_ run the `pilot-build.sh` script prior to running `deploy.sh` from the repository root.

In the future, this will be incorporated into the primary `deploy.sh` file, and this step
will not be required.

## Running Automated Tests

Open a shell and cd to the `harbor-rs` subdirectory of the repo.

```shell
cargo test
```

## E2E Testing locally

It is possible to test E2E locally using the AWS Lambda RIE. This requires you to build and
package the lambda in a container as outlined previously. Once built, you can run an instance
of the container and submit requests to it. Everytime you change your code, you will need to
rebuild the container with a new binary.

Run the lambda image in a container.

```shell
docker run --rm \
    --platform linux/amd64 \
    --name harbor-pilot \
    --rm \
    -p 9000:8080 \
    -d harbor/pilot:latest
```

Optionally, watch the docker logs in a new shell.

```shell
docker logs -f <container-id>
```

To post a test event to the container, you need to submit a json request in the API Gateway format.
To help with this, make sure you have run the `save_test_fixtures` function. It will create a valid
APIGateway V2 HTTP Request for you at `tests/fixtures/pilot-request.json`. You can use this for manual
E2E testing with in the following command.

```shell
curl -X POST \
    -H "Content-Type: application/json" \
    -d "@tests/fixtures/pilot-request.json" \
    "http://localhost:9000/2015-03-31/functions/function/invocations"
```

If you need/want to clear the generated test team, run `save_test_fixtures` found in the same file.

## E2E Testing with Sandbox

To post a test event to your sandbox run the following command.

```shell
curl -X POST \
    -H "Content-Type: application/json" \
    -d $(cat tests/fixtures/HARBOR_PILOT_REQUEST | base64 -d) \
    $API_GW_URL
```

## Build script contents

This section explains the contents of the `pilot-build.sh` script. This may be useful for
testing or debugging a local build.

The build process was adapted from the following resources.

- https://aws.amazon.com/blogs/opensource/rust-runtime-for-aws-lambda
- https://github.com/awslabs/aws-lambda-rust-runtime

### Rust toolchain

First, the script installs the rust toolchain following [these instructions](https://www.rust-lang.org/tools/install).

### Cross compile pre-reqs

Next, the script adds the compile target needed by Amazon Linux. The configuration and toolchain used
varies by OS. The Mac instructions have been tested and should work. Windows and Linux that
experience issues or identify improvements should reach out in the Slack channel.

Windows users that experience problems building with the OpenSSL dependency _should_ reference
[these instructions](https://stackoverflow.com/questions/55912871/how-to-work-with-openssl-for-rust-within-a-windows-development-environment)
for additional installation advice on Windows.

### Run builder image

Next, the script refreshes dependencies and refreshes and runs the builder image. See
`Dockerfile.pilot-builder` for details regarding the builder workflow.
