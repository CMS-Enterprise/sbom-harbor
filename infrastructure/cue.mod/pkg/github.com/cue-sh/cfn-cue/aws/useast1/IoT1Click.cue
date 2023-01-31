package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#IoT1Click: {
	#Device: {
		Type: "AWS::IoT1Click::Device"
		Properties: {
			DeviceId: *string | fn.#Fn
			Enabled:  *bool | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Placement: {
		Type: "AWS::IoT1Click::Placement"
		Properties: {
			AssociatedDevices?: *{
				[string]: _
			} | fn.#Fn
			Attributes?: *{
				[string]: _
			} | fn.#Fn
			PlacementName?: *string | fn.#Fn
			ProjectName:    *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Project: {
		Type: "AWS::IoT1Click::Project"
		Properties: {
			Description?:      *string | fn.#Fn
			PlacementTemplate: *{
				DefaultAttributes?: *{
					[string]: _
				} | fn.#Fn
				DeviceTemplates?: *{
					[string]: _
				} | fn.#Fn
			} | fn.#If
			ProjectName?: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
