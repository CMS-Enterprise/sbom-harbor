package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#Signer: {
	#ProfilePermission: {
		Type: "AWS::Signer::ProfilePermission"
		Properties: {
			Action:          *string | fn.#Fn
			Principal:       *string | fn.#Fn
			ProfileName:     *string | fn.#Fn
			ProfileVersion?: *(=~#"^[0-9a-zA-Z]{10}$"#) | fn.#Fn
			StatementId:     *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#SigningProfile: {
		Type: "AWS::Signer::SigningProfile"
		Properties: {
			PlatformId:               *("AWSLambda-SHA384-ECDSA") | fn.#Fn
			SignatureValidityPeriod?: *{
				Type?:  *("DAYS" | "MONTHS" | "YEARS") | fn.#Fn
				Value?: *int | fn.#Fn
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
