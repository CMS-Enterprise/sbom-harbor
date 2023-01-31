package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#SDB: {
	#Domain: {
		Type: "AWS::SDB::Domain"
		Properties: Description?: *string | fn.#Fn
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
