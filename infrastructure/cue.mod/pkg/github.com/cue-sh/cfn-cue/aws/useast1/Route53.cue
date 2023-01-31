package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#Route53: {
	#DNSSEC: {
		Type: "AWS::Route53::DNSSEC"
		Properties: HostedZoneId: *(=~#"^[A-Z0-9]{1,32}$"#) | fn.#Fn
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#HealthCheck: {
		Type: "AWS::Route53::HealthCheck"
		Properties: {
			HealthCheckConfig: *{
				AlarmIdentifier?: *{
					Name:   *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
					Region: *string | fn.#Fn
				} | fn.#If
				ChildHealthChecks?:            [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				EnableSNI?:                    *bool | fn.#Fn
				FailureThreshold?:             *(>=1 & <=10) | fn.#Fn
				FullyQualifiedDomainName?:     *string | fn.#Fn
				HealthThreshold?:              *int | fn.#Fn
				IPAddress?:                    *(=~#"^((([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])\.){3}([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5]))$|^(([0-9a-fA-F]{1,4}:){7,7}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,7}:|([0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,5}(:[0-9a-fA-F]{1,4}){1,2}|([0-9a-fA-F]{1,4}:){1,4}(:[0-9a-fA-F]{1,4}){1,3}|([0-9a-fA-F]{1,4}:){1,3}(:[0-9a-fA-F]{1,4}){1,4}|([0-9a-fA-F]{1,4}:){1,2}(:[0-9a-fA-F]{1,4}){1,5}|[0-9a-fA-F]{1,4}:((:[0-9a-fA-F]{1,4}){1,6})|:((:[0-9a-fA-F]{1,4}){1,7}|:)|fe80:(:[0-9a-fA-F]{0,4}){0,4}%[0-9a-zA-Z]{1,}|::(ffff(:0{1,4}){0,1}:){0,1}((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])|([0-9a-fA-F]{1,4}:){1,4}:((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9]))$"#) | fn.#Fn
				InsufficientDataHealthStatus?: *("Healthy" | "LastKnownStatus" | "Unhealthy") | fn.#Fn
				Inverted?:                     *bool | fn.#Fn
				MeasureLatency?:               *bool | fn.#Fn
				Port?:                         *(>=1 & <=65535) | fn.#Fn
				Regions?:                      [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				RequestInterval?:              *(>=10 & <=30) | fn.#Fn
				ResourcePath?:                 *string | fn.#Fn
				SearchString?:                 *string | fn.#Fn
				Type:                          *("CALCULATED" | "CLOUDWATCH_METRIC" | "HTTP" | "HTTP_STR_MATCH" | "HTTPS" | "HTTPS_STR_MATCH" | "TCP" | "RECOVERY_CONTROL") | fn.#Fn
			} | fn.#If
			HealthCheckTags?: *[...{
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
	#HostedZone: {
		Type: "AWS::Route53::HostedZone"
		Properties: {
			HostedZoneConfig?: *{
				Comment?: *string | fn.#Fn
			} | fn.#If
			HostedZoneTags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			Name?:               *string | fn.#Fn
			QueryLoggingConfig?: *{
				CloudWatchLogsLogGroupArn: *string | fn.#Fn
			} | fn.#If
			VPCs?: *[...{
				VPCId:     *string | fn.#Fn
				VPCRegion: *string | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#KeySigningKey: {
		Type: "AWS::Route53::KeySigningKey"
		Properties: {
			HostedZoneId:            *(=~#"^[A-Z0-9]{1,32}$"#) | fn.#Fn
			KeyManagementServiceArn: *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
			Name:                    *(=~#"^[a-zA-Z0-9_]{3,128}$"#) | fn.#Fn
			Status:                  *("ACTIVE" | "INACTIVE") | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#RecordSet: {
		Type: "AWS::Route53::RecordSet"
		Properties: {
			AliasTarget?: *{
				DNSName:               *string | fn.#Fn
				EvaluateTargetHealth?: *bool | fn.#Fn
				HostedZoneId:          *string | fn.#Fn
			} | fn.#If
			Comment?:     *string | fn.#Fn
			Failover?:    *("PRIMARY" | "SECONDARY") | fn.#Fn
			GeoLocation?: *{
				ContinentCode?:   *("AF" | "AN" | "AS" | "EU" | "NA" | "OC" | "SA") | fn.#Fn
				CountryCode?:     *string | fn.#Fn
				SubdivisionCode?: *string | fn.#Fn
			} | fn.#If
			HealthCheckId?:    *string | fn.#Fn
			HostedZoneId?:     *string | fn.#Fn
			HostedZoneName?:   *string | fn.#Fn
			MultiValueAnswer?: *bool | fn.#Fn
			Name:              *string | fn.#Fn
			Region?:           *string | fn.#Fn
			ResourceRecords?:  [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			SetIdentifier?:    *string | fn.#Fn
			TTL?:              *int | fn.#Fn
			Type:              *("A" | "AAAA" | "CAA" | "CNAME" | "DS" | "MX" | "NAPTR" | "NS" | "PTR" | "SOA" | "SPF" | "SRV" | "TXT") | fn.#Fn
			Weight?:           *int | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#RecordSetGroup: {
		Type: "AWS::Route53::RecordSetGroup"
		Properties: {
			Comment?:        *string | fn.#Fn
			HostedZoneId?:   *string | fn.#Fn
			HostedZoneName?: *string | fn.#Fn
			RecordSets?:     *[...{
				AliasTarget?: *{
					DNSName:               *string | fn.#Fn
					EvaluateTargetHealth?: *bool | fn.#Fn
					HostedZoneId:          *string | fn.#Fn
				} | fn.#If
				Comment?:     *string | fn.#Fn
				Failover?:    *("PRIMARY" | "SECONDARY") | fn.#Fn
				GeoLocation?: *{
					ContinentCode?:   *("AF" | "AN" | "AS" | "EU" | "NA" | "OC" | "SA") | fn.#Fn
					CountryCode?:     *string | fn.#Fn
					SubdivisionCode?: *string | fn.#Fn
				} | fn.#If
				HealthCheckId?:    *string | fn.#Fn
				HostedZoneId?:     *string | fn.#Fn
				HostedZoneName?:   *string | fn.#Fn
				MultiValueAnswer?: *bool | fn.#Fn
				Name:              *string | fn.#Fn
				Region?:           *string | fn.#Fn
				ResourceRecords?:  [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				SetIdentifier?:    *string | fn.#Fn
				TTL?:              *string | fn.#Fn
				Type:              *("A" | "AAAA" | "CAA" | "CNAME" | "DS" | "MX" | "NAPTR" | "NS" | "PTR" | "SOA" | "SPF" | "SRV" | "TXT") | fn.#Fn
				Weight?:           *int | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
