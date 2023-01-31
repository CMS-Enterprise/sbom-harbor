package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#RDS: {
	#DBCluster: {
		Type: "AWS::RDS::DBCluster"
		Properties: {
			AssociatedRoles?: *[...{
				FeatureName?: *string | fn.#Fn
				RoleArn:      *string | fn.#Fn
			}] | fn.#If
			AvailabilityZones?:               [...(*(=~#"[a-z0-9-]+"#) | fn.#Fn)] | (*(=~#"[a-z0-9-]+"#) | fn.#Fn)
			BacktrackWindow?:                 *int | fn.#Fn
			BackupRetentionPeriod?:           *(>=1 & <=35) | fn.#Fn
			CopyTagsToSnapshot?:              *bool | fn.#Fn
			DBClusterIdentifier?:             *string | fn.#Fn
			DBClusterParameterGroupName?:     *string | fn.#Fn
			DBSubnetGroupName?:               *string | fn.#Fn
			DatabaseName?:                    *string | fn.#Fn
			DeletionProtection?:              *bool | fn.#Fn
			EnableCloudwatchLogsExports?:     [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			EnableHttpEndpoint?:              *bool | fn.#Fn
			EnableIAMDatabaseAuthentication?: *bool | fn.#Fn
			Engine:                           *string | fn.#Fn
			EngineMode?:                      *string | fn.#Fn
			EngineVersion?:                   *string | fn.#Fn
			GlobalClusterIdentifier?:         *string | fn.#Fn
			KmsKeyId?:                        *string | fn.#Fn
			MasterUserPassword?:              *string | fn.#Fn
			MasterUsername?:                  *string | fn.#Fn
			Port?:                            *int | fn.#Fn
			PreferredBackupWindow?:           *string | fn.#Fn
			PreferredMaintenanceWindow?:      *string | fn.#Fn
			ReplicationSourceIdentifier?:     *string | fn.#Fn
			RestoreType?:                     *string | fn.#Fn
			ScalingConfiguration?:            *{
				AutoPause?:             *bool | fn.#Fn
				MaxCapacity?:           *int | fn.#Fn
				MinCapacity?:           *int | fn.#Fn
				SecondsUntilAutoPause?: *int | fn.#Fn
			} | fn.#If
			SnapshotIdentifier?:        *string | fn.#Fn
			SourceDBClusterIdentifier?: *string | fn.#Fn
			SourceRegion?:              *string | fn.#Fn
			StorageEncrypted?:          *bool | fn.#Fn
			Tags?:                      *[...{
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
		Type: "AWS::RDS::DBClusterParameterGroup"
		Properties: {
			Description: *string | fn.#Fn
			Family:      *string | fn.#Fn
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
		Type: "AWS::RDS::DBInstance"
		Properties: {
			AllocatedStorage?:         *string | fn.#Fn
			AllowMajorVersionUpgrade?: *bool | fn.#Fn
			AssociatedRoles?:          *[...{
				FeatureName: *string | fn.#Fn
				RoleArn:     *string | fn.#Fn
			}] | fn.#If
			AutoMinorVersionUpgrade?:            *bool | fn.#Fn
			AvailabilityZone?:                   *(=~#"[a-z0-9-]+"#) | fn.#Fn
			BackupRetentionPeriod?:              *(>=0 & <=35) | fn.#Fn
			CACertificateIdentifier?:            *string | fn.#Fn
			CharacterSetName?:                   *string | fn.#Fn
			CopyTagsToSnapshot?:                 *bool | fn.#Fn
			DBClusterIdentifier?:                *string | fn.#Fn
			DBInstanceClass:                     *("db.cv11.18xlarge" | "db.cv11.2xlarge" | "db.cv11.4xlarge" | "db.cv11.9xlarge" | "db.cv11.large" | "db.cv11.medium" | "db.cv11.small" | "db.cv11.xlarge" | "db.m1.large" | "db.m1.medium" | "db.m1.small" | "db.m1.xlarge" | "db.m2.2xlarge" | "db.m2.4xlarge" | "db.m2.xlarge" | "db.m3.2xlarge" | "db.m3.large" | "db.m3.medium" | "db.m3.xlarge" | "db.m4.10xlarge" | "db.m4.16xlarge" | "db.m4.2xlarge" | "db.m4.4xlarge" | "db.m4.large" | "db.m4.xlarge" | "db.m5.12xlarge" | "db.m5.16xlarge" | "db.m5.24xlarge" | "db.m5.2xlarge" | "db.m5.4xlarge" | "db.m5.8xlarge" | "db.m5.large" | "db.m5.xlarge" | "db.m5d.12xlarge" | "db.m5d.16xlarge" | "db.m5d.24xlarge" | "db.m5d.2xlarge" | "db.m5d.4xlarge" | "db.m5d.8xlarge" | "db.m5d.large" | "db.m5d.xlarge" | "db.m6g.12xlarge" | "db.m6g.16xlarge" | "db.m6g.2xlarge" | "db.m6g.4xlarge" | "db.m6g.8xlarge" | "db.m6g.large" | "db.m6g.xlarge" | "db.m6gd.12xlarge" | "db.m6gd.16xlarge" | "db.m6gd.2xlarge" | "db.m6gd.4xlarge" | "db.m6gd.8xlarge" | "db.m6gd.large" | "db.m6gd.xlarge" | "db.mv11.12xlarge" | "db.mv11.24xlarge" | "db.mv11.2xlarge" | "db.mv11.4xlarge" | "db.mv11.large" | "db.mv11.medium" | "db.mv11.xlarge" | "db.r3.2xlarge" | "db.r3.4xlarge" | "db.r3.8xlarge" | "db.r3.large" | "db.r3.xlarge" | "db.r4.16xlarge" | "db.r4.2xlarge" | "db.r4.4xlarge" | "db.r4.8xlarge" | "db.r4.large" | "db.r4.xlarge" | "db.r5.12xlarge" | "db.r5.12xlarge.tpc2.mem2x" | "db.r5.16xlarge" | "db.r5.24xlarge" | "db.r5.2xlarge" | "db.r5.2xlarge.tpc1.mem2x" | "db.r5.2xlarge.tpc2.mem4x" | "db.r5.2xlarge.tpc2.mem8x" | "db.r5.4xlarge" | "db.r5.4xlarge.tpc2.mem2x" | "db.r5.4xlarge.tpc2.mem3x" | "db.r5.4xlarge.tpc2.mem4x" | "db.r5.6xlarge.tpc2.mem4x" | "db.r5.8xlarge" | "db.r5.8xlarge.tpc2.mem3x" | "db.r5.large" | "db.r5.large.tpc1.mem2x" | "db.r5.xlarge" | "db.r5.xlarge.tpc2.mem2x" | "db.r5.xlarge.tpc2.mem4x" | "db.r5b.12xlarge" | "db.r5b.16xlarge" | "db.r5b.24xlarge" | "db.r5b.2xlarge" | "db.r5b.4xlarge" | "db.r5b.8xlarge" | "db.r5b.large" | "db.r5b.xlarge" | "db.r5d.12xlarge" | "db.r5d.16xlarge" | "db.r5d.24xlarge" | "db.r5d.2xlarge" | "db.r5d.4xlarge" | "db.r5d.8xlarge" | "db.r5d.large" | "db.r5d.xlarge" | "db.r6g.12xlarge" | "db.r6g.16xlarge" | "db.r6g.2xlarge" | "db.r6g.4xlarge" | "db.r6g.8xlarge" | "db.r6g.large" | "db.r6g.xlarge" | "db.r6gd.12xlarge" | "db.r6gd.16xlarge" | "db.r6gd.2xlarge" | "db.r6gd.4xlarge" | "db.r6gd.8xlarge" | "db.r6gd.large" | "db.r6gd.xlarge" | "db.rv11.12xlarge" | "db.rv11.24xlarge" | "db.rv11.2xlarge" | "db.rv11.4xlarge" | "db.rv11.large" | "db.rv11.xlarge" | "db.t1.micro" | "db.t2.2xlarge" | "db.t2.large" | "db.t2.medium" | "db.t2.micro" | "db.t2.small" | "db.t2.xlarge" | "db.t3.2xlarge" | "db.t3.large" | "db.t3.medium" | "db.t3.micro" | "db.t3.small" | "db.t3.xlarge" | "db.t4g.2xlarge" | "db.t4g.large" | "db.t4g.medium" | "db.t4g.micro" | "db.t4g.small" | "db.t4g.xlarge" | "db.x1.16xlarge" | "db.x1.32xlarge" | "db.x1e.16xlarge" | "db.x1e.2xlarge" | "db.x1e.32xlarge" | "db.x1e.4xlarge" | "db.x1e.8xlarge" | "db.x1e.xlarge" | "db.x2g.12xlarge" | "db.x2g.16xlarge" | "db.x2g.2xlarge" | "db.x2g.4xlarge" | "db.x2g.8xlarge" | "db.x2g.large" | "db.x2g.xlarge" | "db.z1d.12xlarge" | "db.z1d.2xlarge" | "db.z1d.3xlarge" | "db.z1d.6xlarge" | "db.z1d.large" | "db.z1d.xlarge") | fn.#Fn
			DBInstanceIdentifier?:               *string | fn.#Fn
			DBName?:                             *string | fn.#Fn
			DBParameterGroupName?:               *string | fn.#Fn
			DBSecurityGroups?:                   [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			DBSnapshotIdentifier?:               *string | fn.#Fn
			DBSubnetGroupName?:                  *string | fn.#Fn
			DeleteAutomatedBackups?:             *bool | fn.#Fn
			DeletionProtection?:                 *bool | fn.#Fn
			Domain?:                             *string | fn.#Fn
			DomainIAMRoleName?:                  *string | fn.#Fn
			EnableCloudwatchLogsExports?:        [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			EnableIAMDatabaseAuthentication?:    *bool | fn.#Fn
			EnablePerformanceInsights?:          *bool | fn.#Fn
			Engine?:                             *(=~#"(?i)(aurora|aurora-mysql|aurora-postgresql|mariadb|mysql|oracle-ee|oracle-se2|oracle-se1|oracle-se|postgres|sqlserver-ee|sqlserver-se|sqlserver-ex|sqlserver-web)$"#) | fn.#Fn
			EngineVersion?:                      *string | fn.#Fn
			Iops?:                               *int | fn.#Fn
			KmsKeyId?:                           *string | fn.#Fn
			LicenseModel?:                       *string | fn.#Fn
			MasterUserPassword?:                 *string | fn.#Fn
			MasterUsername?:                     *string | fn.#Fn
			MaxAllocatedStorage?:                *int | fn.#Fn
			MonitoringInterval?:                 *(0 | 1 | 5 | 10 | 15 | 30 | 60) | fn.#Fn
			MonitoringRoleArn?:                  *string | fn.#Fn
			MultiAZ?:                            *bool | fn.#Fn
			OptionGroupName?:                    *string | fn.#Fn
			PerformanceInsightsKMSKeyId?:        *string | fn.#Fn
			PerformanceInsightsRetentionPeriod?: *(7 | 731) | fn.#Fn
			Port?:                               *string | fn.#Fn
			PreferredBackupWindow?:              *string | fn.#Fn
			PreferredMaintenanceWindow?:         *string | fn.#Fn
			ProcessorFeatures?:                  *[...{
				Name?:  *string | fn.#Fn
				Value?: *string | fn.#Fn
			}] | fn.#If
			PromotionTier?:              *(>=0 & <=15) | fn.#Fn
			PubliclyAccessible?:         *bool | fn.#Fn
			SourceDBInstanceIdentifier?: *string | fn.#Fn
			SourceRegion?:               *string | fn.#Fn
			StorageEncrypted?:           *bool | fn.#Fn
			StorageType?:                *string | fn.#Fn
			Tags?:                       *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			Timezone?:                    *string | fn.#Fn
			UseDefaultProcessorFeatures?: *bool | fn.#Fn
			VPCSecurityGroups?:           [...(*string | fn.#Fn)] | (*string | fn.#Fn)
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain" | "Snapshot"
		UpdateReplacePolicy?: "Delete" | "Retain" | "Snapshot"
		Metadata?: [string]: _
		Condition?: string
	}
	#DBParameterGroup: {
		Type: "AWS::RDS::DBParameterGroup"
		Properties: {
			Description: *string | fn.#Fn
			Family:      *string | fn.#Fn
			Parameters?: *{
				[string]: *string | fn.#Fn
			} | fn.#If
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
	#DBProxy: {
		Type: "AWS::RDS::DBProxy"
		Properties: {
			Auth: *[...{
				AuthScheme?:  *("SECRETS") | fn.#Fn
				Description?: *string | fn.#Fn
				IAMAuth?:     *("DISABLED" | "REQUIRED") | fn.#Fn
				SecretArn?:   *string | fn.#Fn
				UserName?:    *string | fn.#Fn
			}] | fn.#If
			DBProxyName:        *(=~#"[0-z]*"#) | fn.#Fn
			DebugLogging?:      *bool | fn.#Fn
			EngineFamily:       *("MYSQL" | "POSTGRESQL") | fn.#Fn
			IdleClientTimeout?: *int | fn.#Fn
			RequireTLS?:        *bool | fn.#Fn
			RoleArn:            *string | fn.#Fn
			Tags?:              *[...{
				Key?:   *(=~#"(\w|\d|\s|\\|-|\.:=+-)*"#) | fn.#Fn
				Value?: *(=~#"(\w|\d|\s|\\|-|\.:=+-)*"#) | fn.#Fn
			}] | fn.#If
			VpcSecurityGroupIds?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			VpcSubnetIds:         [...(*string | fn.#Fn)] | (*string | fn.#Fn)
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#DBProxyEndpoint: {
		Type: "AWS::RDS::DBProxyEndpoint"
		Properties: {
			DBProxyEndpointName: *(=~#"[0-z]*"#) | fn.#Fn
			DBProxyName:         *(=~#"[0-z]*"#) | fn.#Fn
			Tags?:               *[...{
				Key?:   *(=~#"(\w|\d|\s|\\|-|\.:=+-)*"#) | fn.#Fn
				Value?: *(=~#"(\w|\d|\s|\\|-|\.:=+-)*"#) | fn.#Fn
			}] | fn.#If
			TargetRole?:          *("READ_WRITE" | "READ_ONLY") | fn.#Fn
			VpcSecurityGroupIds?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			VpcSubnetIds:         [...(*string | fn.#Fn)] | (*string | fn.#Fn)
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#DBProxyTargetGroup: {
		Type: "AWS::RDS::DBProxyTargetGroup"
		Properties: {
			ConnectionPoolConfigurationInfo?: *{
				ConnectionBorrowTimeout?:   *int | fn.#Fn
				InitQuery?:                 *string | fn.#Fn
				MaxConnectionsPercent?:     *int | fn.#Fn
				MaxIdleConnectionsPercent?: *int | fn.#Fn
				SessionPinningFilters?:     [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			} | fn.#If
			DBClusterIdentifiers?:  [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			DBInstanceIdentifiers?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			DBProxyName:            *(=~#"[A-z][0-z]*"#) | fn.#Fn
			TargetGroupName:        *("default") | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#DBSecurityGroup: {
		Type: "AWS::RDS::DBSecurityGroup"
		Properties: {
			DBSecurityGroupIngress: *[...{
				CIDRIP?:                  *(=~#"^(([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5]).){3}([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])(\/([0-9]|[1-2][0-9]|3[0-2]))$"#) | fn.#Fn
				EC2SecurityGroupId?:      *string | fn.#Fn
				EC2SecurityGroupName?:    *string | fn.#Fn
				EC2SecurityGroupOwnerId?: *string | fn.#Fn
			}] | fn.#If
			EC2VpcId?:        *string | fn.#Fn
			GroupDescription: *string | fn.#Fn
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
	#DBSecurityGroupIngress: {
		Type: "AWS::RDS::DBSecurityGroupIngress"
		Properties: {
			CIDRIP?:                  *(=~#"^(([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5]).){3}([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])(\/([0-9]|[1-2][0-9]|3[0-2]))$"#) | fn.#Fn
			DBSecurityGroupName:      *string | fn.#Fn
			EC2SecurityGroupId?:      *string | fn.#Fn
			EC2SecurityGroupName?:    *string | fn.#Fn
			EC2SecurityGroupOwnerId?: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#DBSubnetGroup: {
		Type: "AWS::RDS::DBSubnetGroup"
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
	#EventSubscription: {
		Type: "AWS::RDS::EventSubscription"
		Properties: {
			Enabled?:         *bool | fn.#Fn
			EventCategories?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			SnsTopicArn:      *string | fn.#Fn
			SourceIds?:       [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			SourceType?:      *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#GlobalCluster: {
		Type: "AWS::RDS::GlobalCluster"
		Properties: {
			DeletionProtection?:        *bool | fn.#Fn
			Engine?:                    *("aurora" | "aurora-mysql" | "aurora-postgresql") | fn.#Fn
			EngineVersion?:             *string | fn.#Fn
			GlobalClusterIdentifier?:   *(=~#"^[a-zA-Z]{1}(?:-?[a-zA-Z0-9]){0,62}$"#) | fn.#Fn
			SourceDBClusterIdentifier?: *string | fn.#Fn
			StorageEncrypted?:          *bool | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#OptionGroup: {
		Type: "AWS::RDS::OptionGroup"
		Properties: {
			EngineName:           *string | fn.#Fn
			MajorEngineVersion:   *string | fn.#Fn
			OptionConfigurations: *[...{
				DBSecurityGroupMemberships?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				OptionName:                  *string | fn.#Fn
				OptionSettings?:             *[...{
					Name?:  *string | fn.#Fn
					Value?: *string | fn.#Fn
				}] | fn.#If
				OptionVersion?:               *string | fn.#Fn
				Port?:                        *int | fn.#Fn
				VpcSecurityGroupMemberships?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			}] | fn.#If
			OptionGroupDescription: *string | fn.#Fn
			Tags?:                  *[...{
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
