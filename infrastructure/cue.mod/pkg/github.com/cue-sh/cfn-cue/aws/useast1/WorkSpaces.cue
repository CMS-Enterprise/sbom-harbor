package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#WorkSpaces: {
	#ConnectionAlias: {
		Type: "AWS::WorkSpaces::ConnectionAlias"
		Properties: {
			ConnectionString: *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"^[.0-9a-zA-Z\-]{1,255}$"#)) | fn.#Fn
			Tags?:            *[...{
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
	#Workspace: {
		Type: "AWS::WorkSpaces::Workspace"
		Properties: {
			BundleId:                     *string | fn.#Fn
			DirectoryId:                  *string | fn.#Fn
			RootVolumeEncryptionEnabled?: *bool | fn.#Fn
			Tags?:                        *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			UserName:                     *string | fn.#Fn
			UserVolumeEncryptionEnabled?: *bool | fn.#Fn
			VolumeEncryptionKey?:         *string | fn.#Fn
			WorkspaceProperties?:         *{
				ComputeTypeName?:                     *("GRAPHICS" | "GRAPHICSPRO" | "PERFORMANCE" | "POWER" | "POWERPRO" | "STANDARD" | "VALUE") | fn.#Fn
				RootVolumeSizeGib?:                   *int | fn.#Fn
				RunningMode?:                         *("ALWAYS_ON" | "AUTO_STOP") | fn.#Fn
				RunningModeAutoStopTimeoutInMinutes?: *int | fn.#Fn
				UserVolumeSizeGib?:                   *int | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
