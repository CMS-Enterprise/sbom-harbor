package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#CodeCommit: {
	#Repository: {
		Type: "AWS::CodeCommit::Repository"
		Properties: {
			Code?: *{
				BranchName?: *string | fn.#Fn
				S3:          *{
					Bucket:         *string | fn.#Fn
					Key:            *string | fn.#Fn
					ObjectVersion?: *string | fn.#Fn
				} | fn.#If
			} | fn.#If
			RepositoryDescription?: *string | fn.#Fn
			RepositoryName:         *(strings.MinRunes(1) & strings.MaxRunes(100) & (=~#"^[a-zA-Z0-9._\-]+(?<!\.git)$"#)) | fn.#Fn
			Tags?:                  *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			Triggers?: *[...{
				Branches?:      [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				CustomData?:    *string | fn.#Fn
				DestinationArn: *string | fn.#Fn
				Events:         [...(*("all" | "createReference" | "deleteReference" | "updateReference") | fn.#Fn)] | (*("all" | "createReference" | "deleteReference" | "updateReference") | fn.#Fn)
				Name:           *string | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
