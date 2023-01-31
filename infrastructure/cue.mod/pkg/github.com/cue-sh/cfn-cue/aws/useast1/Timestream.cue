package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#Timestream: {
	#Database: {
		Type: "AWS::Timestream::Database"
		Properties: {
			DatabaseName?: *(=~#"^[a-zA-Z0-9_.-]{3,64}$"#) | fn.#Fn
			KmsKeyId?:     *(strings.MinRunes(1) & strings.MaxRunes(2048)) | fn.#Fn
			Tags?:         *[...{
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
	#ScheduledQuery: {
		Type: "AWS::Timestream::ScheduledQuery"
		Properties: {
			ClientToken?:             *(strings.MinRunes(32) & strings.MaxRunes(128)) | fn.#Fn
			ErrorReportConfiguration: *{
				S3Configuration: *{
					BucketName:        *(strings.MinRunes(3) & strings.MaxRunes(63) & (=~#"[a-z0-9][\.\-a-z0-9]{1,61}[a-z0-9]"#)) | fn.#Fn
					EncryptionOption?: *("SSE_S3" | "SSE_KMS") | fn.#Fn
					ObjectKeyPrefix?:  *(strings.MinRunes(1) & strings.MaxRunes(896) & (=~#"[a-zA-Z0-9|!\-_*'\(\)]([a-zA-Z0-9]|[!\-_*'\(\)\/.])+"#)) | fn.#Fn
				} | fn.#If
			} | fn.#If
			KmsKeyId?:                 *(strings.MinRunes(1) & strings.MaxRunes(2048)) | fn.#Fn
			NotificationConfiguration: *{
				SnsConfiguration: *{
					TopicArn: *(strings.MinRunes(1) & strings.MaxRunes(2048)) | fn.#Fn
				} | fn.#If
			} | fn.#If
			QueryString:           *(strings.MinRunes(1) & strings.MaxRunes(262144)) | fn.#Fn
			ScheduleConfiguration: *{
				ScheduleExpression: *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
			} | fn.#If
			ScheduledQueryExecutionRoleArn: *(strings.MinRunes(1) & strings.MaxRunes(2048)) | fn.#Fn
			ScheduledQueryName?:            *(strings.MinRunes(1) & strings.MaxRunes(64) & (=~#"[a-zA-Z0-9_.-]+"#)) | fn.#Fn
			Tags?:                          *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			TargetConfiguration?: *{
				TimestreamConfiguration: *{
					DatabaseName:      *string | fn.#Fn
					DimensionMappings: *[...{
						DimensionValueType: *("VARCHAR") | fn.#Fn
						Name:               *string | fn.#Fn
					}] | fn.#If
					MeasureNameColumn?:    *string | fn.#Fn
					MixedMeasureMappings?: *[...{
						MeasureName?:                   *string | fn.#Fn
						MeasureValueType:               *("BIGINT" | "BOOLEAN" | "DOUBLE" | "VARCHAR" | "MULTI") | fn.#Fn
						MultiMeasureAttributeMappings?: *[...{
							MeasureValueType:                 *("BIGINT" | "BOOLEAN" | "DOUBLE" | "VARCHAR") | fn.#Fn
							SourceColumn:                     *string | fn.#Fn
							TargetMultiMeasureAttributeName?: *string | fn.#Fn
						}] | fn.#If
						SourceColumn?:      *string | fn.#Fn
						TargetMeasureName?: *string | fn.#Fn
					}] | fn.#If
					MultiMeasureMappings?: *{
						MultiMeasureAttributeMappings: *[...{
							MeasureValueType:                 *("BIGINT" | "BOOLEAN" | "DOUBLE" | "VARCHAR") | fn.#Fn
							SourceColumn:                     *string | fn.#Fn
							TargetMultiMeasureAttributeName?: *string | fn.#Fn
						}] | fn.#If
						TargetMultiMeasureName?: *string | fn.#Fn
					} | fn.#If
					TableName:  *string | fn.#Fn
					TimeColumn: *string | fn.#Fn
				} | fn.#If
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Table: {
		Type: "AWS::Timestream::Table"
		Properties: {
			DatabaseName:         *(=~#"^[a-zA-Z0-9_.-]{3,64}$"#) | fn.#Fn
			RetentionProperties?: *{
				[string]: _
			} | fn.#Fn
			TableName?: *(=~#"^[a-zA-Z0-9_.-]{3,64}$"#) | fn.#Fn
			Tags?:      *[...{
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
