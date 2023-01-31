package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#IoT: {
	#AccountAuditConfiguration: {
		Type: "AWS::IoT::AccountAuditConfiguration"
		Properties: {
			AccountId:                *(strings.MinRunes(12) & strings.MaxRunes(12)) | fn.#Fn
			AuditCheckConfigurations: *{
				AuthenticatedCognitoRoleOverlyPermissiveCheck?: *{
					Enabled?: *bool | fn.#Fn
				} | fn.#If
				CaCertificateExpiringCheck?: *{
					Enabled?: *bool | fn.#Fn
				} | fn.#If
				CaCertificateKeyQualityCheck?: *{
					Enabled?: *bool | fn.#Fn
				} | fn.#If
				ConflictingClientIdsCheck?: *{
					Enabled?: *bool | fn.#Fn
				} | fn.#If
				DeviceCertificateExpiringCheck?: *{
					Enabled?: *bool | fn.#Fn
				} | fn.#If
				DeviceCertificateKeyQualityCheck?: *{
					Enabled?: *bool | fn.#Fn
				} | fn.#If
				DeviceCertificateSharedCheck?: *{
					Enabled?: *bool | fn.#Fn
				} | fn.#If
				IotPolicyOverlyPermissiveCheck?: *{
					Enabled?: *bool | fn.#Fn
				} | fn.#If
				IotRoleAliasAllowsAccessToUnusedServicesCheck?: *{
					Enabled?: *bool | fn.#Fn
				} | fn.#If
				IotRoleAliasOverlyPermissiveCheck?: *{
					Enabled?: *bool | fn.#Fn
				} | fn.#If
				LoggingDisabledCheck?: *{
					Enabled?: *bool | fn.#Fn
				} | fn.#If
				RevokedCaCertificateStillActiveCheck?: *{
					Enabled?: *bool | fn.#Fn
				} | fn.#If
				RevokedDeviceCertificateStillActiveCheck?: *{
					Enabled?: *bool | fn.#Fn
				} | fn.#If
				UnauthenticatedCognitoRoleOverlyPermissiveCheck?: *{
					Enabled?: *bool | fn.#Fn
				} | fn.#If
			} | fn.#If
			AuditNotificationTargetConfigurations?: *{
				Sns?: *{
					Enabled?:   *bool | fn.#Fn
					RoleArn?:   *(strings.MinRunes(20) & strings.MaxRunes(2048)) | fn.#Fn
					TargetArn?: *string | fn.#Fn
				} | fn.#If
			} | fn.#If
			RoleArn: *(strings.MinRunes(20) & strings.MaxRunes(2048)) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Authorizer: {
		Type: "AWS::IoT::Authorizer"
		Properties: {
			AuthorizerFunctionArn: *string | fn.#Fn
			AuthorizerName?:       *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"[\w=,@-]+"#)) | fn.#Fn
			SigningDisabled?:      *bool | fn.#Fn
			Status?:               *("ACTIVE" | "INACTIVE") | fn.#Fn
			Tags?:                 *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			TokenKeyName?:           *string | fn.#Fn
			TokenSigningPublicKeys?: *{
				[string]: *string | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Certificate: {
		Type: "AWS::IoT::Certificate"
		Properties: {
			CACertificatePem?:          *(strings.MinRunes(1) & strings.MaxRunes(65536)) | fn.#Fn
			CertificateMode?:           *("DEFAULT" | "SNI_ONLY") | fn.#Fn
			CertificatePem?:            *(strings.MinRunes(1) & strings.MaxRunes(65536)) | fn.#Fn
			CertificateSigningRequest?: *string | fn.#Fn
			Status:                     *("ACTIVE" | "INACTIVE" | "REVOKED" | "PENDING_TRANSFER" | "PENDING_ACTIVATION") | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#CustomMetric: {
		Type: "AWS::IoT::CustomMetric"
		Properties: {
			DisplayName?: *string | fn.#Fn
			MetricName?:  *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"[a-zA-Z0-9:_-]+"#)) | fn.#Fn
			MetricType:   *("string-list" | "ip-address-list" | "number-list" | "number") | fn.#Fn
			Tags?:        *[...{
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
	#Dimension: {
		Type: "AWS::IoT::Dimension"
		Properties: {
			Name?:        *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"[a-zA-Z0-9:_-]+"#)) | fn.#Fn
			StringValues: [...(*(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn)
			Tags?:        *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			Type: *("TOPIC_FILTER") | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#DomainConfiguration: {
		Type: "AWS::IoT::DomainConfiguration"
		Properties: {
			AuthorizerConfig?: *{
				AllowAuthorizerOverride?: *bool | fn.#Fn
				DefaultAuthorizerName?:   *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[\w=,@-]+$"#)) | fn.#Fn
			} | fn.#If
			DomainConfigurationName?:   *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[\w.-]+$"#)) | fn.#Fn
			DomainConfigurationStatus?: *("ENABLED" | "DISABLED") | fn.#Fn
			DomainName?:                *(strings.MinRunes(1) & strings.MaxRunes(253)) | fn.#Fn
			ServerCertificateArns?:     [...(*(strings.MinRunes(1) & strings.MaxRunes(2048) & (=~#"^arn:aws(-cn|-us-gov|-iso-b|-iso)?:acm:[a-z]{2}-(gov-|iso-|isob-)?[a-z]{4,9}-\d{1}:\d{12}:certificate/[a-zA-Z0-9/-]+$"#)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(2048) & (=~#"^arn:aws(-cn|-us-gov|-iso-b|-iso)?:acm:[a-z]{2}-(gov-|iso-|isob-)?[a-z]{4,9}-\d{1}:\d{12}:certificate/[a-zA-Z0-9/-]+$"#)) | fn.#Fn)
			ServiceType?:               *("DATA" | "CREDENTIAL_PROVIDER" | "JOBS") | fn.#Fn
			Tags?:                      *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			ValidationCertificateArn?: *(=~#"^arn:aws(-cn|-us-gov|-iso-b|-iso)?:acm:[a-z]{2}-(gov-|iso-|isob-)?[a-z]{4,9}-\d{1}:\d{12}:certificate/[a-zA-Z0-9/-]+$"#) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#FleetMetric: {
		Type: "AWS::IoT::FleetMetric"
		Properties: {
			AggregationField?: *string | fn.#Fn
			AggregationType?:  *{
				Name:   *string | fn.#Fn
				Values: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			} | fn.#If
			Description?:  *string | fn.#Fn
			IndexName?:    *string | fn.#Fn
			MetricName:    *string | fn.#Fn
			Period?:       *int | fn.#Fn
			QueryString?:  *string | fn.#Fn
			QueryVersion?: *string | fn.#Fn
			Tags?:         *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			Unit?: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#JobTemplate: {
		Type: "AWS::IoT::JobTemplate"
		Properties: {
			AbortConfig?: *{
				[string]: _
			} | fn.#Fn
			Description:                 *string | fn.#Fn
			Document?:                   *string | fn.#Fn
			DocumentSource?:             *(strings.MinRunes(1) & strings.MaxRunes(1350)) | fn.#Fn
			JobArn?:                     *string | fn.#Fn
			JobExecutionsRolloutConfig?: *{
				[string]: _
			} | fn.#Fn
			JobTemplateId:       *(strings.MinRunes(1) & strings.MaxRunes(64) & (=~#"[a-zA-Z0-9_-]+"#)) | fn.#Fn
			PresignedUrlConfig?: *{
				[string]: _
			} | fn.#Fn
			Tags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			TimeoutConfig?: *{
				[string]: _
			} | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Logging: {
		Type: "AWS::IoT::Logging"
		Properties: {
			AccountId:       *(strings.MinRunes(12) & strings.MaxRunes(12) & (=~#"^[0-9]{12}$"#)) | fn.#Fn
			DefaultLogLevel: *("ERROR" | "WARN" | "INFO" | "DEBUG" | "DISABLED") | fn.#Fn
			RoleArn:         *(strings.MinRunes(20) & strings.MaxRunes(2048)) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#MitigationAction: {
		Type: "AWS::IoT::MitigationAction"
		Properties: {
			ActionName?:  *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"[a-zA-Z0-9:_-]+"#)) | fn.#Fn
			ActionParams: *{
				AddThingsToThingGroupParams?: *{
					OverrideDynamicGroups?: *bool | fn.#Fn
					ThingGroupNames:        [...(*(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn)
				} | fn.#If
				EnableIoTLoggingParams?: *{
					LogLevel:          *("DEBUG" | "INFO" | "ERROR" | "WARN") | fn.#Fn
					RoleArnForLogging: *(strings.MinRunes(20) & strings.MaxRunes(2048)) | fn.#Fn
				} | fn.#If
				PublishFindingToSnsParams?: *{
					TopicArn: *(strings.MinRunes(20) & strings.MaxRunes(2048)) | fn.#Fn
				} | fn.#If
				ReplaceDefaultPolicyVersionParams?: *{
					TemplateName: *("BLANK_POLICY") | fn.#Fn
				} | fn.#If
				UpdateCACertificateParams?: *{
					Action: *("DEACTIVATE") | fn.#Fn
				} | fn.#If
				UpdateDeviceCertificateParams?: *{
					Action: *("DEACTIVATE") | fn.#Fn
				} | fn.#If
			} | fn.#If
			RoleArn: *string | fn.#Fn
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
	#Policy: {
		Type: "AWS::IoT::Policy"
		Properties: {
			PolicyDocument: *{
				{
					[string]: _
				}
				Version: string | *"2012-10-17"
			} | fn.#Fn
			PolicyName?: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#PolicyPrincipalAttachment: {
		Type: "AWS::IoT::PolicyPrincipalAttachment"
		Properties: {
			PolicyName: *string | fn.#Fn
			Principal:  *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ProvisioningTemplate: {
		Type: "AWS::IoT::ProvisioningTemplate"
		Properties: {
			Description?:         *string | fn.#Fn
			Enabled?:             *bool | fn.#Fn
			PreProvisioningHook?: *{
				PayloadVersion?: *string | fn.#Fn
				TargetArn?:      *string | fn.#Fn
			} | fn.#If
			ProvisioningRoleArn: *string | fn.#Fn
			Tags?:               *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			TemplateBody:  *string | fn.#Fn
			TemplateName?: *(strings.MinRunes(1) & strings.MaxRunes(36) & (=~#"^[0-9A-Za-z_-]+$"#)) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ResourceSpecificLogging: {
		Type: "AWS::IoT::ResourceSpecificLogging"
		Properties: {
			LogLevel:   *("ERROR" | "WARN" | "INFO" | "DEBUG" | "DISABLED") | fn.#Fn
			TargetName: *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"[a-zA-Z0-9:_-]+"#)) | fn.#Fn
			TargetType: *("THING_GROUP") | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ScheduledAudit: {
		Type: "AWS::IoT::ScheduledAudit"
		Properties: {
			DayOfMonth?:         *(=~#"^([1-9]|[12][0-9]|3[01])$|^LAST$"#) | fn.#Fn
			DayOfWeek?:          *("SUN" | "MON" | "TUE" | "WED" | "THU" | "FRI" | "SAT") | fn.#Fn
			Frequency:           *("DAILY" | "WEEKLY" | "BIWEEKLY" | "MONTHLY") | fn.#Fn
			ScheduledAuditName?: *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"[a-zA-Z0-9:_-]+"#)) | fn.#Fn
			Tags?:               *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			TargetCheckNames: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#SecurityProfile: {
		Type: "AWS::IoT::SecurityProfile"
		Properties: {
			AdditionalMetricsToRetainV2?: *[...{
				Metric:           *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"[a-zA-Z0-9:_-]+"#)) | fn.#Fn
				MetricDimension?: *{
					DimensionName: *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"[a-zA-Z0-9:_-]+"#)) | fn.#Fn
					Operator?:     *("IN" | "NOT_IN") | fn.#Fn
				} | fn.#If
			}] | fn.#If
			AlertTargets?: *{
				[string]: {
					AlertTargetArn: *string | fn.#Fn
					RoleArn:        *string | fn.#Fn
				}
			} | fn.#If
			Behaviors?: *[...{
				Criteria?: *{
					ComparisonOperator?:           *("less-than" | "less-than-equals" | "greater-than" | "greater-than-equals" | "in-cidr-set" | "not-in-cidr-set" | "in-port-set" | "not-in-port-set" | "in-set" | "not-in-set") | fn.#Fn
					ConsecutiveDatapointsToAlarm?: *(>=1 & <=10) | fn.#Fn
					ConsecutiveDatapointsToClear?: *(>=1 & <=10) | fn.#Fn
					DurationSeconds?:              *int | fn.#Fn
					MlDetectionConfig?:            *{
						ConfidenceLevel?: *("LOW" | "MEDIUM" | "HIGH") | fn.#Fn
					} | fn.#If
					StatisticalThreshold?: *{
						Statistic?: *("Average" | "p0" | "p0.1" | "p0.01" | "p1" | "p10" | "p50" | "p90" | "p99" | "p99.9" | "p99.99" | "p100") | fn.#Fn
					} | fn.#If
					Value?: *{
						Cidrs?:   [...(*string | fn.#Fn)] | (*string | fn.#Fn)
						Count?:   *string | fn.#Fn
						Number?:  *number | fn.#Fn
						Numbers?: [...(*number | fn.#Fn)] | (*number | fn.#Fn)
						Ports?:   [...(*int | fn.#Fn)] | (*int | fn.#Fn)
						Strings?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					} | fn.#If
				} | fn.#If
				Metric?:          *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"[a-zA-Z0-9:_-]+"#)) | fn.#Fn
				MetricDimension?: *{
					DimensionName: *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"[a-zA-Z0-9:_-]+"#)) | fn.#Fn
					Operator?:     *("IN" | "NOT_IN") | fn.#Fn
				} | fn.#If
				Name:            *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"[a-zA-Z0-9:_-]+"#)) | fn.#Fn
				SuppressAlerts?: *bool | fn.#Fn
			}] | fn.#If
			SecurityProfileDescription?: *string | fn.#Fn
			SecurityProfileName?:        *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"[a-zA-Z0-9:_-]+"#)) | fn.#Fn
			Tags?:                       *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			TargetArns?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Thing: {
		Type: "AWS::IoT::Thing"
		Properties: {
			AttributePayload?: *{
				Attributes?: *{
					[string]: *string | fn.#Fn
				} | fn.#If
			} | fn.#If
			ThingName?: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ThingPrincipalAttachment: {
		Type: "AWS::IoT::ThingPrincipalAttachment"
		Properties: {
			Principal: *string | fn.#Fn
			ThingName: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#TopicRule: {
		Type: "AWS::IoT::TopicRule"
		Properties: {
			RuleName?: *string | fn.#Fn
			Tags?:     *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			TopicRulePayload: *{
				Actions: *[...{
					CloudwatchAlarm?: *{
						AlarmName:   *string | fn.#Fn
						RoleArn:     *string | fn.#Fn
						StateReason: *string | fn.#Fn
						StateValue:  *string | fn.#Fn
					} | fn.#If
					CloudwatchLogs?: *{
						LogGroupName: *string | fn.#Fn
						RoleArn:      *string | fn.#Fn
					} | fn.#If
					CloudwatchMetric?: *{
						MetricName:       *string | fn.#Fn
						MetricNamespace:  *string | fn.#Fn
						MetricTimestamp?: *string | fn.#Fn
						MetricUnit:       *string | fn.#Fn
						MetricValue:      *string | fn.#Fn
						RoleArn:          *string | fn.#Fn
					} | fn.#If
					DynamoDB?: *{
						HashKeyField:   *string | fn.#Fn
						HashKeyType?:   *string | fn.#Fn
						HashKeyValue:   *string | fn.#Fn
						PayloadField?:  *string | fn.#Fn
						RangeKeyField?: *string | fn.#Fn
						RangeKeyType?:  *string | fn.#Fn
						RangeKeyValue?: *string | fn.#Fn
						RoleArn:        *string | fn.#Fn
						TableName:      *string | fn.#Fn
					} | fn.#If
					DynamoDBv2?: *{
						PutItem?: *{
							TableName: *string | fn.#Fn
						} | fn.#If
						RoleArn?: *string | fn.#Fn
					} | fn.#If
					Elasticsearch?: *{
						Endpoint: *string | fn.#Fn
						Id:       *string | fn.#Fn
						Index:    *string | fn.#Fn
						RoleArn:  *string | fn.#Fn
						Type:     *string | fn.#Fn
					} | fn.#If
					Firehose?: *{
						BatchMode?:         *bool | fn.#Fn
						DeliveryStreamName: *string | fn.#Fn
						RoleArn:            *string | fn.#Fn
						Separator?:         *string | fn.#Fn
					} | fn.#If
					Http?: *{
						Auth?: *{
							Sigv4?: *{
								RoleArn:       *string | fn.#Fn
								ServiceName:   *string | fn.#Fn
								SigningRegion: *string | fn.#Fn
							} | fn.#If
						} | fn.#If
						ConfirmationUrl?: *string | fn.#Fn
						Headers?:         *[...{
							Key:   *string | fn.#Fn
							Value: *string | fn.#Fn
						}] | fn.#If
						Url: *string | fn.#Fn
					} | fn.#If
					IotAnalytics?: *{
						BatchMode?:  *bool | fn.#Fn
						ChannelName: *string | fn.#Fn
						RoleArn:     *string | fn.#Fn
					} | fn.#If
					IotEvents?: *{
						BatchMode?: *bool | fn.#Fn
						InputName:  *string | fn.#Fn
						MessageId?: *string | fn.#Fn
						RoleArn:    *string | fn.#Fn
					} | fn.#If
					IotSiteWise?: *{
						PutAssetPropertyValueEntries: *[...{
							AssetId?:       *string | fn.#Fn
							EntryId?:       *string | fn.#Fn
							PropertyAlias?: *string | fn.#Fn
							PropertyId?:    *string | fn.#Fn
							PropertyValues: *[...{
								Quality?:  *string | fn.#Fn
								Timestamp: *{
									OffsetInNanos?: *string | fn.#Fn
									TimeInSeconds:  *string | fn.#Fn
								} | fn.#If
								Value: *{
									BooleanValue?: *string | fn.#Fn
									DoubleValue?:  *string | fn.#Fn
									IntegerValue?: *string | fn.#Fn
									StringValue?:  *string | fn.#Fn
								} | fn.#If
							}] | fn.#If
						}] | fn.#If
						RoleArn: *string | fn.#Fn
					} | fn.#If
					Kafka?: *{
						ClientProperties: *{
							[string]: *string | fn.#Fn
						} | fn.#If
						DestinationArn: *string | fn.#Fn
						Key?:           *string | fn.#Fn
						Partition?:     *string | fn.#Fn
						Topic:          *string | fn.#Fn
					} | fn.#If
					Kinesis?: *{
						PartitionKey?: *string | fn.#Fn
						RoleArn:       *string | fn.#Fn
						StreamName:    *string | fn.#Fn
					} | fn.#If
					Lambda?: *{
						FunctionArn?: *string | fn.#Fn
					} | fn.#If
					OpenSearch?: *{
						Endpoint: *string | fn.#Fn
						Id:       *string | fn.#Fn
						Index:    *string | fn.#Fn
						RoleArn:  *string | fn.#Fn
						Type:     *string | fn.#Fn
					} | fn.#If
					Republish?: *{
						Qos?:    *int | fn.#Fn
						RoleArn: *string | fn.#Fn
						Topic:   *string | fn.#Fn
					} | fn.#If
					S3?: *{
						BucketName: *string | fn.#Fn
						CannedAcl?: *("private" | "public-read" | "public-read-write" | "aws-exec-read" | "authenticated-read" | "bucket-owner-read" | "bucket-owner-full-control" | "log-delivery-write") | fn.#Fn
						Key:        *string | fn.#Fn
						RoleArn:    *string | fn.#Fn
					} | fn.#If
					Sns?: *{
						MessageFormat?: *string | fn.#Fn
						RoleArn:        *string | fn.#Fn
						TargetArn:      *string | fn.#Fn
					} | fn.#If
					Sqs?: *{
						QueueUrl:   *string | fn.#Fn
						RoleArn:    *string | fn.#Fn
						UseBase64?: *bool | fn.#Fn
					} | fn.#If
					StepFunctions?: *{
						ExecutionNamePrefix?: *string | fn.#Fn
						RoleArn:              *string | fn.#Fn
						StateMachineName:     *string | fn.#Fn
					} | fn.#If
					Timestream?: *{
						BatchMode?:   *bool | fn.#Fn
						DatabaseName: *string | fn.#Fn
						Dimensions:   *[...{
							Name:  *string | fn.#Fn
							Value: *string | fn.#Fn
						}] | fn.#If
						RoleArn:    *string | fn.#Fn
						TableName:  *string | fn.#Fn
						Timestamp?: *{
							Unit:  *string | fn.#Fn
							Value: *string | fn.#Fn
						} | fn.#If
					} | fn.#If
				}] | fn.#If
				AwsIotSqlVersion?: *string | fn.#Fn
				Description?:      *string | fn.#Fn
				ErrorAction?:      *{
					CloudwatchAlarm?: *{
						AlarmName:   *string | fn.#Fn
						RoleArn:     *string | fn.#Fn
						StateReason: *string | fn.#Fn
						StateValue:  *string | fn.#Fn
					} | fn.#If
					CloudwatchLogs?: *{
						LogGroupName: *string | fn.#Fn
						RoleArn:      *string | fn.#Fn
					} | fn.#If
					CloudwatchMetric?: *{
						MetricName:       *string | fn.#Fn
						MetricNamespace:  *string | fn.#Fn
						MetricTimestamp?: *string | fn.#Fn
						MetricUnit:       *string | fn.#Fn
						MetricValue:      *string | fn.#Fn
						RoleArn:          *string | fn.#Fn
					} | fn.#If
					DynamoDB?: *{
						HashKeyField:   *string | fn.#Fn
						HashKeyType?:   *string | fn.#Fn
						HashKeyValue:   *string | fn.#Fn
						PayloadField?:  *string | fn.#Fn
						RangeKeyField?: *string | fn.#Fn
						RangeKeyType?:  *string | fn.#Fn
						RangeKeyValue?: *string | fn.#Fn
						RoleArn:        *string | fn.#Fn
						TableName:      *string | fn.#Fn
					} | fn.#If
					DynamoDBv2?: *{
						PutItem?: *{
							TableName: *string | fn.#Fn
						} | fn.#If
						RoleArn?: *string | fn.#Fn
					} | fn.#If
					Elasticsearch?: *{
						Endpoint: *string | fn.#Fn
						Id:       *string | fn.#Fn
						Index:    *string | fn.#Fn
						RoleArn:  *string | fn.#Fn
						Type:     *string | fn.#Fn
					} | fn.#If
					Firehose?: *{
						BatchMode?:         *bool | fn.#Fn
						DeliveryStreamName: *string | fn.#Fn
						RoleArn:            *string | fn.#Fn
						Separator?:         *string | fn.#Fn
					} | fn.#If
					Http?: *{
						Auth?: *{
							Sigv4?: *{
								RoleArn:       *string | fn.#Fn
								ServiceName:   *string | fn.#Fn
								SigningRegion: *string | fn.#Fn
							} | fn.#If
						} | fn.#If
						ConfirmationUrl?: *string | fn.#Fn
						Headers?:         *[...{
							Key:   *string | fn.#Fn
							Value: *string | fn.#Fn
						}] | fn.#If
						Url: *string | fn.#Fn
					} | fn.#If
					IotAnalytics?: *{
						BatchMode?:  *bool | fn.#Fn
						ChannelName: *string | fn.#Fn
						RoleArn:     *string | fn.#Fn
					} | fn.#If
					IotEvents?: *{
						BatchMode?: *bool | fn.#Fn
						InputName:  *string | fn.#Fn
						MessageId?: *string | fn.#Fn
						RoleArn:    *string | fn.#Fn
					} | fn.#If
					IotSiteWise?: *{
						PutAssetPropertyValueEntries: *[...{
							AssetId?:       *string | fn.#Fn
							EntryId?:       *string | fn.#Fn
							PropertyAlias?: *string | fn.#Fn
							PropertyId?:    *string | fn.#Fn
							PropertyValues: *[...{
								Quality?:  *string | fn.#Fn
								Timestamp: *{
									OffsetInNanos?: *string | fn.#Fn
									TimeInSeconds:  *string | fn.#Fn
								} | fn.#If
								Value: *{
									BooleanValue?: *string | fn.#Fn
									DoubleValue?:  *string | fn.#Fn
									IntegerValue?: *string | fn.#Fn
									StringValue?:  *string | fn.#Fn
								} | fn.#If
							}] | fn.#If
						}] | fn.#If
						RoleArn: *string | fn.#Fn
					} | fn.#If
					Kafka?: *{
						ClientProperties: *{
							[string]: *string | fn.#Fn
						} | fn.#If
						DestinationArn: *string | fn.#Fn
						Key?:           *string | fn.#Fn
						Partition?:     *string | fn.#Fn
						Topic:          *string | fn.#Fn
					} | fn.#If
					Kinesis?: *{
						PartitionKey?: *string | fn.#Fn
						RoleArn:       *string | fn.#Fn
						StreamName:    *string | fn.#Fn
					} | fn.#If
					Lambda?: *{
						FunctionArn?: *string | fn.#Fn
					} | fn.#If
					OpenSearch?: *{
						Endpoint: *string | fn.#Fn
						Id:       *string | fn.#Fn
						Index:    *string | fn.#Fn
						RoleArn:  *string | fn.#Fn
						Type:     *string | fn.#Fn
					} | fn.#If
					Republish?: *{
						Qos?:    *int | fn.#Fn
						RoleArn: *string | fn.#Fn
						Topic:   *string | fn.#Fn
					} | fn.#If
					S3?: *{
						BucketName: *string | fn.#Fn
						CannedAcl?: *("private" | "public-read" | "public-read-write" | "aws-exec-read" | "authenticated-read" | "bucket-owner-read" | "bucket-owner-full-control" | "log-delivery-write") | fn.#Fn
						Key:        *string | fn.#Fn
						RoleArn:    *string | fn.#Fn
					} | fn.#If
					Sns?: *{
						MessageFormat?: *string | fn.#Fn
						RoleArn:        *string | fn.#Fn
						TargetArn:      *string | fn.#Fn
					} | fn.#If
					Sqs?: *{
						QueueUrl:   *string | fn.#Fn
						RoleArn:    *string | fn.#Fn
						UseBase64?: *bool | fn.#Fn
					} | fn.#If
					StepFunctions?: *{
						ExecutionNamePrefix?: *string | fn.#Fn
						RoleArn:              *string | fn.#Fn
						StateMachineName:     *string | fn.#Fn
					} | fn.#If
					Timestream?: *{
						BatchMode?:   *bool | fn.#Fn
						DatabaseName: *string | fn.#Fn
						Dimensions:   *[...{
							Name:  *string | fn.#Fn
							Value: *string | fn.#Fn
						}] | fn.#If
						RoleArn:    *string | fn.#Fn
						TableName:  *string | fn.#Fn
						Timestamp?: *{
							Unit:  *string | fn.#Fn
							Value: *string | fn.#Fn
						} | fn.#If
					} | fn.#If
				} | fn.#If
				RuleDisabled?: *bool | fn.#Fn
				Sql:           *string | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#TopicRuleDestination: {
		Type: "AWS::IoT::TopicRuleDestination"
		Properties: {
			HttpUrlProperties?: *{
				ConfirmationUrl?: *string | fn.#Fn
			} | fn.#If
			Status?:        *("ENABLED" | "IN_PROGRESS" | "DISABLED") | fn.#Fn
			VpcProperties?: *{
				RoleArn?:        *string | fn.#Fn
				SecurityGroups?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				SubnetIds?:      [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				VpcId?:          *string | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
