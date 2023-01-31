package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#Route53RecoveryReadiness: {
	#Cell: {
		Type: "AWS::Route53RecoveryReadiness::Cell"
		Properties: {
			CellName: *(=~#"[a-zA-Z0-9_]+"#) | fn.#Fn
			Cells?:   [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			Tags?:    *[...{
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
	#ReadinessCheck: {
		Type: "AWS::Route53RecoveryReadiness::ReadinessCheck"
		Properties: {
			ReadinessCheckName: *(strings.MinRunes(1) & strings.MaxRunes(64) & (=~#"[a-zA-Z0-9_]+"#)) | fn.#Fn
			ResourceSetName?:   *(strings.MinRunes(1) & strings.MaxRunes(64) & (=~#"[a-zA-Z0-9_]+"#)) | fn.#Fn
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
	#RecoveryGroup: {
		Type: "AWS::Route53RecoveryReadiness::RecoveryGroup"
		Properties: {
			Cells?:            [...(*(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn)
			RecoveryGroupName: *(strings.MinRunes(1) & strings.MaxRunes(64) & (=~#"[a-zA-Z0-9_]+"#)) | fn.#Fn
			Tags?:             *[...{
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
	#ResourceSet: {
		Type: "AWS::Route53RecoveryReadiness::ResourceSet"
		Properties: {
			ResourceSetName: *string | fn.#Fn
			ResourceSetType: *string | fn.#Fn
			Resources:       *[...{
				ComponentId?:       *string | fn.#Fn
				DnsTargetResource?: *{
					DomainName?:     *string | fn.#Fn
					HostedZoneArn?:  *string | fn.#Fn
					RecordSetId?:    *string | fn.#Fn
					RecordType?:     *string | fn.#Fn
					TargetResource?: *{
						NLBResource?: *{
							Arn?: *string | fn.#Fn
						} | fn.#If
						R53Resource?: *{
							DomainName?:  *string | fn.#Fn
							RecordSetId?: *string | fn.#Fn
						} | fn.#If
					} | fn.#If
				} | fn.#If
				ReadinessScopes?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				ResourceArn?:     *string | fn.#Fn
			}] | fn.#If
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
