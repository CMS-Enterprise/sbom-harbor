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
    - Install Python dependencies: `poetry install`.
    - Install pre-commit hooks: `pre-commit install`

5. Configure AWS Credentials
    
    AWS credentials are managed via SSO with GSuite integration which will provide _temporary_ credentials with a 12 hour expiration. `aws-vault` is a CLI tool that makes retrieving and refreshing these temporary credentials easy and more secure by storing them in an encrypted manner (e.g.: Keychain on macOS)
    - Install `aws-vault`
        - Using Homebrew: `brew install --cask aws-vault`
    - Edit `~/.aws/config` 
        - open the file and delete any existing contents
        - add the following
            ```ini
            [profile sandbox]
            sso_account_id = <YOUR ACCOUNT ID>
            sso_start_url = https://d-9267715b9a.awsapps.com/start
            sso_region = us-west-2
            sso_role_name = AWSAdministratorAccess
            region = us-west-2
            ```
            
            Insert the account ID you were provided for your personal sandbox.

            > **Note** 
            >`sso_region` and `region` are _different parameters_! `sso_region` is where the SSO instance is located, do not change this! `region` is where your resources will be deployed, so choose one close to you; `us-east-2` and `us-west-2` are recommended, as `us-east-1` is known for major outages and other wierdness.

## Getting AWS Credentials

`aws-vault exec sandbox`

This really only needs to be done every 12 hours. This will open a browser to leverage GSuite SSO. If prompted to choose a Google account, choose your `@aquia.io` account. When prompted to "Authorize request", click the orange "Allow" box, then return to your terminal where you will be in a _subshell_. Run `env | grep AWS` to see that your subshell now contains all the necessary environment variables for the `aws`, `cdk`, and python code need to build and deploy.

Run `aws-vault list` to see that you have a `sso.GetRoleCredentials` session and will display how much time is left or if it has expired.

`aws-vault clear` will delete all cached temporary credentials.

`aws-vault login sandbox` will open a browser window with a temporary (12 hour) session into the AWS management console.

>Note: The first time using `aws-vault` will prompt you to create a password for the "aws-vault" keychain. Subsequent uses will require re-entering this same password. When given the choice, select "Always allow". The default timeout is 5 mins which can be changed. See below


### Modify aws-vault password timeout
For macOS, open `Keychain Access`, go to `File > Add Keychain` and choose `aws-vault.keychain-db`. The "aws-vault" keychain should now appear on the left under "Custom Keychains". Right click it, and choose "Change settings...". Keep "Lock when sleeping" selected, and set "Lock after __ minutes" to something greater than 60, as anything less will be annoying.

### Add aws-vault session info to your terminal prompt
If you're using `zsh` + `oh-my-zsh`, the following can be added to enable a right side prompt displaying the current time left on your AWS credential session. If you already have your own custom theme, you can add this to it. If not you will need to copy the theme you're using into its own file (`<theme name>.zsh-theme`) and update the `ZSH_THEME="<theme name>"` variable to enable it.

Add the following to the top of your `<theme name>.zsh-theme` file:
```bash
prompt_aws_vault() {
  [[ $AWS_VAULT != '' ]] && echo "AWS($AWS_VAULT)"
}

aws_session_time_left() {
  if [[ $AWS_SESSION_EXPIRATION != '' ]]; then
    zulu_time_now="`date -u +'%Y-%m-%dT%H:%M:%SZ'`" # TODO: see important note above

    aws_session_expiration_epoch="`date -j -u -f '%Y-%m-%dT%H:%M:%SZ' $AWS_SESSION_EXPIRATION '+%s'`" # TODO: see important note above
    zulu_time_now_epoch="`date -j -u -f '%Y-%m-%dT%H:%M:%SZ' $zulu_time_now '+%s'`"                   # TODO: see important note above

    if [[ $zulu_time_now < $AWS_SESSION_EXPIRATION ]]; then
      secs="`expr $aws_session_expiration_epoch - $zulu_time_now_epoch`"
      echo " +`printf '%dh:%02dm:%02ds\n' $((secs/3600)) $((secs%3600/60)) $((secs%60))`"
    else
      echo "EXPIRED"
      # secs="`expr $zulu_time_now_epoch - $aws_session_expiration_epoch`"
      # echo " -`printf '%dh:%02dm:%02ds\n' $((secs/3600)) $((secs%3600/60)) $((secs%60))`"
    fi
  fi
}

RPROMPT='%{$FG[202]%}$(prompt_aws_vault)%{$reset_color%}$(aws_session_time_left)'
```
Save the theme file and `source ~/.zshrc` You should now see something like **AWS(sandbox) +11h:59m:59s** on the right side of your terminal prompt. If nothing appears it is likely you are not in an aws-vault subshell and need to run `aws-vault exec sandbox`

> **Note**
> The following sections require AWS credentials to be available as environment variables.

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

First, `cd ui` to enter the `ui/` directory.

`yarn install`

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

`./deploy.sh` will use poetry to install and build python code. `-e` will include Enrichments, and `-u` will include the UI (essentially calling `./deploy-ui.sh` as a last step)

`./deploy-ui.sh` will use `yarn` to install dependencies, build static assets, and upload them to the `web-assets` S3 bucket.
