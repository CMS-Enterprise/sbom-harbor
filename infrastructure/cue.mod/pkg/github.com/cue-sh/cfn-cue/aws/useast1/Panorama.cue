package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#Panorama: {
	#ApplicationInstance: {
		Type: "AWS::Panorama::ApplicationInstance"
		Properties: {
			ApplicationInstanceIdToReplace?: *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"^[a-zA-Z0-9\-\_]+$"#)) | fn.#Fn
			DefaultRuntimeContextDevice:     *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"^[a-zA-Z0-9\-\_]+$"#)) | fn.#Fn
			Description?:                    *(=~#"^.*$"#) | fn.#Fn
			DeviceId?:                       *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"^[a-zA-Z0-9\-\_]+$"#)) | fn.#Fn
			ManifestOverridesPayload?:       *{
				PayloadData?: *(=~#"^.+$"#) | fn.#Fn
			} | fn.#If
			ManifestPayload: *{
				PayloadData?: *(strings.MinRunes(1) & strings.MaxRunes(51200) & (=~#"^.+$"#)) | fn.#Fn
			} | fn.#If
			Name?:           *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"^[a-zA-Z0-9\-\_]+$"#)) | fn.#Fn
			RuntimeRoleArn?: *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"^arn:[a-z0-9][-.a-z0-9]{0,62}:iam::[0-9]{12}:role/.+$"#)) | fn.#Fn
			StatusFilter?:   *("DEPLOYMENT_SUCCEEDED" | "DEPLOYMENT_FAILED" | "REMOVAL_SUCCEEDED" | "REMOVAL_FAILED" | "PROCESSING_DEPLOYMENT" | "PROCESSING_REMOVAL") | fn.#Fn
			Tags?:           *[...{
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
	#Package: {
		Type: "AWS::Panorama::Package"
		Properties: {
			PackageName: *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[a-zA-Z0-9\-\_]+$"#)) | fn.#Fn
			Tags?:       *[...{
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
	#PackageVersion: {
		Type: "AWS::Panorama::PackageVersion"
		Properties: {
			MarkLatest?:                *bool | fn.#Fn
			OwnerAccount?:              *(strings.MinRunes(1) & strings.MaxRunes(12) & (=~#"^[0-9a-z\_]+$"#)) | fn.#Fn
			PackageId:                  *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"^[a-zA-Z0-9\-\_\/]+$"#)) | fn.#Fn
			PackageVersion:             *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"^([0-9]+)\.([0-9]+)$"#)) | fn.#Fn
			PatchVersion:               *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"^[a-z0-9]+$"#)) | fn.#Fn
			UpdatedLatestPatchVersion?: *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"^[a-z0-9]+$"#)) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
