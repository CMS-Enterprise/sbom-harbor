package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#SQS: {
	#Queue: {
		Type: "AWS::SQS::Queue"
		Properties: {
			ContentBasedDeduplication?:     *bool | fn.#Fn
			DeduplicationScope?:            *string | fn.#Fn
			DelaySeconds?:                  *(>=0 & <=900) | fn.#Fn
			FifoQueue?:                     *bool | fn.#Fn
			FifoThroughputLimit?:           *string | fn.#Fn
			KmsDataKeyReusePeriodSeconds?:  *(>=60 & <=86400) | fn.#Fn
			KmsMasterKeyId?:                *string | fn.#Fn
			MaximumMessageSize?:            *(>=1024 & <=262144) | fn.#Fn
			MessageRetentionPeriod?:        *(>=60 & <=1209600) | fn.#Fn
			QueueName?:                     *string | fn.#Fn
			ReceiveMessageWaitTimeSeconds?: *(>=0 & <=20) | fn.#Fn
			RedriveAllowPolicy?:            *{
				[string]: _
			} | fn.#Fn
			RedrivePolicy?: *{
				[string]: _
			} | fn.#Fn
			Tags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			VisibilityTimeout?: *(>=0 & <=43200) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#QueuePolicy: {
		Type: "AWS::SQS::QueuePolicy"
		Properties: {
			PolicyDocument: *{
				{
					[string]: _
				}
				Version: string | *"2012-10-17"
			} | fn.#Fn
			Queues: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
