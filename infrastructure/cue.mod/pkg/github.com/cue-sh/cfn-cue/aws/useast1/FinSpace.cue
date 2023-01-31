package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#FinSpace: {
	#Environment: {
		Type: "AWS::FinSpace::Environment"
		Properties: {
			DataBundles?:          [...(*(=~#"^arn:aws:finspace:[A-Za-z0-9_/.-]{0,63}:\d*:data-bundle/[0-9A-Za-z_-]{1,128}$"#) | fn.#Fn)] | (*(=~#"^arn:aws:finspace:[A-Za-z0-9_/.-]{0,63}:\d*:data-bundle/[0-9A-Za-z_-]{1,128}$"#) | fn.#Fn)
			Description?:          *(=~#"^[a-zA-Z0-9. ]{1,1000}$"#) | fn.#Fn
			FederationMode?:       *("LOCAL" | "FEDERATED") | fn.#Fn
			FederationParameters?: *{
				ApplicationCallBackURL?: *(=~#"^https?://[-a-zA-Z0-9+&amp;@#/%?=~_|!:,.;]*[-a-zA-Z0-9+&amp;@#/%=~_|]{1,1000}"#) | fn.#Fn
				AttributeMap?:           *{
					[string]: _
				} | fn.#Fn
				FederationProviderName?: *string | fn.#Fn
				FederationURN?:          *string | fn.#Fn
				SamlMetadataDocument?:   *(strings.MinRunes(1000) & strings.MaxRunes(10000000) & (=~#".*"#)) | fn.#Fn
				SamlMetadataURL?:        *(=~#"^https?://[-a-zA-Z0-9+&amp;@#/%?=~_|!:,.;]*[-a-zA-Z0-9+&amp;@#/%=~_|]{1,1000}"#) | fn.#Fn
			} | fn.#If
			KmsKeyId?:            *string | fn.#Fn
			Name:                 *(=~#"^[a-zA-Z0-9]+[a-zA-Z0-9-]*[a-zA-Z0-9]{1,255}$"#) | fn.#Fn
			SuperuserParameters?: *{
				EmailAddress?: *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"[A-Z0-9a-z._%+-]+@[A-Za-z0-9.-]+[.]+[A-Za-z]+"#)) | fn.#Fn
				FirstName?:    *(strings.MinRunes(1) & strings.MaxRunes(50) & (=~#"^[a-zA-Z0-9]{1,50}$"#)) | fn.#Fn
				LastName?:     *(strings.MinRunes(1) & strings.MaxRunes(50) & (=~#"^[a-zA-Z0-9]{1,50}$"#)) | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
