package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#AutoScalingPlans: {
	#ScalingPlan: {
		Type: "AWS::AutoScalingPlans::ScalingPlan"
		Properties: {
			ApplicationSource: *{
				CloudFormationStackARN?: *string | fn.#Fn
				TagFilters?:             *[...{
					Key:     *string | fn.#Fn
					Values?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				}] | fn.#If
			} | fn.#If
			ScalingInstructions: *[...{
				CustomizedLoadMetricSpecification?: *{
					Dimensions?: *[...{
						Name:  *string | fn.#Fn
						Value: *string | fn.#Fn
					}] | fn.#If
					MetricName: *string | fn.#Fn
					Namespace:  *string | fn.#Fn
					Statistic:  *string | fn.#Fn
					Unit?:      *string | fn.#Fn
				} | fn.#If
				DisableDynamicScaling?:             *bool | fn.#Fn
				MaxCapacity:                        *int | fn.#Fn
				MinCapacity:                        *int | fn.#Fn
				PredefinedLoadMetricSpecification?: *{
					PredefinedLoadMetricType: *string | fn.#Fn
					ResourceLabel?:           *string | fn.#Fn
				} | fn.#If
				PredictiveScalingMaxCapacityBehavior?: *("SetForecastCapacityToMaxCapacity" | "SetMaxCapacityAboveForecastCapacity" | "SetMaxCapacityToForecastCapacity") | fn.#Fn
				PredictiveScalingMaxCapacityBuffer?:   *int | fn.#Fn
				PredictiveScalingMode?:                *("ForecastAndScale" | "ForecastOnly") | fn.#Fn
				ResourceId:                            *string | fn.#Fn
				ScalableDimension:                     *("autoscaling:autoScalingGroup:DesiredCapacity" | "dynamodb:index:ReadCapacityUnits" | "dynamodb:index:WriteCapacityUnits" | "dynamodb:table:ReadCapacityUnits" | "dynamodb:table:WriteCapacityUnits" | "ec2:spot-fleet-request:TargetCapacity" | "ecs:service:DesiredCount" | "rds:cluster:ReadReplicaCount") | fn.#Fn
				ScalingPolicyUpdateBehavior?:          *string | fn.#Fn
				ScheduledActionBufferTime?:            *int | fn.#Fn
				ServiceNamespace:                      *("autoscaling" | "dynamodb" | "ec2" | "ecs" | "rds") | fn.#Fn
				TargetTrackingConfigurations:          *[...{
					CustomizedScalingMetricSpecification?: *{
						Dimensions?: *[...{
							Name:  *string | fn.#Fn
							Value: *string | fn.#Fn
						}] | fn.#If
						MetricName: *string | fn.#Fn
						Namespace:  *string | fn.#Fn
						Statistic:  *("Average" | "Maximum" | "Minimum" | "SampleCount" | "Sum") | fn.#Fn
						Unit?:      *string | fn.#Fn
					} | fn.#If
					DisableScaleIn?:                       *bool | fn.#Fn
					EstimatedInstanceWarmup?:              *int | fn.#Fn
					PredefinedScalingMetricSpecification?: *{
						PredefinedScalingMetricType: *("ALBRequestCountPerTarget" | "ASGAverageCPUUtilization" | "ASGAverageNetworkIn" | "ASGAverageNetworkOut") | fn.#Fn
						ResourceLabel?:              *string | fn.#Fn
					} | fn.#If
					ScaleInCooldown?:  *int | fn.#Fn
					ScaleOutCooldown?: *int | fn.#Fn
					TargetValue:       *number | fn.#Fn
				}] | fn.#If
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
