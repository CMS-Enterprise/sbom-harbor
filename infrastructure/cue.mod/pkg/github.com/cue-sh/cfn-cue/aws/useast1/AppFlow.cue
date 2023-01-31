package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#AppFlow: {
	#ConnectorProfile: {
		Type: "AWS::AppFlow::ConnectorProfile"
		Properties: {
			ConnectionMode:          *("Public" | "Private") | fn.#Fn
			ConnectorProfileConfig?: *{
				ConnectorProfileCredentials: *{
					Amplitude?: *{
						ApiKey:    *(=~#"\S+"#) | fn.#Fn
						SecretKey: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					Datadog?: *{
						ApiKey:         *(=~#"\S+"#) | fn.#Fn
						ApplicationKey: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					Dynatrace?: *{
						ApiToken: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					GoogleAnalytics?: *{
						AccessToken?:           *(=~#"\S+"#) | fn.#Fn
						ClientId:               *(=~#"\S+"#) | fn.#Fn
						ClientSecret:           *(=~#"\S+"#) | fn.#Fn
						ConnectorOAuthRequest?: *{
							AuthCode?:    *string | fn.#Fn
							RedirectUri?: *string | fn.#Fn
						} | fn.#If
						RefreshToken?: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					InforNexus?: *{
						AccessKeyId:     *(=~#"\S+"#) | fn.#Fn
						Datakey:         *(=~#"\S+"#) | fn.#Fn
						SecretAccessKey: *(=~#"\S+"#) | fn.#Fn
						UserId:          *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					Marketo?: *{
						AccessToken?:           *(=~#"\S+"#) | fn.#Fn
						ClientId:               *(=~#"\S+"#) | fn.#Fn
						ClientSecret:           *(=~#"\S+"#) | fn.#Fn
						ConnectorOAuthRequest?: *{
							AuthCode?:    *string | fn.#Fn
							RedirectUri?: *string | fn.#Fn
						} | fn.#If
					} | fn.#If
					Redshift?: *{
						Password: *(=~#"\S+"#) | fn.#Fn
						Username: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					SAPOData?: *{
						BasicAuthCredentials?: *{
							[string]: _
						} | fn.#Fn
						OAuthCredentials?: *{
							[string]: _
						} | fn.#Fn
					} | fn.#If
					Salesforce?: *{
						AccessToken?:           *(=~#"\S+"#) | fn.#Fn
						ClientCredentialsArn?:  *(=~#"arn:aws:secretsmanager:.*:[0-9]+:.*"#) | fn.#Fn
						ConnectorOAuthRequest?: *{
							AuthCode?:    *string | fn.#Fn
							RedirectUri?: *string | fn.#Fn
						} | fn.#If
						RefreshToken?: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					ServiceNow?: *{
						Password: *(=~#"\S+"#) | fn.#Fn
						Username: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					Singular?: *{
						ApiKey: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					Slack?: *{
						AccessToken?:           *(=~#"\S+"#) | fn.#Fn
						ClientId:               *(=~#"\S+"#) | fn.#Fn
						ClientSecret:           *(=~#"\S+"#) | fn.#Fn
						ConnectorOAuthRequest?: *{
							AuthCode?:    *string | fn.#Fn
							RedirectUri?: *string | fn.#Fn
						} | fn.#If
					} | fn.#If
					Snowflake?: *{
						Password: *(=~#"\S+"#) | fn.#Fn
						Username: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					Trendmicro?: *{
						ApiSecretKey: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					Veeva?: *{
						Password: *(=~#"\S+"#) | fn.#Fn
						Username: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					Zendesk?: *{
						AccessToken?:           *(=~#"\S+"#) | fn.#Fn
						ClientId:               *(=~#"\S+"#) | fn.#Fn
						ClientSecret:           *(=~#"\S+"#) | fn.#Fn
						ConnectorOAuthRequest?: *{
							AuthCode?:    *string | fn.#Fn
							RedirectUri?: *string | fn.#Fn
						} | fn.#If
					} | fn.#If
				} | fn.#If
				ConnectorProfileProperties?: *{
					Datadog?: *{
						InstanceUrl: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					Dynatrace?: *{
						InstanceUrl: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					InforNexus?: *{
						InstanceUrl: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					Marketo?: *{
						InstanceUrl: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					Redshift?: *{
						BucketName:    *(strings.MinRunes(3) & strings.MaxRunes(63) & (=~#"\S+"#)) | fn.#Fn
						BucketPrefix?: *string | fn.#Fn
						DatabaseUrl:   *(=~#"\S+"#) | fn.#Fn
						RoleArn:       *(=~#"arn:aws:iam:.*:[0-9]+:.*"#) | fn.#Fn
					} | fn.#If
					SAPOData?: *{
						ApplicationHostUrl?:     *(=~#"^(https?)://[-a-zA-Z0-9+&amp;@#/%?=~_|!:,.;]*[-a-zA-Z0-9+&amp;@#/%=~_|]"#) | fn.#Fn
						ApplicationServicePath?: *(=~#"\S+"#) | fn.#Fn
						ClientNumber?:           *(strings.MinRunes(3) & strings.MaxRunes(3) & (=~#"^\d{3}$"#)) | fn.#Fn
						LogonLanguage?:          *(=~#"^[a-zA-Z0-9_]*$"#) | fn.#Fn
						OAuthProperties?:        *{
							AuthCodeUrl?: *(=~#"^(https?)://[-a-zA-Z0-9+&amp;@#/%?=~_|!:,.;]*[-a-zA-Z0-9+&amp;@#/%=~_|]"#) | fn.#Fn
							OAuthScopes?: [...(*(=~#"[/\w]*"#) | fn.#Fn)] | (*(=~#"[/\w]*"#) | fn.#Fn)
							TokenUrl?:    *(=~#"^(https?)://[-a-zA-Z0-9+&amp;@#/%?=~_|!:,.;]*[-a-zA-Z0-9+&amp;@#/%=~_|]"#) | fn.#Fn
						} | fn.#If
						PortNumber?:             *(>=1 & <=65535) | fn.#Fn
						PrivateLinkServiceName?: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					Salesforce?: *{
						InstanceUrl?:          *(=~#"\S+"#) | fn.#Fn
						isSandboxEnvironment?: *bool | fn.#Fn
					} | fn.#If
					ServiceNow?: *{
						InstanceUrl: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					Slack?: *{
						InstanceUrl: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					Snowflake?: *{
						AccountName?:            *(=~#"\S+"#) | fn.#Fn
						BucketName:              *(strings.MinRunes(3) & strings.MaxRunes(63) & (=~#"\S+"#)) | fn.#Fn
						BucketPrefix?:           *string | fn.#Fn
						PrivateLinkServiceName?: *(=~#"\S+"#) | fn.#Fn
						Region?:                 *(=~#"\S+"#) | fn.#Fn
						Stage:                   *(=~#"\S+"#) | fn.#Fn
						Warehouse:               *(=~#"[\s\w/!@#+=.-]*"#) | fn.#Fn
					} | fn.#If
					Veeva?: *{
						InstanceUrl: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					Zendesk?: *{
						InstanceUrl: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
				} | fn.#If
			} | fn.#If
			ConnectorProfileName: *(=~#"[\w/!@#+=.-]+"#) | fn.#Fn
			ConnectorType:        *("Salesforce" | "Singular" | "Slack" | "Redshift" | "Marketo" | "Googleanalytics" | "Zendesk" | "Servicenow" | "SAPOData" | "Datadog" | "Trendmicro" | "Snowflake" | "Dynatrace" | "Infornexus" | "Amplitude" | "Veeva") | fn.#Fn
			KMSArn?:              *(strings.MinRunes(20) & strings.MaxRunes(2048) & (=~#"arn:aws:kms:.*:[0-9]+:.*"#)) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Flow: {
		Type: "AWS::AppFlow::Flow"
		Properties: {
			Description?:              *(=~#"[\w!@#\-.?,\s]*"#) | fn.#Fn
			DestinationFlowConfigList: *[...{
				ConnectorProfileName?:          *(=~#"[\w/!@#+=.-]+"#) | fn.#Fn
				ConnectorType:                  *("SAPOData" | "Salesforce" | "Singular" | "Slack" | "Redshift" | "S3" | "Marketo" | "Googleanalytics" | "Zendesk" | "Servicenow" | "Datadog" | "Trendmicro" | "Snowflake" | "Dynatrace" | "Infornexus" | "Amplitude" | "Veeva" | "EventBridge" | "Upsolver" | "LookoutMetrics") | fn.#Fn
				DestinationConnectorProperties: *{
					EventBridge?: *{
						ErrorHandlingConfig?: *{
							BucketName?:       *(strings.MinRunes(3) & strings.MaxRunes(63) & (=~#"\S+"#)) | fn.#Fn
							BucketPrefix?:     *string | fn.#Fn
							FailOnFirstError?: *bool | fn.#Fn
						} | fn.#If
						Object: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					LookoutMetrics?: *{
						Object?: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					Redshift?: *{
						BucketPrefix?:        *string | fn.#Fn
						ErrorHandlingConfig?: *{
							BucketName?:       *(strings.MinRunes(3) & strings.MaxRunes(63) & (=~#"\S+"#)) | fn.#Fn
							BucketPrefix?:     *string | fn.#Fn
							FailOnFirstError?: *bool | fn.#Fn
						} | fn.#If
						IntermediateBucketName: *(strings.MinRunes(3) & strings.MaxRunes(63) & (=~#"\S+"#)) | fn.#Fn
						Object:                 *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					S3?: *{
						BucketName:            *(strings.MinRunes(3) & strings.MaxRunes(63) & (=~#"\S+"#)) | fn.#Fn
						BucketPrefix?:         *string | fn.#Fn
						S3OutputFormatConfig?: *{
							AggregationConfig?: *{
								AggregationType?: *("None" | "SingleFile") | fn.#Fn
							} | fn.#If
							FileType?:     *("CSV" | "JSON" | "PARQUET") | fn.#Fn
							PrefixConfig?: *{
								PrefixFormat?: *("YEAR" | "MONTH" | "DAY" | "HOUR" | "MINUTE") | fn.#Fn
								PrefixType?:   *("FILENAME" | "PATH" | "PATH_AND_FILENAME") | fn.#Fn
							} | fn.#If
						} | fn.#If
					} | fn.#If
					Salesforce?: *{
						ErrorHandlingConfig?: *{
							BucketName?:       *(strings.MinRunes(3) & strings.MaxRunes(63) & (=~#"\S+"#)) | fn.#Fn
							BucketPrefix?:     *string | fn.#Fn
							FailOnFirstError?: *bool | fn.#Fn
						} | fn.#If
						IdFieldNames?:       [...(*string | fn.#Fn)] | (*string | fn.#Fn)
						Object:              *(=~#"\S+"#) | fn.#Fn
						WriteOperationType?: *("INSERT" | "UPSERT" | "UPDATE") | fn.#Fn
					} | fn.#If
					Snowflake?: *{
						BucketPrefix?:        *string | fn.#Fn
						ErrorHandlingConfig?: *{
							BucketName?:       *(strings.MinRunes(3) & strings.MaxRunes(63) & (=~#"\S+"#)) | fn.#Fn
							BucketPrefix?:     *string | fn.#Fn
							FailOnFirstError?: *bool | fn.#Fn
						} | fn.#If
						IntermediateBucketName: *(strings.MinRunes(3) & strings.MaxRunes(63) & (=~#"\S+"#)) | fn.#Fn
						Object:                 *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					Upsolver?: *{
						BucketName:           *(strings.MinRunes(16) & strings.MaxRunes(63) & (=~#"^(upsolver-appflow)\S*"#)) | fn.#Fn
						BucketPrefix?:        *string | fn.#Fn
						S3OutputFormatConfig: *{
							AggregationConfig?: *{
								AggregationType?: *("None" | "SingleFile") | fn.#Fn
							} | fn.#If
							FileType?:    *("CSV" | "JSON" | "PARQUET") | fn.#Fn
							PrefixConfig: *{
								PrefixFormat?: *("YEAR" | "MONTH" | "DAY" | "HOUR" | "MINUTE") | fn.#Fn
								PrefixType?:   *("FILENAME" | "PATH" | "PATH_AND_FILENAME") | fn.#Fn
							} | fn.#If
						} | fn.#If
					} | fn.#If
					Zendesk?: *{
						ErrorHandlingConfig?: *{
							BucketName?:       *(strings.MinRunes(3) & strings.MaxRunes(63) & (=~#"\S+"#)) | fn.#Fn
							BucketPrefix?:     *string | fn.#Fn
							FailOnFirstError?: *bool | fn.#Fn
						} | fn.#If
						IdFieldNames?:       [...(*string | fn.#Fn)] | (*string | fn.#Fn)
						Object:              *(=~#"\S+"#) | fn.#Fn
						WriteOperationType?: *("INSERT" | "UPSERT" | "UPDATE") | fn.#Fn
					} | fn.#If
				} | fn.#If
			}] | fn.#If
			FlowName:         *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"[a-zA-Z0-9][\w!@#.-]+"#)) | fn.#Fn
			KMSArn?:          *(strings.MinRunes(20) & strings.MaxRunes(2048) & (=~#"arn:aws:kms:.*:[0-9]+:.*"#)) | fn.#Fn
			SourceFlowConfig: *{
				ConnectorProfileName?:  *(=~#"[\w/!@#+=.-]+"#) | fn.#Fn
				ConnectorType:          *("SAPOData" | "Salesforce" | "Singular" | "Slack" | "Redshift" | "S3" | "Marketo" | "Googleanalytics" | "Zendesk" | "Servicenow" | "Datadog" | "Trendmicro" | "Snowflake" | "Dynatrace" | "Infornexus" | "Amplitude" | "Veeva" | "EventBridge" | "Upsolver" | "LookoutMetrics") | fn.#Fn
				IncrementalPullConfig?: *{
					DatetimeTypeFieldName?: *string | fn.#Fn
				} | fn.#If
				SourceConnectorProperties: *{
					Amplitude?: *{
						Object: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					Datadog?: *{
						Object: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					Dynatrace?: *{
						Object: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					GoogleAnalytics?: *{
						Object: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					InforNexus?: *{
						Object: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					Marketo?: *{
						Object: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					S3?: *{
						BucketName:           *(strings.MinRunes(3) & strings.MaxRunes(63) & (=~#"\S+"#)) | fn.#Fn
						BucketPrefix:         *string | fn.#Fn
						S3InputFormatConfig?: *{
							S3InputFileType?: *("CSV" | "JSON") | fn.#Fn
						} | fn.#If
					} | fn.#If
					SAPOData?: *{
						ObjectPath: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					Salesforce?: *{
						EnableDynamicFieldUpdate?: *bool | fn.#Fn
						IncludeDeletedRecords?:    *bool | fn.#Fn
						Object:                    *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					ServiceNow?: *{
						Object: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					Singular?: *{
						Object: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					Slack?: *{
						Object: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					Trendmicro?: *{
						Object: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					Veeva?: *{
						DocumentType?:       *(=~#"[\s\w_-]+"#) | fn.#Fn
						IncludeAllVersions?: *bool | fn.#Fn
						IncludeRenditions?:  *bool | fn.#Fn
						IncludeSourceFiles?: *bool | fn.#Fn
						Object:              *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
					Zendesk?: *{
						Object: *(=~#"\S+"#) | fn.#Fn
					} | fn.#If
				} | fn.#If
			} | fn.#If
			Tags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			Tasks: *[...{
				ConnectorOperator?: *{
					Amplitude?:       *("BETWEEN") | fn.#Fn
					Datadog?:         *("PROJECTION" | "BETWEEN" | "EQUAL_TO" | "ADDITION" | "MULTIPLICATION" | "DIVISION" | "SUBTRACTION" | "MASK_ALL" | "MASK_FIRST_N" | "MASK_LAST_N" | "VALIDATE_NON_NULL" | "VALIDATE_NON_ZERO" | "VALIDATE_NON_NEGATIVE" | "VALIDATE_NUMERIC" | "NO_OP") | fn.#Fn
					Dynatrace?:       *("PROJECTION" | "BETWEEN" | "EQUAL_TO" | "ADDITION" | "MULTIPLICATION" | "DIVISION" | "SUBTRACTION" | "MASK_ALL" | "MASK_FIRST_N" | "MASK_LAST_N" | "VALIDATE_NON_NULL" | "VALIDATE_NON_ZERO" | "VALIDATE_NON_NEGATIVE" | "VALIDATE_NUMERIC" | "NO_OP") | fn.#Fn
					GoogleAnalytics?: *("PROJECTION" | "BETWEEN") | fn.#Fn
					InforNexus?:      *("PROJECTION" | "BETWEEN" | "EQUAL_TO" | "ADDITION" | "MULTIPLICATION" | "DIVISION" | "SUBTRACTION" | "MASK_ALL" | "MASK_FIRST_N" | "MASK_LAST_N" | "VALIDATE_NON_NULL" | "VALIDATE_NON_ZERO" | "VALIDATE_NON_NEGATIVE" | "VALIDATE_NUMERIC" | "NO_OP") | fn.#Fn
					Marketo?:         *("PROJECTION" | "LESS_THAN" | "GREATER_THAN" | "BETWEEN" | "ADDITION" | "MULTIPLICATION" | "DIVISION" | "SUBTRACTION" | "MASK_ALL" | "MASK_FIRST_N" | "MASK_LAST_N" | "VALIDATE_NON_NULL" | "VALIDATE_NON_ZERO" | "VALIDATE_NON_NEGATIVE" | "VALIDATE_NUMERIC" | "NO_OP") | fn.#Fn
					S3?:              *("PROJECTION" | "LESS_THAN" | "GREATER_THAN" | "BETWEEN" | "LESS_THAN_OR_EQUAL_TO" | "GREATER_THAN_OR_EQUAL_TO" | "EQUAL_TO" | "NOT_EQUAL_TO" | "ADDITION" | "MULTIPLICATION" | "DIVISION" | "SUBTRACTION" | "MASK_ALL" | "MASK_FIRST_N" | "MASK_LAST_N" | "VALIDATE_NON_NULL" | "VALIDATE_NON_ZERO" | "VALIDATE_NON_NEGATIVE" | "VALIDATE_NUMERIC" | "NO_OP") | fn.#Fn
					SAPOData?:        *("PROJECTION" | "LESS_THAN" | "CONTAINS" | "GREATER_THAN" | "BETWEEN" | "LESS_THAN_OR_EQUAL_TO" | "GREATER_THAN_OR_EQUAL_TO" | "EQUAL_TO" | "NOT_EQUAL_TO" | "ADDITION" | "MULTIPLICATION" | "DIVISION" | "SUBTRACTION" | "MASK_ALL" | "MASK_FIRST_N" | "MASK_LAST_N" | "VALIDATE_NON_NULL" | "VALIDATE_NON_ZERO" | "VALIDATE_NON_NEGATIVE" | "VALIDATE_NUMERIC" | "NO_OP") | fn.#Fn
					Salesforce?:      *("PROJECTION" | "LESS_THAN" | "CONTAINS" | "GREATER_THAN" | "BETWEEN" | "LESS_THAN_OR_EQUAL_TO" | "GREATER_THAN_OR_EQUAL_TO" | "EQUAL_TO" | "NOT_EQUAL_TO" | "ADDITION" | "MULTIPLICATION" | "DIVISION" | "SUBTRACTION" | "MASK_ALL" | "MASK_FIRST_N" | "MASK_LAST_N" | "VALIDATE_NON_NULL" | "VALIDATE_NON_ZERO" | "VALIDATE_NON_NEGATIVE" | "VALIDATE_NUMERIC" | "NO_OP") | fn.#Fn
					ServiceNow?:      *("PROJECTION" | "LESS_THAN" | "CONTAINS" | "GREATER_THAN" | "BETWEEN" | "LESS_THAN_OR_EQUAL_TO" | "GREATER_THAN_OR_EQUAL_TO" | "EQUAL_TO" | "NOT_EQUAL_TO" | "ADDITION" | "MULTIPLICATION" | "DIVISION" | "SUBTRACTION" | "MASK_ALL" | "MASK_FIRST_N" | "MASK_LAST_N" | "VALIDATE_NON_NULL" | "VALIDATE_NON_ZERO" | "VALIDATE_NON_NEGATIVE" | "VALIDATE_NUMERIC" | "NO_OP") | fn.#Fn
					Singular?:        *("PROJECTION" | "EQUAL_TO" | "ADDITION" | "MULTIPLICATION" | "DIVISION" | "SUBTRACTION" | "MASK_ALL" | "MASK_FIRST_N" | "MASK_LAST_N" | "VALIDATE_NON_NULL" | "VALIDATE_NON_ZERO" | "VALIDATE_NON_NEGATIVE" | "VALIDATE_NUMERIC" | "NO_OP") | fn.#Fn
					Slack?:           *("PROJECTION" | "BETWEEN" | "EQUAL_TO" | "ADDITION" | "MULTIPLICATION" | "DIVISION" | "SUBTRACTION" | "MASK_ALL" | "MASK_FIRST_N" | "MASK_LAST_N" | "VALIDATE_NON_NULL" | "VALIDATE_NON_ZERO" | "VALIDATE_NON_NEGATIVE" | "VALIDATE_NUMERIC" | "NO_OP") | fn.#Fn
					Trendmicro?:      *("PROJECTION" | "EQUAL_TO" | "ADDITION" | "MULTIPLICATION" | "DIVISION" | "SUBTRACTION" | "MASK_ALL" | "MASK_FIRST_N" | "MASK_LAST_N" | "VALIDATE_NON_NULL" | "VALIDATE_NON_ZERO" | "VALIDATE_NON_NEGATIVE" | "VALIDATE_NUMERIC" | "NO_OP") | fn.#Fn
					Veeva?:           *("PROJECTION" | "LESS_THAN" | "GREATER_THAN" | "BETWEEN" | "LESS_THAN_OR_EQUAL_TO" | "GREATER_THAN_OR_EQUAL_TO" | "EQUAL_TO" | "NOT_EQUAL_TO" | "ADDITION" | "MULTIPLICATION" | "DIVISION" | "SUBTRACTION" | "MASK_ALL" | "MASK_FIRST_N" | "MASK_LAST_N" | "VALIDATE_NON_NULL" | "VALIDATE_NON_ZERO" | "VALIDATE_NON_NEGATIVE" | "VALIDATE_NUMERIC" | "NO_OP") | fn.#Fn
					Zendesk?:         *("PROJECTION" | "GREATER_THAN" | "ADDITION" | "MULTIPLICATION" | "DIVISION" | "SUBTRACTION" | "MASK_ALL" | "MASK_FIRST_N" | "MASK_LAST_N" | "VALIDATE_NON_NULL" | "VALIDATE_NON_ZERO" | "VALIDATE_NON_NEGATIVE" | "VALIDATE_NUMERIC" | "NO_OP") | fn.#Fn
				} | fn.#If
				DestinationField?: *string | fn.#Fn
				SourceFields:      [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				TaskProperties?:   *[...{
					Key:   *("VALUE" | "VALUES" | "DATA_TYPE" | "UPPER_BOUND" | "LOWER_BOUND" | "SOURCE_DATA_TYPE" | "DESTINATION_DATA_TYPE" | "VALIDATION_ACTION" | "MASK_VALUE" | "MASK_LENGTH" | "TRUNCATE_LENGTH" | "MATH_OPERATION_FIELDS_ORDER" | "CONCAT_FORMAT" | "SUBFIELD_CATEGORY_MAP" | "EXCLUDE_SOURCE_FIELDS_LIST") | fn.#Fn
					Value: *(=~#".+"#) | fn.#Fn
				}] | fn.#If
				TaskType: *("Arithmetic" | "Filter" | "Map" | "Map_all" | "Mask" | "Merge" | "Truncate" | "Validate") | fn.#Fn
			}] | fn.#If
			TriggerConfig: *{
				TriggerProperties?: *{
					DataPullMode?:      *("Incremental" | "Complete") | fn.#Fn
					ScheduleEndTime?:   *number | fn.#Fn
					ScheduleExpression: *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
					ScheduleOffset?:    *number | fn.#Fn
					ScheduleStartTime?: *number | fn.#Fn
					TimeZone?:          *string | fn.#Fn
				} | fn.#If
				TriggerType: *("Scheduled" | "Event" | "OnDemand") | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
