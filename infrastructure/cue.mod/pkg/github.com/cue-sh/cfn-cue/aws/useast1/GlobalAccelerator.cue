package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#GlobalAccelerator: {
	#Accelerator: {
		Type: "AWS::GlobalAccelerator::Accelerator"
		Properties: {
			Enabled?:       *bool | fn.#Fn
			IpAddressType?: *("IPV4" | "IPV6") | fn.#Fn
			IpAddresses?:   [...(*(=~#"^(?:[0-9]{1,3}\.){3}[0-9]{1,3}$"#) | fn.#Fn)] | (*(=~#"^(?:[0-9]{1,3}\.){3}[0-9]{1,3}$"#) | fn.#Fn)
			Name:           *(strings.MinRunes(1) & strings.MaxRunes(64) & (=~#"^[a-zA-Z0-9_-]{0,64}$"#)) | fn.#Fn
			Tags?:          *[...{
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
	#EndpointGroup: {
		Type: "AWS::GlobalAccelerator::EndpointGroup"
		Properties: {
			EndpointConfigurations?: *[...{
				ClientIPPreservationEnabled?: *bool | fn.#Fn
				EndpointId:                   *string | fn.#Fn
				Weight?:                      *int | fn.#Fn
			}] | fn.#If
			EndpointGroupRegion:         *string | fn.#Fn
			HealthCheckIntervalSeconds?: *int | fn.#Fn
			HealthCheckPath?:            *string | fn.#Fn
			HealthCheckPort?:            *(>=-1 & <=65535) | fn.#Fn
			HealthCheckProtocol?:        *("TCP" | "HTTP" | "HTTPS") | fn.#Fn
			ListenerArn:                 *string | fn.#Fn
			PortOverrides?:              *[...{
				EndpointPort: *int | fn.#Fn
				ListenerPort: *int | fn.#Fn
			}] | fn.#If
			ThresholdCount?:        *int | fn.#Fn
			TrafficDialPercentage?: *number | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Listener: {
		Type: "AWS::GlobalAccelerator::Listener"
		Properties: {
			AcceleratorArn:  *string | fn.#Fn
			ClientAffinity?: *("NONE" | "SOURCE_IP") | fn.#Fn
			PortRanges:      *[...{
				FromPort: *int | fn.#Fn
				ToPort:   *int | fn.#Fn
			}] | fn.#If
			Protocol: *("TCP" | "UDP") | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
