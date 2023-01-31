package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#IoTFleetHub: {
	#Application: {
		Type: "AWS::IoTFleetHub::Application"
		Properties: {
			ApplicationDescription?: *(strings.MinRunes(1) & strings.MaxRunes(2048) & (=~#"^[ -~]*$"#)) | fn.#Fn
			ApplicationName:         *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"^[ -~]*$"#)) | fn.#Fn
			RoleArn:                 *(strings.MinRunes(1) & strings.MaxRunes(1600) & (=~#"^arn:[!-~]+$"#)) | fn.#Fn
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
