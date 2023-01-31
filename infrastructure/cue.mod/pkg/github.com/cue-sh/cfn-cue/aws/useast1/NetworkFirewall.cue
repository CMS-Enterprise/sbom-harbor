package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#NetworkFirewall: {
	#Firewall: {
		Type: "AWS::NetworkFirewall::Firewall"
		Properties: {
			DeleteProtection?:               *bool | fn.#Fn
			Description?:                    *(=~#"^.*$"#) | fn.#Fn
			FirewallName:                    *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[a-zA-Z0-9-]+$"#)) | fn.#Fn
			FirewallPolicyArn:               *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"^arn:aws.*$"#)) | fn.#Fn
			FirewallPolicyChangeProtection?: *bool | fn.#Fn
			SubnetChangeProtection?:         *bool | fn.#Fn
			SubnetMappings:                  *[...{
				SubnetId: *string | fn.#Fn
			}] | fn.#If
			Tags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			VpcId: *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^vpc-[0-9a-f]+$"#)) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#FirewallPolicy: {
		Type: "AWS::NetworkFirewall::FirewallPolicy"
		Properties: {
			Description?:   *(strings.MinRunes(1) & strings.MaxRunes(512) & (=~#"^.*$"#)) | fn.#Fn
			FirewallPolicy: *{
				StatefulDefaultActions?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				StatefulEngineOptions?:  *{
					RuleOrder?: *("DEFAULT_ACTION_ORDER" | "STRICT_ORDER") | fn.#Fn
				} | fn.#If
				StatefulRuleGroupReferences?: *[...{
					Priority?:   *(>=1 & <=65535) | fn.#Fn
					ResourceArn: *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"^(arn:aws.*)$"#)) | fn.#Fn
				}] | fn.#If
				StatelessCustomActions?: *[...{
					ActionDefinition: *{
						PublishMetricAction?: *{
							Dimensions: *[...{
								Value: *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[a-zA-Z0-9-_ ]+$"#)) | fn.#Fn
							}] | fn.#If
						} | fn.#If
					} | fn.#If
					ActionName: *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[a-zA-Z0-9]+$"#)) | fn.#Fn
				}] | fn.#If
				StatelessDefaultActions:         [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				StatelessFragmentDefaultActions: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				StatelessRuleGroupReferences?:   *[...{
					Priority:    *(>=1 & <=65535) | fn.#Fn
					ResourceArn: *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"^(arn:aws.*)$"#)) | fn.#Fn
				}] | fn.#If
			} | fn.#If
			FirewallPolicyName: *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[a-zA-Z0-9-]+$"#)) | fn.#Fn
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
	#LoggingConfiguration: {
		Type: "AWS::NetworkFirewall::LoggingConfiguration"
		Properties: {
			FirewallArn:          *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"^arn:aws.*$"#)) | fn.#Fn
			FirewallName?:        *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[a-zA-Z0-9-]+$"#)) | fn.#Fn
			LoggingConfiguration: *{
				LogDestinationConfigs: *[...{
					LogDestination: *{
						[string]: *string | fn.#Fn
					} | fn.#If
					LogDestinationType: *("S3" | "CloudWatchLogs" | "KinesisDataFirehose") | fn.#Fn
					LogType:            *("ALERT" | "FLOW") | fn.#Fn
				}] | fn.#If
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#RuleGroup: {
		Type: "AWS::NetworkFirewall::RuleGroup"
		Properties: {
			Capacity:     *int | fn.#Fn
			Description?: *(strings.MinRunes(1) & strings.MaxRunes(512) & (=~#"^.*$"#)) | fn.#Fn
			RuleGroup?:   *{
				RuleVariables?: *{
					IPSets?: *{
						[string]: Definition?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					} | fn.#If
					PortSets?: *{
						[string]: Definition?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					} | fn.#If
				} | fn.#If
				RulesSource: *{
					RulesSourceList?: *{
						GeneratedRulesType: *("ALLOWLIST" | "DENYLIST") | fn.#Fn
						TargetTypes:        [...(*("TLS_SNI" | "HTTP_HOST") | fn.#Fn)] | (*("TLS_SNI" | "HTTP_HOST") | fn.#Fn)
						Targets:            [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					} | fn.#If
					RulesString?:   *string | fn.#Fn
					StatefulRules?: *[...{
						Action: *("PASS" | "DROP" | "ALERT") | fn.#Fn
						Header: *{
							Destination:     *(strings.MinRunes(1) & strings.MaxRunes(1024) & (=~#"^.*$"#)) | fn.#Fn
							DestinationPort: *(strings.MinRunes(1) & strings.MaxRunes(1024) & (=~#"^.*$"#)) | fn.#Fn
							Direction:       *("FORWARD" | "ANY") | fn.#Fn
							Protocol:        *("IP" | "TCP" | "UDP" | "ICMP" | "HTTP" | "FTP" | "TLS" | "SMB" | "DNS" | "DCERPC" | "SSH" | "SMTP" | "IMAP" | "MSN" | "KRB5" | "IKEV2" | "TFTP" | "NTP" | "DHCP") | fn.#Fn
							Source:          *(strings.MinRunes(1) & strings.MaxRunes(1024) & (=~#"^.*$"#)) | fn.#Fn
							SourcePort:      *(strings.MinRunes(1) & strings.MaxRunes(1024) & (=~#"^.*$"#)) | fn.#Fn
						} | fn.#If
						RuleOptions: *[...{
							Keyword:   *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^.*$"#)) | fn.#Fn
							Settings?: [...(*(strings.MinRunes(1) & strings.MaxRunes(8192) & (=~#"^.*$"#)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(8192) & (=~#"^.*$"#)) | fn.#Fn)
						}] | fn.#If
					}] | fn.#If
					StatelessRulesAndCustomActions?: *{
						CustomActions?: *[...{
							ActionDefinition: *{
								PublishMetricAction?: *{
									Dimensions: *[...{
										Value: *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[a-zA-Z0-9-_ ]+$"#)) | fn.#Fn
									}] | fn.#If
								} | fn.#If
							} | fn.#If
							ActionName: *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[a-zA-Z0-9]+$"#)) | fn.#Fn
						}] | fn.#If
						StatelessRules: *[...{
							Priority:       *(>=1 & <=65535) | fn.#Fn
							RuleDefinition: *{
								Actions:         [...(*string | fn.#Fn)] | (*string | fn.#Fn)
								MatchAttributes: *{
									DestinationPorts?: *[...{
										FromPort: *int | fn.#Fn
										ToPort:   *int | fn.#Fn
									}] | fn.#If
									Destinations?: *[...{
										AddressDefinition: *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"^([a-fA-F\d:\.]+/\d{1,3})$"#)) | fn.#Fn
									}] | fn.#If
									Protocols?:   [...(*int | fn.#Fn)] | (*int | fn.#Fn)
									SourcePorts?: *[...{
										FromPort: *int | fn.#Fn
										ToPort:   *int | fn.#Fn
									}] | fn.#If
									Sources?: *[...{
										AddressDefinition: *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"^([a-fA-F\d:\.]+/\d{1,3})$"#)) | fn.#Fn
									}] | fn.#If
									TCPFlags?: *[...{
										Flags:  [...(*("FIN" | "SYN" | "RST" | "PSH" | "ACK" | "URG" | "ECE" | "CWR") | fn.#Fn)] | (*("FIN" | "SYN" | "RST" | "PSH" | "ACK" | "URG" | "ECE" | "CWR") | fn.#Fn)
										Masks?: [...(*("FIN" | "SYN" | "RST" | "PSH" | "ACK" | "URG" | "ECE" | "CWR") | fn.#Fn)] | (*("FIN" | "SYN" | "RST" | "PSH" | "ACK" | "URG" | "ECE" | "CWR") | fn.#Fn)
									}] | fn.#If
								} | fn.#If
							} | fn.#If
						}] | fn.#If
					} | fn.#If
				} | fn.#If
				StatefulRuleOptions?: *{
					RuleOrder?: *("DEFAULT_ACTION_ORDER" | "STRICT_ORDER") | fn.#Fn
				} | fn.#If
			} | fn.#If
			RuleGroupName: *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[a-zA-Z0-9-]+$"#)) | fn.#Fn
			Tags?:         *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			Type: *("STATELESS" | "STATEFUL") | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
