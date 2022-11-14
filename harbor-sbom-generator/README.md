# Overview

The Harbor SBOM Generator is a Lambda function that clones repositories from
GitHub, generates SBOMs, and upload the SBOMs to Harbor. The following envars
are embedded at build time and used to provide a default configuration for the Lambda.

- `GH_FETCH_TOKEN` (required) - A PAT to pass in requests to GitHub to ensure rate
  limiting rules are not trigger.
- `CF_DOMAIN` (required) - The CloudFront domain of the Harbor instance to interact with.
- `HARBOR_USERNAME` (required) - The username to use to log in to Harbor.
- `HARBOR_PASSWORD` (required) - The password to use to log in to Harbor.
- `GITHUB_ORG` (optional) - The GitHub organization to process. Defaults to `CMSgov`.
- `GITHUB_REPO` (optional) - The name of the GitHub repository to process. If not set,
  all repositories will be processed.


### Alpha 1

- [x] - Clone Repositories
- [x] - Generate SBOMs
- [x] - Create Harbor entities
- [x] - Upload to Harbor
- [x] - Basic unit tests

#### Known issues

- The alpha does not look up teams/projects/codebases to see if they already exist.
  Instead, it just creates a new team for the `GITHUB_ORG` and a project/codebase
  for each repository.
- The SBOMs are generated using Syft, and have not been validated.
- When running locally, I get a consistent number of generated SBOMs. When running in
  AWS I get a variable number, but always get more. There must be a concurrency issue
  that needs to be resolved.
- Despite the issues outlined above, I am proposing the PR now to that others can
  begin testing now.

## Roadmap

- [] - Validate generated SBOMs.
- [] - Lookup teams/projects/codebases and, if available, use existing.
- [] - Update the logic to allow passing parameters via the event to
  override environment variables.
- [] - Incorporate Git SHA & tags to add the ability to manage versioned SBOMs.
- [] - Remove `GH_FETCH_TOKEN` requirement.
- [] - Add cleanup capability to unit tests.
- [] - Add e2e tests.
- [] - Add the necessary infrastructure to support URL based invocation
  by the UI or user's CI.
- [] - Create a GitHub action and related documentation so that Harbor
  users can install the action and post to Harbor automatically.
- [] - Support configurable alternatives to Syft.

## Installing the Lambda Runtime Interface Emulator

See the [documentation](https://docs.aws.amazon.com/lambda/latest/dg/runtimes-images.html#runtimes-images-lp) for full details.

For M1/M2 macs:

```shell
mkdir -p ~/.aws-lambda-rie \
&& curl -Lo ~/.aws-lambda-rie/aws-lambda-rie \
  https://github.com/aws/aws-lambda-runtime-interface-emulator/releases/latest/download/aws-lambda-rie-arm64 \
&& chmod +x ~/.aws-lambda-rie/aws-lambda-rie
```

For other OSes see the [github repo](https://github.com/aws/aws-lambda-runtime-interface-emulator/).

## Testing locally

Running the tests locally requires that you have both `git` and `syft` installed and available on your `$PATH`.

From the `harbor-sbom-generator` directory, build the docker image.

```shell
docker build -t harbor/sbom-generator:latest .
```

Set the following environment variables in your shell or your profile.

```shell
GH_FETCH_TOKEN
CF_DOMAIN
HARBOR_USERNAME
HARBOR_PASSWORD
```

Run the docker image.
- For faster testing with a single repo, modify the following
  command to pass a `GITHUB_REPO` envar.
- To test against a different organization, modify the following
  command to pass a `GITHUB_ORG` envar.

```shell
docker run --rm --env \
    AWS_LAMBDA_FUNCTION_TIMEOUT=900 \
    --env AWS_LAMBDA_FUNCTION_MEMORY_SIZE=6016 \
    --env GH_FETCH_TOKEN=$(echo $GH_FETCH_TOKEN) \
    --env CF_DOMAIN=$(echo $CF_DOMAIN) \
    --env HARBOR_USERNAME=$(echo $HARBOR_USERNAME) \
    --env HARBOR_PASSWORD=$(echo $HARBOR_PASSWORD) \
    -d -v ~/.aws-lambda-rie:/aws-lambda --entrypoint /aws-lambda/aws-lambda-rie  \
    -p 9000:8080 harbor/sbom-generator:latest /main
```

Optionally, watch the logs in a new console.

```shell
docker logs -f <container-id/>
```

Trigger the lambda function.

```
curl -m 900 -XPOST "http://localhost:9000/2015-03-31/functions/function/invocations" -d '{}'
```

## Deploy to your sandbox.

From the *root directory of the repository* perform the following steps.

Ensure you are logged in with `aws-vault`.

```shell
aws-vault exec sandbox
```

Start a new poetry shell.

```shell
poetry shell
```

Build the python assets.

```shell
poetry run clean && poetry run build && poetry run package
```

Deploy the generator lambda.

```shell
cdk deploy sandbox-harbor-sbom-generator-use1 --require-approval "never" --concurrency 5
```

Get the ARN for the lambda in your sandbox.

```shell
GENERATOR_ARN=$(aws lambda list-functions | jq -r '.Functions[].FunctionArn' | grep SBOMGeneratorLambda)
echo $GENERATOR_ARN
```

Invoke the lambda with the `aws-cli`.

```shell
aws lambda invoke --function-name "$GENERATOR_ARN" --invocation-type Event /dev/null
```
