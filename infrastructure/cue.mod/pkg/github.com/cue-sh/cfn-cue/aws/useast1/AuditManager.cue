package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#AuditManager: {
	#Assessment: {
		Type: "AWS::AuditManager::Assessment"
		Properties: {
			AssessmentReportsDestination?: *{
				Destination?:     *string | fn.#Fn
				DestinationType?: *("S3") | fn.#Fn
			} | fn.#If
			AwsAccount?: *{
				EmailAddress?: *(strings.MinRunes(1) & strings.MaxRunes(320) & (=~#"^.*@.*$"#)) | fn.#Fn
				Id?:           *(strings.MinRunes(12) & strings.MaxRunes(12) & (=~#"^[0-9]{12}$"#)) | fn.#Fn
				Name?:         *(strings.MinRunes(1) & strings.MaxRunes(50) & (=~#"^[\u0020-\u007E]+$"#)) | fn.#Fn
			} | fn.#If
			Description?: *string | fn.#Fn
			FrameworkId?: *(strings.MinRunes(32) & strings.MaxRunes(36) & (=~#"^([a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}|.*\S.*)$"#)) | fn.#Fn
			Name?:        *(strings.MinRunes(1) & strings.MaxRunes(127) & (=~#"^[a-zA-Z0-9-_\.]+$"#)) | fn.#Fn
			Roles?:       *[...{
				RoleArn?:  *(strings.MinRunes(20) & strings.MaxRunes(2048) & (=~#"^arn:.*:iam:.*"#)) | fn.#Fn
				RoleType?: *("PROCESS_OWNER" | "RESOURCE_OWNER") | fn.#Fn
			}] | fn.#If
			Scope?: *{
				AwsAccounts?: *[...{
					EmailAddress?: *(strings.MinRunes(1) & strings.MaxRunes(320) & (=~#"^.*@.*$"#)) | fn.#Fn
					Id?:           *(strings.MinRunes(12) & strings.MaxRunes(12) & (=~#"^[0-9]{12}$"#)) | fn.#Fn
					Name?:         *(strings.MinRunes(1) & strings.MaxRunes(50) & (=~#"^[\u0020-\u007E]+$"#)) | fn.#Fn
				}] | fn.#If
				AwsServices?: *[...{
					ServiceName?: *string | fn.#Fn
				}] | fn.#If
			} | fn.#If
			Status?: *("ACTIVE" | "INACTIVE") | fn.#Fn
			Tags?:   *[...{
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
