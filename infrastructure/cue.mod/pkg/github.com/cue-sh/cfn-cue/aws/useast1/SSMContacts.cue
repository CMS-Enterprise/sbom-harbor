package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#SSMContacts: {
	#Contact: {
		Type: "AWS::SSMContacts::Contact"
		Properties: {
			Alias:       *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"^[a-z0-9_\-\.]*$"#)) | fn.#Fn
			DisplayName: *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"^[a-zA-Z0-9_\-\s]*$"#)) | fn.#Fn
			Plan:        *[...{
				DurationInMinutes: *int | fn.#Fn
				Targets?:          *[...{
					ChannelTargetInfo?: *{
						ChannelId:              *string | fn.#Fn
						RetryIntervalInMinutes: *int | fn.#Fn
					} | fn.#If
					ContactTargetInfo?: *{
						ContactId:   *string | fn.#Fn
						IsEssential: *bool | fn.#Fn
					} | fn.#If
				}] | fn.#If
			}] | fn.#If
			Type: *("PERSONAL" | "CUSTOM" | "SERVICE" | "ESCALATION") | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ContactChannel: {
		Type: "AWS::SSMContacts::ContactChannel"
		Properties: {
			ChannelAddress:   *string | fn.#Fn
			ChannelName:      *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"[a-zA-Z 0-9_\-+'&amp;\uD83C-\uDBFF\uDC00-\uDFFF\u2000-\u3300]+"#)) | fn.#Fn
			ChannelType:      *("SMS" | "VOICE" | "EMAIL") | fn.#Fn
			ContactId:        *(strings.MinRunes(1) & strings.MaxRunes(2048) & (=~#"arn:[-\w+=\/,.@]+:[-\w+=\/,.@]+:[-\w+=\/,.@]*:[0-9]+:([\w+=\/,.@:-]+)*"#)) | fn.#Fn
			DeferActivation?: *bool | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
