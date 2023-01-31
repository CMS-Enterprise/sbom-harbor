package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#IoTAnalytics: {
	#Channel: {
		Type: "AWS::IoTAnalytics::Channel"
		Properties: {
			ChannelName?:    *string | fn.#Fn
			ChannelStorage?: *{
				CustomerManagedS3?: *{
					Bucket:     *string | fn.#Fn
					KeyPrefix?: *string | fn.#Fn
					RoleArn:    *string | fn.#Fn
				} | fn.#If
				ServiceManagedS3?: *{} | fn.#If
			} | fn.#If
			RetentionPeriod?: *{
				NumberOfDays?: *int | fn.#Fn
				Unlimited?:    *bool | fn.#Fn
			} | fn.#If
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
	#Dataset: {
		Type: "AWS::IoTAnalytics::Dataset"
		Properties: {
			Actions: *[...{
				ActionName:       *string | fn.#Fn
				ContainerAction?: *{
					ExecutionRoleArn:      *string | fn.#Fn
					Image:                 *string | fn.#Fn
					ResourceConfiguration: *{
						ComputeType:    *string | fn.#Fn
						VolumeSizeInGB: *int | fn.#Fn
					} | fn.#If
					Variables?: *[...{
						DatasetContentVersionValue?: *{
							DatasetName?: *string | fn.#Fn
						} | fn.#If
						DoubleValue?:        *number | fn.#Fn
						OutputFileUriValue?: *{
							FileName?: *string | fn.#Fn
						} | fn.#If
						StringValue?: *string | fn.#Fn
						VariableName: *string | fn.#Fn
					}] | fn.#If
				} | fn.#If
				QueryAction?: *{
					Filters?: *[...{
						DeltaTime?: *{
							OffsetSeconds:  *int | fn.#Fn
							TimeExpression: *string | fn.#Fn
						} | fn.#If
					}] | fn.#If
					SqlQuery: *string | fn.#Fn
				} | fn.#If
			}] | fn.#If
			ContentDeliveryRules?: *[...{
				Destination: *{
					IotEventsDestinationConfiguration?: *{
						InputName: *string | fn.#Fn
						RoleArn:   *string | fn.#Fn
					} | fn.#If
					S3DestinationConfiguration?: *{
						Bucket:             *string | fn.#Fn
						GlueConfiguration?: *{
							DatabaseName: *string | fn.#Fn
							TableName:    *string | fn.#Fn
						} | fn.#If
						Key:     *string | fn.#Fn
						RoleArn: *string | fn.#Fn
					} | fn.#If
				} | fn.#If
				EntryName?: *string | fn.#Fn
			}] | fn.#If
			DatasetName?:   *string | fn.#Fn
			LateDataRules?: *[...{
				RuleConfiguration: *{
					DeltaTimeSessionWindowConfiguration?: *{
						TimeoutInMinutes: *int | fn.#Fn
					} | fn.#If
				} | fn.#If
				RuleName?: *string | fn.#Fn
			}] | fn.#If
			RetentionPeriod?: *{
				NumberOfDays: *int | fn.#Fn
				Unlimited:    *bool | fn.#Fn
			} | fn.#If
			Tags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			Triggers?: *[...{
				Schedule?: *{
					ScheduleExpression: *string | fn.#Fn
				} | fn.#If
				TriggeringDataset?: *{
					DatasetName: *string | fn.#Fn
				} | fn.#If
			}] | fn.#If
			VersioningConfiguration?: *{
				MaxVersions?: *int | fn.#Fn
				Unlimited?:   *bool | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Datastore: {
		Type: "AWS::IoTAnalytics::Datastore"
		Properties: {
			DatastoreName?:       *string | fn.#Fn
			DatastorePartitions?: *{
				Partitions?: *[...{
					Partition?: *{
						AttributeName: *string | fn.#Fn
					} | fn.#If
					TimestampPartition?: *{
						AttributeName:    *string | fn.#Fn
						TimestampFormat?: *string | fn.#Fn
					} | fn.#If
				}] | fn.#If
			} | fn.#If
			DatastoreStorage?: *{
				CustomerManagedS3?: *{
					Bucket:     *string | fn.#Fn
					KeyPrefix?: *string | fn.#Fn
					RoleArn:    *string | fn.#Fn
				} | fn.#If
				IotSiteWiseMultiLayerStorage?: *{
					CustomerManagedS3Storage: *{
						Bucket:     *string | fn.#Fn
						KeyPrefix?: *string | fn.#Fn
					} | fn.#If
				} | fn.#If
				ServiceManagedS3?: *{} | fn.#If
			} | fn.#If
			FileFormatConfiguration?: *{
				JsonConfiguration?:    *{} | fn.#If
				ParquetConfiguration?: *{
					SchemaDefinition?: *{
						Columns?: *[...{
							Name: *string | fn.#Fn
							Type: *string | fn.#Fn
						}] | fn.#If
					} | fn.#If
				} | fn.#If
			} | fn.#If
			RetentionPeriod?: *{
				NumberOfDays?: *int | fn.#Fn
				Unlimited?:    *bool | fn.#Fn
			} | fn.#If
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
	#Pipeline: {
		Type: "AWS::IoTAnalytics::Pipeline"
		Properties: {
			PipelineActivities: *[...{
				AddAttributes?: *{
					Attributes?: *{
						[string]: _
					} | fn.#Fn
					Name?: *string | fn.#Fn
					Next?: *string | fn.#Fn
				} | fn.#If
				Channel?: *{
					ChannelName?: *string | fn.#Fn
					Name?:        *string | fn.#Fn
					Next?:        *string | fn.#Fn
				} | fn.#If
				Datastore?: *{
					DatastoreName?: *string | fn.#Fn
					Name?:          *string | fn.#Fn
				} | fn.#If
				DeviceRegistryEnrich?: *{
					Attribute?: *string | fn.#Fn
					Name?:      *string | fn.#Fn
					Next?:      *string | fn.#Fn
					RoleArn?:   *string | fn.#Fn
					ThingName?: *string | fn.#Fn
				} | fn.#If
				DeviceShadowEnrich?: *{
					Attribute?: *string | fn.#Fn
					Name?:      *string | fn.#Fn
					Next?:      *string | fn.#Fn
					RoleArn?:   *string | fn.#Fn
					ThingName?: *string | fn.#Fn
				} | fn.#If
				Filter?: *{
					Filter?: *string | fn.#Fn
					Name?:   *string | fn.#Fn
					Next?:   *string | fn.#Fn
				} | fn.#If
				Lambda?: *{
					BatchSize?:  *int | fn.#Fn
					LambdaName?: *string | fn.#Fn
					Name?:       *string | fn.#Fn
					Next?:       *string | fn.#Fn
				} | fn.#If
				Math?: *{
					Attribute?: *string | fn.#Fn
					Math?:      *string | fn.#Fn
					Name?:      *string | fn.#Fn
					Next?:      *string | fn.#Fn
				} | fn.#If
				RemoveAttributes?: *{
					Attributes?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					Name?:       *string | fn.#Fn
					Next?:       *string | fn.#Fn
				} | fn.#If
				SelectAttributes?: *{
					Attributes?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					Name?:       *string | fn.#Fn
					Next?:       *string | fn.#Fn
				} | fn.#If
			}] | fn.#If
			PipelineName?: *string | fn.#Fn
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
}
