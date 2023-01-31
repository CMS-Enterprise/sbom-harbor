package cfn

import (
	"github.com/cue-sh/cfn-cue/aws/useast1/iam"
)

Stacks: {
	"dev-\(_baseNames.opsCommon)-use1": {}
	"prod-\(_baseNames.opsCommon)-use1": {}

	[StackName= =~_baseNames.opsCommon]: {
		// let stack = Stacks[StackName]

		Template: {
			Resources: CloudFormationRole: iam.#Role & {
				Properties: {
					RoleName: "ops-cloudformation-execution"
					ManagedPolicyArns: [
						"arn:aws:iam::aws:policy/AdministratorAccess",
					]
					Path: "/delegatedadmin/adodeveloper/service-role/"
					AssumeRolePolicyDocument: {
						Version: "2012-10-17"
						Statement: [
							{
								Effect: "Allow"
								Principal: {
									Service: "cloudformation.amazonaws.com"
								}
								Action: "sts:AssumeRole"
							},
						]
					}
				}
			}

			Outputs: CloudFormationRoleArn: {
				Value: "Fn::GetAtt": ["CloudFormationRole", "Arn"]
			}

		}
	}
}
