package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#AmplifyUIBuilder: {
	#Component: {
		Type: "AWS::AmplifyUIBuilder::Component"
		Properties: {
			BindingProperties?: *{
				[string]: {
					BindingProperties?: *{
						Bucket?:        *string | fn.#Fn
						DefaultValue?:  *string | fn.#Fn
						Field?:         *string | fn.#Fn
						Key?:           *string | fn.#Fn
						Model?:         *string | fn.#Fn
						Predicates?:    *[..._#Predicate] | fn.#If
						UserAttribute?: *string | fn.#Fn
					} | fn.#If
					DefaultValue?: *string | fn.#Fn
					Type?:         *string | fn.#Fn
				}
			} | fn.#If
			Children?:             *[..._#ComponentChild] | fn.#If
			CollectionProperties?: *{
				[string]: {
					Identifiers?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					Model:        *string | fn.#Fn
					Predicate?:   *_#Predicate | fn.#If
					Sort?:        *[...{
						Direction: *string | fn.#Fn
						Field:     *string | fn.#Fn
					}] | fn.#If
				}
			} | fn.#If
			ComponentType?: *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
			Name?:          *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
			Overrides?:     *{
				[string]: {}
			} | fn.#If
			Properties?: *{
				[string]: _#ComponentProperty
			} | fn.#If
			SourceId?: *string | fn.#Fn
			Tags?:     *{
				[string]: *string | fn.#Fn
			} | fn.#If
			Variants?: *[...{
				Overrides?:     *{} | fn.#If
				VariantValues?: *{} | fn.#If
			}] | fn.#If
			_#Predicate: {
				And?:      *[..._#Predicate] | fn.#If
				Field?:    *string | fn.#Fn
				Operand?:  *string | fn.#Fn
				Operator?: *string | fn.#Fn
				Or?:       *[..._#Predicate] | fn.#If
			}
			_#ComponentChild: {
				Children?:     *[..._#ComponentChild] | fn.#If
				ComponentType: *string | fn.#Fn
				Name:          *string | fn.#Fn
				Properties:    *{} | fn.#If
			}
			_#ComponentProperty: {
				BindingProperties?: *{
					Field?:   *string | fn.#Fn
					Property: *string | fn.#Fn
				} | fn.#If
				Bindings?:                    *{} | fn.#If
				CollectionBindingProperties?: *{
					Field?:   *string | fn.#Fn
					Property: *string | fn.#Fn
				} | fn.#If
				Concat?:    *[..._#ComponentProperty] | fn.#If
				Condition?: *{
					Else?:     *_#ComponentProperty | fn.#If
					Field?:    *string | fn.#Fn
					Operand?:  *string | fn.#Fn
					Operator?: *string | fn.#Fn
					Property?: *string | fn.#Fn
					Then?:     *_#ComponentProperty | fn.#If
				} | fn.#If
				Configured?:    *bool | fn.#Fn
				DefaultValue?:  *string | fn.#Fn
				Event?:         *string | fn.#Fn
				ImportedValue?: *string | fn.#Fn
				Model?:         *string | fn.#Fn
				Type?:          *string | fn.#Fn
				UserAttribute?: *string | fn.#Fn
				Value?:         *string | fn.#Fn
			}
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Theme: {
		Type: "AWS::AmplifyUIBuilder::Theme"
		Properties: {
			Name:       *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
			Overrides?: *[..._#ThemeValues] | fn.#If
			Tags?:      *{
				[string]: *string | fn.#Fn
			} | fn.#If
			Values: *[..._#ThemeValues] | fn.#If
			_#ThemeValues: {
				Key?:   *string | fn.#Fn
				Value?: *{
					Children?: *[..._#ThemeValues] | fn.#If
					Value?:    *string | fn.#Fn
				} | fn.#If
			}
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
