package cfn

import (
	"github.com/cue-sh/cfn-cue/aws/useast1/ecs"
	"github.com/cue-sh/cfn-cue/aws/useast1/ec2"
	"github.com/cue-sh/cfn-cue/aws/useast1/ecr"
	"github.com/cue-sh/cfn-cue/aws/useast1/iam"
	"github.com/cue-sh/cfn-cue/aws/useast1/logs"
)

Stacks: {
	"dev-\(_baseNames.ghaRunners)-use1": {}

	[StackName= =~_baseNames.ghaRunners]: {
		let stack = Stacks[StackName]
		let vpcSubnetStackName = "\(stack.Environment)-\(_baseNames.vpcSubnets)-\(stack.RegionCode)"

		Template: {
			Resources: {
				EcrRepo: ecr.#Repository & {
					Properties: RepositoryName: _baseNames.ghaRunners
				}

				EcsCluster: ecs.#Cluster & {
					Properties: ClusterName: StackName
				}

				ExecutionRole: iam.#Role & {
					Properties: {
						RoleName: StackName
						ManagedPolicyArns: [
							"arn:aws:iam::aws:policy/service-role/AmazonECSTaskExecutionRolePolicy",
						]
						Path: "/delegatedadmin/adodeveloper/service-role/"
						AssumeRolePolicyDocument: {
							Version: "2012-10-17"
							Statement: [
								{
									Effect: "Allow"
									Principal: {
										Service: "ecs-tasks.amazonaws.com"
									}
									Action: "sts:AssumeRole"
								},
							]
						}
					}
				}

				TaskSecurityGroup: ec2.#SecurityGroup & {
					Properties: {
						GroupName:        StackName
						GroupDescription: StackName
						VpcId: "Fn::ImportValue": "\(vpcSubnetStackName)-VpcId"
					}
				}

				TaskSecurityGroupEgress: ec2.#SecurityGroupEgress & {
					Properties: {
						GroupId: Ref: "TaskSecurityGroup"
						CidrIp:     "0.0.0.0/0"
						FromPort:   443
						ToPort:     443
						IpProtocol: "tcp"
					}
				}

				TaskLogGroup: logs.#LogGroup & {
					Properties: {
						LogGroupName:    StackName
						RetentionInDays: 30
					}
				}

				TaskDefinition: ecs.#TaskDefinition & {
					Properties: {
						Cpu: "1024"
						ExecutionRoleArn: "Fn::GetAtt": "ExecutionRole.Arn"
						Family:      _baseNames.ghaRunners
						Memory:      "2048"
						NetworkMode: "awsvpc"
						RequiresCompatibilities: ["FARGATE"]
						// TaskRoleArn: "Fn::GetAtt": "TaskRole.Arn"
						ContainerDefinitions: [
							{
								Image: "Fn::Join": [":", [{"Fn::GetAtt": "EcrRepo.RepositoryUri"}, "latest"]]
								LogConfiguration: {
									LogDriver: "awslogs"
									Options: {
										"awslogs-group": StackName
										"awslogs-region": Ref: "AWS::Region"
										"awslogs-stream-prefix": StackName
									}}
								Name: _baseNames.ghaRunners
							}]
					}
				}

				ECSService: ecs.#Service & {
					Properties: {
						Cluster: Ref: "EcsCluster"
						DeploymentConfiguration: {
							MaximumPercent:        200
							MinimumHealthyPercent: 50
						}
						DesiredCount: 0
						LaunchType:   "FARGATE"
						NetworkConfiguration: {
							AwsvpcConfiguration: {
								Subnets: [
									{"Fn::ImportValue": "\(vpcSubnetStackName)-SubnetApplicationsAId"},
									{"Fn::ImportValue": "\(vpcSubnetStackName)-SubnetApplicationsBId"},
								]
								SecurityGroups: [{Ref: "TaskSecurityGroup"}]
							}
						}
						ServiceName: StackName
						TaskDefinition: Ref: "TaskDefinition"
					}
				}
			}

			Outputs: {

				EcrRepoUri: {
					Value: "Fn::GetAtt": "EcrRepo.RepositoryUri"
				}
			}
		}
	}
}
