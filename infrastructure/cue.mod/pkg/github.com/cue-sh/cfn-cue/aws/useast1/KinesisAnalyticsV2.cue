package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#KinesisAnalyticsV2: {
	#Application: {
		Type: "AWS::KinesisAnalyticsV2::Application"
		Properties: {
			ApplicationConfiguration?: *{
				ApplicationCodeConfiguration?: *{
					CodeContent: *{
						S3ContentLocation?: *{
							BucketARN?:     *string | fn.#Fn
							FileKey?:       *string | fn.#Fn
							ObjectVersion?: *string | fn.#Fn
						} | fn.#If
						TextContent?:    *string | fn.#Fn
						ZipFileContent?: *string | fn.#Fn
					} | fn.#If
					CodeContentType: *string | fn.#Fn
				} | fn.#If
				ApplicationSnapshotConfiguration?: *{
					SnapshotsEnabled: *bool | fn.#Fn
				} | fn.#If
				EnvironmentProperties?: *{
					PropertyGroups?: *[...{
						PropertyGroupId?: *string | fn.#Fn
						PropertyMap?:     *{
							[string]: _
						} | fn.#Fn
					}] | fn.#If
				} | fn.#If
				FlinkApplicationConfiguration?: *{
					CheckpointConfiguration?: *{
						CheckpointInterval?:         *int | fn.#Fn
						CheckpointingEnabled?:       *bool | fn.#Fn
						ConfigurationType:           *string | fn.#Fn
						MinPauseBetweenCheckpoints?: *int | fn.#Fn
					} | fn.#If
					MonitoringConfiguration?: *{
						ConfigurationType: *string | fn.#Fn
						LogLevel?:         *string | fn.#Fn
						MetricsLevel?:     *string | fn.#Fn
					} | fn.#If
					ParallelismConfiguration?: *{
						AutoScalingEnabled?: *bool | fn.#Fn
						ConfigurationType:   *string | fn.#Fn
						Parallelism?:        *int | fn.#Fn
						ParallelismPerKPU?:  *int | fn.#Fn
					} | fn.#If
				} | fn.#If
				SqlApplicationConfiguration?: *{
					Inputs?: *[...{
						InputParallelism?: *{
							Count?: *int | fn.#Fn
						} | fn.#If
						InputProcessingConfiguration?: *{
							InputLambdaProcessor?: *{
								ResourceARN: *string | fn.#Fn
							} | fn.#If
						} | fn.#If
						InputSchema: *{
							RecordColumns: *[...{
								Mapping?: *string | fn.#Fn
								Name:     *string | fn.#Fn
								SqlType:  *string | fn.#Fn
							}] | fn.#If
							RecordEncoding?: *string | fn.#Fn
							RecordFormat:    *{
								MappingParameters?: *{
									CSVMappingParameters?: *{
										RecordColumnDelimiter: *string | fn.#Fn
										RecordRowDelimiter:    *string | fn.#Fn
									} | fn.#If
									JSONMappingParameters?: *{
										RecordRowPath: *string | fn.#Fn
									} | fn.#If
								} | fn.#If
								RecordFormatType: *string | fn.#Fn
							} | fn.#If
						} | fn.#If
						KinesisFirehoseInput?: *{
							ResourceARN: *string | fn.#Fn
						} | fn.#If
						KinesisStreamsInput?: *{
							ResourceARN: *string | fn.#Fn
						} | fn.#If
						NamePrefix: *string | fn.#Fn
					}] | fn.#If
				} | fn.#If
				ZeppelinApplicationConfiguration?: *{
					CatalogConfiguration?: *{
						GlueDataCatalogConfiguration?: *{
							DatabaseARN?: *string | fn.#Fn
						} | fn.#If
					} | fn.#If
					CustomArtifactsConfiguration?:     *{} | fn.#If
					DeployAsApplicationConfiguration?: *{
						S3ContentLocation: *{
							BasePath:  *string | fn.#Fn
							BucketARN: *string | fn.#Fn
						} | fn.#If
					} | fn.#If
					MonitoringConfiguration?: *{
						LogLevel?: *string | fn.#Fn
					} | fn.#If
				} | fn.#If
			} | fn.#If
			ApplicationDescription?: *string | fn.#Fn
			ApplicationMode?:        *string | fn.#Fn
			ApplicationName?:        *string | fn.#Fn
			RuntimeEnvironment:      *("FLINK-1_11" | "FLINK-1_13" | "FLINK-1_6" | "FLINK-1_8" | "SQL-1_0" | "ZEPPELIN-FLINK-1_0" | "ZEPPELIN-FLINK-2_0") | fn.#Fn
			ServiceExecutionRole:    *string | fn.#Fn
			Tags?:                   *[...{
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
	#ApplicationCloudWatchLoggingOption: {
		Type: "AWS::KinesisAnalyticsV2::ApplicationCloudWatchLoggingOption"
		Properties: {
			ApplicationName:         *string | fn.#Fn
			CloudWatchLoggingOption: *{
				LogStreamARN: *string | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ApplicationOutput: {
		Type: "AWS::KinesisAnalyticsV2::ApplicationOutput"
		Properties: {
			ApplicationName: *string | fn.#Fn
			Output:          *{
				DestinationSchema: *{
					RecordFormatType?: *string | fn.#Fn
				} | fn.#If
				KinesisFirehoseOutput?: *{
					ResourceARN: *string | fn.#Fn
				} | fn.#If
				KinesisStreamsOutput?: *{
					ResourceARN: *string | fn.#Fn
				} | fn.#If
				LambdaOutput?: *{
					ResourceARN: *string | fn.#Fn
				} | fn.#If
				Name?: *string | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ApplicationReferenceDataSource: {
		Type: "AWS::KinesisAnalyticsV2::ApplicationReferenceDataSource"
		Properties: {
			ApplicationName:     *string | fn.#Fn
			ReferenceDataSource: *{
				ReferenceSchema: *{
					RecordColumns: *[...{
						Mapping?: *string | fn.#Fn
						Name:     *string | fn.#Fn
						SqlType:  *string | fn.#Fn
					}] | fn.#If
					RecordEncoding?: *string | fn.#Fn
					RecordFormat:    *{
						MappingParameters?: *{
							CSVMappingParameters?: *{
								RecordColumnDelimiter: *string | fn.#Fn
								RecordRowDelimiter:    *string | fn.#Fn
							} | fn.#If
							JSONMappingParameters?: *{
								RecordRowPath: *string | fn.#Fn
							} | fn.#If
						} | fn.#If
						RecordFormatType: *string | fn.#Fn
					} | fn.#If
				} | fn.#If
				S3ReferenceDataSource?: *{
					BucketARN: *string | fn.#Fn
					FileKey:   *string | fn.#Fn
				} | fn.#If
				TableName?: *string | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
