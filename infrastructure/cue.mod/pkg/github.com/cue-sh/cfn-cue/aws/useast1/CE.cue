package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#CE: {
	#AnomalyMonitor: {
		Type: "AWS::CE::AnomalyMonitor"
		Properties: {
			MonitorDimension?:     *("SERVICE") | fn.#Fn
			MonitorName:           *(=~#"[\S\s]*"#) | fn.#Fn
			MonitorSpecification?: *string | fn.#Fn
			MonitorType:           *("DIMENSIONAL" | "CUSTOM") | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#AnomalySubscription: {
		Type: "AWS::CE::AnomalySubscription"
		Properties: {
			Frequency:      *("DAILY" | "IMMEDIATE" | "WEEKLY") | fn.#Fn
			MonitorArnList: [...(*(=~#"^arn:aws[-a-z0-9]*:[a-z0-9]+:[-a-z0-9]*:[0-9]{12}:[-a-zA-Z0-9/:_]+$"#) | fn.#Fn)] | (*(=~#"^arn:aws[-a-z0-9]*:[a-z0-9]+:[-a-z0-9]*:[0-9]{12}:[-a-zA-Z0-9/:_]+$"#) | fn.#Fn)
			Subscribers:    *[...{
				Address: *string | fn.#Fn
				Status?: *("CONFIRMED" | "DECLINED") | fn.#Fn
				Type:    *("EMAIL" | "SNS") | fn.#Fn
			}] | fn.#If
			SubscriptionName: *(=~#"[\S\s]*"#) | fn.#Fn
			Threshold:        *number | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#CostCategory: {
		Type: "AWS::CE::CostCategory"
		Properties: {
			DefaultValue?:     *(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn
			Name:              *(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn
			RuleVersion:       *("CostCategoryExpression.v1") | fn.#Fn
			Rules:             *string | fn.#Fn
			SplitChargeRules?: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
