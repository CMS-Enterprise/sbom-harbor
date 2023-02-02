package cfn

import (
	"strings"
	"github.com/cue-sh/cfn-cue/aws/useast1/ec2"
)

Stacks: {

	"dev-\(_baseNames.gateways)-use1": {}
	"prod-\(_baseNames.gateways)-use1": {}

	[StackName= =~_baseNames.gateways]: {
		let stack = Stacks[StackName]
		let vpcSubnetStackName = "\(stack.Environment)-\(_baseNames.vpcSubnets)-\(stack.RegionCode)"

		DependsOn: [vpcSubnetStackName]
		TagsEnabled: false // because modifying tags on the EIP causes a recreation of the NAT GW
		Template: {
			Description: "Internet, NAT, VPN, and Transit gateways for \(vpcSubnetStackName)."

			Resources: InternetGateway: ec2.#InternetGateway & {
				Properties: Tags: [
					{
						Key:   "Name"
						Value: StackName
					},
				]
			}

			Outputs: InternetGatewayId: {
				Description: "Internet Gateway ID"
				Value: Ref:   "InternetGateway"
				Export: Name: "\(StackName)-InternetGatewayId"
			}

			Resources: InternetGatewayAttachment: ec2.#VPCGatewayAttachment & {
				Properties: {
					InternetGatewayId: Ref:   "InternetGateway"
					VpcId: "Fn::ImportValue": "\(vpcSubnetStackName)-VpcId"
				}
			}
			let AZs = ["a", "b"]
			for i, azLowerCase in AZs {
				let azUpperCase = strings.ToUpper(azLowerCase)
				Resources: "ElasticIp\(azUpperCase)": ec2.#EIP

				Resources: "NatGateway\(azUpperCase)": ec2.#NatGateway & {
					DependsOn: "InternetGatewayAttachment"
					Properties: {
						AllocationId: "Fn::GetAtt":  "ElasticIp\(azUpperCase).AllocationId"
						SubnetId: "Fn::ImportValue": "\(vpcSubnetStackName)-SubnetPublic\(azUpperCase)Id"
						Tags: [
							{
								Key:   "Name"
								Value: StackName
							},
						]
					}
				}

				Outputs: "NatGateway\(azUpperCase)Id": {
					Description: "NAT \(azUpperCase) ID"
					Value: Ref:   "NatGateway\(azUpperCase)"
					Export: Name: "\(StackName)-NatGateway\(azUpperCase)Id"
				}

				Outputs: "NatGateway\(azUpperCase)IP": {
					Value: Ref:   "ElasticIp\(azUpperCase)"
					Export: Name: "\(StackName)-NatGateway\(azUpperCase)IP"
				}
			}
		}
	}
}
