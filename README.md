# SBOM Harbor

See the [Development Standards](https://aquia.atlassian.net/wiki/spaces/SBOM/pages/698908677/Development+Standards) Confluence page for standard practices.

## Requirements

This project depends on Python version: `3.9.10` and Node.js version: `^18.0.0`. The following tools are required for development:

- [Python Version Management](https://github.com/pyenv/pyenv) (pyenv)
- [AWS CDK](https://docs.aws.amazon.com/cdk/v2/guide/getting_started.html#getting_started_install)
- [Poetry](https://python-poetry.org/docs/) build tool
- Python [Pre-commit](https://pre-commit.com/).
- [Node Version Manager](https://github.com/nvm-sh/nvm) (nvm), for installing:
    - [Node.js](https://nodejs.org/en/) `^v18.0.0`
    - [npm](https://github.com/npm/cli) `v8.x`
- [Yarn](https://classic.yarnpkg.com/lang/en/docs/install) dependency manager for `node`
- [aws-vault](https://github.com/99designs/aws-vault) AWS credential manager

## Getting Started

1. `jq` command-line JSON processor by following the [`jq` Download Guide](https://stedolan.github.io/jq/download/)

2. Set up Python
    - Install Pyenv (either option):
        - Using Homebrew: `brew update && brew install pyenv`
        - From GitHub: `git clone https://github.com/pyenv/pyenv.git ~/.pyenv`
    - Install Python version: `pyenv install 3.9.10`
    - Install Poetry: `curl -sSL https://install.python-poetry.org | python3 -`

3. Set up Node.js
    - Install `nvm` (either option):
        - Using Homebrew: `brew update && brew install nvm`
        - Using the [`nvm` install script](https://github.com/nvm-sh/nvm#install--update-script)
    - Install Node version defined in the [`.nvmrc`](https://github.com/cms-enterprise/sbom-harbor/blob/main/.nvmrc) file with the latest `npm`, and enable [`corepack`](https://yarnpkg.com/getting-started/install#install-corepack) in order to use `yarn`:
        ```sh
        $ nvm install --default --latest-npm
        $ corepack enable
        $ yarn
        ```
    - Enable `corepack` to use `yarn`:
        `corepack enable` ([docs)

4. Configure virtual environment and install local dependencies:
    - Clone Repository: `git clone git@github.com:cms-enterprise/sbom-harbor.git`
    - Cd into repo: `cd cyclonedx-python`
    - Start Poetry shell: `poetry shell`. This creates your virtual environment where your dependencies can be installed.
    - Install Python dependencies: `poetry install`.
    - Install pre-commit hooks: `pre-commit install`

5. [Configure gpg for git commit signing](https://docs.github.com/en/authentication/managing-commit-signature-verification)


6. Configure AWS Credentials

    AWS credentials are managed via SSO with GSuite integration which will provide _temporary_ credentials with a 12 hour expiration. `aws-vault` is a CLI tool that makes retrieving and refreshing these temporary credentials easy and more secure by storing them in an encrypted manner (e.g.: Keychain on macOS)
    - Install `aws-vault`
        - Using Homebrew: `brew install --cask aws-vault`
    - Edit `~/.aws/config` (see [instructions in the Aquia confluence](https://aquia.atlassian.net/wiki/spaces/SBOM/pages/772276225/AWS+Config+and+Usage))

## Verify AWS Credentials

Run `aws sts get-caller-identity`

The AWS CLI will automatically execute `aws-vault` for you in the background. When a new OIDC token needs to be retrieved this will open a browser to leverage GSuite SSO. If prompted to choose a Google account, choose your `@aquia.io` account. When prompted to "Authorize request", click the orange "Allow" box.

Run `aws-vault list` to see that you have a `sso.GetRoleCredentials` session and will display how much time is left or if it has expired.

`aws-vault clear` will delete all cached temporary credentials.

`aws-vault login sandbox-vault` will open a browser window with a temporary (12 hour) session into the AWS management console.

>Note: The first time using `aws-vault` will prompt you to create a password for the "aws-vault" keychain. Subsequent uses will require re-entering this same password. When given the choice, select "Always allow". The default timeout is 5 mins which can be changed. See below

### Modify aws-vault password timeout
For macOS, open `Keychain Access`, go to `File > Add Keychain` and choose `aws-vault.keychain-db`. The "aws-vault" keychain should now appear on the left under "Custom Keychains". Right click it, and choose "Change settings...". Keep "Lock when sleeping" selected, and set "Lock after __ minutes" to something greater than 60, as anything less will be annoying.

## Developing Infrastructure (Python, AWS CDK)

`poetry run clean`

Uses Poetry to remove unnecessary artifacts.

`poetry run build`

Uses Poetry to build the python project into a single artifact.

`poetry run test`

Runs Pytest unit tests located in the tests/ folder.

`poetry run package`

Re-Packages the project and all dependencies into a zip file compatible with AWS Lambda.

`poetry run deploy`

Deploys the zip file to AWS Lambda using AWS CDK.


## Developing UX/UI (Node.js, TypeScript, Yarn)]

First, set environment variables for local development,

```sh
ENVIRONMENT=<CMS_environment_name> source ./deploy-preamble.sh
```

Then, `cd ui` to enter the `ui/` directory, and use the following commands:

`yarn`

Installs `node_modules` dependencies for the root package and each package in the `workspaces` directories defined in package.json

`yarn start`

Starts webpack-dev-server and serves the UI application on `localhost:3000` for local development.

`yarn build`

Creates a production build of the UI application.

`yarn fix`

Auto-formats code according to the `eslint` configuration.

`yarn analyze`

Generates a dependency graph of the built production bundle.

## Using the deploy shell scripts

`./deploy.sh` and `./deploy-ui.sh` scripts can be used as convenience wrappers to the `poetry` and `yarn` commands listed above.

`./deploy.sh` will use poetry to install and build python code. `-e` will include Enrichments, and `-u` will include the UI (essentially calling `./deploy-ui.sh` as a last step).

`./deploy-ui.sh` will use `yarn` to install dependencies, build static assets, and upload them to the `web-assets` S3 bucket.

**Note**: If it is not immediately obvious, since the deploy scripts described above use `poetry` you must run them from inside a `poetry` shell and you must install all dependencies (i.e. run `poetry shell` and then `poetry install`).

These scripts will _default_ to `ENVIRONMENT=e<ticket id>` using AWS credentials for the `cms-dev` environment, where the ticket id is pulled from the branch name following the convention: `ISPGCASP-<ticket id #>/<hyphen-delimited-description>`.

### Deploying to CMS

To deploy to CMS accounts, `aws-cms-oit-sbom-dev` or `aws-cms-oit-sbom-prod`, you will need access to the `sbom-application-admin` role. You will also need the [ctkey cli tool](https://cloud.cms.gov/getting-started-access-key-cli-tool) for retrieving temporary credentials, and the [AWS configuration](https://aquia.atlassian.net/wiki/spaces/SBOM/pages/772276225/AWS+Config+and+Usage).

>Note: The link above to ctkey cli must be viewed over CMS vpn.

In cases where it is necessary to manually initiate deployments to CMS, execute the deploy script like so:

CMS dev `ENVIRONMENT=dev AWS_PROFILE=cms-dev ./deploy.sh -e -u -p`
CMS prod `ENVIRONMENT=prod AWS_PROFILE=cms-prod ./deploy.sh -e -u -p`

To deploy to a temporary ephemeral environment, `e1234` you can execute the scripts like so `ENVIRONMENT=e1234 AWS_PROFILE=cms-dev ./deploy.sh -e -u -p`

### DESTROY ephemeral environments

Your automatically generated ephemeral environment would have a name matching the pattern: `^e\d{4}$`

Simply run `./destroy.sh` to delete the current branch's ephemeral environment.

If necessary you can specify a different environment:
`ENVIRONMENT=something ./destroy.sh`
