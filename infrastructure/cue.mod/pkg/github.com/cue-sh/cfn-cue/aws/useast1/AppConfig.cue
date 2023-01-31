package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#AppConfig: {
	#Application: {
		Type: "AWS::AppConfig::Application"
		Properties: {
			Description?: *string | fn.#Fn
			Name:         *string | fn.#Fn
			Tags?:        *[...{
				Key?:   *string | fn.#Fn
				Value?: *string | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ConfigurationProfile: {
		Type: "AWS::AppConfig::ConfigurationProfile"
		Properties: {
			ApplicationId:     *string | fn.#Fn
			Description?:      *string | fn.#Fn
			LocationUri:       *string | fn.#Fn
			Name:              *string | fn.#Fn
			RetrievalRoleArn?: *string | fn.#Fn
			Tags?:             *[...{
				Key?:   *string | fn.#Fn
				Value?: *string | fn.#Fn
			}] | fn.#If
			Type?:       *string | fn.#Fn
			Validators?: *[...{
				Content?: *string | fn.#Fn
				Type?:    *string | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Deployment: {
		Type: "AWS::AppConfig::Deployment"
		Properties: {
			ApplicationId:          *string | fn.#Fn
			ConfigurationProfileId: *string | fn.#Fn
			ConfigurationVersion:   *string | fn.#Fn
			DeploymentStrategyId:   *string | fn.#Fn
			Description?:           *string | fn.#Fn
			EnvironmentId:          *string | fn.#Fn
			Tags?:                  *[...{
				Key?:   *string | fn.#Fn
				Value?: *string | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#DeploymentStrategy: {
		Type: "AWS::AppConfig::DeploymentStrategy"
		Properties: {
			DeploymentDurationInMinutes: *number | fn.#Fn
			Description?:                *string | fn.#Fn
			FinalBakeTimeInMinutes?:     *number | fn.#Fn
			GrowthFactor:                *number | fn.#Fn
			GrowthType?:                 *string | fn.#Fn
			Name:                        *string | fn.#Fn
			ReplicateTo:                 *string | fn.#Fn
			Tags?:                       *[...{
				Key?:   *string | fn.#Fn
				Value?: *string | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Environment: {
		Type: "AWS::AppConfig::Environment"
		Properties: {
			ApplicationId: *string | fn.#Fn
			Description?:  *string | fn.#Fn
			Monitors?:     *[...{
				AlarmArn?:     *string | fn.#Fn
				AlarmRoleArn?: *string | fn.#Fn
			}] | fn.#If
			Name:  *string | fn.#Fn
			Tags?: *[...{
				Key?:   *string | fn.#Fn
				Value?: *string | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#HostedConfigurationVersion: {
		Type: "AWS::AppConfig::HostedConfigurationVersion"
		Properties: {
			ApplicationId:          *string | fn.#Fn
			ConfigurationProfileId: *string | fn.#Fn
			Content:                *string | fn.#Fn
			ContentType:            *string | fn.#Fn
			Description?:           *string | fn.#Fn
			LatestVersionNumber?:   *number | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
