# SBOM API & UI

#### Requiremements

- Python version: 3.9.10
- Node.js version: 18.x

#### Build Dependencies

- [Python Version Management](https://github.com/pyenv/pyenv) (pyenv)
- The [AWS CDK](https://docs.aws.amazon.com/cdk/v2/guide/getting_started.html#getting_started_install)
- The [Poetry](https://python-poetry.org/docs/) build tool
- Python [Pre-commit](https://pre-commit.com/).
- [Node Version Manager](https://github.com/nvm-sh/nvm) (nvm), for installing:
    - [Node.js](https://nodejs.org/en/) v18
    - [npm](https://github.com/npm/cli) v8
- [Yarn](https://classic.yarnpkg.com/lang/en/docs/install) dependency manager for `node`
- [Lerna](https://lerna.js.org/) monorepo/multi-package management tool for `npm/yarn`

#### Environment Variables

- `DT_API_BASE`: API endpoint url for Dependency Track.
    - Example: `http://localhost:8081/api`

- `DT_API_KEY`: String associated to the project in DT that authorizes use of the API.
    - Example: `thpYLdR39cUmL4718tjnFMOdnf4c3GAAPc`

#### Initialize

1. Set up Python
    - Install Pyenv (either option):
        - `brew update && brew install pyenv`
        - `git clone https://github.com/pyenv/pyenv.git ~/.pyenv`
    - Install Python version: `pyenv install 3.9.10`
    - Install Poetry: `curl -sSL https://install.python-poetry.org | python3 -`

2. Set up Node.js
    - Install `nvm` (either option):
        - using homebrew: `brew update && brew install nvm`
        - using the nvm install script: [see documentation](https://github.com/nvm-sh/nvm#install--update-script)
    - Install Node version and update `npm`:
        ```sh
        nvm install 18
        nvm alias default 18
        nvm use 18
        npm i -g npm
        ```
    - Install Yarn and Lerna (either option):
        - `brew install yarn lerna`
        - `npm i -g yarn lerna`

3. Configure virtual envrionment and install local dependencies:
    - Clone Repository: `git clone git@github.com:aquia-inc/cyclonedx-python.git`
    - Cd into repo: `cd cyclonedx-python`
    - Start Poetry shell: `poetry shell`. This creates your virtual environment where your deps can be installed.
    - Install Python dependencies: `poetry install --ui`. Passing the `--ui` flag also installs the UI dependencies.
    - Install pre-commit hooks: `pre-commit install`

#### Build and Deploy

##### `poetry run clean`

- Uses Poetry to remove unnecessary artifacts.
- If `--ui` flag is passed, also cleans the UI artifacts and build outputs.

##### `poetry run build`

- Uses Poetry to build the python project into a single artifact.
- If `--ui` flag is passed, also builds the UI.

##### `poetry run test`

- Runs Pytest unit tests located in the tests/ folder.
- If `--ui` flag is passed, also runs Jest unit tests for the UI.

##### `poetry run package`

- Re-Packages the project and all dependencies into a zip file compatible with AWS Lambda.

##### `poetry run deploy`

- Deploys the zip file to AWS Lambda using AWS CDK.

#### Development Standards

##### Commit Messages

All commit messages must be structured to match the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/#summary)standard as follows:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

Commits with messages that do not follow this structure will fail precommit checks. See the [@commitlint/config-conventional](https://github.com/conventional-changelog/commitlint/tree/master/%40commitlint/config-conventional) package for more information.
