package cfn

import (
	"github.com/cue-sh/cfn-cue/aws/useast1/ecs"
	"github.com/cue-sh/cfn-cue/aws/useast1/ec2"
	"github.com/cue-sh/cfn-cue/aws/useast1/ecr"
	"github.com/cue-sh/cfn-cue/aws/useast1/iam"
	"github.com/cue-sh/cfn-cue/aws/useast1/logs"
	"github.com/cue-sh/cfn-cue/aws/useast1/secretsmanager"
)

Stacks: {
	"dev-\(_baseNames.ghaRunners)-use1": {}

	[StackName= =~_baseNames.ghaRunners]: {
		let stack = Stacks[StackName]
		let vpcSubnetStackName = "\(stack.Environment)-\(_baseNames.vpcSubnets)-\(stack.RegionCode)"
		let opsCommonStackName = "\(stack.Environment)-\(_baseNames.opsCommon)-\(stack.RegionCode)"

		Template: {
			Resources: {
				EcrRepo: ecr.#Repository & {
					Properties: RepositoryName: _baseNames.ghaRunners
				}

				GithubTokenSecret: secretsmanager.#Secret & {
					Properties: {
						Name: "\(_baseNames.ghaRunners)-GithubToken"
					}
				}

				ExecutionRole: iam.#Role & {
					Properties: {
						RoleName: _baseNames.ghaRunners
						Path:     "/delegatedadmin/adodeveloper/service-role/"
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
						ManagedPolicyArns: [
							"arn:aws:iam::aws:policy/service-role/AmazonECSTaskExecutionRolePolicy",
						]
						Policies: [
							{
								PolicyName: "AllowReadSecret"
								PolicyDocument: {
									Version: "2012-10-17"
									Statement: [
										{
											Effect: "Allow"
											Action: "secretsmanager:GetSecretValue"
											Resource: Ref: "GithubTokenSecret"
										},
									]
								}
							},
						]
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
						LogGroupName:    _baseNames.ghaRunners
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
								Secrets: [
									{
										Name: "GITHUB_TOKEN"
										ValueFrom: Ref: "GithubTokenSecret"
									},
								]

								LogConfiguration: {
									LogDriver: "awslogs"
									Options: {
										"awslogs-group": _baseNames.ghaRunners
										"awslogs-region": Ref: "AWS::Region"
										"awslogs-stream-prefix": _baseNames.ghaRunners
									}}
								Name: _baseNames.ghaRunners
							},
						]
					}
				}

				ECSService: ecs.#Service & {
					Properties: {
						Cluster: "Fn::ImportValue": "\(opsCommonStackName)-EcsClusterArn"
						DeploymentConfiguration: {
							MaximumPercent:        200
							MinimumHealthyPercent: 50
						}
						DesiredCount: 1
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
						ServiceName: _baseNames.ghaRunners
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
