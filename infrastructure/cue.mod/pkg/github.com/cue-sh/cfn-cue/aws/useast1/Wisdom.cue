package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#Wisdom: {
	#Assistant: {
		Type: "AWS::Wisdom::Assistant"
		Properties: {
			Description?:                       *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
			Name:                               *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
			ServerSideEncryptionConfiguration?: *{
				KmsKeyId?: *(strings.MinRunes(1) & strings.MaxRunes(4096)) | fn.#Fn
			} | fn.#If
			Tags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			Type: *("AGENT") | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#AssistantAssociation: {
		Type: "AWS::Wisdom::AssistantAssociation"
		Properties: {
			AssistantId: *(=~#"^[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}$"#) | fn.#Fn
			Association: *{
				KnowledgeBaseId: *(=~#"^[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}$"#) | fn.#Fn
			} | fn.#If
			AssociationType: *("KNOWLEDGE_BASE") | fn.#Fn
			Tags?:           *[...{
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
	#KnowledgeBase: {
		Type: "AWS::Wisdom::KnowledgeBase"
		Properties: {
			Description?:            *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
			KnowledgeBaseType:       *("EXTERNAL" | "CUSTOM") | fn.#Fn
			Name:                    *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
			RenderingConfiguration?: *{
				TemplateUri?: *(strings.MinRunes(1) & strings.MaxRunes(4096)) | fn.#Fn
			} | fn.#If
			ServerSideEncryptionConfiguration?: *{
				KmsKeyId?: *(strings.MinRunes(1) & strings.MaxRunes(4096)) | fn.#Fn
			} | fn.#If
			SourceConfiguration?: *{
				AppIntegrations?: *{
					AppIntegrationArn: *(strings.MinRunes(1) & strings.MaxRunes(2048) & (=~#"^arn:[a-z-]+?:[a-z-]+?:[a-z0-9-]*?:([0-9]{12})?:[a-zA-Z0-9-:/]+$"#)) | fn.#Fn
					ObjectFields:      [...(*(strings.MinRunes(1) & strings.MaxRunes(4096)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(4096)) | fn.#Fn)
				} | fn.#If
			} | fn.#If
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
}
