package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#Route53Resolver: {
	#FirewallDomainList: {
		Type: "AWS::Route53Resolver::FirewallDomainList"
		Properties: {
			DomainFileUrl?: *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
			Domains?:       [...(*(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn)
			Name?:          *(strings.MinRunes(1) & strings.MaxRunes(64) & (=~#"(?!^[0-9]+$)([a-zA-Z0-9\-_' ']+)"#)) | fn.#Fn
			Tags?:          *[...{
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
	#FirewallRuleGroup: {
		Type: "AWS::Route53Resolver::FirewallRuleGroup"
		Properties: {
			FirewallRules?: *[...{
				Action:                *("ALLOW" | "BLOCK" | "ALERT") | fn.#Fn
				BlockOverrideDnsType?: *("CNAME") | fn.#Fn
				BlockOverrideDomain?:  *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
				BlockOverrideTtl?:     *int | fn.#Fn
				BlockResponse?:        *("NODATA" | "NXDOMAIN" | "OVERRIDE") | fn.#Fn
				FirewallDomainListId:  *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
				Priority:              *int | fn.#Fn
			}] | fn.#If
			Name?: *(strings.MinRunes(1) & strings.MaxRunes(64) & (=~#"(?!^[0-9]+$)([a-zA-Z0-9\-_' ']+)"#)) | fn.#Fn
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
	#FirewallRuleGroupAssociation: {
		Type: "AWS::Route53Resolver::FirewallRuleGroupAssociation"
		Properties: {
			FirewallRuleGroupId: *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
			MutationProtection?: *("ENABLED" | "DISABLED") | fn.#Fn
			Name?:               *(=~#"(?!^[0-9]+$)([a-zA-Z0-9\-_' ']+)"#) | fn.#Fn
			Priority:            *int | fn.#Fn
			Tags?:               *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			VpcId: *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ResolverConfig: {
		Type: "AWS::Route53Resolver::ResolverConfig"
		Properties: {
			AutodefinedReverseFlag: *("DISABLE") | fn.#Fn
			ResourceId:             *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ResolverDNSSECConfig: {
		Type: "AWS::Route53Resolver::ResolverDNSSECConfig"
		Properties: ResourceId?: *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ResolverEndpoint: {
		Type: "AWS::Route53Resolver::ResolverEndpoint"
		Properties: {
			Direction:   *("INBOUND" | "OUTBOUND") | fn.#Fn
			IpAddresses: *[...{
				Ip?:      *string | fn.#Fn
				SubnetId: *string | fn.#Fn
			}] | fn.#If
			Name?:            *string | fn.#Fn
			SecurityGroupIds: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			Tags?:            *[...{
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
	#ResolverQueryLoggingConfig: {
		Type: "AWS::Route53Resolver::ResolverQueryLoggingConfig"
		Properties: {
			DestinationArn?: *(strings.MinRunes(1) & strings.MaxRunes(600)) | fn.#Fn
			Name?:           *(strings.MinRunes(1) & strings.MaxRunes(64) & (=~#"(?!^[0-9]+$)([a-zA-Z0-9\-_' ']+)"#)) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ResolverQueryLoggingConfigAssociation: {
		Type: "AWS::Route53Resolver::ResolverQueryLoggingConfigAssociation"
		Properties: {
			ResolverQueryLogConfigId?: *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
			ResourceId?:               *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ResolverRule: {
		Type: "AWS::Route53Resolver::ResolverRule"
		Properties: {
			DomainName:          *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
			Name?:               *string | fn.#Fn
			ResolverEndpointId?: *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
			RuleType:            *("FORWARD" | "SYSTEM" | "RECURSIVE") | fn.#Fn
			Tags?:               *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			TargetIps?: *[...{
				Ip:    *string | fn.#Fn
				Port?: *string | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ResolverRuleAssociation: {
		Type: "AWS::Route53Resolver::ResolverRuleAssociation"
		Properties: {
			Name?:          *string | fn.#Fn
			ResolverRuleId: *string | fn.#Fn
			VPCId:          *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
