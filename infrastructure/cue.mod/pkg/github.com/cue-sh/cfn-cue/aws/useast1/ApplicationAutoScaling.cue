package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"time"
)

#ApplicationAutoScaling: {
	#ScalableTarget: {
		Type: "AWS::ApplicationAutoScaling::ScalableTarget"
		Properties: {
			MaxCapacity:       *int | fn.#Fn
			MinCapacity:       *int | fn.#Fn
			ResourceId:        *string | fn.#Fn
			RoleARN:           *string | fn.#Fn
			ScalableDimension: *string | fn.#Fn
			ScheduledActions?: *[...{
				EndTime?:              *time.Time | fn.#Fn
				ScalableTargetAction?: *{
					MaxCapacity?: *int | fn.#Fn
					MinCapacity?: *int | fn.#Fn
				} | fn.#If
				Schedule:            *string | fn.#Fn
				ScheduledActionName: *string | fn.#Fn
				StartTime?:          *time.Time | fn.#Fn
				Timezone?:           *string | fn.#Fn
			}] | fn.#If
			ServiceNamespace: *string | fn.#Fn
			SuspendedState?:  *{
				DynamicScalingInSuspended?:  *bool | fn.#Fn
				DynamicScalingOutSuspended?: *bool | fn.#Fn
				ScheduledScalingSuspended?:  *bool | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ScalingPolicy: {
		Type: "AWS::ApplicationAutoScaling::ScalingPolicy"
		Properties: {
			PolicyName:                      *string | fn.#Fn
			PolicyType:                      *("StepScaling" | "TargetTrackingScaling") | fn.#Fn
			ResourceId?:                     *string | fn.#Fn
			ScalableDimension?:              *string | fn.#Fn
			ScalingTargetId?:                *string | fn.#Fn
			ServiceNamespace?:               *string | fn.#Fn
			StepScalingPolicyConfiguration?: *{
				AdjustmentType?:         *string | fn.#Fn
				Cooldown?:               *int | fn.#Fn
				MetricAggregationType?:  *string | fn.#Fn
				MinAdjustmentMagnitude?: *int | fn.#Fn
				StepAdjustments?:        *[...{
					MetricIntervalLowerBound?: *number | fn.#Fn
					MetricIntervalUpperBound?: *number | fn.#Fn
					ScalingAdjustment:         *int | fn.#Fn
				}] | fn.#If
			} | fn.#If
			TargetTrackingScalingPolicyConfiguration?: *{
				CustomizedMetricSpecification?: *{
					Dimensions?: *[...{
						Name:  *string | fn.#Fn
						Value: *string | fn.#Fn
					}] | fn.#If
					MetricName: *string | fn.#Fn
					Namespace:  *string | fn.#Fn
					Statistic:  *("Average" | "Maximum" | "Minimum" | "SampleCount" | "Sum") | fn.#Fn
					Unit?:      *string | fn.#Fn
				} | fn.#If
				DisableScaleIn?:                *bool | fn.#Fn
				PredefinedMetricSpecification?: *{
					PredefinedMetricType: *("ALBRequestCountPerTarget" | "AppStreamAverageCapacityUtilization" | "CassandraReadCapacityUtilization" | "CassandraWriteCapacityUtilization" | "ComprehendInferenceUtilization" | "DynamoDBReadCapacityUtilization" | "DynamoDBWriteCapacityUtilization" | "EC2SpotFleetRequestAverageCPUUtilization" | "EC2SpotFleetRequestAverageNetworkIn" | "EC2SpotFleetRequestAverageNetworkOut" | "ECSServiceAverageCPUUtilization" | "ECSServiceAverageMemoryUtilization" | "ElastiCacheDatabaseMemoryUsageCountedForEvictPercentage" | "ElastiCachePrimaryEngineCPUUtilization" | "ElastiCacheReplicaEngineCPUUtilization" | "KafkaBrokerStorageUtilization" | "LambdaProvisionedConcurrencyUtilization" | "NeptuneReaderAverageCPUUtilization" | "RDSReaderAverageCPUUtilization" | "RDSReaderAverageDatabaseConnections" | "SageMakerVariantInvocationsPerInstance") | fn.#Fn
					ResourceLabel?:       *string | fn.#Fn
				} | fn.#If
				ScaleInCooldown?:  *int | fn.#Fn
				ScaleOutCooldown?: *int | fn.#Fn
				TargetValue:       *number | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
