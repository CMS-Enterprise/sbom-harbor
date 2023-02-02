package cfn

import (
	"github.com/cue-sh/cfn-cue/aws/useast1/ec2"
)

Stacks: {
	"dev-\(_baseNames.endpoints)-use1": {}
	"prod-\(_baseNames.endpoints)-use1": {}

	[StackName= =~_baseNames.endpoints]: {
		let stack = Stacks[StackName]
		let vpcSubnetStackName = "\(stack.Environment)-\(_baseNames.vpcSubnets)-\(stack.RegionCode)"
		let routeStackName = "\(stack.Environment)-\(_baseNames.routes)-\(stack.RegionCode)"

		DependsOn: [vpcSubnetStackName, routeStackName]
		Template: {
			Description: "Private endpoints to reach AWS services"

			Resources: {
				endpointS3: ec2.#VPCEndpoint & {
					Properties: {
						PolicyDocument: {
							"Version": "2012-10-17"
							"Statement": [{
								"Effect":    "Allow"
								"Principal": "*"
								"Action": ["s3:*"]
								"Resource": ["*"]
							}]
						}
						RouteTableIds: [
							{"Fn::ImportValue": "\(routeStackName)-ApplicationsRouteTableAId"},
							{"Fn::ImportValue": "\(routeStackName)-ApplicationsRouteTableBId"},
						]
						ServiceName:     "com.amazonaws.\(stack.Region).s3"
						VpcEndpointType: "Gateway"
						VpcId: "Fn::ImportValue": "\(vpcSubnetStackName)-VpcId"
					}
				}

				endpointDDB: ec2.#VPCEndpoint & {
					Properties: {
						PolicyDocument: {{
							"Version": "2012-10-17"
							"Statement": [{
								"Effect":    "Allow"
								"Principal": "*"
								"Action": ["dynamodb:*"]
								"Resource": ["*"]
							}]
						}}
						RouteTableIds: [
							{"Fn::ImportValue": "\(routeStackName)-ApplicationsRouteTableAId"},
							{"Fn::ImportValue": "\(routeStackName)-ApplicationsRouteTableBId"},
						]
						ServiceName:     "com.amazonaws.\(stack.Region).dynamodb"
						VpcEndpointType: "Gateway"
						VpcId: "Fn::ImportValue": "\(vpcSubnetStackName)-VpcId"
					}
				}

				endpointEcrApi: ec2.#VPCEndpoint & {
					Properties: {
						PolicyDocument: {
							"Version": "2012-10-17"
							"Statement": [{
								"Effect":    "Allow"
								"Principal": "*"
								"Action": [ "ecr:*"]
								"Resource": [ "*"]
							}]
						}
						ServiceName:       "com.amazonaws.\(stack.Region).ecr.api"
						PrivateDnsEnabled: true
						VpcEndpointType:   "Interface"
						VpcId: "Fn::ImportValue": "\(vpcSubnetStackName)-VpcId"
						SubnetIds: [
							{"Fn::ImportValue": "\(vpcSubnetStackName)-SubnetApplicationsAId"},
							{"Fn::ImportValue": "\(vpcSubnetStackName)-SubnetApplicationsBId"},
						]
						SecurityGroupIds: [ {Ref: "EcrInterfaceEndpointSecurityGroup"}]
					}
				}

				endpointEcrDkr: ec2.#VPCEndpoint & {
					Properties: {
						PolicyDocument: {
							"Version": "2012-10-17"
							"Statement": [{
								"Effect":    "Allow"
								"Principal": "*"
								"Action": [ "ecr:*"]
								"Resource": [ "*"]
							}]
						}
						ServiceName:       "com.amazonaws.\(stack.Region).ecr.dkr"
						PrivateDnsEnabled: true
						VpcEndpointType:   "Interface"
						VpcId: "Fn::ImportValue": "\(vpcSubnetStackName)-VpcId"
						SubnetIds: [
							{"Fn::ImportValue": "\(vpcSubnetStackName)-SubnetApplicationsAId"},
							{"Fn::ImportValue": "\(vpcSubnetStackName)-SubnetApplicationsBId"},
						]
						SecurityGroupIds: [ {Ref: "EcrInterfaceEndpointSecurityGroup"}]
					}
				}

				EcrInterfaceEndpointSecurityGroup: ec2.#SecurityGroup & {
					Properties: {
						GroupDescription: "ECR VPC Endpoint for \(vpcSubnetStackName)-VpcId"
						VpcId: "Fn::ImportValue": "\(vpcSubnetStackName)-VpcId"
					}
				}

				EcrInterfaceSecurityGroupIngress: ec2.#SecurityGroupIngress & {
					Properties: {
						Description: "Allow the ECR service to recieve communication from the VPC on port 443"
						GroupId: Ref: "EcrInterfaceEndpointSecurityGroup"
						IpProtocol: "tcp"
						FromPort:   443
						ToPort:     443
						CidrIp: "Fn::ImportValue": "\(vpcSubnetStackName)-VpcCidr"
					}
				}

				// the ec2 api is primarily for k8s to make calls to attach volumes to instances
				Ec2ApiEndpoint: ec2.#VPCEndpoint & {
					Properties: {
						PolicyDocument: {
							"Version": "2012-10-17"
							"Statement": [{
								"Effect":    "Allow"
								"Principal": "*"
								"Action": [ "ec2:*"]
								"Resource": [ "*"]
							}]
						}
						ServiceName:       "com.amazonaws.\(stack.Region).ec2"
						PrivateDnsEnabled: true
						VpcEndpointType:   "Interface"
						VpcId: "Fn::ImportValue": "\(vpcSubnetStackName)-VpcId"
						SubnetIds: [
							{"Fn::ImportValue": "\(vpcSubnetStackName)-SubnetApplicationsAId"},
							{"Fn::ImportValue": "\(vpcSubnetStackName)-SubnetApplicationsBId"},
						]
						SecurityGroupIds: [ {Ref: "Ec2ApiEndpointSecurityGroup"}]
					}
				}

				Ec2ApiEndpointSecurityGroup: ec2.#SecurityGroup & {
					Properties: {
						GroupDescription: "EC2 API VPC Endpoint for \(vpcSubnetStackName)-VpcId"
						VpcId: "Fn::ImportValue": "\(vpcSubnetStackName)-VpcId"
					}
				}

				Ec2ApiEndpointSecurityGroupIngress: ec2.#SecurityGroupIngress & {
					Properties: {
						Description: "Allow the EC2 API to recieve communication from the VPC on port 443"
						GroupId: Ref: "Ec2ApiEndpointSecurityGroup"
						IpProtocol: "tcp"
						FromPort:   443
						ToPort:     443
						CidrIp: "Fn::ImportValue": "\(vpcSubnetStackName)-VpcCidr"
					}
				}
			}
		}
	}
}
