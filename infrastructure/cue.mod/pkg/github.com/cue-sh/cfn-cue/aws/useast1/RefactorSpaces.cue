package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#RefactorSpaces: {
	#Application: {
		Type: "AWS::RefactorSpaces::Application"
		Properties: {
			ApiGatewayProxy?: *{
				EndpointType?: *("REGIONAL" | "PRIVATE") | fn.#Fn
				StageName?:    *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[-a-zA-Z0-9_]*$"#)) | fn.#Fn
			} | fn.#If
			EnvironmentIdentifier?: *(strings.MinRunes(14) & strings.MaxRunes(14) & (=~#"^env-([0-9A-Za-z]{10}$)"#)) | fn.#Fn
			Name?:                  *(strings.MinRunes(3) & strings.MaxRunes(63) & (=~#"^(?!app-)[a-zA-Z0-9]+[a-zA-Z0-9-_ ]+$"#)) | fn.#Fn
			ProxyType?:             *("API_GATEWAY") | fn.#Fn
			Tags?:                  *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			VpcId?: *(strings.MinRunes(12) & strings.MaxRunes(21) & (=~#"^vpc-[-a-f0-9]{8}([-a-f0-9]{9})?$"#)) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Environment: {
		Type: "AWS::RefactorSpaces::Environment"
		Properties: {
			Description?:       *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"^[a-zA-Z0-9-_\s\.\!\*\#\@\']+$"#)) | fn.#Fn
			Name?:              *(strings.MinRunes(3) & strings.MaxRunes(63) & (=~#"^(?!env-)[a-zA-Z0-9]+[a-zA-Z0-9-_ ]+$"#)) | fn.#Fn
			NetworkFabricType?: *("TRANSIT_GATEWAY") | fn.#Fn
			Tags?:              *[...{
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
	#Route: {
		Type: "AWS::RefactorSpaces::Route"
		Properties: {
			ApplicationIdentifier: *(strings.MinRunes(14) & strings.MaxRunes(14) & (=~#"^app-([0-9A-Za-z]{10}$)"#)) | fn.#Fn
			EnvironmentIdentifier: *(strings.MinRunes(14) & strings.MaxRunes(14) & (=~#"^env-([0-9A-Za-z]{10}$)"#)) | fn.#Fn
			RouteType?:            *("DEFAULT" | "URI_PATH") | fn.#Fn
			ServiceIdentifier:     *(strings.MinRunes(14) & strings.MaxRunes(14) & (=~#"^svc-([0-9A-Za-z]{10}$)"#)) | fn.#Fn
			Tags?:                 *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			UriPathRoute?: *{
				ActivationState:    *("ACTIVE") | fn.#Fn
				IncludeChildPaths?: *bool | fn.#Fn
				Methods?:           [...(*("DELETE" | "GET" | "HEAD" | "OPTIONS" | "PATCH" | "POST" | "PUT") | fn.#Fn)] | (*("DELETE" | "GET" | "HEAD" | "OPTIONS" | "PATCH" | "POST" | "PUT") | fn.#Fn)
				SourcePath?:        *(strings.MinRunes(1) & strings.MaxRunes(2048) & (=~#"^(/[a-zA-Z0-9._-]+)+$"#)) | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Service: {
		Type: "AWS::RefactorSpaces::Service"
		Properties: {
			ApplicationIdentifier: *(strings.MinRunes(14) & strings.MaxRunes(14) & (=~#"^app-([0-9A-Za-z]{10}$)"#)) | fn.#Fn
			Description?:          *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"^[a-zA-Z0-9-_\s\.\!\*\#\@\']+$"#)) | fn.#Fn
			EndpointType?:         *("LAMBDA" | "URL") | fn.#Fn
			EnvironmentIdentifier: *(strings.MinRunes(14) & strings.MaxRunes(14) & (=~#"^env-([0-9A-Za-z]{10}$)"#)) | fn.#Fn
			LambdaEndpoint?:       *{
				Arn: *(strings.MinRunes(1) & strings.MaxRunes(2048) & (=~#"^arn:(aws[a-zA-Z-]*)?:lambda:[a-z]{2}((-gov)|(-iso(b?)))?-[a-z]+-\d{1}:\d{12}:function:[a-zA-Z0-9-_]+(:(\$LATEST|[a-zA-Z0-9-_]+))?$"#)) | fn.#Fn
			} | fn.#If
			Name?: *(strings.MinRunes(3) & strings.MaxRunes(63) & (=~#"^(?!svc-)[a-zA-Z0-9]+[a-zA-Z0-9-_ ]+$"#)) | fn.#Fn
			Tags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			UrlEndpoint?: *{
				HealthUrl?: *(strings.MinRunes(1) & strings.MaxRunes(2048) & (=~#"^https?://[-a-zA-Z0-9+\x38@#/%?=~_|!:,.;]*[-a-zA-Z0-9+\x38@#/%=~_|]$"#)) | fn.#Fn
				Url:        *(strings.MinRunes(1) & strings.MaxRunes(2048) & (=~#"^https?://[-a-zA-Z0-9+\x38@#/%?=~_|!:,.;]*[-a-zA-Z0-9+\x38@#/%=~_|]$"#)) | fn.#Fn
			} | fn.#If
			VpcId?: *(strings.MinRunes(12) & strings.MaxRunes(21) & (=~#"^vpc-[-a-f0-9]{8}([-a-f0-9]{9})?$"#)) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
