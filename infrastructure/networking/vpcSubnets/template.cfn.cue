package cfn

import (
	"github.com/cue-sh/cfn-cue/aws/useast1/ec2"
	"strings"
)

Stacks: {
	"dev-\(_baseNames.vpcSubnets)-use1": {
		Profile: "default"
		Template: Description: "VPC and subnets for use by dev and ephemeral environments for SBOM Harbor."
	}

	"prod-\(_baseNames.vpcSubnets)-use1": {
		Profile: "cms-prod"
		Template: Description: "VPC and subnets for use by SBOM Harbor"
	}

	"sand-\(_baseNames.vpcSubnets)-use1": {}

	[StackName= =~_baseNames.vpcSubnets]: {
		let stack = Stacks[StackName]

		let cidrByRegion = {
			"us-west-1": 10
			"us-west-2": 20
			"us-east-2": 30
			"us-east-1": 40
		}

		let cidrByEnvironment = {
			"dev":  0
			"prod": 1
		}

		let cidrPrefix = "10.\((cidrByRegion[stack.Region]+cidrByEnvironment[stack.Environment]) & uint8)" // using uint8 to enforce an upper range of 255

		Template: {
			Resources: Vpc: ec2.#VPC & {
				Properties: {
					CidrBlock:          "\(cidrPrefix).0.0/16"
					EnableDnsHostnames: true
					EnableDnsSupport:   true
					Tags: [
						{
							Key:   "Name"
							Value: "\(stack.Environment)-harbor-vpc"
						},
					]
				}
			}

			Outputs: VpcId: {
				Value: Ref:   "Vpc"
				Export: Name: "\(StackName)-VpcId"
			}

			Outputs: VpcCidr: {
				Value: "Fn::GetAtt": "Vpc.CidrBlock"
				Export: Name:        "\(StackName)-VpcCidr"
			}

			// ranges derived from this subnet calculator
			// https://www.davidc.net/sites/default/subnets/subnets.html?network=10.10.0.0&mask=16&division=57.ff9c8939c399020
			let subnets = [
				// [ purpose, cidr, AZ, scope]

				// target all public subnets with 0.0/22
				["Public", "0.0/24", "a", "public"],
				["Public", "1.0/24", "b", "public"],
				// ["Public", "2.0/24", "c", "public"],
				// ["Public", "3.0/24", "d", "public"],

				// ranges available for future use
				// 10.10.4.0/22
				// 10.10.8.0/21
				// 10.10.16.0/20
				// 10.10.32.0/19
				// 10.10.64.0/18

				// target all application subnets with 128.0/17
				["Applications", "128.0/19", "a", "private"],
				["Applications", "160.0/19", "b", "private"],
				// ["Applications", "192.0/19", "c", "private"],
				// ["Applications", "224.0/19", "d", "private"],
			]

			for i, subnet in subnets {
				let subnetCidr = cidrPrefix + "." + subnet[1]
				let purpose = subnet[0]
				let az = subnet[2]
				let azUpperCase = strings.ToUpper(az)
				let scope = subnet[3]
				let logicalId = "Subnet\(purpose)\(azUpperCase)"
				Resources: "\(logicalId)": ec2.#Subnet & {
					Properties: {
						AvailabilityZone:    "\(stack.Region)\(az)"
						CidrBlock:           subnetCidr
						MapPublicIpOnLaunch: scope == "public"
						Tags: [
							{
								Key:   "Name"
								Value: "\(StackName)-\(purpose)-\(az)"
							},
							{
								Key:   "Scope"
								Value: scope
							},
						]
						VpcId: Ref: "Vpc"
					}
				}

				Outputs: "\(logicalId)Id": {
					Value: Ref:   logicalId
					Export: Name: "\(StackName)-\(logicalId)Id"
				}

				Outputs: "\(logicalId)Cidr": {
					Value: subnetCidr
					Export: Name: "\(StackName)-\(logicalId)Cidr"
				}
			} // for subnets
		} // Template
	} // Stack
}
