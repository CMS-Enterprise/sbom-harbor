package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#MediaConvert: {
	#JobTemplate: {
		Type: "AWS::MediaConvert::JobTemplate"
		Properties: {
			AccelerationSettings?: *{
				Mode: *string | fn.#Fn
			} | fn.#If
			Category?:        *string | fn.#Fn
			Description?:     *string | fn.#Fn
			HopDestinations?: *[...{
				Priority?:    *int | fn.#Fn
				Queue?:       *string | fn.#Fn
				WaitMinutes?: *int | fn.#Fn
			}] | fn.#If
			Name?:        *string | fn.#Fn
			Priority?:    *int | fn.#Fn
			Queue?:       *string | fn.#Fn
			SettingsJson: *{
				[string]: _
			} | fn.#Fn
			StatusUpdateInterval?: *string | fn.#Fn
			Tags?:                 *{
				[string]: _
			} | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Preset: {
		Type: "AWS::MediaConvert::Preset"
		Properties: {
			Category?:    *string | fn.#Fn
			Description?: *string | fn.#Fn
			Name?:        *string | fn.#Fn
			SettingsJson: *{
				[string]: _
			} | fn.#Fn
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
	#Queue: {
		Type: "AWS::MediaConvert::Queue"
		Properties: {
			Description?: *string | fn.#Fn
			Name?:        *string | fn.#Fn
			PricingPlan?: *string | fn.#Fn
			Status?:      *string | fn.#Fn
			Tags?:        *{
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
