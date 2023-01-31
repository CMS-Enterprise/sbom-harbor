package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#ServiceCatalogAppRegistry: {
	#Application: {
		Type: "AWS::ServiceCatalogAppRegistry::Application"
		Properties: {
			Description?: *string | fn.#Fn
			Name:         *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"\w+"#)) | fn.#Fn
			Tags?:        *{
				[string]: *string | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#AttributeGroup: {
		Type: "AWS::ServiceCatalogAppRegistry::AttributeGroup"
		Properties: {
			Attributes: *{
				[string]: _
			} | fn.#Fn
			Description?: *string | fn.#Fn
			Name:         *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"\w+"#)) | fn.#Fn
			Tags?:        *{
				[string]: *string | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#AttributeGroupAssociation: {
		Type: "AWS::ServiceCatalogAppRegistry::AttributeGroupAssociation"
		Properties: {
			Application:    *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"\w+|[a-z0-9]{12}"#)) | fn.#Fn
			AttributeGroup: *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"\w+|[a-z0-9]{12}"#)) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ResourceAssociation: {
		Type: "AWS::ServiceCatalogAppRegistry::ResourceAssociation"
		Properties: {
			Application:  *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"\w+|[a-z0-9]{12}"#)) | fn.#Fn
			Resource:     *(=~#"\w+|arn:aws[-a-z]*:cloudformation:[a-z]{2}(-gov)?-[a-z]+-\d:\d{12}:stack/[a-zA-Z][-A-Za-z0-9]{0,127}/[0-9a-f]{8}(-[0-9a-f]{4}){3}-[0-9a-f]{12}"#) | fn.#Fn
			ResourceType: *("CFN_STACK") | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
