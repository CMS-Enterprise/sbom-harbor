package cfn

import (
	"strings"
	"github.com/cue-sh/cfn-cue/aws/useast1/ec2"
)

Stacks: {
	"dev-\(_baseNames.routes)-use1": {}
	"prod-\(_baseNames.routes)-use1": {}

	[StackName= =~_baseNames.routes]: {
		let stack = Stacks[StackName]
		let vpcSubnetStackName = "\(stack.Environment)-\(_baseNames.vpcSubnets)-\(stack.RegionCode)"
		let gatewayStackName = "\(stack.Environment)-\(_baseNames.gateways)-\(stack.RegionCode)"
		let AZs = ["a", "b"]

		DependsOn: [gatewayStackName]

		Template: {
			Description: "Routing for \(vpcSubnetStackName)"

			// applied to public subnets, quad zero routes to internet gateway
			Resources: PublicRouteTable: ec2.#RouteTable & {
				Properties: {
					VpcId: "Fn::ImportValue": "\(vpcSubnetStackName)-VpcId"
					Tags: [
						{
							Key:   "Name"
							Value: "\(StackName)-Public"
						},
					]
				}
			}

			Outputs: PublicRouteTableId: {
				Value: Ref:   "PublicRouteTable"
				Export: Name: "\(StackName)-PublicRouteTableId"
			}

			Resources: QuadZeroToIgwRoute: ec2.#Route & {
				Properties: {
					DestinationCidrBlock: "0.0.0.0/0"
					GatewayId: "Fn::ImportValue": "\(gatewayStackName)-InternetGatewayId"
					RouteTableId: Ref:            "PublicRouteTable"
				}
			}

			// route quad zero through NAT gateway
			for az in AZs {
				let azUpperCase = strings.ToUpper(az)
				let logicalId = "ApplicationsRouteTable\(azUpperCase)"
				Resources: "\(logicalId)": ec2.#RouteTable & {
					Properties: {
						VpcId: "Fn::ImportValue": "\(vpcSubnetStackName)-VpcId"
						Tags: [
							{
								Key:   "Name"
								Value: "\(vpcSubnetStackName)-\(logicalId)"
							},
						]
					}
				}

				Outputs: "\(logicalId)Id": {
					Value: Ref:   "\(logicalId)"
					Export: Name: "\(StackName)-\(logicalId)Id"
				}

				Resources: "QuadZeroToNatRoute\(azUpperCase)": ec2.#Route & {
					Properties: {
						DestinationCidrBlock: "0.0.0.0/0"
						RouteTableId: Ref:               logicalId
						NatGatewayId: "Fn::ImportValue": "\(gatewayStackName)-NatGateway\(azUpperCase)Id"
					}
				}

				Resources: "\(logicalId)Association": ec2.#SubnetRouteTableAssociation & {
					Properties: {
						RouteTableId: Ref:           logicalId
						SubnetId: "Fn::ImportValue": "\(vpcSubnetStackName)-SubnetApplications\(azUpperCase)Id"
					}
				}

				Resources: "PublicRouteTable\(azUpperCase)Association": ec2.#SubnetRouteTableAssociation & {
					Properties: {
						RouteTableId: Ref:           "PublicRouteTable"
						SubnetId: "Fn::ImportValue": "\(vpcSubnetStackName)-SubnetPublic\(azUpperCase)Id"
					}
				}
			}
		} // Template
	}
}
