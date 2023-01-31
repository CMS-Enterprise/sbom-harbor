package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#HealthLake: {
	#FHIRDatastore: {
		Type: "AWS::HealthLake::FHIRDatastore"
		Properties: {
			DatastoreName?:       *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
			DatastoreTypeVersion: *("R4") | fn.#Fn
			PreloadDataConfig?:   *{
				PreloadDataType: *("SYNTHEA") | fn.#Fn
			} | fn.#If
			SseConfiguration?: *{
				KmsEncryptionConfig: *{
					CmkType:   *("CUSTOMER_MANAGED_KMS_KEY" | "AWS_OWNED_KMS_KEY") | fn.#Fn
					KmsKeyId?: *(strings.MinRunes(1) & strings.MaxRunes(400) & (=~#"(arn:aws((-us-gov)|(-iso)|(-iso-b)|(-cn))?:kms:)?([a-z]{2}-[a-z]+(-[a-z]+)?-\d:)?(\d{12}:)?(((key/)?[a-zA-Z0-9-_]+)|(alias/[a-zA-Z0-9:/_-]+))"#)) | fn.#Fn
				} | fn.#If
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
}
