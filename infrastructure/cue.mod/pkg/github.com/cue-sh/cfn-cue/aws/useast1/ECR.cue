package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#ECR: {
	#PublicRepository: {
		Type: "AWS::ECR::PublicRepository"
		Properties: {
			RepositoryCatalogData?: *{
				[string]: _
			} | fn.#Fn
			RepositoryName?:       *(strings.MinRunes(2) & strings.MaxRunes(256) & (=~#"^(?=.{2,256}$)((?:[a-z0-9]+(?:[._-][a-z0-9]+)*/)*[a-z0-9]+(?:[._-][a-z0-9]+)*)$"#)) | fn.#Fn
			RepositoryPolicyText?: *{
				[string]: _
			} | fn.#Fn
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
	#RegistryPolicy: {
		Type: "AWS::ECR::RegistryPolicy"
		Properties: PolicyText: *{
			[string]: _
		} | fn.#Fn
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ReplicationConfiguration: {
		Type: "AWS::ECR::ReplicationConfiguration"
		Properties: ReplicationConfiguration: *{
			Rules: *[...{
				Destinations: *[...{
					Region:     *(=~#"[0-9a-z-]{2,25}"#) | fn.#Fn
					RegistryId: *(=~#"^[0-9]{12}$"#) | fn.#Fn
				}] | fn.#If
				RepositoryFilters?: *[...{
					Filter:     *(=~#"^(?:[a-z0-9]+(?:[._-][a-z0-9]*)*/)*[a-z0-9]*(?:[._-][a-z0-9]*)*$"#) | fn.#Fn
					FilterType: *("PREFIX_MATCH") | fn.#Fn
				}] | fn.#If
			}] | fn.#If
		} | fn.#If
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Repository: {
		Type: "AWS::ECR::Repository"
		Properties: {
			EncryptionConfiguration?: *{
				EncryptionType: *("AES256" | "KMS") | fn.#Fn
				KmsKey?:        *(strings.MinRunes(1) & strings.MaxRunes(2048)) | fn.#Fn
			} | fn.#If
			ImageScanningConfiguration?: *{
				ScanOnPush?: *bool | fn.#Fn
			} | fn.#If
			ImageTagMutability?: *("MUTABLE" | "IMMUTABLE") | fn.#Fn
			LifecyclePolicy?:    *{
				LifecyclePolicyText?: *(strings.MinRunes(100) & strings.MaxRunes(30720)) | fn.#Fn
				RegistryId?:          *(strings.MinRunes(12) & strings.MaxRunes(12) & (=~#"^[0-9]{12}$"#)) | fn.#Fn
			} | fn.#If
			RepositoryName?:       *(strings.MinRunes(2) & strings.MaxRunes(256) & (=~#"^(?=.{2,256}$)((?:[a-z0-9]+(?:[._-][a-z0-9]+)*/)*[a-z0-9]+(?:[._-][a-z0-9]+)*)$"#)) | fn.#Fn
			RepositoryPolicyText?: *{
				[string]: _
			} | fn.#Fn
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
