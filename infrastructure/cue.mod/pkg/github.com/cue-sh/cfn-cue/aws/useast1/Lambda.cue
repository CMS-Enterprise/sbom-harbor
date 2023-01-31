package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#Lambda: {
	#Alias: {
		Type: "AWS::Lambda::Alias"
		Properties: {
			Description?:                  *string | fn.#Fn
			FunctionName:                  *string | fn.#Fn
			FunctionVersion:               *string | fn.#Fn
			Name:                          *string | fn.#Fn
			ProvisionedConcurrencyConfig?: *{
				ProvisionedConcurrentExecutions: *int | fn.#Fn
			} | fn.#If
			RoutingConfig?: *{
				AdditionalVersionWeights: *[...{
					FunctionVersion: *string | fn.#Fn
					FunctionWeight:  *number | fn.#Fn
				}] | fn.#If
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		UpdatePolicy?: [string]: _
		Metadata?: [string]: _
		Condition?: string
	}
	#CodeSigningConfig: {
		Type: "AWS::Lambda::CodeSigningConfig"
		Properties: {
			AllowedPublishers: *{
				SigningProfileVersionArns: [...(*(strings.MinRunes(12) & strings.MaxRunes(1024) & (=~#"arn:(aws[a-zA-Z0-9-]*):([a-zA-Z0-9\-])+:([a-z]{2}(-gov)?-[a-z]+-\d{1})?:(\d{12})?:(.*)"#)) | fn.#Fn)] | (*(strings.MinRunes(12) & strings.MaxRunes(1024) & (=~#"arn:(aws[a-zA-Z0-9-]*):([a-zA-Z0-9\-])+:([a-z]{2}(-gov)?-[a-z]+-\d{1})?:(\d{12})?:(.*)"#)) | fn.#Fn)
			} | fn.#If
			CodeSigningPolicies?: *{
				UntrustedArtifactOnDeployment: *("Warn" | "Enforce") | fn.#Fn
			} | fn.#If
			Description?: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#EventInvokeConfig: {
		Type: "AWS::Lambda::EventInvokeConfig"
		Properties: {
			DestinationConfig?: *{
				OnFailure?: *{
					Destination: *string | fn.#Fn
				} | fn.#If
				OnSuccess?: *{
					Destination: *string | fn.#Fn
				} | fn.#If
			} | fn.#If
			FunctionName:              *string | fn.#Fn
			MaximumEventAgeInSeconds?: *int | fn.#Fn
			MaximumRetryAttempts?:     *int | fn.#Fn
			Qualifier:                 *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#EventSourceMapping: {
		Type: "AWS::Lambda::EventSourceMapping"
		Properties: {
			BatchSize?:                  *(>=1 & <=10000) | fn.#Fn
			BisectBatchOnFunctionError?: *bool | fn.#Fn
			DestinationConfig?:          *{
				OnFailure?: *{
					Destination?: *(strings.MinRunes(12) & strings.MaxRunes(1024) & (=~#"arn:(aws[a-zA-Z0-9-]*):([a-zA-Z0-9\-])+:([a-z]{2}(-gov)?-[a-z]+-\d{1})?:(\d{12})?:(.*)"#)) | fn.#Fn
				} | fn.#If
			} | fn.#If
			Enabled?:        *bool | fn.#Fn
			EventSourceArn?: *(strings.MinRunes(12) & strings.MaxRunes(1024) & (=~#"arn:(aws[a-zA-Z0-9-]*):([a-zA-Z0-9\-])+:([a-z]{2}(-gov)?-[a-z]+-\d{1})?:(\d{12})?:(.*)"#)) | fn.#Fn
			FilterCriteria?: *{
				[string]: _
			} | fn.#Fn
			FunctionName:                    *(strings.MinRunes(1) & strings.MaxRunes(140) & (=~#"(arn:(aws[a-zA-Z-]*)?:lambda:)?([a-z]{2}(-gov)?-[a-z]+-\d{1}:)?(\d{12}:)?(function:)?([a-zA-Z0-9-_]+)(:(\$LATEST|[a-zA-Z0-9-_]+))?"#)) | fn.#Fn
			FunctionResponseTypes?:          [...(*("ReportBatchItemFailures") | fn.#Fn)] | (*("ReportBatchItemFailures") | fn.#Fn)
			MaximumBatchingWindowInSeconds?: *(>=0 & <=300) | fn.#Fn
			MaximumRecordAgeInSeconds?:      *(>=-1 & <=604800) | fn.#Fn
			MaximumRetryAttempts?:           *(>=-1 & <=10000) | fn.#Fn
			ParallelizationFactor?:          *(>=1 & <=10) | fn.#Fn
			Queues?:                         [...(*(strings.MinRunes(1) & strings.MaxRunes(1000) & (=~#"[\s\S]*"#)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(1000) & (=~#"[\s\S]*"#)) | fn.#Fn)
			SelfManagedEventSource?:         *{
				Endpoints?: *{
					KafkaBootstrapServers?: [...(*(strings.MinRunes(1) & strings.MaxRunes(300) & (=~#"^(([a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9\-]*[a-zA-Z0-9])\.)*([A-Za-z0-9]|[A-Za-z0-9][A-Za-z0-9\-]*[A-Za-z0-9]):[0-9]{1,5}"#)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(300) & (=~#"^(([a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9\-]*[a-zA-Z0-9])\.)*([A-Za-z0-9]|[A-Za-z0-9][A-Za-z0-9\-]*[A-Za-z0-9]):[0-9]{1,5}"#)) | fn.#Fn)
				} | fn.#If
			} | fn.#If
			SourceAccessConfigurations?: *[...{
				Type?: *("BASIC_AUTH" | "VPC_SUBNET" | "VPC_SECURITY_GROUP" | "SASL_SCRAM_512_AUTH" | "SASL_SCRAM_256_AUTH" | "VIRTUAL_HOST" | "CLIENT_CERTIFICATE_TLS_AUTH" | "SERVER_ROOT_CA_CERTIFICATE") | fn.#Fn
				URI?:  *(strings.MinRunes(1) & strings.MaxRunes(200) & (=~#"[a-zA-Z0-9-\/*:_+=.@-]*"#)) | fn.#Fn
			}] | fn.#If
			StartingPosition?:          *(("AT_TIMESTAMP" | "LATEST" | "TRIM_HORIZON") & (strings.MinRunes(6) & strings.MaxRunes(12)) & (=~#"(LATEST|TRIM_HORIZON)+"#)) | fn.#Fn
			StartingPositionTimestamp?: *number | fn.#Fn
			Topics?:                    [...(*(strings.MinRunes(1) & strings.MaxRunes(249) & (=~#"^[^.]([a-zA-Z0-9\-_.]+)"#)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(249) & (=~#"^[^.]([a-zA-Z0-9\-_.]+)"#)) | fn.#Fn)
			TumblingWindowInSeconds?:   *int | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Function: {
		Type: "AWS::Lambda::Function"
		Properties: {
			Architectures?: [...(*("x86_64" | "arm64") | fn.#Fn)] | (*("x86_64" | "arm64") | fn.#Fn)
			Code:           *{
				ImageUri?:        *string | fn.#Fn
				S3Bucket?:        *(strings.MinRunes(3) & strings.MaxRunes(63) & (=~#"^[0-9A-Za-z\.\-_]*(?<!\.)$"#)) | fn.#Fn
				S3Key?:           *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
				S3ObjectVersion?: *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
				ZipFile?:         *string | fn.#Fn
			} | fn.#If
			CodeSigningConfigArn?: *(=~#"arn:(aws[a-zA-Z-]*)?:lambda:[a-z]{2}((-gov)|(-iso(b?)))?-[a-z]+-\d{1}:\d{12}:code-signing-config:csc-[a-z0-9]{17}"#) | fn.#Fn
			DeadLetterConfig?:     *{
				TargetArn?: *(=~#"^(arn:(aws[a-zA-Z-]*)?:[a-z0-9-.]+:.*)|()$"#) | fn.#Fn
			} | fn.#If
			Description?: *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
			Environment?: *{
				Variables?: *{
					[string]: *string | fn.#Fn
				} | fn.#If
			} | fn.#If
			FileSystemConfigs?: *[...{
				Arn:            *(=~#"^arn:aws[a-zA-Z-]*:elasticfilesystem:[a-z]{2}((-gov)|(-iso(b?)))?-[a-z]+-\d{1}:\d{12}:access-point/fsap-[a-f0-9]{17}$"#) | fn.#Fn
				LocalMountPath: *(=~#"^/mnt/[a-zA-Z0-9-_.]+$"#) | fn.#Fn
			}] | fn.#If
			FunctionName?: *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
			Handler?:      *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[^\s]+$"#)) | fn.#Fn
			ImageConfig?:  *{
				Command?:          [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				EntryPoint?:       [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				WorkingDirectory?: *string | fn.#Fn
			} | fn.#If
			KmsKeyArn?:                    *(=~#"^(arn:(aws[a-zA-Z-]*)?:[a-z0-9-.]+:.*)|()$"#) | fn.#Fn
			Layers?:                       [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			MemorySize?:                   *(>=128 & <=10240) | fn.#Fn
			PackageType?:                  *("Image" | "Zip") | fn.#Fn
			ReservedConcurrentExecutions?: *int | fn.#Fn
			Role:                          *(=~#"^arn:(aws[a-zA-Z-]*)?:iam::\d{12}:role/?[a-zA-Z_0-9+=,.@\-_/]+$"#) | fn.#Fn
			Runtime?:                      *("dotnetcore1.0" | "dotnetcore2.0" | "dotnetcore2.1" | "dotnetcore3.1" | "go1.x" | "java11" | "java8" | "java8.al2" | "nodejs" | "nodejs10.x" | "nodejs12.x" | "nodejs14.x" | "nodejs4.3" | "nodejs4.3-edge" | "nodejs6.10" | "nodejs8.10" | "provided" | "provided.al2" | "python2.7" | "python3.6" | "python3.7" | "python3.8" | "python3.9" | "ruby2.5" | "ruby2.7") | fn.#Fn
			Tags?:                         *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			Timeout?:       *(>=1 & <=900) | fn.#Fn
			TracingConfig?: *{
				Mode?: *("Active" | "PassThrough") | fn.#Fn
			} | fn.#If
			VpcConfig?: *{
				SecurityGroupIds?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				SubnetIds?:        [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#LayerVersion: {
		Type: "AWS::Lambda::LayerVersion"
		Properties: {
			CompatibleArchitectures?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			CompatibleRuntimes?:      [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			Content:                  *{
				S3Bucket:         *string | fn.#Fn
				S3Key:            *string | fn.#Fn
				S3ObjectVersion?: *string | fn.#Fn
			} | fn.#If
			Description?: *string | fn.#Fn
			LayerName?:   *(strings.MinRunes(1) & strings.MaxRunes(140)) | fn.#Fn
			LicenseInfo?: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#LayerVersionPermission: {
		Type: "AWS::Lambda::LayerVersionPermission"
		Properties: {
			Action:          *string | fn.#Fn
			LayerVersionArn: *string | fn.#Fn
			OrganizationId?: *string | fn.#Fn
			Principal:       *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Permission: {
		Type: "AWS::Lambda::Permission"
		Properties: {
			Action:            *string | fn.#Fn
			EventSourceToken?: *string | fn.#Fn
			FunctionName:      *string | fn.#Fn
			Principal:         *string | fn.#Fn
			SourceAccount?:    *string | fn.#Fn
			SourceArn?:        *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Version: {
		Type: "AWS::Lambda::Version"
		Properties: {
			CodeSha256?:                   *string | fn.#Fn
			Description?:                  *string | fn.#Fn
			FunctionName:                  *string | fn.#Fn
			ProvisionedConcurrencyConfig?: *{
				ProvisionedConcurrentExecutions: *int | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
