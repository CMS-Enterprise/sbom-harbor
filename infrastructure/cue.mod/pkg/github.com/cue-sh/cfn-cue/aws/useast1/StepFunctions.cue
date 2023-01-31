package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#StepFunctions: {
	#Activity: {
		Type: "AWS::StepFunctions::Activity"
		Properties: {
			Name:  *(strings.MinRunes(1) & strings.MaxRunes(80)) | fn.#Fn
			Tags?: *[...{
				Key:   *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
				Value: *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#StateMachine: {
		Type: "AWS::StepFunctions::StateMachine"
		Properties: {
			Definition?:           *{} | fn.#If
			DefinitionS3Location?: *{
				Bucket:   *string | fn.#Fn
				Key:      *string | fn.#Fn
				Version?: *string | fn.#Fn
			} | fn.#If
			DefinitionString?:        *(strings.MinRunes(1) & strings.MaxRunes(1048576)) | fn.#Fn
			DefinitionSubstitutions?: *{
				[string]: *string | fn.#Fn
			} | fn.#If
			LoggingConfiguration?: *{
				Destinations?: *[...{
					CloudWatchLogsLogGroup?: *{
						LogGroupArn?: *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
					} | fn.#If
				}] | fn.#If
				IncludeExecutionData?: *bool | fn.#Fn
				Level?:                *("ALL" | "ERROR" | "FATAL" | "OFF") | fn.#Fn
			} | fn.#If
			RoleArn:           *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
			StateMachineName?: *(strings.MinRunes(1) & strings.MaxRunes(80)) | fn.#Fn
			StateMachineType?: *("STANDARD" | "EXPRESS") | fn.#Fn
			Tags?:             *[...{
				Key:   *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
				Value: *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
			}] | fn.#If
			TracingConfiguration?: *{
				Enabled?: *bool | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
