package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#CustomerProfiles: {
	#Domain: {
		Type: "AWS::CustomerProfiles::Domain"
		Properties: {
			DeadLetterQueueUrl?:    *string | fn.#Fn
			DefaultEncryptionKey?:  *string | fn.#Fn
			DefaultExpirationDays?: *(>=1 & <=1098) | fn.#Fn
			DomainName:             *(strings.MinRunes(1) & strings.MaxRunes(64) & (=~#"^[a-zA-Z0-9_-]+$"#)) | fn.#Fn
			Tags?:                  *[...{
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
	#Integration: {
		Type: "AWS::CustomerProfiles::Integration"
		Properties: {
			DomainName:      *(strings.MinRunes(1) & strings.MaxRunes(64) & (=~#"^[a-zA-Z0-9_-]+$"#)) | fn.#Fn
			FlowDefinition?: *{
				Description?:     *(=~#"[\w!@#\-.?,\s]*"#) | fn.#Fn
				FlowName:         *(=~#"[a-zA-Z0-9][\w!@#.-]+"#) | fn.#Fn
				KmsArn:           *(strings.MinRunes(20) & strings.MaxRunes(2048) & (=~#"arn:aws:kms:.*:[0-9]+:.*"#)) | fn.#Fn
				SourceFlowConfig: *{
					ConnectorProfileName?:  *(=~#"[\w/!@#+=.-]+"#) | fn.#Fn
					ConnectorType:          *("Salesforce" | "Marketo" | "ServiceNow" | "Zendesk" | "S3") | fn.#Fn
					IncrementalPullConfig?: *{
						DatetimeTypeFieldName?: *string | fn.#Fn
					} | fn.#If
					SourceConnectorProperties: *{
						Marketo?: *{
							Object: *(=~#"\S+"#) | fn.#Fn
						} | fn.#If
						S3?: *{
							BucketName:    *(strings.MinRunes(3) & strings.MaxRunes(63) & (=~#"\S+"#)) | fn.#Fn
							BucketPrefix?: *(=~#".*"#) | fn.#Fn
						} | fn.#If
						Salesforce?: *{
							EnableDynamicFieldUpdate?: *bool | fn.#Fn
							IncludeDeletedRecords?:    *bool | fn.#Fn
							Object:                    *(=~#"\S+"#) | fn.#Fn
						} | fn.#If
						ServiceNow?: *{
							Object: *(=~#"\S+"#) | fn.#Fn
						} | fn.#If
						Zendesk?: *{
							Object: *(=~#"\S+"#) | fn.#Fn
						} | fn.#If
					} | fn.#If
				} | fn.#If
				Tasks: *[...{
					ConnectorOperator?: *{
						Marketo?:    *("PROJECTION" | "LESS_THAN" | "GREATER_THAN" | "BETWEEN" | "ADDITION" | "MULTIPLICATION" | "DIVISION" | "SUBTRACTION" | "MASK_ALL" | "MASK_FIRST_N" | "MASK_LAST_N" | "VALIDATE_NON_NULL" | "VALIDATE_NON_ZERO" | "VALIDATE_NON_NEGATIVE" | "VALIDATE_NUMERIC" | "NO_OP") | fn.#Fn
						S3?:         *("PROJECTION" | "LESS_THAN" | "GREATER_THAN" | "BETWEEN" | "LESS_THAN_OR_EQUAL_TO" | "GREATER_THAN_OR_EQUAL_TO" | "EQUAL_TO" | "NOT_EQUAL_TO" | "ADDITION" | "MULTIPLICATION" | "DIVISION" | "SUBTRACTION" | "MASK_ALL" | "MASK_FIRST_N" | "MASK_LAST_N" | "VALIDATE_NON_NULL" | "VALIDATE_NON_ZERO" | "VALIDATE_NON_NEGATIVE" | "VALIDATE_NUMERIC" | "NO_OP") | fn.#Fn
						Salesforce?: *("PROJECTION" | "LESS_THAN" | "GREATER_THAN" | "CONTAINS" | "BETWEEN" | "LESS_THAN_OR_EQUAL_TO" | "GREATER_THAN_OR_EQUAL_TO" | "EQUAL_TO" | "NOT_EQUAL_TO" | "ADDITION" | "MULTIPLICATION" | "DIVISION" | "SUBTRACTION" | "MASK_ALL" | "MASK_FIRST_N" | "MASK_LAST_N" | "VALIDATE_NON_NULL" | "VALIDATE_NON_ZERO" | "VALIDATE_NON_NEGATIVE" | "VALIDATE_NUMERIC" | "NO_OP") | fn.#Fn
						ServiceNow?: *("PROJECTION" | "LESS_THAN" | "GREATER_THAN" | "CONTAINS" | "BETWEEN" | "LESS_THAN_OR_EQUAL_TO" | "GREATER_THAN_OR_EQUAL_TO" | "EQUAL_TO" | "NOT_EQUAL_TO" | "ADDITION" | "MULTIPLICATION" | "DIVISION" | "SUBTRACTION" | "MASK_ALL" | "MASK_FIRST_N" | "MASK_LAST_N" | "VALIDATE_NON_NULL" | "VALIDATE_NON_ZERO" | "VALIDATE_NON_NEGATIVE" | "VALIDATE_NUMERIC" | "NO_OP") | fn.#Fn
						Zendesk?:    *("PROJECTION" | "GREATER_THAN" | "ADDITION" | "MULTIPLICATION" | "DIVISION" | "SUBTRACTION" | "MASK_ALL" | "MASK_FIRST_N" | "MASK_LAST_N" | "VALIDATE_NON_NULL" | "VALIDATE_NON_ZERO" | "VALIDATE_NON_NEGATIVE" | "VALIDATE_NUMERIC" | "NO_OP") | fn.#Fn
					} | fn.#If
					DestinationField?: *(=~#".*"#) | fn.#Fn
					SourceFields:      [...(*(=~#".*"#) | fn.#Fn)] | (*(=~#".*"#) | fn.#Fn)
					TaskProperties?:   *[...{
						OperatorPropertyKey: *("VALUE" | "VALUES" | "DATA_TYPE" | "UPPER_BOUND" | "LOWER_BOUND" | "SOURCE_DATA_TYPE" | "DESTINATION_DATA_TYPE" | "VALIDATION_ACTION" | "MASK_VALUE" | "MASK_LENGTH" | "TRUNCATE_LENGTH" | "MATH_OPERATION_FIELDS_ORDER" | "CONCAT_FORMAT" | "SUBFIELD_CATEGORY_MAP") | fn.#Fn
						Property:            *(=~#".+"#) | fn.#Fn
					}] | fn.#If
					TaskType: *("Arithmetic" | "Filter" | "Map" | "Mask" | "Merge" | "Truncate" | "Validate") | fn.#Fn
				}] | fn.#If
				TriggerConfig: *{
					TriggerProperties?: *{
						Scheduled?: *{
							DataPullMode?:       *("Incremental" | "Complete") | fn.#Fn
							FirstExecutionFrom?: *number | fn.#Fn
							ScheduleEndTime?:    *number | fn.#Fn
							ScheduleExpression:  *(=~#".*"#) | fn.#Fn
							ScheduleOffset?:     *int | fn.#Fn
							ScheduleStartTime?:  *number | fn.#Fn
							Timezone?:           *(=~#".*"#) | fn.#Fn
						} | fn.#If
					} | fn.#If
					TriggerType: *("Scheduled" | "Event" | "OnDemand") | fn.#Fn
				} | fn.#If
			} | fn.#If
			ObjectTypeName: *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"^[a-zA-Z_][a-zA-Z_0-9-]*$"#)) | fn.#Fn
			Tags?:          *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			Uri?: *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ObjectType: {
		Type: "AWS::CustomerProfiles::ObjectType"
		Properties: {
			AllowProfileCreation?: *bool | fn.#Fn
			Description?:          *(strings.MinRunes(1) & strings.MaxRunes(1000)) | fn.#Fn
			DomainName:            *(strings.MinRunes(1) & strings.MaxRunes(64) & (=~#"^[a-zA-Z0-9_-]+$"#)) | fn.#Fn
			EncryptionKey?:        *string | fn.#Fn
			ExpirationDays?:       *(>=1 & <=1098) | fn.#Fn
			Fields?:               *[...{
				Name?:            *(strings.MinRunes(1) & strings.MaxRunes(64) & (=~#"^[a-zA-Z0-9_-]+$"#)) | fn.#Fn
				ObjectTypeField?: *{
					ContentType?: *("STRING" | "NUMBER" | "PHONE_NUMBER" | "EMAIL_ADDRESS" | "NAME") | fn.#Fn
					Source?:      *(strings.MinRunes(1) & strings.MaxRunes(1000)) | fn.#Fn
					Target?:      *(strings.MinRunes(1) & strings.MaxRunes(1000)) | fn.#Fn
				} | fn.#If
			}] | fn.#If
			Keys?: *[...{
				Name?:              *(strings.MinRunes(1) & strings.MaxRunes(64) & (=~#"^[a-zA-Z0-9_-]+$"#)) | fn.#Fn
				ObjectTypeKeyList?: *[...{
					FieldNames?:          [...(*(strings.MinRunes(1) & strings.MaxRunes(64) & (=~#"^[a-zA-Z0-9_-]+$"#)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(64) & (=~#"^[a-zA-Z0-9_-]+$"#)) | fn.#Fn)
					StandardIdentifiers?: [...(*("PROFILE" | "UNIQUE" | "SECONDARY" | "LOOKUP_ONLY" | "NEW_ONLY") | fn.#Fn)] | (*("PROFILE" | "UNIQUE" | "SECONDARY" | "LOOKUP_ONLY" | "NEW_ONLY") | fn.#Fn)
				}] | fn.#If
			}] | fn.#If
			ObjectTypeName?: *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"^[a-zA-Z_][a-zA-Z_0-9-]*$"#)) | fn.#Fn
			Tags?:           *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			TemplateId?: *(strings.MinRunes(1) & strings.MaxRunes(64) & (=~#"^[a-zA-Z0-9_-]+$"#)) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
