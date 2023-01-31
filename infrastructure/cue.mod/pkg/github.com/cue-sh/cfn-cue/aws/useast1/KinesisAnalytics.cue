package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#KinesisAnalytics: {
	#Application: {
		Type: "AWS::KinesisAnalytics::Application"
		Properties: {
			ApplicationCode?:        *string | fn.#Fn
			ApplicationDescription?: *string | fn.#Fn
			ApplicationName?:        *string | fn.#Fn
			Inputs:                  *[...{
				InputParallelism?: *{
					Count?: *int | fn.#Fn
				} | fn.#If
				InputProcessingConfiguration?: *{
					InputLambdaProcessor?: *{
						ResourceARN: *string | fn.#Fn
						RoleARN:     *string | fn.#Fn
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
					RoleARN:     *string | fn.#Fn
				} | fn.#If
				KinesisStreamsInput?: *{
					ResourceARN: *string | fn.#Fn
					RoleARN:     *string | fn.#Fn
				} | fn.#If
				NamePrefix: *string | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ApplicationOutput: {
		Type: "AWS::KinesisAnalytics::ApplicationOutput"
		Properties: {
			ApplicationName: *string | fn.#Fn
			Output:          *{
				DestinationSchema: *{
					RecordFormatType?: *string | fn.#Fn
				} | fn.#If
				KinesisFirehoseOutput?: *{
					ResourceARN: *string | fn.#Fn
					RoleARN:     *string | fn.#Fn
				} | fn.#If
				KinesisStreamsOutput?: *{
					ResourceARN: *string | fn.#Fn
					RoleARN:     *string | fn.#Fn
				} | fn.#If
				LambdaOutput?: *{
					ResourceARN: *string | fn.#Fn
					RoleARN:     *string | fn.#Fn
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
		Type: "AWS::KinesisAnalytics::ApplicationReferenceDataSource"
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
					BucketARN:        *string | fn.#Fn
					FileKey:          *string | fn.#Fn
					ReferenceRoleARN: *string | fn.#Fn
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
