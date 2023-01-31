package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#ResourceGroups: {
	#Group: {
		Type: "AWS::ResourceGroups::Group"
		Properties: {
			Configuration?: *[...{
				Parameters?: *[...{
					Name?:   *string | fn.#Fn
					Values?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				}] | fn.#If
				Type?: *string | fn.#Fn
			}] | fn.#If
			Description?:   *string | fn.#Fn
			Name:           *string | fn.#Fn
			ResourceQuery?: *{
				Query?: *{
					ResourceTypeFilters?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					StackIdentifier?:     *string | fn.#Fn
					TagFilters?:          *[...{
						Key?:    *string | fn.#Fn
						Values?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					}] | fn.#If
				} | fn.#If
				Type?: *("TAG_FILTERS_1_0" | "CLOUDFORMATION_STACK_1_0") | fn.#Fn
			} | fn.#If
			Resources?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			Tags?:      *[...{
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
