package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#WAFRegional: {
	#ByteMatchSet: {
		Type: "AWS::WAFRegional::ByteMatchSet"
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
	#GeoMatchSet: {
		Type: "AWS::WAFRegional::GeoMatchSet"
		Properties: {
			GeoMatchConstraints?: *[...{
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
	#IPSet: {
		Type: "AWS::WAFRegional::IPSet"
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
	#RateBasedRule: {
		Type: "AWS::WAFRegional::RateBasedRule"
		Properties: {
			MatchPredicates?: *[...{
				DataId:  *string | fn.#Fn
				Negated: *bool | fn.#Fn
				Type:    *string | fn.#Fn
			}] | fn.#If
			MetricName: *string | fn.#Fn
			Name:       *string | fn.#Fn
			RateKey:    *string | fn.#Fn
			RateLimit:  *int | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#RegexPatternSet: {
		Type: "AWS::WAFRegional::RegexPatternSet"
		Properties: {
			Name:                *string | fn.#Fn
			RegexPatternStrings: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Rule: {
		Type: "AWS::WAFRegional::Rule"
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
		Type: "AWS::WAFRegional::SizeConstraintSet"
		Properties: {
			Name:             *string | fn.#Fn
			SizeConstraints?: *[...{
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
		Type: "AWS::WAFRegional::SqlInjectionMatchSet"
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
		Type: "AWS::WAFRegional::WebACL"
		Properties: {
			DefaultAction: *{
				Type: *string | fn.#Fn
			} | fn.#If
			MetricName: *string | fn.#Fn
			Name:       *string | fn.#Fn
			Rules?:     *[...{
				Action: *{
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
	#WebACLAssociation: {
		Type: "AWS::WAFRegional::WebACLAssociation"
		Properties: {
			ResourceArn: *string | fn.#Fn
			WebACLId:    *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#XssMatchSet: {
		Type: "AWS::WAFRegional::XssMatchSet"
		Properties: {
			Name:            *string | fn.#Fn
			XssMatchTuples?: *[...{
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
