package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#CertificateManager: {
	#Account: {
		Type: "AWS::CertificateManager::Account"
		Properties: ExpiryEventsConfiguration: *{
			DaysBeforeExpiry?: *(>=1 & <=45) | fn.#Fn
		} | fn.#If
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Certificate: {
		Type: "AWS::CertificateManager::Certificate"
		Properties: {
			CertificateAuthorityArn?:                  *string | fn.#Fn
			CertificateTransparencyLoggingPreference?: *string | fn.#Fn
			DomainName:                                *string | fn.#Fn
			DomainValidationOptions?:                  *[...{
				DomainName:        *string | fn.#Fn
				HostedZoneId?:     *string | fn.#Fn
				ValidationDomain?: *string | fn.#Fn
			}] | fn.#If
			SubjectAlternativeNames?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			Tags?:                    *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			ValidationMethod?: *("DNS" | "EMAIL") | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
