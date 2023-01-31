package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#AppIntegrations: {
	#EventIntegration: {
		Type: "AWS::AppIntegrations::EventIntegration"
		Properties: {
			Description?:   *(strings.MinRunes(1) & strings.MaxRunes(1000)) | fn.#Fn
			EventBridgeBus: *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"^[a-zA-Z0-9/\._\-]+$"#)) | fn.#Fn
			EventFilter:    *{
				Source: *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"^aws\.partner\/.*$"#)) | fn.#Fn
			} | fn.#If
			Name:  *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"^[a-zA-Z0-9/\._\-]+$"#)) | fn.#Fn
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
}
