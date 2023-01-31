package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#DevOpsGuru: {
	#NotificationChannel: {
		Type: "AWS::DevOpsGuru::NotificationChannel"
		Properties: Config: *{
			Sns?: *{
				TopicArn?: *(strings.MinRunes(36) & strings.MaxRunes(1024) & (=~#"^arn:aws[a-z0-9-]*:sns:[a-z0-9-]+:\d{12}:[^:]+$"#)) | fn.#Fn
			} | fn.#If
		} | fn.#If
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ResourceCollection: {
		Type: "AWS::DevOpsGuru::ResourceCollection"
		Properties: ResourceCollectionFilter: *{
			CloudFormation?: *{
				StackNames?: [...(*(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[a-zA-Z*]+[a-zA-Z0-9-]*$"#)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[a-zA-Z*]+[a-zA-Z0-9-]*$"#)) | fn.#Fn)
			} | fn.#If
		} | fn.#If
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
