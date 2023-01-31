package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#LookoutEquipment: {
	#InferenceScheduler: {
		Type: "AWS::LookoutEquipment::InferenceScheduler"
		Properties: {
			DataDelayOffsetInMinutes?: *int | fn.#Fn
			DataInputConfiguration:    *{
				[string]: _
			} | fn.#Fn
			DataOutputConfiguration: *{
				[string]: _
			} | fn.#Fn
			DataUploadFrequency:     *("PT5M" | "PT10M" | "PT15M" | "PT30M" | "PT1H") | fn.#Fn
			InferenceSchedulerName?: *(strings.MinRunes(1) & strings.MaxRunes(200) & (=~#"^[0-9a-zA-Z_-]{1,200}$"#)) | fn.#Fn
			ModelName:               *(strings.MinRunes(1) & strings.MaxRunes(200) & (=~#"^[0-9a-zA-Z_-]{1,200}$"#)) | fn.#Fn
			RoleArn:                 *(strings.MinRunes(20) & strings.MaxRunes(2048) & (=~#"arn:aws(-[^:]+)?:iam::[0-9]{12}:role/.+"#)) | fn.#Fn
			ServerSideKmsKeyId?:     *(strings.MinRunes(1) & strings.MaxRunes(2048) & (=~#"^[A-Za-z0-9][A-Za-z0-9:_/+=,@.-]{0,2048}$"#)) | fn.#Fn
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
