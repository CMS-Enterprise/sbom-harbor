package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#LookoutMetrics: {
	#Alert: {
		Type: "AWS::LookoutMetrics::Alert"
		Properties: {
			Action: *{
				LambdaConfiguration?: *{
					LambdaArn: *(=~#"arn:([a-z\d-]+):.*:.*:.*:.+"#) | fn.#Fn
					RoleArn:   *(=~#"arn:([a-z\d-]+):.*:.*:.*:.+"#) | fn.#Fn
				} | fn.#If
				SNSConfiguration?: *{
					RoleArn:     *(=~#"arn:([a-z\d-]+):.*:.*:.*:.+"#) | fn.#Fn
					SnsTopicArn: *(=~#"arn:([a-z\d-]+):.*:.*:.*:.+"#) | fn.#Fn
				} | fn.#If
			} | fn.#If
			AlertDescription?:         *(=~#".*\S.*"#) | fn.#Fn
			AlertName?:                *(strings.MinRunes(1) & strings.MaxRunes(63) & (=~#"^[a-zA-Z0-9][a-zA-Z0-9\-_]*"#)) | fn.#Fn
			AlertSensitivityThreshold: *int | fn.#Fn
			AnomalyDetectorArn:        *(=~#"arn:([a-z\d-]+):.*:.*:.*:.+"#) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#AnomalyDetector: {
		Type: "AWS::LookoutMetrics::AnomalyDetector"
		Properties: {
			AnomalyDetectorConfig: *{
				AnomalyDetectorFrequency: *("PT5M" | "PT10M" | "PT1H" | "P1D") | fn.#Fn
			} | fn.#If
			AnomalyDetectorDescription?: *(=~#".*\S.*"#) | fn.#Fn
			AnomalyDetectorName?:        *(strings.MinRunes(1) & strings.MaxRunes(63) & (=~#"^[a-zA-Z0-9][a-zA-Z0-9\-_]*"#)) | fn.#Fn
			KmsKeyArn?:                  *(strings.MinRunes(20) & strings.MaxRunes(2048) & (=~#"arn:aws.*:kms:.*:[0-9]{12}:key/.*"#)) | fn.#Fn
			MetricSetList:               *[...{
				DimensionList?: [...(*(strings.MinRunes(1) & strings.MaxRunes(63) & (=~#"^[a-zA-Z0-9][a-zA-Z0-9\-_]*"#)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(63) & (=~#"^[a-zA-Z0-9][a-zA-Z0-9\-_]*"#)) | fn.#Fn)
				MetricList:     *[...{
					AggregationFunction: *("AVG" | "SUM") | fn.#Fn
					MetricName:          *(strings.MinRunes(1) & strings.MaxRunes(63) & (=~#"^[a-zA-Z0-9][a-zA-Z0-9\-_]*"#)) | fn.#Fn
					Namespace?:          *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"[^:].*"#)) | fn.#Fn
				}] | fn.#If
				MetricSetDescription?: *(=~#".*\S.*"#) | fn.#Fn
				MetricSetFrequency?:   *("PT5M" | "PT10M" | "PT1H" | "P1D") | fn.#Fn
				MetricSetName:         *(strings.MinRunes(1) & strings.MaxRunes(63) & (=~#"^[a-zA-Z0-9][a-zA-Z0-9\-_]*"#)) | fn.#Fn
				MetricSource:          *{
					AppFlowConfig?: *{
						FlowName: *(=~#"[a-zA-Z0-9][\w!@#.-]+"#) | fn.#Fn
						RoleArn:  *(=~#"arn:([a-z\d-]+):.*:.*:.*:.+"#) | fn.#Fn
					} | fn.#If
					CloudwatchConfig?: *{
						RoleArn: *(=~#"arn:([a-z\d-]+):.*:.*:.*:.+"#) | fn.#Fn
					} | fn.#If
					RDSSourceConfig?: *{
						DBInstanceIdentifier: *(strings.MinRunes(1) & strings.MaxRunes(63) & (=~#"^[a-zA-Z](?!.*--)(?!.*-$)[0-9a-zA-Z\-]*$"#)) | fn.#Fn
						DatabaseHost:         *(strings.MinRunes(1) & strings.MaxRunes(253) & (=~#".*\S.*"#)) | fn.#Fn
						DatabaseName:         *(strings.MinRunes(1) & strings.MaxRunes(64) & (=~#"[a-zA-Z0-9_]+"#)) | fn.#Fn
						DatabasePort:         *(>=1 & <=65535) | fn.#Fn
						RoleArn:              *(=~#"arn:([a-z\d-]+):.*:.*:.*:.+"#) | fn.#Fn
						SecretManagerArn:     *(=~#"arn:([a-z\d-]+):.*:.*:secret:AmazonLookoutMetrics-.+"#) | fn.#Fn
						TableName:            *(strings.MinRunes(1) & strings.MaxRunes(100) & (=~#"^[a-zA-Z][a-zA-Z0-9_]*$"#)) | fn.#Fn
						VpcConfiguration:     *{
							SecurityGroupIdList: [...(*(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"[-0-9a-zA-Z]+"#)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"[-0-9a-zA-Z]+"#)) | fn.#Fn)
							SubnetIdList:        [...(*(=~#"[\-0-9a-zA-Z]+"#) | fn.#Fn)] | (*(=~#"[\-0-9a-zA-Z]+"#) | fn.#Fn)
						} | fn.#If
					} | fn.#If
					RedshiftSourceConfig?: *{
						ClusterIdentifier: *(strings.MinRunes(1) & strings.MaxRunes(63) & (=~#"^[a-z](?!.*--)(?!.*-$)[0-9a-z\-]*$"#)) | fn.#Fn
						DatabaseHost:      *(strings.MinRunes(1) & strings.MaxRunes(253) & (=~#".*\S.*"#)) | fn.#Fn
						DatabaseName:      *(strings.MinRunes(1) & strings.MaxRunes(100) & (=~#"[a-z0-9]+"#)) | fn.#Fn
						DatabasePort:      *(>=1 & <=65535) | fn.#Fn
						RoleArn:           *(=~#"arn:([a-z\d-]+):.*:.*:.*:.+"#) | fn.#Fn
						SecretManagerArn:  *(=~#"arn:([a-z\d-]+):.*:.*:secret:AmazonLookoutMetrics-.+"#) | fn.#Fn
						TableName:         *(strings.MinRunes(1) & strings.MaxRunes(100) & (=~#"^[a-zA-Z][a-zA-Z0-9_]*$"#)) | fn.#Fn
						VpcConfiguration:  *{
							SecurityGroupIdList: [...(*(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"[-0-9a-zA-Z]+"#)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"[-0-9a-zA-Z]+"#)) | fn.#Fn)
							SubnetIdList:        [...(*(=~#"[\-0-9a-zA-Z]+"#) | fn.#Fn)] | (*(=~#"[\-0-9a-zA-Z]+"#) | fn.#Fn)
						} | fn.#If
					} | fn.#If
					S3SourceConfig?: *{
						FileFormatDescriptor: *{
							CsvFormatDescriptor?: *{
								Charset?:         *(=~#"^[a-zA-Z0-9][a-zA-Z0-9\-_]*"#) | fn.#Fn
								ContainsHeader?:  *bool | fn.#Fn
								Delimiter?:       *(=~#"[^\r\n]"#) | fn.#Fn
								FileCompression?: *("NONE" | "GZIP") | fn.#Fn
								HeaderList?:      [...(*(strings.MinRunes(1) & strings.MaxRunes(63) & (=~#"^[a-zA-Z0-9][a-zA-Z0-9\-_]*"#)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(63) & (=~#"^[a-zA-Z0-9][a-zA-Z0-9\-_]*"#)) | fn.#Fn)
								QuoteSymbol?:     *(=~#"[^\r\n]|^$"#) | fn.#Fn
							} | fn.#If
							JsonFormatDescriptor?: *{
								Charset?:         *(=~#"^[a-zA-Z0-9][a-zA-Z0-9\-_]*"#) | fn.#Fn
								FileCompression?: *("NONE" | "GZIP") | fn.#Fn
							} | fn.#If
						} | fn.#If
						HistoricalDataPathList?: [...(*(=~#"^s3://[a-z0-9].+$"#) | fn.#Fn)] | (*(=~#"^s3://[a-z0-9].+$"#) | fn.#Fn)
						RoleArn:                 *(=~#"arn:([a-z\d-]+):.*:.*:.*:.+"#) | fn.#Fn
						TemplatedPathList?:      [...(*(=~#"^s3://[a-zA-Z0-9_\-\/ {}=]+$"#) | fn.#Fn)] | (*(=~#"^s3://[a-zA-Z0-9_\-\/ {}=]+$"#) | fn.#Fn)
					} | fn.#If
				} | fn.#If
				Offset?:          *int | fn.#Fn
				TimestampColumn?: *{
					ColumnFormat?: *(=~#".*\S.*"#) | fn.#Fn
					ColumnName?:   *(strings.MinRunes(1) & strings.MaxRunes(63) & (=~#"^[a-zA-Z0-9][a-zA-Z0-9\-_]*"#)) | fn.#Fn
				} | fn.#If
				Timezone?: *(=~#".*\S.*"#) | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
