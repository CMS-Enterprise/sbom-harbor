package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#Redshift: {
	#Cluster: {
		Type: "AWS::Redshift::Cluster"
		Properties: {
			AllowVersionUpgrade?:              *bool | fn.#Fn
			AquaConfigurationStatus?:          *string | fn.#Fn
			AutomatedSnapshotRetentionPeriod?: *int | fn.#Fn
			AvailabilityZone?:                 *string | fn.#Fn
			AvailabilityZoneRelocation?:       *bool | fn.#Fn
			AvailabilityZoneRelocationStatus?: *string | fn.#Fn
			Classic?:                          *bool | fn.#Fn
			ClusterIdentifier?:                *string | fn.#Fn
			ClusterParameterGroupName?:        *string | fn.#Fn
			ClusterSecurityGroups?:            [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			ClusterSubnetGroupName?:           *string | fn.#Fn
			ClusterType:                       *("multi-node" | "single-node") | fn.#Fn
			ClusterVersion?:                   *("1.0") | fn.#Fn
			DBName:                            *string | fn.#Fn
			DeferMaintenance?:                 *bool | fn.#Fn
			DeferMaintenanceDuration?:         *int | fn.#Fn
			DeferMaintenanceEndTime?:          *string | fn.#Fn
			DeferMaintenanceStartTime?:        *string | fn.#Fn
			DestinationRegion?:                *string | fn.#Fn
			ElasticIp?:                        *string | fn.#Fn
			Encrypted?:                        *bool | fn.#Fn
			EnhancedVpcRouting?:               *bool | fn.#Fn
			HsmClientCertificateIdentifier?:   *string | fn.#Fn
			HsmConfigurationIdentifier?:       *string | fn.#Fn
			IamRoles?:                         [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			KmsKeyId?:                         *string | fn.#Fn
			LoggingProperties?:                *{
				BucketName:   *string | fn.#Fn
				S3KeyPrefix?: *string | fn.#Fn
			} | fn.#If
			MaintenanceTrackName?:          *string | fn.#Fn
			ManualSnapshotRetentionPeriod?: *int | fn.#Fn
			MasterUserPassword:             *string | fn.#Fn
			MasterUsername:                 *string | fn.#Fn
			NodeType:                       *("dc1.8xlarge" | "dc1.large" | "dc2.8xlarge" | "dc2.large" | "ds1.8xlarge" | "ds1.xlarge" | "ds2.8xlarge" | "ds2.xlarge" | "ra3.16xlarge" | "ra3.4xlarge" | "ra3.xlplus") | fn.#Fn
			NumberOfNodes?:                 *(>=1 & <=100) | fn.#Fn
			OwnerAccount?:                  *string | fn.#Fn
			Port?:                          *int | fn.#Fn
			PreferredMaintenanceWindow?:    *string | fn.#Fn
			PubliclyAccessible?:            *bool | fn.#Fn
			ResourceAction?:                *string | fn.#Fn
			RevisionTarget?:                *string | fn.#Fn
			RotateEncryptionKey?:           *bool | fn.#Fn
			SnapshotClusterIdentifier?:     *string | fn.#Fn
			SnapshotCopyGrantName?:         *string | fn.#Fn
			SnapshotCopyManual?:            *bool | fn.#Fn
			SnapshotCopyRetentionPeriod?:   *int | fn.#Fn
			SnapshotIdentifier?:            *string | fn.#Fn
			Tags?:                          *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			VpcSecurityGroupIds?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain" | "Snapshot"
		UpdateReplacePolicy?: "Delete" | "Retain" | "Snapshot"
		Metadata?: [string]: _
		Condition?: string
	}
	#ClusterParameterGroup: {
		Type: "AWS::Redshift::ClusterParameterGroup"
		Properties: {
			Description:          *string | fn.#Fn
			ParameterGroupFamily: *string | fn.#Fn
			Parameters?:          *[...{
				ParameterName:  *string | fn.#Fn
				ParameterValue: *string | fn.#Fn
			}] | fn.#If
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
	#ClusterSecurityGroup: {
		Type: "AWS::Redshift::ClusterSecurityGroup"
		Properties: {
			Description: *string | fn.#Fn
			Tags?:       *[...{
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
	#ClusterSecurityGroupIngress: {
		Type: "AWS::Redshift::ClusterSecurityGroupIngress"
		Properties: {
			CIDRIP?:                  *(=~#"^(([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5]).){3}([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])(\/([0-9]|[1-2][0-9]|3[0-2]))$"#) | fn.#Fn
			ClusterSecurityGroupName: *string | fn.#Fn
			EC2SecurityGroupName?:    *string | fn.#Fn
			EC2SecurityGroupOwnerId?: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ClusterSubnetGroup: {
		Type: "AWS::Redshift::ClusterSubnetGroup"
		Properties: {
			Description: *string | fn.#Fn
			SubnetIds:   [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			Tags?:       *[...{
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
	#EndpointAccess: {
		Type: "AWS::Redshift::EndpointAccess"
		Properties: {
			ClusterIdentifier?:  *string | fn.#Fn
			EndpointName:        *(=~#"^(?=^[a-z][a-z0-9]*(-[a-z0-9]+)*$).{1,30}$"#) | fn.#Fn
			ResourceOwner?:      *(=~#"^\d{12}$"#) | fn.#Fn
			SubnetGroupName?:    *(=~#"^(?=^[a-zA-Z0-9-]+$).{1,255}$"#) | fn.#Fn
			VpcSecurityGroupIds: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#EndpointAuthorization: {
		Type: "AWS::Redshift::EndpointAuthorization"
		Properties: {
			Account:           *(=~#"^\d{12}$"#) | fn.#Fn
			ClusterIdentifier: *(=~#"^(?=^[a-z][a-z0-9]*(-[a-z0-9]+)*$).{1,63}$"#) | fn.#Fn
			Force?:            *bool | fn.#Fn
			VpcIds?:           [...(*(=~#"^vpc-\d{1,17}$"#) | fn.#Fn)] | (*(=~#"^vpc-\d{1,17}$"#) | fn.#Fn)
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#EventSubscription: {
		Type: "AWS::Redshift::EventSubscription"
		Properties: {
			Enabled?:         *bool | fn.#Fn
			EventCategories?: [...(*("configuration" | "management" | "monitoring" | "security" | "pending") | fn.#Fn)] | (*("configuration" | "management" | "monitoring" | "security" | "pending") | fn.#Fn)
			Severity?:        *("ERROR" | "INFO") | fn.#Fn
			SnsTopicArn?:     *string | fn.#Fn
			SourceIds?:       [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			SourceType?:      *("cluster" | "cluster-parameter-group" | "cluster-security-group" | "cluster-snapshot" | "scheduled-action") | fn.#Fn
			SubscriptionName: *(=~#"^(?=^[a-z][a-z0-9]*(-[a-z0-9]+)*$).{1,255}$"#) | fn.#Fn
			Tags?:            *[...{
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
	#ScheduledAction: {
		Type: "AWS::Redshift::ScheduledAction"
		Properties: {
			Enable?:                     *bool | fn.#Fn
			EndTime?:                    *string | fn.#Fn
			IamRole?:                    *string | fn.#Fn
			Schedule?:                   *string | fn.#Fn
			ScheduledActionDescription?: *(=~#"^(?=^[\x09\x0a\x0d\x20-\xff]*$).{1,255}$"#) | fn.#Fn
			ScheduledActionName:         *(=~#"^(?=^[a-z][a-z0-9]*(-[a-z0-9]+)*$).{1,60}$"#) | fn.#Fn
			StartTime?:                  *string | fn.#Fn
			TargetAction?:               *{
				PauseCluster?: *{
					ClusterIdentifier: *string | fn.#Fn
				} | fn.#If
				ResizeCluster?: *{
					Classic?:          *bool | fn.#Fn
					ClusterIdentifier: *string | fn.#Fn
					ClusterType?:      *string | fn.#Fn
					NodeType?:         *string | fn.#Fn
					NumberOfNodes?:    *int | fn.#Fn
				} | fn.#If
				ResumeCluster?: *{
					ClusterIdentifier: *string | fn.#Fn
				} | fn.#If
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
