package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#DMS: {
	#Certificate: {
		Type: "AWS::DMS::Certificate"
		Properties: {
			CertificateIdentifier?: *string | fn.#Fn
			CertificatePem?:        *string | fn.#Fn
			CertificateWallet?:     *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Endpoint: {
		Type: "AWS::DMS::Endpoint"
		Properties: {
			CertificateArn?: *string | fn.#Fn
			DatabaseName?:   *string | fn.#Fn
			DocDbSettings?:  *{
				SecretsManagerAccessRoleArn?: *string | fn.#Fn
				SecretsManagerSecretId?:      *string | fn.#Fn
			} | fn.#If
			DynamoDbSettings?: *{
				ServiceAccessRoleArn?: *string | fn.#Fn
			} | fn.#If
			ElasticsearchSettings?: *{
				EndpointUri?:             *string | fn.#Fn
				ErrorRetryDuration?:      *int | fn.#Fn
				FullLoadErrorPercentage?: *int | fn.#Fn
				ServiceAccessRoleArn?:    *string | fn.#Fn
			} | fn.#If
			EndpointIdentifier?:        *string | fn.#Fn
			EndpointType:               *("source" | "target") | fn.#Fn
			EngineName:                 *string | fn.#Fn
			ExtraConnectionAttributes?: *string | fn.#Fn
			IbmDb2Settings?:            *{
				SecretsManagerAccessRoleArn?: *string | fn.#Fn
				SecretsManagerSecretId?:      *string | fn.#Fn
			} | fn.#If
			KafkaSettings?: *{
				Broker?:                      *string | fn.#Fn
				IncludeControlDetails?:       *bool | fn.#Fn
				IncludeNullAndEmpty?:         *bool | fn.#Fn
				IncludeTableAlterOperations?: *bool | fn.#Fn
				IncludeTransactionDetails?:   *bool | fn.#Fn
				NoHexPrefix?:                 *bool | fn.#Fn
				PartitionIncludeSchemaTable?: *bool | fn.#Fn
				SaslPassword?:                *string | fn.#Fn
				SaslUserName?:                *string | fn.#Fn
				SecurityProtocol?:            *string | fn.#Fn
				SslCaCertificateArn?:         *string | fn.#Fn
				SslClientCertificateArn?:     *string | fn.#Fn
				SslClientKeyArn?:             *string | fn.#Fn
				SslClientKeyPassword?:        *string | fn.#Fn
				Topic?:                       *string | fn.#Fn
			} | fn.#If
			KinesisSettings?: *{
				IncludeControlDetails?:       *bool | fn.#Fn
				IncludeNullAndEmpty?:         *bool | fn.#Fn
				IncludeTableAlterOperations?: *bool | fn.#Fn
				IncludeTransactionDetails?:   *bool | fn.#Fn
				MessageFormat?:               *string | fn.#Fn
				NoHexPrefix?:                 *bool | fn.#Fn
				PartitionIncludeSchemaTable?: *bool | fn.#Fn
				ServiceAccessRoleArn?:        *string | fn.#Fn
				StreamArn?:                   *string | fn.#Fn
			} | fn.#If
			KmsKeyId?:                   *string | fn.#Fn
			MicrosoftSqlServerSettings?: *{
				SecretsManagerAccessRoleArn?: *string | fn.#Fn
				SecretsManagerSecretId?:      *string | fn.#Fn
			} | fn.#If
			MongoDbSettings?: *{
				AuthMechanism?:               *string | fn.#Fn
				AuthSource?:                  *string | fn.#Fn
				AuthType?:                    *string | fn.#Fn
				DatabaseName?:                *string | fn.#Fn
				DocsToInvestigate?:           *string | fn.#Fn
				ExtractDocId?:                *string | fn.#Fn
				NestingLevel?:                *string | fn.#Fn
				Password?:                    *string | fn.#Fn
				Port?:                        *int | fn.#Fn
				SecretsManagerAccessRoleArn?: *string | fn.#Fn
				SecretsManagerSecretId?:      *string | fn.#Fn
				ServerName?:                  *string | fn.#Fn
				Username?:                    *string | fn.#Fn
			} | fn.#If
			MySqlSettings?: *{
				SecretsManagerAccessRoleArn?: *string | fn.#Fn
				SecretsManagerSecretId?:      *string | fn.#Fn
			} | fn.#If
			NeptuneSettings?: *{
				ErrorRetryDuration?:   *int | fn.#Fn
				IamAuthEnabled?:       *bool | fn.#Fn
				MaxFileSize?:          *int | fn.#Fn
				MaxRetryCount?:        *int | fn.#Fn
				S3BucketFolder?:       *string | fn.#Fn
				S3BucketName?:         *string | fn.#Fn
				ServiceAccessRoleArn?: *string | fn.#Fn
			} | fn.#If
			OracleSettings?: *{
				SecretsManagerAccessRoleArn?:          *string | fn.#Fn
				SecretsManagerOracleAsmAccessRoleArn?: *string | fn.#Fn
				SecretsManagerOracleAsmSecretId?:      *string | fn.#Fn
				SecretsManagerSecretId?:               *string | fn.#Fn
			} | fn.#If
			Password?:           *string | fn.#Fn
			Port?:               *int | fn.#Fn
			PostgreSqlSettings?: *{
				SecretsManagerAccessRoleArn?: *string | fn.#Fn
				SecretsManagerSecretId?:      *string | fn.#Fn
			} | fn.#If
			RedisSettings?: *{
				AuthPassword?:        *string | fn.#Fn
				AuthType?:            *string | fn.#Fn
				AuthUserName?:        *string | fn.#Fn
				Port?:                *number | fn.#Fn
				ServerName?:          *string | fn.#Fn
				SslCaCertificateArn?: *string | fn.#Fn
				SslSecurityProtocol?: *string | fn.#Fn
			} | fn.#If
			RedshiftSettings?: *{
				SecretsManagerAccessRoleArn?: *string | fn.#Fn
				SecretsManagerSecretId?:      *string | fn.#Fn
			} | fn.#If
			ResourceIdentifier?: *string | fn.#Fn
			S3Settings?:         *{
				BucketFolder?:            *string | fn.#Fn
				BucketName?:              *string | fn.#Fn
				CompressionType?:         *string | fn.#Fn
				CsvDelimiter?:            *string | fn.#Fn
				CsvRowDelimiter?:         *string | fn.#Fn
				ExternalTableDefinition?: *string | fn.#Fn
				ServiceAccessRoleArn?:    *string | fn.#Fn
			} | fn.#If
			ServerName?:     *string | fn.#Fn
			SslMode?:        *("none" | "require" | "verify-ca" | "verify-full") | fn.#Fn
			SybaseSettings?: *{
				SecretsManagerAccessRoleArn?: *string | fn.#Fn
				SecretsManagerSecretId?:      *string | fn.#Fn
			} | fn.#If
			Tags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			Username?: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#EventSubscription: {
		Type: "AWS::DMS::EventSubscription"
		Properties: {
			Enabled?:          *bool | fn.#Fn
			EventCategories?:  [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			SnsTopicArn:       *string | fn.#Fn
			SourceIds?:        [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			SourceType?:       *string | fn.#Fn
			SubscriptionName?: *string | fn.#Fn
			Tags?:             *[...{
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
	#ReplicationInstance: {
		Type: "AWS::DMS::ReplicationInstance"
		Properties: {
			AllocatedStorage?:                 *int | fn.#Fn
			AllowMajorVersionUpgrade?:         *bool | fn.#Fn
			AutoMinorVersionUpgrade?:          *bool | fn.#Fn
			AvailabilityZone?:                 *(=~#"[a-z0-9-]+"#) | fn.#Fn
			EngineVersion?:                    *string | fn.#Fn
			KmsKeyId?:                         *string | fn.#Fn
			MultiAZ?:                          *bool | fn.#Fn
			PreferredMaintenanceWindow?:       *string | fn.#Fn
			PubliclyAccessible?:               *bool | fn.#Fn
			ReplicationInstanceClass:          *string | fn.#Fn
			ReplicationInstanceIdentifier?:    *string | fn.#Fn
			ReplicationSubnetGroupIdentifier?: *string | fn.#Fn
			ResourceIdentifier?:               *string | fn.#Fn
			Tags?:                             *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			VpcSecurityGroupIds?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ReplicationSubnetGroup: {
		Type: "AWS::DMS::ReplicationSubnetGroup"
		Properties: {
			ReplicationSubnetGroupDescription: *string | fn.#Fn
			ReplicationSubnetGroupIdentifier?: *string | fn.#Fn
			SubnetIds:                         [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			Tags?:                             *[...{
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
	#ReplicationTask: {
		Type: "AWS::DMS::ReplicationTask"
		Properties: {
			CdcStartPosition?:          *string | fn.#Fn
			CdcStartTime?:              *number | fn.#Fn
			CdcStopPosition?:           *string | fn.#Fn
			MigrationType:              *string | fn.#Fn
			ReplicationInstanceArn:     *string | fn.#Fn
			ReplicationTaskIdentifier?: *string | fn.#Fn
			ReplicationTaskSettings?:   *string | fn.#Fn
			ResourceIdentifier?:        *string | fn.#Fn
			SourceEndpointArn:          *string | fn.#Fn
			TableMappings:              *string | fn.#Fn
			Tags?:                      *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			TargetEndpointArn: *string | fn.#Fn
			TaskData?:         *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
