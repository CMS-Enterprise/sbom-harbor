package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#Amplify: {
	#App: {
		Type: "AWS::Amplify::App"
		Properties: {
			AccessToken?:              *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
			AutoBranchCreationConfig?: *{
				AutoBranchCreationPatterns?: [...(*(strings.MinRunes(1) & strings.MaxRunes(2048)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(2048)) | fn.#Fn)
				BasicAuthConfig?:            *{
					EnableBasicAuth?: *bool | fn.#Fn
					Password?:        *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
					Username?:        *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
				} | fn.#If
				BuildSpec?:                *(strings.MinRunes(1) & strings.MaxRunes(25000)) | fn.#Fn
				EnableAutoBranchCreation?: *bool | fn.#Fn
				EnableAutoBuild?:          *bool | fn.#Fn
				EnablePerformanceMode?:    *bool | fn.#Fn
				EnablePullRequestPreview?: *bool | fn.#Fn
				EnvironmentVariables?:     *[...{
					Name:  *(=~#"(?s).*"#) | fn.#Fn
					Value: *(=~#"(?s).*"#) | fn.#Fn
				}] | fn.#If
				PullRequestEnvironmentName?: *(=~#"(?s).*"#) | fn.#Fn
				Stage?:                      *("EXPERIMENTAL" | "BETA" | "PULL_REQUEST" | "PRODUCTION" | "DEVELOPMENT") | fn.#Fn
			} | fn.#If
			BasicAuthConfig?: *{
				EnableBasicAuth?: *bool | fn.#Fn
				Password?:        *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
				Username?:        *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
			} | fn.#If
			BuildSpec?:     *(strings.MinRunes(1) & strings.MaxRunes(25000) & (=~#"(?s).+"#)) | fn.#Fn
			CustomHeaders?: *(=~#"(?s).*"#) | fn.#Fn
			CustomRules?:   *[...{
				Condition?: *(=~#"(?s).*"#) | fn.#Fn
				Source:     *(strings.MinRunes(1) & strings.MaxRunes(2048) & (=~#"(?s).+"#)) | fn.#Fn
				Status?:    *(strings.MinRunes(3) & strings.MaxRunes(7) & (=~#".{3,7}"#)) | fn.#Fn
				Target:     *(strings.MinRunes(1) & strings.MaxRunes(2048) & (=~#"(?s).+"#)) | fn.#Fn
			}] | fn.#If
			Description?:              *(=~#"(?s).*"#) | fn.#Fn
			EnableBranchAutoDeletion?: *bool | fn.#Fn
			EnvironmentVariables?:     *[...{
				Name:  *(=~#"(?s).*"#) | fn.#Fn
				Value: *(=~#"(?s).*"#) | fn.#Fn
			}] | fn.#If
			IAMServiceRole?: *(strings.MinRunes(1) & strings.MaxRunes(1000) & (=~#"(?s).*"#)) | fn.#Fn
			Name:            *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"(?s).+"#)) | fn.#Fn
			OauthToken?:     *(=~#"(?s).*"#) | fn.#Fn
			Repository?:     *(=~#"(?s).*"#) | fn.#Fn
			Tags?:           *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Branch: {
		Type: "AWS::Amplify::Branch"
		Properties: {
			AppId:            *(strings.MinRunes(1) & strings.MaxRunes(20) & (=~#"d[a-z0-9]+"#)) | fn.#Fn
			BasicAuthConfig?: *{
				EnableBasicAuth?: *bool | fn.#Fn
				Password:         *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
				Username:         *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
			} | fn.#If
			BranchName:                *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"(?s).+"#)) | fn.#Fn
			BuildSpec?:                *(strings.MinRunes(1) & strings.MaxRunes(25000) & (=~#"(?s).+"#)) | fn.#Fn
			Description?:              *(=~#"(?s).*"#) | fn.#Fn
			EnableAutoBuild?:          *bool | fn.#Fn
			EnablePerformanceMode?:    *bool | fn.#Fn
			EnablePullRequestPreview?: *bool | fn.#Fn
			EnvironmentVariables?:     *[...{
				Name:  *(=~#"(?s).*"#) | fn.#Fn
				Value: *(=~#"(?s).*"#) | fn.#Fn
			}] | fn.#If
			PullRequestEnvironmentName?: *(=~#"(?s).*"#) | fn.#Fn
			Stage?:                      *("EXPERIMENTAL" | "BETA" | "PULL_REQUEST" | "PRODUCTION" | "DEVELOPMENT") | fn.#Fn
			Tags?:                       *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Domain: {
		Type: "AWS::Amplify::Domain"
		Properties: {
			AppId:                          *(strings.MinRunes(1) & strings.MaxRunes(20) & (=~#"d[a-z0-9]+"#)) | fn.#Fn
			AutoSubDomainCreationPatterns?: [...(*(strings.MinRunes(1) & strings.MaxRunes(2048) & (=~#"(?s).+"#)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(2048) & (=~#"(?s).+"#)) | fn.#Fn)
			AutoSubDomainIAMRole?:          *(=~#"^$|^arn:.+:iam::\d{12}:role.+"#) | fn.#Fn
			DomainName:                     *(=~#"^(((?!-)[A-Za-z0-9-]{0,62}[A-Za-z0-9])\.)+((?!-)[A-Za-z0-9-]{1,62}[A-Za-z0-9])(\.)?$"#) | fn.#Fn
			EnableAutoSubDomain?:           *bool | fn.#Fn
			SubDomainSettings:              *[...{
				BranchName: *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"(?s).+"#)) | fn.#Fn
				Prefix:     *(=~#"(?s).*"#) | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
