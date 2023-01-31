package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#NetworkManager: {
	#CustomerGatewayAssociation: {
		Type: "AWS::NetworkManager::CustomerGatewayAssociation"
		Properties: {
			CustomerGatewayArn: *string | fn.#Fn
			DeviceId:           *string | fn.#Fn
			GlobalNetworkId:    *string | fn.#Fn
			LinkId?:            *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Device: {
		Type: "AWS::NetworkManager::Device"
		Properties: {
			Description?:    *string | fn.#Fn
			GlobalNetworkId: *string | fn.#Fn
			Location?:       *{
				Address?:   *string | fn.#Fn
				Latitude?:  *string | fn.#Fn
				Longitude?: *string | fn.#Fn
			} | fn.#If
			Model?:        *string | fn.#Fn
			SerialNumber?: *string | fn.#Fn
			SiteId?:       *string | fn.#Fn
			Tags?:         *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			Type?:   *string | fn.#Fn
			Vendor?: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#GlobalNetwork: {
		Type: "AWS::NetworkManager::GlobalNetwork"
		Properties: {
			Description?: *string | fn.#Fn
			Tags?:        *[...{
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
	#Link: {
		Type: "AWS::NetworkManager::Link"
		Properties: {
			Bandwidth: *{
				DownloadSpeed?: *int | fn.#Fn
				UploadSpeed?:   *int | fn.#Fn
			} | fn.#If
			Description?:    *string | fn.#Fn
			GlobalNetworkId: *string | fn.#Fn
			Provider?:       *string | fn.#Fn
			SiteId:          *string | fn.#Fn
			Tags?:           *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			Type?: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#LinkAssociation: {
		Type: "AWS::NetworkManager::LinkAssociation"
		Properties: {
			DeviceId:        *string | fn.#Fn
			GlobalNetworkId: *string | fn.#Fn
			LinkId:          *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Site: {
		Type: "AWS::NetworkManager::Site"
		Properties: {
			Description?:    *string | fn.#Fn
			GlobalNetworkId: *string | fn.#Fn
			Location?:       *{
				Address?:   *string | fn.#Fn
				Latitude?:  *string | fn.#Fn
				Longitude?: *string | fn.#Fn
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
	#TransitGatewayRegistration: {
		Type: "AWS::NetworkManager::TransitGatewayRegistration"
		Properties: {
			GlobalNetworkId:   *string | fn.#Fn
			TransitGatewayArn: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
