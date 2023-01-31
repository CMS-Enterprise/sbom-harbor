package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#EFS: {
	#AccessPoint: {
		Type: "AWS::EFS::AccessPoint"
		Properties: {
			AccessPointTags?: *[...{
				Key?:   *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
				Value?: *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
			}] | fn.#If
			ClientToken?: *string | fn.#Fn
			FileSystemId: *string | fn.#Fn
			PosixUser?:   *{
				Gid:            *string | fn.#Fn
				SecondaryGids?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				Uid:            *string | fn.#Fn
			} | fn.#If
			RootDirectory?: *{
				CreationInfo?: *{
					OwnerGid:    *string | fn.#Fn
					OwnerUid:    *string | fn.#Fn
					Permissions: *(=~#"^[0-7]{3,4}$"#) | fn.#Fn
				} | fn.#If
				Path?: *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#FileSystem: {
		Type: "AWS::EFS::FileSystem"
		Properties: {
			AvailabilityZoneName?: *string | fn.#Fn
			BackupPolicy?:         *{
				Status: *string | fn.#Fn
			} | fn.#If
			BypassPolicyLockoutSafetyCheck?: *bool | fn.#Fn
			Encrypted?:                      *bool | fn.#Fn
			FileSystemPolicy?:               *{
				[string]: _
			} | fn.#Fn
			FileSystemTags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			KmsKeyId?:          *string | fn.#Fn
			LifecyclePolicies?: *[...{
				TransitionToIA?:                  *("AFTER_14_DAYS" | "AFTER_30_DAYS" | "AFTER_60_DAYS" | "AFTER_7_DAYS" | "AFTER_90_DAYS") | fn.#Fn
				TransitionToPrimaryStorageClass?: *string | fn.#Fn
			}] | fn.#If
			PerformanceMode?:              *("generalPurpose" | "maxIO") | fn.#Fn
			ProvisionedThroughputInMibps?: *number | fn.#Fn
			ThroughputMode?:               *("bursting" | "provisioned") | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#MountTarget: {
		Type: "AWS::EFS::MountTarget"
		Properties: {
			FileSystemId:   *string | fn.#Fn
			IpAddress?:     *string | fn.#Fn
			SecurityGroups: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			SubnetId:       *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
