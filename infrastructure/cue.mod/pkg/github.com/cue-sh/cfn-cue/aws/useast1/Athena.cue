package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#Athena: {
	#DataCatalog: {
		Type: "AWS::Athena::DataCatalog"
		Properties: {
			Description?: *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
			Name:         *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
			Parameters?:  *{
				[string]: *string | fn.#Fn
			} | fn.#If
			Tags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			Type: *("LAMBDA" | "GLUE" | "HIVE") | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#NamedQuery: {
		Type: "AWS::Athena::NamedQuery"
		Properties: {
			Database:     *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
			Description?: *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
			Name?:        *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
			QueryString:  *(strings.MinRunes(1) & strings.MaxRunes(262144)) | fn.#Fn
			WorkGroup?:   *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#PreparedStatement: {
		Type: "AWS::Athena::PreparedStatement"
		Properties: {
			Description?:   *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
			QueryStatement: *(strings.MinRunes(1) & strings.MaxRunes(262144)) | fn.#Fn
			StatementName:  *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
			WorkGroup:      *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#WorkGroup: {
		Type: "AWS::Athena::WorkGroup"
		Properties: {
			Description?:           *string | fn.#Fn
			Name:                   *(=~#"[a-zA-Z0-9._-]{1,128}"#) | fn.#Fn
			RecursiveDeleteOption?: *bool | fn.#Fn
			State?:                 *("ENABLED" | "DISABLED") | fn.#Fn
			Tags?:                  *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			WorkGroupConfiguration?: *{
				BytesScannedCutoffPerQuery?:    *int | fn.#Fn
				EnforceWorkGroupConfiguration?: *bool | fn.#Fn
				EngineVersion?:                 *{
					EffectiveEngineVersion?: *string | fn.#Fn
					SelectedEngineVersion?:  *string | fn.#Fn
				} | fn.#If
				PublishCloudWatchMetricsEnabled?: *bool | fn.#Fn
				RequesterPaysEnabled?:            *bool | fn.#Fn
				ResultConfiguration?:             *{
					EncryptionConfiguration?: *{
						EncryptionOption: *("SSE_S3" | "SSE_KMS" | "CSE_KMS") | fn.#Fn
						KmsKey?:          *string | fn.#Fn
					} | fn.#If
					OutputLocation?: *string | fn.#Fn
				} | fn.#If
			} | fn.#If
			WorkGroupConfigurationUpdates?: *{
				BytesScannedCutoffPerQuery?:    *int | fn.#Fn
				EnforceWorkGroupConfiguration?: *bool | fn.#Fn
				EngineVersion?:                 *{
					EffectiveEngineVersion?: *string | fn.#Fn
					SelectedEngineVersion?:  *string | fn.#Fn
				} | fn.#If
				PublishCloudWatchMetricsEnabled?:  *bool | fn.#Fn
				RemoveBytesScannedCutoffPerQuery?: *bool | fn.#Fn
				RequesterPaysEnabled?:             *bool | fn.#Fn
				ResultConfigurationUpdates?:       *{
					EncryptionConfiguration?: *{
						EncryptionOption: *("SSE_S3" | "SSE_KMS" | "CSE_KMS") | fn.#Fn
						KmsKey?:          *string | fn.#Fn
					} | fn.#If
					OutputLocation?:                *string | fn.#Fn
					RemoveEncryptionConfiguration?: *bool | fn.#Fn
					RemoveOutputLocation?:          *bool | fn.#Fn
				} | fn.#If
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
