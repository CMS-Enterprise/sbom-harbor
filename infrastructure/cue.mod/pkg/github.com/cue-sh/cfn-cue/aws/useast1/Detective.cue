package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#Detective: {
	#Graph: {
		Type: "AWS::Detective::Graph"
		Properties: Tags?: *[...{
			Key:   *string | fn.#Fn
			Value: *string | fn.#Fn
		}] | fn.#If
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#MemberInvitation: {
		Type: "AWS::Detective::MemberInvitation"
		Properties: {
			DisableEmailNotification?: *bool | fn.#Fn
			GraphArn:                  *(=~#"arn:aws(-[\w]+)*:detective:(([a-z]+-)+[0-9]+):[0-9]{12}:graph:[0-9a-f]{32}"#) | fn.#Fn
			MemberEmailAddress:        *(=~#".*@.*"#) | fn.#Fn
			MemberId:                  *(=~#"[0-9]{12}"#) | fn.#Fn
			Message?:                  *(strings.MinRunes(1) & strings.MaxRunes(1000)) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
