package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#Location: {
	#GeofenceCollection: {
		Type: "AWS::Location::GeofenceCollection"
		Properties: {
			CollectionName:         *(strings.MinRunes(1) & strings.MaxRunes(100) & (=~#"^[-._\w]+$"#)) | fn.#Fn
			Description?:           *string | fn.#Fn
			KmsKeyId?:              *(strings.MinRunes(1) & strings.MaxRunes(2048)) | fn.#Fn
			PricingPlan:            *("RequestBasedUsage" | "MobileAssetTracking" | "MobileAssetManagement") | fn.#Fn
			PricingPlanDataSource?: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Map: {
		Type: "AWS::Location::Map"
		Properties: {
			Configuration: *{
				Style: *(strings.MinRunes(1) & strings.MaxRunes(100) & (=~#"^[-._\w]+$"#)) | fn.#Fn
			} | fn.#If
			Description?: *string | fn.#Fn
			MapName:      *(strings.MinRunes(1) & strings.MaxRunes(100) & (=~#"^[-._\w]+$"#)) | fn.#Fn
			PricingPlan:  *("RequestBasedUsage" | "MobileAssetTracking" | "MobileAssetManagement") | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#PlaceIndex: {
		Type: "AWS::Location::PlaceIndex"
		Properties: {
			DataSource:               *string | fn.#Fn
			DataSourceConfiguration?: *{
				IntendedUse?: *("SingleUse" | "Storage") | fn.#Fn
			} | fn.#If
			Description?: *string | fn.#Fn
			IndexName:    *(strings.MinRunes(1) & strings.MaxRunes(100) & (=~#"^[-._\w]+$"#)) | fn.#Fn
			PricingPlan:  *("RequestBasedUsage" | "MobileAssetTracking" | "MobileAssetManagement") | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#RouteCalculator: {
		Type: "AWS::Location::RouteCalculator"
		Properties: {
			CalculatorName: *(strings.MinRunes(1) & strings.MaxRunes(100) & (=~#"^[-._\w]+$"#)) | fn.#Fn
			DataSource:     *string | fn.#Fn
			Description?:   *string | fn.#Fn
			PricingPlan:    *("RequestBasedUsage" | "MobileAssetManagement") | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Tracker: {
		Type: "AWS::Location::Tracker"
		Properties: {
			Description?:           *string | fn.#Fn
			KmsKeyId?:              *(strings.MinRunes(1) & strings.MaxRunes(2048)) | fn.#Fn
			PositionFiltering?:     *("TimeBased" | "DistanceBased") | fn.#Fn
			PricingPlan:            *("RequestBasedUsage" | "MobileAssetTracking" | "MobileAssetManagement") | fn.#Fn
			PricingPlanDataSource?: *string | fn.#Fn
			TrackerName:            *(strings.MinRunes(1) & strings.MaxRunes(100) & (=~#"^[-._\w]+$"#)) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#TrackerConsumer: {
		Type: "AWS::Location::TrackerConsumer"
		Properties: {
			ConsumerArn: *(=~#"^arn(:[a-z0-9]+([.-][a-z0-9]+)*){2}(:([a-z0-9]+([.-][a-z0-9]+)*)?){2}:([^/].*)?$"#) | fn.#Fn
			TrackerName: *(strings.MinRunes(1) & strings.MaxRunes(100) & (=~#"^[-._\w]+$"#)) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
