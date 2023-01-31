package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#Route53RecoveryControl: {
	#Cluster: {
		Type: "AWS::Route53RecoveryControl::Cluster"
		Properties: {
			Name?: *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
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
	#ControlPanel: {
		Type: "AWS::Route53RecoveryControl::ControlPanel"
		Properties: {
			ClusterArn?: *string | fn.#Fn
			Name:        *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
			Tags?:       *[...{
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
	#RoutingControl: {
		Type: "AWS::Route53RecoveryControl::RoutingControl"
		Properties: {
			ClusterArn?:      *string | fn.#Fn
			ControlPanelArn?: *string | fn.#Fn
			Name:             *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#SafetyRule: {
		Type: "AWS::Route53RecoveryControl::SafetyRule"
		Properties: {
			AssertionRule?: *{
				AssertedControls: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				WaitPeriodMs:     *int | fn.#Fn
			} | fn.#If
			ControlPanelArn: *string | fn.#Fn
			GatingRule?:     *{
				GatingControls: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				TargetControls: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				WaitPeriodMs:   *int | fn.#Fn
			} | fn.#If
			Name:       *string | fn.#Fn
			RuleConfig: *{
				Inverted:  *bool | fn.#Fn
				Threshold: *int | fn.#Fn
				Type:      *("AND" | "OR" | "ATLEAST") | fn.#Fn
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
}
