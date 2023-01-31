package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#CDK: {
	#Metadata: {
		Type: "AWS::CDK::Metadata"
		Properties: {
			Analytics?: *string | fn.#Fn
			Modules?:   *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
