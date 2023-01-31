package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#EventSchemas: {
	#Discoverer: {
		Type: "AWS::EventSchemas::Discoverer"
		Properties: {
			CrossAccount?: *bool | fn.#Fn
			Description?:  *string | fn.#Fn
			SourceArn:     *string | fn.#Fn
			Tags?:         *[...{
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
	#Registry: {
		Type: "AWS::EventSchemas::Registry"
		Properties: {
			Description?:  *string | fn.#Fn
			RegistryName?: *string | fn.#Fn
			Tags?:         *[...{
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
	#RegistryPolicy: {
		Type: "AWS::EventSchemas::RegistryPolicy"
		Properties: {
			Policy: *{
				[string]: _
			} | fn.#Fn
			RegistryName: *string | fn.#Fn
			RevisionId?:  *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Schema: {
		Type: "AWS::EventSchemas::Schema"
		Properties: {
			Content:      *string | fn.#Fn
			Description?: *string | fn.#Fn
			RegistryName: *string | fn.#Fn
			SchemaName?:  *string | fn.#Fn
			Tags?:        *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			Type: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
