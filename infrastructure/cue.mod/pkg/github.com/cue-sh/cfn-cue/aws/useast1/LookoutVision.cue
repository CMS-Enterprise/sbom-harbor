package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#LookoutVision: {
	#Project: {
		Type: "AWS::LookoutVision::Project"
		Properties: ProjectName: *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"[a-zA-Z0-9][a-zA-Z0-9_\-]*"#)) | fn.#Fn
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
