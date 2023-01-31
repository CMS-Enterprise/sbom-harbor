package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#MediaConnect: {
	#Flow: {
		Type: "AWS::MediaConnect::Flow"
		Properties: {
			AvailabilityZone?: *string | fn.#Fn
			Name:              *string | fn.#Fn
			Source:            *{
				Decryption?: *{
					Algorithm?:                    *("aes128" | "aes192" | "aes256") | fn.#Fn
					ConstantInitializationVector?: *string | fn.#Fn
					DeviceId?:                     *string | fn.#Fn
					KeyType?:                      *("speke" | "static-key" | "srt-password") | fn.#Fn
					Region?:                       *string | fn.#Fn
					ResourceId?:                   *string | fn.#Fn
					RoleArn:                       *string | fn.#Fn
					SecretArn?:                    *string | fn.#Fn
					Url?:                          *string | fn.#Fn
				} | fn.#If
				Description?:      *string | fn.#Fn
				EntitlementArn?:   *string | fn.#Fn
				IngestIp?:         *string | fn.#Fn
				IngestPort?:       *int | fn.#Fn
				MaxBitrate?:       *int | fn.#Fn
				MaxLatency?:       *int | fn.#Fn
				MinLatency?:       *int | fn.#Fn
				Name?:             *string | fn.#Fn
				Protocol?:         *("zixi-push" | "rtp-fec" | "rtp" | "rist" | "srt-listener") | fn.#Fn
				SourceArn?:        *string | fn.#Fn
				SourceIngestPort?: *string | fn.#Fn
				StreamId?:         *string | fn.#Fn
				VpcInterfaceName?: *string | fn.#Fn
				WhitelistCidr?:    *string | fn.#Fn
			} | fn.#If
			SourceFailoverConfig?: *{
				RecoveryWindow?: *int | fn.#Fn
				State?:          *("ENABLED" | "DISABLED") | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#FlowEntitlement: {
		Type: "AWS::MediaConnect::FlowEntitlement"
		Properties: {
			DataTransferSubscriberFeePercent?: *int | fn.#Fn
			Description:                       *string | fn.#Fn
			Encryption?:                       *{
				Algorithm:                     *("aes128" | "aes192" | "aes256") | fn.#Fn
				ConstantInitializationVector?: *string | fn.#Fn
				DeviceId?:                     *string | fn.#Fn
				KeyType?:                      *("speke" | "static-key") | fn.#Fn
				Region?:                       *string | fn.#Fn
				ResourceId?:                   *string | fn.#Fn
				RoleArn:                       *string | fn.#Fn
				SecretArn?:                    *string | fn.#Fn
				Url?:                          *string | fn.#Fn
			} | fn.#If
			EntitlementStatus?: *("ENABLED" | "DISABLED") | fn.#Fn
			FlowArn:            *string | fn.#Fn
			Name:               *string | fn.#Fn
			Subscribers:        [...(*string | fn.#Fn)] | (*string | fn.#Fn)
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#FlowOutput: {
		Type: "AWS::MediaConnect::FlowOutput"
		Properties: {
			CidrAllowList?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			Description?:   *string | fn.#Fn
			Destination?:   *string | fn.#Fn
			Encryption?:    *{
				Algorithm?: *("aes128" | "aes192" | "aes256") | fn.#Fn
				KeyType?:   *("static-key" | "srt-password") | fn.#Fn
				RoleArn:    *string | fn.#Fn
				SecretArn:  *string | fn.#Fn
			} | fn.#If
			FlowArn:                 *string | fn.#Fn
			MaxLatency?:             *int | fn.#Fn
			MinLatency?:             *int | fn.#Fn
			Name?:                   *string | fn.#Fn
			Port?:                   *int | fn.#Fn
			Protocol:                *("zixi-push" | "rtp-fec" | "rtp" | "zixi-pull" | "rist" | "srt-listener") | fn.#Fn
			RemoteId?:               *string | fn.#Fn
			SmoothingLatency?:       *int | fn.#Fn
			StreamId?:               *string | fn.#Fn
			VpcInterfaceAttachment?: *{
				VpcInterfaceName?: *string | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#FlowSource: {
		Type: "AWS::MediaConnect::FlowSource"
		Properties: {
			Decryption?: *{
				Algorithm:                     *("aes128" | "aes192" | "aes256") | fn.#Fn
				ConstantInitializationVector?: *string | fn.#Fn
				DeviceId?:                     *string | fn.#Fn
				KeyType?:                      *("speke" | "static-key") | fn.#Fn
				Region?:                       *string | fn.#Fn
				ResourceId?:                   *string | fn.#Fn
				RoleArn:                       *string | fn.#Fn
				SecretArn?:                    *string | fn.#Fn
				Url?:                          *string | fn.#Fn
			} | fn.#If
			Description:       *string | fn.#Fn
			EntitlementArn?:   *string | fn.#Fn
			FlowArn?:          *string | fn.#Fn
			IngestPort?:       *int | fn.#Fn
			MaxBitrate?:       *int | fn.#Fn
			MaxLatency?:       *int | fn.#Fn
			Name:              *string | fn.#Fn
			Protocol?:         *("zixi-push" | "rtp-fec" | "rtp" | "rist") | fn.#Fn
			StreamId?:         *string | fn.#Fn
			VpcInterfaceName?: *string | fn.#Fn
			WhitelistCidr?:    *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#FlowVpcInterface: {
		Type: "AWS::MediaConnect::FlowVpcInterface"
		Properties: {
			FlowArn:          *string | fn.#Fn
			Name:             *string | fn.#Fn
			RoleArn:          *string | fn.#Fn
			SecurityGroupIds: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			SubnetId:         *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
