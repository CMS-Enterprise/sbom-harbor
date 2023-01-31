package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#CodeDeploy: {
	#Application: {
		Type: "AWS::CodeDeploy::Application"
		Properties: {
			ApplicationName?: *string | fn.#Fn
			ComputePlatform?: *("ECS" | "Lambda" | "Server") | fn.#Fn
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
	#DeploymentConfig: {
		Type: "AWS::CodeDeploy::DeploymentConfig"
		Properties: {
			ComputePlatform?:      *string | fn.#Fn
			DeploymentConfigName?: *string | fn.#Fn
			MinimumHealthyHosts?:  *{
				Type:  *("FLEET_PERCENT" | "HOST_COUNT") | fn.#Fn
				Value: *int | fn.#Fn
			} | fn.#If
			TrafficRoutingConfig?: *{
				TimeBasedCanary?: *{
					CanaryInterval:   *int | fn.#Fn
					CanaryPercentage: *int | fn.#Fn
				} | fn.#If
				TimeBasedLinear?: *{
					LinearInterval:   *int | fn.#Fn
					LinearPercentage: *int | fn.#Fn
				} | fn.#If
				Type: *string | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#DeploymentGroup: {
		Type: "AWS::CodeDeploy::DeploymentGroup"
		Properties: {
			AlarmConfiguration?: *{
				Alarms?: *[...{
					Name?: *string | fn.#Fn
				}] | fn.#If
				Enabled?:                *bool | fn.#Fn
				IgnorePollAlarmFailure?: *bool | fn.#Fn
			} | fn.#If
			ApplicationName:            *string | fn.#Fn
			AutoRollbackConfiguration?: *{
				Enabled?: *bool | fn.#Fn
				Events?:  [...(*("DEPLOYMENT_FAILURE" | "DEPLOYMENT_STOP_ON_ALARM" | "DEPLOYMENT_STOP_ON_REQUEST") | fn.#Fn)] | (*("DEPLOYMENT_FAILURE" | "DEPLOYMENT_STOP_ON_ALARM" | "DEPLOYMENT_STOP_ON_REQUEST") | fn.#Fn)
			} | fn.#If
			AutoScalingGroups?:                [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			BlueGreenDeploymentConfiguration?: *{
				DeploymentReadyOption?: *{
					ActionOnTimeout?:   *string | fn.#Fn
					WaitTimeInMinutes?: *int | fn.#Fn
				} | fn.#If
				GreenFleetProvisioningOption?: *{
					Action?: *string | fn.#Fn
				} | fn.#If
				TerminateBlueInstancesOnDeploymentSuccess?: *{
					Action?:                       *string | fn.#Fn
					TerminationWaitTimeInMinutes?: *int | fn.#Fn
				} | fn.#If
			} | fn.#If
			Deployment?: *{
				Description?:                   *string | fn.#Fn
				IgnoreApplicationStopFailures?: *bool | fn.#Fn
				Revision:                       *{
					GitHubLocation?: *{
						CommitId:   *string | fn.#Fn
						Repository: *string | fn.#Fn
					} | fn.#If
					RevisionType?: *string | fn.#Fn
					S3Location?:   *{
						Bucket:      *string | fn.#Fn
						BundleType?: *string | fn.#Fn
						ETag?:       *string | fn.#Fn
						Key:         *string | fn.#Fn
						Version?:    *string | fn.#Fn
					} | fn.#If
				} | fn.#If
			} | fn.#If
			DeploymentConfigName?: *string | fn.#Fn
			DeploymentGroupName?:  *string | fn.#Fn
			DeploymentStyle?:      *{
				DeploymentOption?: *("WITHOUT_TRAFFIC_CONTROL" | "WITH_TRAFFIC_CONTROL") | fn.#Fn
				DeploymentType?:   *("BLUE_GREEN" | "IN_PLACE") | fn.#Fn
			} | fn.#If
			ECSServices?: *[...{
				ClusterName: *string | fn.#Fn
				ServiceName: *string | fn.#Fn
			}] | fn.#If
			Ec2TagFilters?: *[...{
				Key?:   *string | fn.#Fn
				Type?:  *string | fn.#Fn
				Value?: *string | fn.#Fn
			}] | fn.#If
			Ec2TagSet?: *{
				Ec2TagSetList?: *[...{
					Ec2TagGroup?: *[...{
						Key?:   *string | fn.#Fn
						Type?:  *string | fn.#Fn
						Value?: *string | fn.#Fn
					}] | fn.#If
				}] | fn.#If
			} | fn.#If
			LoadBalancerInfo?: *{
				ElbInfoList?: *[...{
					Name?: *string | fn.#Fn
				}] | fn.#If
				TargetGroupInfoList?: *[...{
					Name?: *string | fn.#Fn
				}] | fn.#If
			} | fn.#If
			OnPremisesInstanceTagFilters?: *[...{
				Key?:   *string | fn.#Fn
				Type?:  *string | fn.#Fn
				Value?: *string | fn.#Fn
			}] | fn.#If
			OnPremisesTagSet?: *{
				OnPremisesTagSetList?: *[...{
					OnPremisesTagGroup?: *[...{
						Key?:   *string | fn.#Fn
						Type?:  *string | fn.#Fn
						Value?: *string | fn.#Fn
					}] | fn.#If
				}] | fn.#If
			} | fn.#If
			ServiceRoleArn:         *string | fn.#Fn
			TriggerConfigurations?: *[...{
				TriggerEvents?:    [...(*("DeploymentFailure" | "DeploymentReady" | "DeploymentRollback" | "DeploymentStart" | "DeploymentStop" | "DeploymentSuccess" | "InstanceFailure" | "InstanceReady" | "InstanceStart" | "InstanceSuccess") | fn.#Fn)] | (*("DeploymentFailure" | "DeploymentReady" | "DeploymentRollback" | "DeploymentStart" | "DeploymentStop" | "DeploymentSuccess" | "InstanceFailure" | "InstanceReady" | "InstanceStart" | "InstanceSuccess") | fn.#Fn)
				TriggerName?:      *string | fn.#Fn
				TriggerTargetArn?: *string | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
