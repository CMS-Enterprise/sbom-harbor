package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#ResilienceHub: {
	#App: {
		Type: "AWS::ResilienceHub::App"
		Properties: {
			AppTemplateBody:      *(=~#"^[\w\s:,-\.'{}\[\]:"]+$"#) | fn.#Fn
			Description?:         *string | fn.#Fn
			Name:                 *(=~#"^[A-Za-z0-9][A-Za-z0-9_\-]{1,59}$"#) | fn.#Fn
			ResiliencyPolicyArn?: *(=~#"^arn:(aws|aws-cn|aws-iso|aws-iso-[a-z]{1}|aws-us-gov):[A-Za-z0-9][A-Za-z0-9_/.-]{0,62}:([a-z]{2}-((iso[a-z]{0,1}-)|(gov-)){0,1}[a-z]+-[0-9]):[0-9]{12}:[A-Za-z0-9][A-Za-z0-9:_/+=,@.-]{0,1023}$"#) | fn.#Fn
			ResourceMappings:     *[...{
				LogicalStackName?:  *string | fn.#Fn
				MappingType:        *(=~#"CfnStack|Resource"#) | fn.#Fn
				PhysicalResourceId: *{
					AwsAccountId?: *(=~#"^[0-9]{12}$"#) | fn.#Fn
					AwsRegion?:    *(=~#"^[a-z]{2}-((iso[a-z]{0,1}-)|(gov-)){0,1}[a-z]+-[0-9]$"#) | fn.#Fn
					Identifier:    *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
					Type:          *(=~#"Arn|Native"#) | fn.#Fn
				} | fn.#If
				ResourceName?: *(=~#"^[A-Za-z0-9][A-Za-z0-9_\-]{1,59}$"#) | fn.#Fn
			}] | fn.#If
			Tags?: *{
				[string]: *string | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ResiliencyPolicy: {
		Type: "AWS::ResilienceHub::ResiliencyPolicy"
		Properties: {
			DataLocationConstraint?: *("AnyLocation" | "SameContinent" | "SameCountry") | fn.#Fn
			Policy:                  *{
				[string]: {
					RpoInSecs: *int | fn.#Fn
					RtoInSecs: *int | fn.#Fn
				}
			} | fn.#If
			PolicyDescription?: *string | fn.#Fn
			PolicyName:         *(=~#"^[A-Za-z0-9][A-Za-z0-9_\-]{1,59}$"#) | fn.#Fn
			Tags?:              *{
				[string]: *string | fn.#Fn
			} | fn.#If
			Tier: *("MissionCritical" | "Critical" | "Important" | "CoreServices" | "NonCritical") | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
