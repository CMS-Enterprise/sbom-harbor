package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#FIS: {
	#ExperimentTemplate: {
		Type: "AWS::FIS::ExperimentTemplate"
		Properties: {
			Actions?: *{
				[string]: {
					ActionId:     *string | fn.#Fn
					Description?: *string | fn.#Fn
					Parameters?:  *{
						[string]: *string | fn.#Fn
					} | fn.#If
					StartAfter?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					Targets?:    *{
						[string]: *string | fn.#Fn
					} | fn.#If
				}
			} | fn.#If
			Description:    *string | fn.#Fn
			RoleArn:        *string | fn.#Fn
			StopConditions: *[...{
				Source: *string | fn.#Fn
				Value?: *(strings.MinRunes(20) & strings.MaxRunes(2048)) | fn.#Fn
			}] | fn.#If
			Tags: *{
				[string]: *string | fn.#Fn
			} | fn.#If
			Targets: *{
				[string]: {
					Filters?: *[...{
						Path:   *string | fn.#Fn
						Values: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					}] | fn.#If
					ResourceArns?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					ResourceTags?: *{
						[string]: *string | fn.#Fn
					} | fn.#If
					ResourceType:  *string | fn.#Fn
					SelectionMode: *string | fn.#Fn
				}
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
