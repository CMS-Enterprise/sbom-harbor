package cfn

import (
	"github.com/cue-sh/cfn-cue/aws/useast1/ec2"
)

Stacks: {

	"dev-\(_baseNames.nacls)-use1": {}

	[StackName= =~_baseNames.nacls]: {
		let stack = Stacks[StackName]
		let vpcSubnetStackName = "\(stack.Environment)-\(_baseNames.vpcSubnets)-\(stack.RegionCode)"

		Template: {
			Description: "Network Access Control Lists (NACLs) for \(vpcSubnetStackName)"

			let nacls = [
				"RestrictiveNacl",
			]

			let naclAssociations = {RestrictiveNacl: [
				"ApplicationsA",
				"ApplicationsB",
				"PublicA",
				"PublicB",
			]}

			for nacl in nacls {
				Resources: "\(nacl)": ec2.#NetworkAcl & {
					Properties: {
						VpcId: "Fn::ImportValue": "\(vpcSubnetStackName)-VpcId"
						Tags: [
							{
								Key:   "Name"
								Value: "\(StackName)-\(nacl)"
							},
						]
					}
				}

				Outputs: "\(nacl)Id": {
					Value: Ref:   nacl
					Export: Name: "\(StackName)-\(nacl)Id"
				}

				for subnet in naclAssociations[nacl] {
					Resources: "\(subnet)\(nacl)Association": ec2.#SubnetNetworkAclAssociation & {
						Properties: {
							NetworkAclId: Ref:           nacl
							SubnetId: "Fn::ImportValue": "\(vpcSubnetStackName)-Subnet\(subnet)Id"
						}
					}
				}

			} // for nacl

			Resources: {

				[=~"NaclEntry$"]: ec2.#NetworkAclEntry & {
					Properties: {
						NetworkAclId: Ref: "RestrictiveNacl"
						RuleAction: "allow"
						CidrBlock:  string | {} | *"0.0.0.0/0"
					}
				}
				[=~"^Ingress"]: Properties: Egress: false
				[=~"^Egress"]: Properties: Egress:  true
				[=~"Tcp"]: Properties: Protocol:    6
				[=~"Udp"]: Properties: Protocol:    17

				//
				// INGRESS RULES
				//

				// IngressICMPRestrictiveNaclEntry: Properties: {
				//  PortRange: {
				//   From: 0
				//   To:   65535
				//  }
				//  Icmp: {
				//   Code: -1
				//   Type: -1
				//  }
				//  Protocol:   1
				//  RuleNumber: 10
				//  CidrBlock:  "10.0.0.0/8"
				// }

				IngressTcp80RestrictiveNaclEntry: Properties: {
					PortRange: {
						From: 80
						To:   80
					}
					RuleNumber: 100
				}

				IngressTcp443RestrictiveNaclEntry: Properties: {
					PortRange: {
						From: 443
						To:   443
					}
					RuleNumber: 101
				}

				// ephemeral ports
				IngressTcpEphemeralRestrictiveNaclEntry: Properties: {
					PortRange: {
						From: 1024
						To:   65535
					}
					RuleNumber: 102
				}

				// DNS on tcp 53
				IngressTcpDnsRestrictiveNaclEntry: Properties: {
					PortRange: {
						From: 53
						To:   53
					}
					RuleNumber: 103
				}

				// DNS on udp 53
				IngressUdpDnsRestrictiveNaclEntry: Properties: {
					PortRange: {
						From: 53
						To:   53
					}
					RuleNumber: 104
				}

				// NTP traffic
				IngressUdpNtpRestrictiveNaclEntry: Properties: {
					PortRange: {
						From: 123
						To:   123
					}
					RuleNumber: 105
				}

				// ALL TRAFFIC ALL PORTS FROM VPC
				IngressAllTrafficRestrictiveNaclEntry: Properties: {
					CidrBlock: "Fn::ImportValue": "\(vpcSubnetStackName)-VpcCidr"
					PortRange: {
						From: 0
						To:   65535
					}
					Protocol:   -1 // ALL TRAFFIC!
					RuleNumber: 106
				}

				//
				// EGRESS RULES
				//

				// EgressICMPRestrictiveNaclEntry: Properties: {
				//  PortRange: {
				//   From: 0
				//   To:   65535
				//  }
				//  Icmp: {
				//   Code: -1
				//   Type: -1
				//  }
				//  Protocol:   1
				//  RuleNumber: 10
				//  CidrBlock:  "10.0.0.0/8"
				// }

				EgressTcp80RestrictiveNaclEntry: Properties: {
					PortRange: {
						From: 80
						To:   80
					}
					RuleNumber: 100
				}

				EgressTcp443RestrictiveNaclEntry: Properties: {
					PortRange: {
						From: 443
						To:   443
					}
					RuleNumber: 101
				}

				// ephemeral ports
				EgressTcpEphemeralRestrictiveNaclEntry: Properties: {
					PortRange: {
						From: 1024
						To:   65535
					}
					RuleNumber: 102
				}

				// DNS on tcp 53
				EgressTcpDnsRestrictiveNaclEntry: Properties: {
					PortRange: {
						From: 53
						To:   53
					}
					RuleNumber: 103
				}

				// DNS on udp 53
				EgressUdpDnsRestrictiveNaclEntry: Properties: {
					PortRange: {
						From: 53
						To:   53
					}
					RuleNumber: 104
				}

				// NTP traffic
				EgressUdpNtpRestrictiveNaclEntry: Properties: {
					PortRange: {
						From: 123
						To:   123
					}
					RuleNumber: 105
				}

				// ALL TRAFFIC FROM VPC
				EgressAllTrafficRestrictiveNaclEntry: Properties: {
					CidrBlock: "Fn::ImportValue": "\(vpcSubnetStackName)-VpcCidr"
					PortRange: {
						From: 0
						To:   65535
					}
					Protocol:   -1 // ALL TRAFFIC
					RuleNumber: 106
				}

			} // Resources
		}
	}
}
