package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#XRay: {
	#Group: {
		Type: "AWS::XRay::Group"
		Properties: {
			FilterExpression?:      *string | fn.#Fn
			GroupName?:             *(strings.MinRunes(1) & strings.MaxRunes(32)) | fn.#Fn
			InsightsConfiguration?: *{
				InsightsEnabled?:      *bool | fn.#Fn
				NotificationsEnabled?: *bool | fn.#Fn
			} | fn.#If
			Tags?: [...(*{
				[string]: _
			} | fn.#Fn)] | (*{
				[string]: _
			} | fn.#Fn)
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#SamplingRule: {
		Type: "AWS::XRay::SamplingRule"
		Properties: {
			RuleName?:     *(strings.MinRunes(1) & strings.MaxRunes(32)) | fn.#Fn
			SamplingRule?: *{
				Attributes?: *{
					[string]: *string | fn.#Fn
				} | fn.#If
				FixedRate?:     *number | fn.#Fn
				HTTPMethod?:    *string | fn.#Fn
				Host?:          *string | fn.#Fn
				Priority?:      *(>=1 & <=9999) | fn.#Fn
				ReservoirSize?: *int | fn.#Fn
				ResourceARN?:   *string | fn.#Fn
				RuleARN?:       *string | fn.#Fn
				RuleName?:      *(strings.MinRunes(1) & strings.MaxRunes(32)) | fn.#Fn
				ServiceName?:   *string | fn.#Fn
				ServiceType?:   *string | fn.#Fn
				URLPath?:       *string | fn.#Fn
				Version?:       *int | fn.#Fn
			} | fn.#If
			SamplingRuleRecord?: *{
				CreatedAt?:    *string | fn.#Fn
				ModifiedAt?:   *string | fn.#Fn
				SamplingRule?: *{
					Attributes?: *{
						[string]: *string | fn.#Fn
					} | fn.#If
					FixedRate?:     *number | fn.#Fn
					HTTPMethod?:    *string | fn.#Fn
					Host?:          *string | fn.#Fn
					Priority?:      *(>=1 & <=9999) | fn.#Fn
					ReservoirSize?: *int | fn.#Fn
					ResourceARN?:   *string | fn.#Fn
					RuleARN?:       *string | fn.#Fn
					RuleName?:      *(strings.MinRunes(1) & strings.MaxRunes(32)) | fn.#Fn
					ServiceName?:   *string | fn.#Fn
					ServiceType?:   *string | fn.#Fn
					URLPath?:       *string | fn.#Fn
					Version?:       *int | fn.#Fn
				} | fn.#If
			} | fn.#If
			SamplingRuleUpdate?: *{
				Attributes?: *{
					[string]: *string | fn.#Fn
				} | fn.#If
				FixedRate?:     *number | fn.#Fn
				HTTPMethod?:    *string | fn.#Fn
				Host?:          *string | fn.#Fn
				Priority?:      *(>=1 & <=9999) | fn.#Fn
				ReservoirSize?: *int | fn.#Fn
				ResourceARN?:   *string | fn.#Fn
				RuleARN?:       *string | fn.#Fn
				RuleName?:      *(strings.MinRunes(1) & strings.MaxRunes(32)) | fn.#Fn
				ServiceName?:   *string | fn.#Fn
				ServiceType?:   *string | fn.#Fn
				URLPath?:       *string | fn.#Fn
			} | fn.#If
			Tags?: [...(*{
				[string]: _
			} | fn.#Fn)] | (*{
				[string]: _
			} | fn.#Fn)
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
