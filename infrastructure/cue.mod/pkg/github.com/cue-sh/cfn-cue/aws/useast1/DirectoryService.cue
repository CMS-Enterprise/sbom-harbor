package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#DirectoryService: {
	#MicrosoftAD: {
		Type: "AWS::DirectoryService::MicrosoftAD"
		Properties: {
			CreateAlias?: *bool | fn.#Fn
			Edition?:     *("Enterprise" | "Standard") | fn.#Fn
			EnableSso?:   *bool | fn.#Fn
			Name:         *string | fn.#Fn
			Password:     *string | fn.#Fn
			ShortName?:   *string | fn.#Fn
			VpcSettings:  *{
				SubnetIds: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				VpcId:     *string | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#SimpleAD: {
		Type: "AWS::DirectoryService::SimpleAD"
		Properties: {
			CreateAlias?: *bool | fn.#Fn
			Description?: *string | fn.#Fn
			EnableSso?:   *bool | fn.#Fn
			Name:         *string | fn.#Fn
			Password:     *string | fn.#Fn
			ShortName?:   *string | fn.#Fn
			Size:         *("Large" | "Small") | fn.#Fn
			VpcSettings:  *{
				SubnetIds: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				VpcId:     *string | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
