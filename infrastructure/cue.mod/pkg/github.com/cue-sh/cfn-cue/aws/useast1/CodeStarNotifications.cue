package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#CodeStarNotifications: {
	#NotificationRule: {
		Type: "AWS::CodeStarNotifications::NotificationRule"
		Properties: {
			CreatedBy?:   *(strings.MinRunes(1) & strings.MaxRunes(2048)) | fn.#Fn
			DetailType:   *("BASIC" | "FULL") | fn.#Fn
			EventTypeId?: *(strings.MinRunes(1) & strings.MaxRunes(2048)) | fn.#Fn
			EventTypeIds: [...(*(strings.MinRunes(1) & strings.MaxRunes(200)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(200)) | fn.#Fn)
			Name:         *(strings.MinRunes(1) & strings.MaxRunes(64) & (=~#"[A-Za-z0-9\-_ ]+$"#)) | fn.#Fn
			Resource:     *(=~#"^arn:aws[^:\s]*:[^:\s]*:[^:\s]*:[0-9]{12}:[^\s]+$"#) | fn.#Fn
			Status?:      *("ENABLED" | "DISABLED") | fn.#Fn
			Tags?:        *{
				[string]: _
			} | fn.#Fn
			TargetAddress?: *(strings.MinRunes(1) & strings.MaxRunes(2048)) | fn.#Fn
			Targets:        *[...{
				TargetAddress: *string | fn.#Fn
				TargetType:    *string | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
