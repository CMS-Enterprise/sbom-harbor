package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#RAM: {
	#ResourceShare: {
		Type: "AWS::RAM::ResourceShare"
		Properties: {
			AllowExternalPrincipals?: *bool | fn.#Fn
			Name:                     *string | fn.#Fn
			PermissionArns?:          [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			Principals?:              [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			ResourceArns?:            [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			Tags?:                    *[...{
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
