package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#Kinesis: {
	#Stream: {
		Type: "AWS::Kinesis::Stream"
		Properties: {
			Name?:                 *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[a-zA-Z0-9_.-]+$"#)) | fn.#Fn
			RetentionPeriodHours?: *(>=1 & <=8760) | fn.#Fn
			ShardCount:            *(>=1 & <=100000) | fn.#Fn
			StreamEncryption?:     *{
				EncryptionType: *("KMS") | fn.#Fn
				KeyId:          *(strings.MinRunes(1) & strings.MaxRunes(2048)) | fn.#Fn
			} | fn.#If
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
	#StreamConsumer: {
		Type: "AWS::Kinesis::StreamConsumer"
		Properties: {
			ConsumerName: *string | fn.#Fn
			StreamARN:    *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
