package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#GameLift: {
	#Alias: {
		Type: "AWS::GameLift::Alias"
		Properties: {
			Description?:    *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
			Name:            *(strings.MinRunes(1) & strings.MaxRunes(1024) & (=~#".*\S.*"#)) | fn.#Fn
			RoutingStrategy: *{
				FleetId?: *(=~#"^fleet-\S+"#) | fn.#Fn
				Message?: *string | fn.#Fn
				Type:     *("SIMPLE" | "TERMINAL") | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Build: {
		Type: "AWS::GameLift::Build"
		Properties: {
			Name?:            *string | fn.#Fn
			OperatingSystem?: *string | fn.#Fn
			StorageLocation?: *{
				Bucket:         *string | fn.#Fn
				Key:            *string | fn.#Fn
				ObjectVersion?: *string | fn.#Fn
				RoleArn:        *string | fn.#Fn
			} | fn.#If
			Version?: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Fleet: {
		Type: "AWS::GameLift::Fleet"
		Properties: {
			BuildId?:                  *(=~#"^build-\S+|^arn:.*:build\/build-\S+"#) | fn.#Fn
			CertificateConfiguration?: *{
				CertificateType: *("DISABLED" | "GENERATED") | fn.#Fn
			} | fn.#If
			Description?:           *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
			DesiredEC2Instances?:   *int | fn.#Fn
			EC2InboundPermissions?: *[...{
				FromPort: *(>=1 & <=60000) | fn.#Fn
				IpRange:  *(=~#"(^(([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])\.){3}([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])(\/([0-9]|[1-2][0-9]|3[0-2]))$)"#) | fn.#Fn
				Protocol: *("TCP" | "UDP") | fn.#Fn
				ToPort:   *(>=1 & <=60000) | fn.#Fn
			}] | fn.#If
			EC2InstanceType?: *(("c3.2xlarge" | "c3.4xlarge" | "c3.8xlarge" | "c3.large" | "c3.xlarge" | "c4.2xlarge" | "c4.4xlarge" | "c4.8xlarge" | "c4.large" | "c4.xlarge" | "c5.12xlarge" | "c5.18xlarge" | "c5.24xlarge" | "c5.2xlarge" | "c5.4xlarge" | "c5.9xlarge" | "c5.large" | "c5.xlarge" | "c5a.12xlarge" | "c5a.16xlarge" | "c5a.24xlarge" | "c5a.2xlarge" | "c5a.4xlarge" | "c5a.8xlarge" | "c5a.large" | "c5a.xlarge" | "c6g.12xlarge" | "c6g.16xlarge" | "c6g.2xlarge" | "c6g.4xlarge" | "c6g.8xlarge" | "c6g.large" | "c6g.medium" | "c6g.xlarge" | "m3.2xlarge" | "m3.large" | "m3.medium" | "m3.xlarge" | "m4.10xlarge" | "m4.2xlarge" | "m4.4xlarge" | "m4.large" | "m4.xlarge" | "m5.12xlarge" | "m5.16xlarge" | "m5.24xlarge" | "m5.2xlarge" | "m5.4xlarge" | "m5.8xlarge" | "m5.large" | "m5.xlarge" | "m5a.12xlarge" | "m5a.16xlarge" | "m5a.24xlarge" | "m5a.2xlarge" | "m5a.4xlarge" | "m5a.8xlarge" | "m5a.large" | "m5a.xlarge" | "m6g.12xlarge" | "m6g.16xlarge" | "m6g.2xlarge" | "m6g.4xlarge" | "m6g.8xlarge" | "m6g.large" | "m6g.medium" | "m6g.xlarge" | "r3.2xlarge" | "r3.4xlarge" | "r3.8xlarge" | "r3.large" | "r3.xlarge" | "r4.16xlarge" | "r4.2xlarge" | "r4.4xlarge" | "r4.8xlarge" | "r4.large" | "r4.xlarge" | "r5.12xlarge" | "r5.16xlarge" | "r5.24xlarge" | "r5.2xlarge" | "r5.4xlarge" | "r5.8xlarge" | "r5.large" | "r5.xlarge" | "r5a.12xlarge" | "r5a.16xlarge" | "r5a.24xlarge" | "r5a.2xlarge" | "r5a.4xlarge" | "r5a.8xlarge" | "r5a.large" | "r5a.xlarge" | "r6g.12xlarge" | "r6g.16xlarge" | "r6g.2xlarge" | "r6g.4xlarge" | "r6g.8xlarge" | "r6g.large" | "r6g.medium" | "r6g.xlarge" | "t2.large" | "t2.medium" | "t2.micro" | "t2.small") & (=~#"^.*..*$"#)) | fn.#Fn
			FleetType?:       *("ON_DEMAND" | "SPOT") | fn.#Fn
			InstanceRoleARN?: *(=~#"^arn:aws(-.*)?:[a-z-]+:(([a-z]+-)+[0-9])?:([0-9]{12})?:[^.]+$"#) | fn.#Fn
			Locations?:       *[...{
				Location:          *(strings.MinRunes(1) & strings.MaxRunes(64) & (=~#"^[a-z]+(-([a-z]+|\d))*"#)) | fn.#Fn
				LocationCapacity?: *{
					DesiredEC2Instances: *int | fn.#Fn
					MaxSize:             *int | fn.#Fn
					MinSize:             *int | fn.#Fn
				} | fn.#If
			}] | fn.#If
			MaxSize?:                        *int | fn.#Fn
			MetricGroups?:                   [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			MinSize?:                        *int | fn.#Fn
			Name?:                           *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
			NewGameSessionProtectionPolicy?: *("FullProtection" | "NoProtection") | fn.#Fn
			PeerVpcAwsAccountId?:            *(strings.MinRunes(1) & strings.MaxRunes(1024) & (=~#"^[0-9]{12}$"#)) | fn.#Fn
			PeerVpcId?:                      *(strings.MinRunes(1) & strings.MaxRunes(1024) & (=~#"^vpc-\S+"#)) | fn.#Fn
			ResourceCreationLimitPolicy?:    *{
				NewGameSessionsPerCreator?: *int | fn.#Fn
				PolicyPeriodInMinutes?:     *int | fn.#Fn
			} | fn.#If
			RuntimeConfiguration?: *{
				GameSessionActivationTimeoutSeconds?: *(>=1 & <=600) | fn.#Fn
				MaxConcurrentGameSessionActivations?: *(>=1 & <=2147483647) | fn.#Fn
				ServerProcesses?:                     *[...{
					ConcurrentExecutions: *int | fn.#Fn
					LaunchPath:           *(strings.MinRunes(1) & strings.MaxRunes(1024) & (=~#"^([Cc]:\\game\S+|/local/game/\S+)"#)) | fn.#Fn
					Parameters?:          *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
				}] | fn.#If
			} | fn.#If
			ScriptId?: *(=~#"^script-\S+|^arn:.*:script\/script-\S+"#) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#GameServerGroup: {
		Type: "AWS::GameLift::GameServerGroup"
		Properties: {
			AutoScalingPolicy?: *{
				EstimatedInstanceWarmup?:    *number | fn.#Fn
				TargetTrackingConfiguration: *{
					TargetValue: *number | fn.#Fn
				} | fn.#If
			} | fn.#If
			BalancingStrategy?:          *("SPOT_ONLY" | "SPOT_PREFERRED" | "ON_DEMAND_ONLY") | fn.#Fn
			DeleteOption?:               *("SAFE_DELETE" | "FORCE_DELETE" | "RETAIN") | fn.#Fn
			GameServerGroupName:         *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"[a-zA-Z0-9-\.]+"#)) | fn.#Fn
			GameServerProtectionPolicy?: *("NO_PROTECTION" | "FULL_PROTECTION") | fn.#Fn
			InstanceDefinitions:         *[...{
				InstanceType:      *string | fn.#Fn
				WeightedCapacity?: *(=~#"^[\u0031-\u0039][\u0030-\u0039]{0,2}$"#) | fn.#Fn
			}] | fn.#If
			LaunchTemplate: *{
				LaunchTemplateId?:   *string | fn.#Fn
				LaunchTemplateName?: *string | fn.#Fn
				Version?:            *string | fn.#Fn
			} | fn.#If
			MaxSize?: *number | fn.#Fn
			MinSize?: *number | fn.#Fn
			RoleArn:  *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"^arn:.*:role\/[\w+=,.@-]+"#)) | fn.#Fn
			Tags?:    *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			VpcSubnets?: [...(*(strings.MinRunes(15) & strings.MaxRunes(24) & (=~#"^subnet-[0-9a-z]+$"#)) | fn.#Fn)] | (*(strings.MinRunes(15) & strings.MaxRunes(24) & (=~#"^subnet-[0-9a-z]+$"#)) | fn.#Fn)
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#GameSessionQueue: {
		Type: "AWS::GameLift::GameSessionQueue"
		Properties: {
			CustomEventData?: *string | fn.#Fn
			Destinations?:    *[...{
				DestinationArn?: *string | fn.#Fn
			}] | fn.#If
			FilterConfiguration?: *{
				AllowedLocations?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			} | fn.#If
			Name:                   *string | fn.#Fn
			NotificationTarget?:    *string | fn.#Fn
			PlayerLatencyPolicies?: *[...{
				MaximumIndividualPlayerLatencyMilliseconds?: *int | fn.#Fn
				PolicyDurationSeconds?:                      *int | fn.#Fn
			}] | fn.#If
			PriorityConfiguration?: *{
				LocationOrder?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				PriorityOrder?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			} | fn.#If
			TimeoutInSeconds?: *int | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#MatchmakingConfiguration: {
		Type: "AWS::GameLift::MatchmakingConfiguration"
		Properties: {
			AcceptanceRequired:        *bool | fn.#Fn
			AcceptanceTimeoutSeconds?: *int | fn.#Fn
			AdditionalPlayerCount?:    *int | fn.#Fn
			BackfillMode?:             *string | fn.#Fn
			CustomEventData?:          *string | fn.#Fn
			Description?:              *string | fn.#Fn
			FlexMatchMode?:            *string | fn.#Fn
			GameProperties?:           *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			GameSessionData?:      *string | fn.#Fn
			GameSessionQueueArns?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			Name:                  *string | fn.#Fn
			NotificationTarget?:   *string | fn.#Fn
			RequestTimeoutSeconds: *int | fn.#Fn
			RuleSetName:           *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#MatchmakingRuleSet: {
		Type: "AWS::GameLift::MatchmakingRuleSet"
		Properties: {
			Name:        *string | fn.#Fn
			RuleSetBody: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Script: {
		Type: "AWS::GameLift::Script"
		Properties: {
			Name?:           *string | fn.#Fn
			StorageLocation: *{
				Bucket:         *string | fn.#Fn
				Key:            *string | fn.#Fn
				ObjectVersion?: *string | fn.#Fn
				RoleArn:        *string | fn.#Fn
			} | fn.#If
			Version?: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
