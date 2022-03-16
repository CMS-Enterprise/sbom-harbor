
#### Python version
3.9.10

#### Build Dependencies
- [Python Version Management](https://github.com/pyenv/pyenv) (pyenv)
- The [AWS CDK](https://docs.aws.amazon.com/cdk/v2/guide/getting_started.html#getting_started_install)
- The [Poetry](https://python-poetry.org/docs/) build tool
- Python [Pre-commit](https://pre-commit.com/).

#### Environment Variables
- DT_API_BASE: API endpoint url for Dependency Track.
  Example: http://localhost:8081/api
- DT_API_KEY: String associated to the project in DT that authorizes use of teh API.
  Example: thpYLdR39cUmL4718tjnFMOdnf4c3GAAPc

#### Initialize
- Install Pyenv (either option):
  - `brew update && brew install pyenv`
  - `git clone https://github.com/pyenv/pyenv.git ~/.pyenv`
- Install Python version: `pyenv install 3.9.10`
- Install Poetry: `curl -sSL https://raw.githubusercontent.com/python-poetry/poetry/master/get-poetry.py | python3`
- Clone Repository: `git clone git@github.com:aquia-inc/cyclonedx-python.git`
- Cd into repo: `cd cyclonedx-python`
- Go into Poetry shell: `poetry shell`.  This creates your virtual environment where your deps can be installed
- Python Dependencies: `poetry install`
- pre-commit hooks: `pre-commit install`

#### Build and Deploy
##### `poetry run clean`
Uses Poetry to remove unnecessary artifacts

##### `poetry run build`
Uses Poetry to build the python project into a single artifact

##### `poetry run test`
Runs Pytest unit tests located in the tests/ folder

##### `poetry run package`
Re-Packages the project and all dependencies into a zip file compatible with AWS Lambda

##### `poetry run deploy`
Deploys the zip file to AWS Lambda using AWS CDK.
