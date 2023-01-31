package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#WAF: {
	#ByteMatchSet: {
		Type: "AWS::WAF::ByteMatchSet"
		Properties: {
			ByteMatchTuples?: *[...{
				FieldToMatch: *{
					Data?: *string | fn.#Fn
					Type:  *string | fn.#Fn
				} | fn.#If
				PositionalConstraint: *string | fn.#Fn
				TargetString?:        *string | fn.#Fn
				TargetStringBase64?:  *string | fn.#Fn
				TextTransformation:   *string | fn.#Fn
			}] | fn.#If
			Name: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#IPSet: {
		Type: "AWS::WAF::IPSet"
		Properties: {
			IPSetDescriptors?: *[...{
				Type:  *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			Name: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Rule: {
		Type: "AWS::WAF::Rule"
		Properties: {
			MetricName:  *string | fn.#Fn
			Name:        *string | fn.#Fn
			Predicates?: *[...{
				DataId:  *string | fn.#Fn
				Negated: *bool | fn.#Fn
				Type:    *("ByteMatch" | "GeoMatch" | "IPMatch" | "RegexMatch" | "SizeConstraint" | "SqlInjectionMatch" | "XssMatch") | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#SizeConstraintSet: {
		Type: "AWS::WAF::SizeConstraintSet"
		Properties: {
			Name:            *string | fn.#Fn
			SizeConstraints: *[...{
				ComparisonOperator: *string | fn.#Fn
				FieldToMatch:       *{
					Data?: *string | fn.#Fn
					Type:  *string | fn.#Fn
				} | fn.#If
				Size:               *int | fn.#Fn
				TextTransformation: *string | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#SqlInjectionMatchSet: {
		Type: "AWS::WAF::SqlInjectionMatchSet"
		Properties: {
			Name:                     *string | fn.#Fn
			SqlInjectionMatchTuples?: *[...{
				FieldToMatch: *{
					Data?: *string | fn.#Fn
					Type:  *string | fn.#Fn
				} | fn.#If
				TextTransformation: *string | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#WebACL: {
		Type: "AWS::WAF::WebACL"
		Properties: {
			DefaultAction: *{
				Type: *string | fn.#Fn
			} | fn.#If
			MetricName: *string | fn.#Fn
			Name:       *string | fn.#Fn
			Rules?:     *[...{
				Action?: *{
					Type: *string | fn.#Fn
				} | fn.#If
				Priority: *int | fn.#Fn
				RuleId:   *string | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#XssMatchSet: {
		Type: "AWS::WAF::XssMatchSet"
		Properties: {
			Name:           *string | fn.#Fn
			XssMatchTuples: *[...{
				FieldToMatch: *{
					Data?: *string | fn.#Fn
					Type:  *string | fn.#Fn
				} | fn.#If
				TextTransformation: *string | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
