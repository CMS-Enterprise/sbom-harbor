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
			getCmdCreds: {
				name: "get-cms-creds.sh"
				args: ["557147098836", "sbom-application-admin"]
				stdout: dagger.#Secret
			}
		}

		filesystem: {

			// load local files
			".": read: {
				contents: dagger.#FS
				exclude: [
					".git*",
					".vscode",
					"cue.mod",
					".editorconfig",
					".env*",
					"*.cue",
					"*.md",
					"target",
					"adr",
					"devenv",
					"docs",
				]
			}

		}

	} // end client:

	actions: {
		//
		// NOTE: fields prefixed with underscore (_) are hidden so as not to be directly accessible by dagger do <action>
		// this allows them to be a part of the pipeline without being excuted directly
		// 

		// convert aws-vault json to map of secrets
		_awsCredentials: core.#DecodeSecret & {
			input:  client.commands.getCmdCreds.stdout
			format: "json"
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

		// build an image with all the necessary tools for use in pipeline steps
		_image: docker.#Build & {
			steps: [
				docker.#Pull & {
					source: "rust:1.71.0"
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
						args: ["install", "zip", "curl", "ca-certificates"]
						flags: {
							"-y":                      true
							"--no-install-recommends": true
						}
					}
				},

				// install aws cli
				docker.#Run & {
					command: {
						name: "bash"
						args: ["-c", "curl https://awscli.amazonaws.com/awscli-exe-linux-\(_arch[client.platform.arch]).zip -o ./awscliv2.zip && unzip -q ./awscliv2.zip && ./aws/install"]
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

		test: {
			docker.#Run & {
				input: _files.output
				env:   _commonEnvVars.output
				command: {
					name: "aws"
					args: ["sts", "get-caller-identity"]
				}
				// export: files: "/src/cdk-outputs.json": _
			}
		}
	} // end actions:
}
