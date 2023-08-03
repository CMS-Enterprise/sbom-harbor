package main

import (
	"dagger.io/dagger"
	"dagger.io/dagger/core"
	"universe.dagger.io/docker"
)

_arch: {
	"aarch64": "aarch64"
	"arm64":   "aarch64"
	"x86_64":  "x86_64"
	"amd64":   "x86_64"
}

//
// Begin dagger plan (a graph of pipeline actions)
//

dagger.#Plan & {

	// the following fields satisfy dagger.#Plan
	// the client is the host on which dagger is being executed
	client: {

		commands: {
			getAwsCreds: {
				name: "bash"
				flags: "-c": _local.awsCredsProcess
				stdout: dagger.#Secret
			}
		}

		filesystem: {

			// load local files
			".": read: {
				contents: dagger.#FS
				include: [
					"Cargo.toml",
					"Cargo.lock",
					"api",
					"cli",
					"extensions",
					"tests",
					"sdk/core",
					"sdk/extension-template",
					"sdk/platform",
				]
			}

		}

	} // end client:

	actions: {
		//
		// NOTE: fields prefixed with underscore (_) are hidden so as not to be directly accessible by dagger do <action>
		// this allows them to be a part of the pipeline without being excuted directly
		// 

		// convert json to map of secrets
		_awsCredentials: core.#DecodeSecret & {
			input:  client.commands.getAwsCreds.stdout
			format: "json"
		}

		// build an image with all the necessary tools for use in pipeline steps
		_image: docker.#Build & {
			steps: [
				docker.#Pull & {
					source: "rust:1.71.0-slim"
				},

				docker.#Run & {
					command: {
						name: "apt-get"
						args: ["update"]
					}
				},

				// apt packages
				docker.#Run & {
					command: {
						name: "apt-get"
						args: ["install", "jq", "curl", "ca-certificates", "unzip", "pkg-config", "libssl-dev", "git"]
						flags: {
							"-y":                      true
							"--no-install-recommends": true
						}
					}
				},

				// update toolchain channels
				docker.#Run & {
					command: {
						name: "rustup"
						args: ["update"]
					}
				},

				// default to nightly channel
				docker.#Run & {
					command: {
						name: "rustup"
						args: ["default", "nightly"]
					}
				},

				// add compilation target
				docker.#Run & {
					command: {
						name: "rustup"
						args: ["target", "add", "aarch64-unknown-linux-gnu"]
					}
				},

				// install aws cli
				docker.#Run & {
					command: {
						name: "bash"
						flags: "-c": "curl -s https://awscli.amazonaws.com/awscli-exe-linux-\(_arch[client.platform.arch]).zip -o ./awscliv2.zip && unzip -q ./awscliv2.zip && ./aws/install"
					}
				},

				// install scorescard
				docker.#Run & {
					command: {
						name: "bash"
						flags: "-c": "curl -sSfL https://github.com/eBay/sbom-scorecard/releases/download/0.0.7/sbom-scorecard-linux-\(client.platform.arch) -o /usr/local/bin/scorecard && chmod u+x /usr/local/bin/scorecard && ls -la /usr/local/bin/scorecard"
					}
				},

				// install syft
				docker.#Run & {
					command: {
						name: "bash"
						flags: "-c": "curl -sSfL https://raw.githubusercontent.com/anchore/syft/main/install.sh | sh -s -- -b /usr/local/bin && chmod u+x /usr/local/bin/syft"
					}
				},

			]
		}

		// add local files  
		_files: docker.#Copy & {
			input:    _image.output
			contents: client.filesystem.".".read.contents
			dest:     "/src"
		}

		// form an object we can pass into containers as environment variables via _commonEnvVars.output
		// Nop = no operation. In other words, let this object be a part of the pipeline even though it doesnt perform an action
		_commonEnvVars: core.#Nop & {
			input: {
				AWS_REGION:             "us-east-1"
				AWS_ACCESS_KEY_ID:      _awsCredentials.output.AccessKeyId.contents & dagger.#Secret
				AWS_SECRET_ACCESS_KEY:  _awsCredentials.output.SecretAccessKey.contents & dagger.#Secret
				AWS_SESSION_TOKEN:      _awsCredentials.output.SessionToken.contents & dagger.#Secret
				AWS_SESSION_EXPIRATION: _awsCredentials.output.Expiration.contents & dagger.#Secret
			}
		}

		// get the secret from secrets manager and format
		// note that DecodeSecret only supports string values, so we use jq to convert port to string
		_getSecrets: docker.#Run & {
			input: _image.output
			env:   _commonEnvVars.output
			command: {
				name: "bash"
				flags: "-c": """
					aws secretsmanager get-secret-value --secret-id \(_local.secretNames.DbConnectionJson) --query SecretString --output text | jq -r '.port = (.port | tostring)' > DbConnectionJson; \\
					aws secretsmanager get-secret-value --secret-id \(_local.secretNames.SnykToken) --query SecretString --output text > SnykToken; \\
					aws secretsmanager get-secret-value --secret-id \(_local.secretNames.IonChannelToken) --query SecretString --output text > IonChannelToken; \\
					aws secretsmanager get-secret-value --secret-id \(_local.secretNames.GitHubPAT) --query SecretString --output text > GitHubPAT;
					"""
			}
			export: secrets: {
				DbConnectionJson: _
				SnykToken:        _
				IonChannelToken:  _
				GitHubPAT:        _
			}
		}

		_DbConnection: core.#DecodeSecret & {
			input:  _getSecrets.export.secrets.DbConnectionJson
			format: "json"
		}

		test: {
			docker.#Run & {
				always: true
				input:  _files.output
				env: {
					_commonEnvVars.output
					DB_HOST:           _DbConnection.output.host.contents
					DB_PORT:           _DbConnection.output.port.contents
					DB_USERNAME:       _DbConnection.output.username.contents
					DB_PASSWORD:       _DbConnection.output.password.contents
					DB_NAME:           _local.dbName
					SNYK_TOKEN:        _getSecrets.export.secrets.SnykToken
					ION_CHANNEL_TOKEN: _getSecrets.export.secrets.IonChannelToken
					GITHUB_PAT:        _getSecrets.export.secrets.GitHubPAT
					SBOM_SCORECARD:    "/usr/local/bin/scorecard"
				}
				command: {
					name: "bash"
					flags: "-c": "cargo update && cargo test"
				}
				workdir: "/src"
			}
		}
	} // end actions:
}
