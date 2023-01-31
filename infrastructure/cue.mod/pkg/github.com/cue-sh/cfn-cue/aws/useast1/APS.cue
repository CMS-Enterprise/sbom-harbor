package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#APS: {
	#RuleGroupsNamespace: {
		Type: "AWS::APS::RuleGroupsNamespace"
		Properties: {
			Data:  *string | fn.#Fn
			Name:  *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
			Tags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			Workspace: *(=~#"^arn:(aws|aws-us-gov|aws-cn):aps:[a-z0-9-]+:[0-9]+:workspace/[a-zA-Z0-9-]+$"#) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Workspace: {
		Type: "AWS::APS::Workspace"
		Properties: {
			AlertManagerDefinition?: *string | fn.#Fn
			Alias?:                  *string | fn.#Fn
			Tags?:                   *[...{
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
}
