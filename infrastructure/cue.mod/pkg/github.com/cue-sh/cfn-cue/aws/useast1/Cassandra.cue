package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#Cassandra: {
	#Keyspace: {
		Type: "AWS::Cassandra::Keyspace"
		Properties: {
			KeyspaceName?: *(=~#"^[a-zA-Z0-9][a-zA-Z0-9_]{1,47}$"#) | fn.#Fn
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
	#Table: {
		Type: "AWS::Cassandra::Table"
		Properties: {
			BillingMode?: *{
				Mode:                   *("PROVISIONED" | "ON_DEMAND") | fn.#Fn
				ProvisionedThroughput?: *{
					ReadCapacityUnits:  *int | fn.#Fn
					WriteCapacityUnits: *int | fn.#Fn
				} | fn.#If
			} | fn.#If
			ClusteringKeyColumns?: *[...{
				Column: *{
					ColumnName: *(=~#"^[a-zA-Z0-9][a-zA-Z0-9_]{1,47}$"#) | fn.#Fn
					ColumnType: *string | fn.#Fn
				} | fn.#If
				OrderBy?: *("ASC" | "DESC") | fn.#Fn
			}] | fn.#If
			DefaultTimeToLive?:       *int | fn.#Fn
			EncryptionSpecification?: *{
				EncryptionType:    *("AWS_OWNED_KMS_KEY" | "CUSTOMER_MANAGED_KMS_KEY") | fn.#Fn
				KmsKeyIdentifier?: *string | fn.#Fn
			} | fn.#If
			KeyspaceName:        *(=~#"^[a-zA-Z0-9][a-zA-Z0-9_]{1,47}$"#) | fn.#Fn
			PartitionKeyColumns: *[...{
				ColumnName: *(=~#"^[a-zA-Z0-9][a-zA-Z0-9_]{1,47}$"#) | fn.#Fn
				ColumnType: *string | fn.#Fn
			}] | fn.#If
			PointInTimeRecoveryEnabled?: *bool | fn.#Fn
			RegularColumns?:             *[...{
				ColumnName: *(=~#"^[a-zA-Z0-9][a-zA-Z0-9_]{1,47}$"#) | fn.#Fn
				ColumnType: *string | fn.#Fn
			}] | fn.#If
			TableName?: *(=~#"^[a-zA-Z0-9][a-zA-Z0-9_]{1,47}$"#) | fn.#Fn
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
