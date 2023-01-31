package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#SSMIncidents: {
	#ReplicationSet: {
		Type: "AWS::SSMIncidents::ReplicationSet"
		Properties: {
			DeletionProtected?: *bool | fn.#Fn
			Regions:            *[...{
				RegionConfiguration?: *{
					SseKmsKeyId: *(=~#"^arn:aws(-(cn|us-gov|iso(-b)?))?:[a-z-]+:(([a-z]+-)+[0-9])?:([0-9]{12})?:[^.]+$"#) | fn.#Fn
				} | fn.#If
				RegionName?: *string | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ResponsePlan: {
		Type: "AWS::SSMIncidents::ResponsePlan"
		Properties: {
			Actions?: *[...{
				SsmAutomation?: *{
					DocumentName:     *string | fn.#Fn
					DocumentVersion?: *string | fn.#Fn
					Parameters?:      *[...{
						Key:    *(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn
						Values: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					}] | fn.#If
					RoleArn:        *(=~#"^arn:aws(-(cn|us-gov))?:[a-z-]+:(([a-z]+-)+[0-9])?:([0-9]{12})?:[^.]+$"#) | fn.#Fn
					TargetAccount?: *("IMPACTED_ACCOUNT" | "RESPONSE_PLAN_OWNER_ACCOUNT") | fn.#Fn
				} | fn.#If
			}] | fn.#If
			ChatChannel?: *{
				ChatbotSns?: [...(*(=~#"^arn:aws(-(cn|us-gov))?:sns:(([a-z]+-)+[0-9])?:([0-9]{12})?:[^.]+$"#) | fn.#Fn)] | (*(=~#"^arn:aws(-(cn|us-gov))?:sns:(([a-z]+-)+[0-9])?:([0-9]{12})?:[^.]+$"#) | fn.#Fn)
			} | fn.#If
			DisplayName?:     *(strings.MinRunes(1) & strings.MaxRunes(200)) | fn.#Fn
			Engagements?:     [...(*(=~#"^arn:aws(-(cn|us-gov))?:ssm-contacts:(([a-z]+-)+[0-9])?:([0-9]{12})?:[^.]+$"#) | fn.#Fn)] | (*(=~#"^arn:aws(-(cn|us-gov))?:ssm-contacts:(([a-z]+-)+[0-9])?:([0-9]{12})?:[^.]+$"#) | fn.#Fn)
			IncidentTemplate: *{
				DedupeString?:        *(strings.MinRunes(1) & strings.MaxRunes(1000)) | fn.#Fn
				Impact:               *(>=1 & <=5) | fn.#Fn
				NotificationTargets?: *[...{
					SnsTopicArn?: *(=~#"^arn:aws(-(cn|us-gov))?:sns:(([a-z]+-)+[0-9])?:([0-9]{12})?:[^.]+$"#) | fn.#Fn
				}] | fn.#If
				Summary?: *(strings.MinRunes(1) & strings.MaxRunes(4000)) | fn.#Fn
				Title:    *string | fn.#Fn
			} | fn.#If
			Name:  *(strings.MinRunes(1) & strings.MaxRunes(200) & (=~#"^[a-zA-Z0-9_-]*$"#)) | fn.#Fn
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
