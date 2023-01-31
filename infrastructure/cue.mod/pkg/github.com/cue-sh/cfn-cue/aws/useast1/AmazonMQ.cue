package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#AmazonMQ: {
	#Broker: {
		Type: "AWS::AmazonMQ::Broker"
		Properties: {
			AuthenticationStrategy?: *string | fn.#Fn
			AutoMinorVersionUpgrade: *bool | fn.#Fn
			BrokerName:              *string | fn.#Fn
			Configuration?:          *{
				Id:       *string | fn.#Fn
				Revision: *int | fn.#Fn
			} | fn.#If
			DeploymentMode:     *("ACTIVE_STANDBY_MULTI_AZ" | "CLUSTER_MULTI_AZ" | "SINGLE_INSTANCE") | fn.#Fn
			EncryptionOptions?: *{
				KmsKeyId?:      *string | fn.#Fn
				UseAwsOwnedKey: *bool | fn.#Fn
			} | fn.#If
			EngineType:          *("ACTIVEMQ" | "RABBITMQ") | fn.#Fn
			EngineVersion:       *string | fn.#Fn
			HostInstanceType:    *("mq.m4.large" | "mq.m5.2xlarge" | "mq.m5.4xlarge" | "mq.m5.large" | "mq.m5.xlarge" | "mq.t2.micro" | "mq.t3.micro") | fn.#Fn
			LdapServerMetadata?: *{
				Hosts:                  [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				RoleBase:               *string | fn.#Fn
				RoleName?:              *string | fn.#Fn
				RoleSearchMatching:     *string | fn.#Fn
				RoleSearchSubtree?:     *bool | fn.#Fn
				ServiceAccountPassword: *string | fn.#Fn
				ServiceAccountUsername: *string | fn.#Fn
				UserBase:               *string | fn.#Fn
				UserRoleName?:          *string | fn.#Fn
				UserSearchMatching:     *string | fn.#Fn
				UserSearchSubtree?:     *bool | fn.#Fn
			} | fn.#If
			Logs?: *{
				Audit?:   *bool | fn.#Fn
				General?: *bool | fn.#Fn
			} | fn.#If
			MaintenanceWindowStartTime?: *{
				DayOfWeek: *string | fn.#Fn
				TimeOfDay: *string | fn.#Fn
				TimeZone:  *string | fn.#Fn
			} | fn.#If
			PubliclyAccessible: *bool | fn.#Fn
			SecurityGroups?:    [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			StorageType?:       *string | fn.#Fn
			SubnetIds?:         [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			Tags?:              *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			Users: *[...{
				ConsoleAccess?: *bool | fn.#Fn
				Groups?:        [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				Password:       *string | fn.#Fn
				Username:       *string | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Configuration: {
		Type: "AWS::AmazonMQ::Configuration"
		Properties: {
			AuthenticationStrategy?: *string | fn.#Fn
			Data:                    *string | fn.#Fn
			Description?:            *string | fn.#Fn
			EngineType:              *("ACTIVEMQ" | "RABBITMQ") | fn.#Fn
			EngineVersion:           *string | fn.#Fn
			Name:                    *string | fn.#Fn
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
	#ConfigurationAssociation: {
		Type: "AWS::AmazonMQ::ConfigurationAssociation"
		Properties: {
			Broker:        *string | fn.#Fn
			Configuration: *{
				Id:       *string | fn.#Fn
				Revision: *int | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
