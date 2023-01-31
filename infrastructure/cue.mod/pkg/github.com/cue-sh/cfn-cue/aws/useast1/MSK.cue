package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#MSK: {
	#Cluster: {
		Type: "AWS::MSK::Cluster"
		Properties: {
			BrokerNodeGroupInfo: *{
				BrokerAZDistribution?: *string | fn.#Fn
				ClientSubnets:         [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				ConnectivityInfo?:     *{
					PublicAccess?: *{
						Type?: *string | fn.#Fn
					} | fn.#If
				} | fn.#If
				InstanceType:    *string | fn.#Fn
				SecurityGroups?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				StorageInfo?:    *{
					EBSStorageInfo?: *{
						VolumeSize?: *int | fn.#Fn
					} | fn.#If
				} | fn.#If
			} | fn.#If
			ClientAuthentication?: *{
				Sasl?: *{
					Iam?: *{
						Enabled: *bool | fn.#Fn
					} | fn.#If
					Scram?: *{
						Enabled: *bool | fn.#Fn
					} | fn.#If
				} | fn.#If
				Tls?: *{
					CertificateAuthorityArnList?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					Enabled?:                     *bool | fn.#Fn
				} | fn.#If
				Unauthenticated?: *{
					Enabled: *bool | fn.#Fn
				} | fn.#If
			} | fn.#If
			ClusterName:        *string | fn.#Fn
			ConfigurationInfo?: *{
				Arn:      *string | fn.#Fn
				Revision: *int | fn.#Fn
			} | fn.#If
			EncryptionInfo?: *{
				EncryptionAtRest?: *{
					DataVolumeKMSKeyId: *string | fn.#Fn
				} | fn.#If
				EncryptionInTransit?: *{
					ClientBroker?: *string | fn.#Fn
					InCluster?:    *bool | fn.#Fn
				} | fn.#If
			} | fn.#If
			EnhancedMonitoring?: *string | fn.#Fn
			KafkaVersion:        *string | fn.#Fn
			LoggingInfo?:        *{
				BrokerLogs: *{
					CloudWatchLogs?: *{
						Enabled:   *bool | fn.#Fn
						LogGroup?: *string | fn.#Fn
					} | fn.#If
					Firehose?: *{
						DeliveryStream?: *string | fn.#Fn
						Enabled:         *bool | fn.#Fn
					} | fn.#If
					S3?: *{
						Bucket?: *string | fn.#Fn
						Enabled: *bool | fn.#Fn
						Prefix?: *string | fn.#Fn
					} | fn.#If
				} | fn.#If
			} | fn.#If
			NumberOfBrokerNodes: *int | fn.#Fn
			OpenMonitoring?:     *{
				Prometheus: *{
					JmxExporter?: *{
						EnabledInBroker: *bool | fn.#Fn
					} | fn.#If
					NodeExporter?: *{
						EnabledInBroker: *bool | fn.#Fn
					} | fn.#If
				} | fn.#If
			} | fn.#If
			Tags?: *{
				[string]: _
			} | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
