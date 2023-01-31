package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#CodeArtifact: {
	#Domain: {
		Type: "AWS::CodeArtifact::Domain"
		Properties: {
			DomainName:                 *(strings.MinRunes(2) & strings.MaxRunes(50) & (=~#"^([a-z][a-z0-9\-]{0,48}[a-z0-9])$"#)) | fn.#Fn
			EncryptionKey?:             *string | fn.#Fn
			PermissionsPolicyDocument?: *{
				[string]: _
			} | fn.#Fn
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
	#Repository: {
		Type: "AWS::CodeArtifact::Repository"
		Properties: {
			Description?:               *string | fn.#Fn
			DomainName:                 *(strings.MinRunes(2) & strings.MaxRunes(50) & (=~#"^([a-z][a-z0-9\-]{0,48}[a-z0-9])$"#)) | fn.#Fn
			DomainOwner?:               *(=~#"[0-9]{12}"#) | fn.#Fn
			ExternalConnections?:       [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			PermissionsPolicyDocument?: *{
				[string]: _
			} | fn.#Fn
			RepositoryName: *(strings.MinRunes(2) & strings.MaxRunes(100) & (=~#"^([A-Za-z0-9][A-Za-z0-9._\-]{1,99})$"#)) | fn.#Fn
			Tags?:          *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			Upstreams?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
