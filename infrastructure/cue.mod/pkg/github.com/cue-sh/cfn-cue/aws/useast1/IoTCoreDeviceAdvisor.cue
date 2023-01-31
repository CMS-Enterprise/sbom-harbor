package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#IoTCoreDeviceAdvisor: {
	#SuiteDefinition: {
		Type: "AWS::IoTCoreDeviceAdvisor::SuiteDefinition"
		Properties: {
			SuiteDefinitionConfiguration: *{
				[string]: _
			} | fn.#Fn
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
}
