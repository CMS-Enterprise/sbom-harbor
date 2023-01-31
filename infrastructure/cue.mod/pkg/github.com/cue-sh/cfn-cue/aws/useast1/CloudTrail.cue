package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#CloudTrail: {
	#Trail: {
		Type: "AWS::CloudTrail::Trail"
		Properties: {
			CloudWatchLogsLogGroupArn?: *string | fn.#Fn
			CloudWatchLogsRoleArn?:     *string | fn.#Fn
			EnableLogFileValidation?:   *bool | fn.#Fn
			EventSelectors?:            *[...{
				DataResources?: *[...{
					Type:    *("AWS::Lambda::Function" | "AWS::S3::Object" | "AWS::DynamoDB::Table" | "AWS::S3Outposts::Object" | "AWS::ManagedBlockchain::Node" | "AWS::S3ObjectLambda::AccessPoint" | "AWS::EC2::Snapshot" | "AWS::S3::AccessPoint" | "AWS::DynamoDB::Stream") | fn.#Fn
					Values?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				}] | fn.#If
				ExcludeManagementEventSources?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				IncludeManagementEvents?:       *bool | fn.#Fn
				ReadWriteType?:                 *("All" | "ReadOnly" | "WriteOnly") | fn.#Fn
			}] | fn.#If
			IncludeGlobalServiceEvents?: *bool | fn.#Fn
			InsightSelectors?:           *[...{
				InsightType?: *string | fn.#Fn
			}] | fn.#If
			IsLogging:            *bool | fn.#Fn
			IsMultiRegionTrail?:  *bool | fn.#Fn
			IsOrganizationTrail?: *bool | fn.#Fn
			KMSKeyId?:            *string | fn.#Fn
			S3BucketName:         *string | fn.#Fn
			S3KeyPrefix?:         *string | fn.#Fn
			SnsTopicName?:        *string | fn.#Fn
			Tags?:                *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			TrailName?: *(strings.MinRunes(3) & strings.MaxRunes(128) & (=~#"(^[a-zA-Z0-9]$)|(^[a-zA-Z0-9]([a-zA-Z0-9\._-])*[a-zA-Z0-9]$)"#)) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
