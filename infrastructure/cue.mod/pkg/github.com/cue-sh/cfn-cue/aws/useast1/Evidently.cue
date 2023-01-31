package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#Evidently: {
	#Experiment: {
		Type: "AWS::Evidently::Experiment"
		Properties: {
			Description?: *string | fn.#Fn
			MetricGoals:  *[...{
				DesiredChange: *("INCREASE" | "DECREASE") | fn.#Fn
				EntityIdKey:   *string | fn.#Fn
				EventPattern:  *string | fn.#Fn
				MetricName:    *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"^[\S]+$"#)) | fn.#Fn
				UnitLabel?:    *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#".*"#)) | fn.#Fn
				ValueKey:      *string | fn.#Fn
			}] | fn.#If
			Name:           *(strings.MinRunes(1) & strings.MaxRunes(127) & (=~#"[-a-zA-Z0-9._]*"#)) | fn.#Fn
			OnlineAbConfig: *{
				ControlTreatmentName?: *(strings.MinRunes(1) & strings.MaxRunes(127) & (=~#"[-a-zA-Z0-9._]*"#)) | fn.#Fn
				TreatmentWeights?:     *[...{
					SplitWeight: *int | fn.#Fn
					Treatment:   *(strings.MinRunes(1) & strings.MaxRunes(127) & (=~#"[-a-zA-Z0-9._]*"#)) | fn.#Fn
				}] | fn.#If
			} | fn.#If
			Project:            *(=~#"([-a-zA-Z0-9._]*)|(arn:[^:]*:[^:]*:[^:]*:[^:]*:project/[-a-zA-Z0-9._]*)"#) | fn.#Fn
			RandomizationSalt?: *(=~#".*"#) | fn.#Fn
			SamplingRate?:      *int | fn.#Fn
			Tags?:              *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			Treatments: *[...{
				Description?:  *string | fn.#Fn
				Feature:       *(=~#"([-a-zA-Z0-9._]*)|(arn:[^:]*:[^:]*:[^:]*:[^:]*:.*)"#) | fn.#Fn
				TreatmentName: *(strings.MinRunes(1) & strings.MaxRunes(127) & (=~#"[-a-zA-Z0-9._]*"#)) | fn.#Fn
				Variation:     *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"[-a-zA-Z0-9._]*"#)) | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Feature: {
		Type: "AWS::Evidently::Feature"
		Properties: {
			DefaultVariation?: *(strings.MinRunes(1) & strings.MaxRunes(127) & (=~#"[-a-zA-Z0-9._]*"#)) | fn.#Fn
			Description?:      *string | fn.#Fn
			EntityOverrides?:  *[...{
				EntityId?:  *string | fn.#Fn
				Variation?: *(strings.MinRunes(1) & strings.MaxRunes(127) & (=~#"[-a-zA-Z0-9._]*"#)) | fn.#Fn
			}] | fn.#If
			EvaluationStrategy?: *("ALL_RULES" | "DEFAULT_VARIATION") | fn.#Fn
			Name:                *(strings.MinRunes(1) & strings.MaxRunes(127) & (=~#"[-a-zA-Z0-9._]*"#)) | fn.#Fn
			Project:             *(=~#"([-a-zA-Z0-9._]*)|(arn:[^:]*:[^:]*:[^:]*:[^:]*:project/[-a-zA-Z0-9._]*)"#) | fn.#Fn
			Tags?:               *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			Variations: *[...{
				BooleanValue?:  *bool | fn.#Fn
				DoubleValue?:   *number | fn.#Fn
				LongValue?:     *number | fn.#Fn
				StringValue?:   *string | fn.#Fn
				VariationName?: *(strings.MinRunes(1) & strings.MaxRunes(127) & (=~#"[-a-zA-Z0-9._]*"#)) | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Launch: {
		Type: "AWS::Evidently::Launch"
		Properties: {
			Description?: *string | fn.#Fn
			Groups:       *[...{
				Description?: *string | fn.#Fn
				Feature:      *string | fn.#Fn
				GroupName:    *(strings.MinRunes(1) & strings.MaxRunes(127) & (=~#"[-a-zA-Z0-9._]*"#)) | fn.#Fn
				Variation:    *string | fn.#Fn
			}] | fn.#If
			MetricMonitors?: *[...{
				EntityIdKey:  *string | fn.#Fn
				EventPattern: *string | fn.#Fn
				MetricName:   *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"^[\S]+$"#)) | fn.#Fn
				UnitLabel?:   *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#".*"#)) | fn.#Fn
				ValueKey:     *string | fn.#Fn
			}] | fn.#If
			Name:                  *(strings.MinRunes(1) & strings.MaxRunes(127) & (=~#"[-a-zA-Z0-9._]*"#)) | fn.#Fn
			Project:               *(=~#"([-a-zA-Z0-9._]*)|(arn:[^:]*:[^:]*:[^:]*:[^:]*:project/[-a-zA-Z0-9._]*)"#) | fn.#Fn
			RandomizationSalt?:    *(=~#".*"#) | fn.#Fn
			ScheduledSplitsConfig: *[...{
				GroupWeights: *[...{
					GroupName:   *(strings.MinRunes(1) & strings.MaxRunes(127) & (=~#"[-a-zA-Z0-9._]*"#)) | fn.#Fn
					SplitWeight: *int | fn.#Fn
				}] | fn.#If
				StartTime: *string | fn.#Fn
			}] | fn.#If
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
	#Project: {
		Type: "AWS::Evidently::Project"
		Properties: {
			DataDelivery?: *{
				[string]: _
			} | fn.#Fn
			Description?: *string | fn.#Fn
			Name:         *(strings.MinRunes(1) & strings.MaxRunes(127) & (=~#"[-a-zA-Z0-9._]*"#)) | fn.#Fn
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
}
