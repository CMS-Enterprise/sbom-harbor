package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#PinpointEmail: {
	#ConfigurationSet: {
		Type: "AWS::PinpointEmail::ConfigurationSet"
		Properties: {
			DeliveryOptions?: *{
				SendingPoolName?: *string | fn.#Fn
			} | fn.#If
			Name:               *string | fn.#Fn
			ReputationOptions?: *{
				ReputationMetricsEnabled?: *bool | fn.#Fn
			} | fn.#If
			SendingOptions?: *{
				SendingEnabled?: *bool | fn.#Fn
			} | fn.#If
			Tags?: *[...{
				Key?:   *string | fn.#Fn
				Value?: *string | fn.#Fn
			}] | fn.#If
			TrackingOptions?: *{
				CustomRedirectDomain?: *string | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ConfigurationSetEventDestination: {
		Type: "AWS::PinpointEmail::ConfigurationSetEventDestination"
		Properties: {
			ConfigurationSetName: *string | fn.#Fn
			EventDestination?:    *{
				CloudWatchDestination?: *{
					DimensionConfigurations?: *[...{
						DefaultDimensionValue: *string | fn.#Fn
						DimensionName:         *string | fn.#Fn
						DimensionValueSource:  *string | fn.#Fn
					}] | fn.#If
				} | fn.#If
				Enabled?:                    *bool | fn.#Fn
				KinesisFirehoseDestination?: *{
					DeliveryStreamArn: *string | fn.#Fn
					IamRoleArn:        *string | fn.#Fn
				} | fn.#If
				MatchingEventTypes:   [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				PinpointDestination?: *{
					ApplicationArn?: *string | fn.#Fn
				} | fn.#If
				SnsDestination?: *{
					TopicArn: *string | fn.#Fn
				} | fn.#If
			} | fn.#If
			EventDestinationName: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#DedicatedIpPool: {
		Type: "AWS::PinpointEmail::DedicatedIpPool"
		Properties: {
			PoolName?: *string | fn.#Fn
			Tags?:     *[...{
				Key?:   *string | fn.#Fn
				Value?: *string | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Identity: {
		Type: "AWS::PinpointEmail::Identity"
		Properties: {
			DkimSigningEnabled?:        *bool | fn.#Fn
			FeedbackForwardingEnabled?: *bool | fn.#Fn
			MailFromAttributes?:        *{
				BehaviorOnMxFailure?: *string | fn.#Fn
				MailFromDomain?:      *string | fn.#Fn
			} | fn.#If
			Name:  *string | fn.#Fn
			Tags?: *[...{
				Key?:   *string | fn.#Fn
				Value?: *string | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
