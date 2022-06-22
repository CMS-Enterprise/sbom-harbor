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

## Getting Started

1. Install `jq` command-line JSON processor by following the [`jq` Download Guide](https://stedolan.github.io/jq/download/)

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
    - Install Node version defined in the [`.nvmrc`](https://github.com/aquia-inc/cyclonedx-python/blob/master/.nvmrc) file with the latest `npm`, and enable [`corepack`](https://yarnpkg.com/getting-started/install#install-corepack) in order to use `yarn`:
        ```sh
        $ nvm install --default --latest-npm
        $ corepack enable
        $ yarn
        ```
    - Enable `corepack` to use `yarn`:
        `corepack enable` ([docs)

4. Configure virtual envrionment and install local dependencies:
    - Clone Repository: `git clone git@github.com:aquia-inc/cyclonedx-python.git`
    - Cd into repo: `cd cyclonedx-python`
    - Start Poetry shell: `poetry shell`. This creates your virtual environment where your deps can be installed.
    - Install Python dependencies: `poetry install --ui`. Passing the `--ui` flag also installs the UI dependencies.
    - Install pre-commit hooks: `pre-commit install`


## Developing Infrastructure (Python, AWS CDK)

##### `poetry run clean`

Uses Poetry to remove unnecessary artifacts. If `--ui` flag is passed, also cleans the UI artifacts and build outputs.

##### `poetry run build`

Uses Poetry to build the python project into a single artifact. If `--ui` flag is passed, this also builds the UI.

##### `poetry run test`

Runs Pytest unit tests located in the tests/ folder. If `--ui` flag is passed, also runs tests for the UI.

##### `poetry run package`

Re-Packages the project and all dependencies into a zip file compatible with AWS Lambda.

##### `poetry run deploy`

Deploys the zip file to AWS Lambda using AWS CDK.


## Developing UX/UI (Node.js, TypeScript, Yarn)

##### `yarn install`

Installs `node_modules` dependencies for the root package and each package in the `workspaces` directories defined in package.json

##### `yarn start`

Starts webpack-dev-server and serves the UI application on `localhost:3000` for local development.

##### `yarn build`

Creates a production build of the UI application.

##### `yarn fix`

Auto-formats code according to the `eslint` configuration.

##### `yarn analyze`

Generates a dependency graph of the built production bundle.
