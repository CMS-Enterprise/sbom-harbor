package main

import (
	"dagger.io/dagger"
	"universe.dagger.io/docker"
)

//
// Begin dagger plan (a graph of pipeline actions)
//

dagger.#Plan & {

	// the following fields satisfy dagger.#Plan
	// the client is the host on which dagger is being executed
	client: {

		commands: {
			// getSecretDbConfig: {
			//  name: "bash"
			//  flags: "-c": "aws --profile cms-dev secretsmanager get-secret-value --secret-id \(_local.secretNames.DbConfig) --query SecretString --output text"
			//  stdout: dagger.#Secret
			// }

			getSecretSnykToken: {
				name: "bash"
				flags: "-c": "aws --profile cms-dev secretsmanager get-secret-value --secret-id \(_local.secretNames.SnykToken) --query SecretString --output text"
				stdout: dagger.#Secret
			}

			getSecretIonChannelToken: {
				name: "bash"
				flags: "-c": "aws --profile cms-dev secretsmanager get-secret-value --secret-id \(_local.secretNames.IonChannelToken) --query SecretString --output text"
				stdout: dagger.#Secret
			}

			getSecretGitHubPAT: {
				name: "bash"
				flags: "-c": "aws --profile cms-dev secretsmanager get-secret-value --secret-id \(_local.secretNames.GitHubPAT) --query SecretString --output text"
				stdout: dagger.#Secret
			}

		}

		// load local files
		filesystem: ".": read: {
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

		network: "unix:///var/run/docker.sock": connect: dagger.#Socket
	} // end client:

	actions: {
		//
		// NOTE: fields prefixed with underscore (_) are hidden so as not to be directly accessible by dagger do <action>
		// this allows them to be a part of the pipeline without being excuted directly
		// 

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
						args: ["install", "curl", "ca-certificates", "unzip", "pkg-config", "libssl-dev", "git"]
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

		test: docker.#Run & {
			always: true
			input:  _files.output
			env: {
				AWS_REGION: "us-east-1"
				// DB_CONFIG:         client.commands.getSecretDbConfig.stdout & dagger.#Secret
				SNYK_TOKEN:        client.commands.getSecretSnykToken.stdout & dagger.#Secret
				ION_CHANNEL_TOKEN: client.commands.getSecretIonChannelToken.stdout & dagger.#Secret
				GITHUB_PAT:        client.commands.getSecretGitHubPAT.stdout & dagger.#Secret
				SBOM_SCORECARD:    "/usr/local/bin/scorecard"
			}
			command: {
				name: "bash"
				flags: "-c": "cargo update && cargo test"
			}
			workdir: "/src"
		} // end test:
	} // end actions:
}
