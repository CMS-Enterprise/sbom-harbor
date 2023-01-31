package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#SecretsManager: {
	#ResourcePolicy: {
		Type: "AWS::SecretsManager::ResourcePolicy"
		Properties: {
			BlockPublicPolicy?: *bool | fn.#Fn
			ResourcePolicy:     *{
				[string]: _
			} | fn.#Fn
			SecretId: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#RotationSchedule: {
		Type: "AWS::SecretsManager::RotationSchedule"
		Properties: {
			HostedRotationLambda?: *{
				KmsKeyArn?:                *string | fn.#Fn
				MasterSecretArn?:          *string | fn.#Fn
				MasterSecretKmsKeyArn?:    *string | fn.#Fn
				RotationLambdaName?:       *string | fn.#Fn
				RotationType:              *string | fn.#Fn
				SuperuserSecretArn?:       *string | fn.#Fn
				SuperuserSecretKmsKeyArn?: *string | fn.#Fn
				VpcSecurityGroupIds?:      *string | fn.#Fn
				VpcSubnetIds?:             *string | fn.#Fn
			} | fn.#If
			RotationLambdaARN?: *string | fn.#Fn
			RotationRules?:     *{
				AutomaticallyAfterDays?: *int | fn.#Fn
			} | fn.#If
			SecretId: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Secret: {
		Type: "AWS::SecretsManager::Secret"
		Properties: {
			Description?:          *string | fn.#Fn
			GenerateSecretString?: *{
				ExcludeCharacters?:       *string | fn.#Fn
				ExcludeLowercase?:        *bool | fn.#Fn
				ExcludeNumbers?:          *bool | fn.#Fn
				ExcludePunctuation?:      *bool | fn.#Fn
				ExcludeUppercase?:        *bool | fn.#Fn
				GenerateStringKey?:       *string | fn.#Fn
				IncludeSpace?:            *bool | fn.#Fn
				PasswordLength?:          *int | fn.#Fn
				RequireEachIncludedType?: *bool | fn.#Fn
				SecretStringTemplate?:    *string | fn.#Fn
			} | fn.#If
			KmsKeyId?:       *string | fn.#Fn
			Name?:           *string | fn.#Fn
			ReplicaRegions?: *[...{
				KmsKeyId?: *string | fn.#Fn
				Region:    *string | fn.#Fn
			}] | fn.#If
			SecretString?: *string | fn.#Fn
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
	#SecretTargetAttachment: {
		Type: "AWS::SecretsManager::SecretTargetAttachment"
		Properties: {
			SecretId:   *string | fn.#Fn
			TargetId:   *string | fn.#Fn
			TargetType: *("AWS::DocDB::DBCluster" | "AWS::DocDB::DBInstance" | "AWS::RDS::DBCluster" | "AWS::RDS::DBInstance" | "AWS::Redshift::Cluster") | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
