package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#CodePipeline: {
	#CustomActionType: {
		Type: "AWS::CodePipeline::CustomActionType"
		Properties: {
			Category:                 *("Approval" | "Build" | "Deploy" | "Invoke" | "Source" | "Test") | fn.#Fn
			ConfigurationProperties?: *[...{
				Description?: *string | fn.#Fn
				Key:          *bool | fn.#Fn
				Name:         *string | fn.#Fn
				Queryable?:   *bool | fn.#Fn
				Required:     *bool | fn.#Fn
				Secret:       *bool | fn.#Fn
				Type?:        *("Boolean" | "Number" | "String") | fn.#Fn
			}] | fn.#If
			InputArtifactDetails: *{
				MaximumCount: *int | fn.#Fn
				MinimumCount: *int | fn.#Fn
			} | fn.#If
			OutputArtifactDetails: *{
				MaximumCount: *int | fn.#Fn
				MinimumCount: *int | fn.#Fn
			} | fn.#If
			Provider:  *string | fn.#Fn
			Settings?: *{
				EntityUrlTemplate?:          *string | fn.#Fn
				ExecutionUrlTemplate?:       *string | fn.#Fn
				RevisionUrlTemplate?:        *string | fn.#Fn
				ThirdPartyConfigurationUrl?: *string | fn.#Fn
			} | fn.#If
			Tags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			Version: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Pipeline: {
		Type: "AWS::CodePipeline::Pipeline"
		Properties: {
			ArtifactStore?: *{
				EncryptionKey?: *{
					Id:   *string | fn.#Fn
					Type: *string | fn.#Fn
				} | fn.#If
				Location: *string | fn.#Fn
				Type:     *("S3") | fn.#Fn
			} | fn.#If
			ArtifactStores?: *[...{
				ArtifactStore: *{
					EncryptionKey?: *{
						Id:   *string | fn.#Fn
						Type: *string | fn.#Fn
					} | fn.#If
					Location: *string | fn.#Fn
					Type:     *("S3") | fn.#Fn
				} | fn.#If
				Region: *string | fn.#Fn
			}] | fn.#If
			DisableInboundStageTransitions?: *[...{
				Reason:    *string | fn.#Fn
				StageName: *string | fn.#Fn
			}] | fn.#If
			Name?:                     *string | fn.#Fn
			RestartExecutionOnUpdate?: *bool | fn.#Fn
			RoleArn:                   *string | fn.#Fn
			Stages:                    *[...{
				Actions: *[...{
					ActionTypeId: *{
						Category: *("Approval" | "Build" | "Deploy" | "Invoke" | "Source" | "Test") | fn.#Fn
						Owner:    *("AWS" | "Custom" | "ThirdParty") | fn.#Fn
						Provider: *string | fn.#Fn
						Version:  *string | fn.#Fn
					} | fn.#If
					Configuration?: *{
						[string]: _
					} | fn.#Fn
					InputArtifacts?: *[...{
						Name: *string | fn.#Fn
					}] | fn.#If
					Name:             *string | fn.#Fn
					Namespace?:       *string | fn.#Fn
					OutputArtifacts?: *[...{
						Name: *string | fn.#Fn
					}] | fn.#If
					Region?:   *string | fn.#Fn
					RoleArn?:  *string | fn.#Fn
					RunOrder?: *int | fn.#Fn
				}] | fn.#If
				Blockers?: *[...{
					Name: *string | fn.#Fn
					Type: *("Schedule") | fn.#Fn
				}] | fn.#If
				Name: *string | fn.#Fn
			}] | fn.#If
			Tags?: *[...{
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
	#Webhook: {
		Type: "AWS::CodePipeline::Webhook"
		Properties: {
			Authentication:              *("GITHUB_HMAC" | "IP" | "UNAUTHENTICATED") | fn.#Fn
			AuthenticationConfiguration: *{
				AllowedIPRange?: *string | fn.#Fn
				SecretToken?:    *string | fn.#Fn
			} | fn.#If
			Filters: *[...{
				JsonPath:     *string | fn.#Fn
				MatchEquals?: *string | fn.#Fn
			}] | fn.#If
			Name?:                   *string | fn.#Fn
			RegisterWithThirdParty?: *bool | fn.#Fn
			TargetAction:            *string | fn.#Fn
			TargetPipeline:          *string | fn.#Fn
			TargetPipelineVersion:   *int | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
