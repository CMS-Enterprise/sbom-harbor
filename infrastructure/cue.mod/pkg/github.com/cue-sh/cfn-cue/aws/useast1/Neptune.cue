package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#Neptune: {
	#DBCluster: {
		Type: "AWS::Neptune::DBCluster"
		Properties: {
			AssociatedRoles?: *[...{
				FeatureName?: *string | fn.#Fn
				RoleArn:      *string | fn.#Fn
			}] | fn.#If
			AvailabilityZones?:           [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			BackupRetentionPeriod?:       *int | fn.#Fn
			DBClusterIdentifier?:         *string | fn.#Fn
			DBClusterParameterGroupName?: *string | fn.#Fn
			DBSubnetGroupName?:           *string | fn.#Fn
			DeletionProtection?:          *bool | fn.#Fn
			EnableCloudwatchLogsExports?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			EngineVersion?:               *string | fn.#Fn
			IamAuthEnabled?:              *bool | fn.#Fn
			KmsKeyId?:                    *string | fn.#Fn
			Port?:                        *int | fn.#Fn
			PreferredBackupWindow?:       *string | fn.#Fn
			PreferredMaintenanceWindow?:  *string | fn.#Fn
			RestoreToTime?:               *string | fn.#Fn
			RestoreType?:                 *string | fn.#Fn
			SnapshotIdentifier?:          *string | fn.#Fn
			SourceDBClusterIdentifier?:   *string | fn.#Fn
			StorageEncrypted?:            *bool | fn.#Fn
			Tags?:                        *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			UseLatestRestorableTime?: *bool | fn.#Fn
			VpcSecurityGroupIds?:     [...(*string | fn.#Fn)] | (*string | fn.#Fn)
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain" | "Snapshot"
		UpdateReplacePolicy?: "Delete" | "Retain" | "Snapshot"
		Metadata?: [string]: _
		Condition?: string
	}
	#DBClusterParameterGroup: {
		Type: "AWS::Neptune::DBClusterParameterGroup"
		Properties: {
			Description: *string | fn.#Fn
			Family:      *string | fn.#Fn
			Name?:       *string | fn.#Fn
			Parameters:  *{
				[string]: _
			} | fn.#Fn
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
	#DBInstance: {
		Type: "AWS::Neptune::DBInstance"
		Properties: {
			AllowMajorVersionUpgrade?:   *bool | fn.#Fn
			AutoMinorVersionUpgrade?:    *bool | fn.#Fn
			AvailabilityZone?:           *string | fn.#Fn
			DBClusterIdentifier?:        *string | fn.#Fn
			DBInstanceClass:             *("db.r4.2xlarge" | "db.r4.4xlarge" | "db.r4.8xlarge" | "db.r4.large" | "db.r4.xlarge" | "db.r5.12xlarge" | "db.r5.16xlarge" | "db.r5.24xlarge" | "db.r5.2xlarge" | "db.r5.4xlarge" | "db.r5.8xlarge" | "db.r5.large" | "db.r5.xlarge" | "db.r5d.12xlarge" | "db.r5d.16xlarge" | "db.r5d.24xlarge" | "db.r5d.2xlarge" | "db.r5d.4xlarge" | "db.r5d.8xlarge" | "db.r5d.large" | "db.r5d.xlarge" | "db.r6g.12xlarge" | "db.r6g.16xlarge" | "db.r6g.2xlarge" | "db.r6g.4xlarge" | "db.r6g.8xlarge" | "db.r6g.large" | "db.r6g.xlarge" | "db.t3.medium" | "db.t4g.medium") | fn.#Fn
			DBInstanceIdentifier?:       *string | fn.#Fn
			DBParameterGroupName?:       *string | fn.#Fn
			DBSnapshotIdentifier?:       *string | fn.#Fn
			DBSubnetGroupName?:          *string | fn.#Fn
			PreferredMaintenanceWindow?: *string | fn.#Fn
			Tags?:                       *[...{
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
	#DBParameterGroup: {
		Type: "AWS::Neptune::DBParameterGroup"
		Properties: {
			Description: *string | fn.#Fn
			Family:      *string | fn.#Fn
			Name?:       *string | fn.#Fn
			Parameters:  *{
				[string]: _
			} | fn.#Fn
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
	#DBSubnetGroup: {
		Type: "AWS::Neptune::DBSubnetGroup"
		Properties: {
			DBSubnetGroupDescription: *string | fn.#Fn
			DBSubnetGroupName?:       *string | fn.#Fn
			SubnetIds:                [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			Tags?:                    *[...{
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
