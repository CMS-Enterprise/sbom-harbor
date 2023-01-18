resource "aws_iam_role" "tfer--AWSControlTowerExecution" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "AWS": "arn:aws:iam::803783872674:root"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  managed_policy_arns  = ["arn:aws:iam::aws:policy/AdministratorAccess"]
  max_session_duration = "3600"
  name                 = "AWSControlTowerExecution"
  path                 = "/"
}

resource "aws_iam_role" "tfer--AWSControlTower_VPCFlowLogsRole" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "vpc-flow-logs.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"]
  max_session_duration = "3600"
  name                 = "AWSControlTower_VPCFlowLogsRole"
  path                 = "/"
}

resource "aws_iam_role" "tfer--AWSReservedSSO_AWSAdministratorAccess_c2c07cf33a60ffc0" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "sts:AssumeRoleWithSAML",
        "sts:TagSession"
      ],
      "Condition": {
        "StringEquals": {
          "SAML:aud": "https://signin.aws.amazon.com/saml"
        }
      },
      "Effect": "Allow",
      "Principal": {
        "Federated": "arn:aws:iam::393419659647:saml-provider/AWSSSO_e6af1ae39565cfa5_DO_NOT_DELETE"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  description          = "Provides full access to AWS services and resources"
  managed_policy_arns  = ["arn:aws:iam::aws:policy/AdministratorAccess"]
  max_session_duration = "43200"
  name                 = "AWSReservedSSO_AWSAdministratorAccess_c2c07cf33a60ffc0"
  path                 = "/aws-reserved/sso.amazonaws.com/us-west-2/"
}

resource "aws_iam_role" "tfer--AWSReservedSSO_AWSOrganizationsFullAccess_41009b386636cd27" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "sts:AssumeRoleWithSAML",
        "sts:TagSession"
      ],
      "Condition": {
        "StringEquals": {
          "SAML:aud": "https://signin.aws.amazon.com/saml"
        }
      },
      "Effect": "Allow",
      "Principal": {
        "Federated": "arn:aws:iam::393419659647:saml-provider/AWSSSO_e6af1ae39565cfa5_DO_NOT_DELETE"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  description          = "Provides full access to AWS Organizations"
  managed_policy_arns  = ["arn:aws:iam::aws:policy/AWSOrganizationsFullAccess"]
  max_session_duration = "43200"
  name                 = "AWSReservedSSO_AWSOrganizationsFullAccess_41009b386636cd27"
  path                 = "/aws-reserved/sso.amazonaws.com/us-west-2/"
}

resource "aws_iam_role" "tfer--AWSReservedSSO_AWSPowerUserAccess_546815fe1bd97154" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "sts:AssumeRoleWithSAML",
        "sts:TagSession"
      ],
      "Condition": {
        "StringEquals": {
          "SAML:aud": "https://signin.aws.amazon.com/saml"
        }
      },
      "Effect": "Allow",
      "Principal": {
        "Federated": "arn:aws:iam::393419659647:saml-provider/AWSSSO_e6af1ae39565cfa5_DO_NOT_DELETE"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  description          = "Provides full access to AWS services and resources, but does not allow management of Users and groups"
  managed_policy_arns  = ["arn:aws:iam::aws:policy/PowerUserAccess"]
  max_session_duration = "43200"
  name                 = "AWSReservedSSO_AWSPowerUserAccess_546815fe1bd97154"
  path                 = "/aws-reserved/sso.amazonaws.com/us-west-2/"
}

resource "aws_iam_role" "tfer--AWSReservedSSO_AWSReadOnlyAccess_d4aae4dfcf0bc4c2" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "sts:AssumeRoleWithSAML",
        "sts:TagSession"
      ],
      "Condition": {
        "StringEquals": {
          "SAML:aud": "https://signin.aws.amazon.com/saml"
        }
      },
      "Effect": "Allow",
      "Principal": {
        "Federated": "arn:aws:iam::393419659647:saml-provider/AWSSSO_e6af1ae39565cfa5_DO_NOT_DELETE"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  description          = "This policy grants permissions to view resources and basic metadata across all AWS services"
  managed_policy_arns  = ["arn:aws:iam::aws:policy/job-function/ViewOnlyAccess"]
  max_session_duration = "43200"
  name                 = "AWSReservedSSO_AWSReadOnlyAccess_d4aae4dfcf0bc4c2"
  path                 = "/aws-reserved/sso.amazonaws.com/us-west-2/"
}

resource "aws_iam_role" "tfer--AWSServiceRoleForAmazonElasticFileSystem" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "elasticfilesystem.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  managed_policy_arns  = ["arn:aws:iam::aws:policy/aws-service-role/AmazonElasticFileSystemServiceRolePolicy"]
  max_session_duration = "3600"
  name                 = "AWSServiceRoleForAmazonElasticFileSystem"
  path                 = "/aws-service-role/elasticfilesystem.amazonaws.com/"
}

resource "aws_iam_role" "tfer--AWSServiceRoleForAmazonGuardDuty" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "guardduty.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  managed_policy_arns  = ["arn:aws:iam::aws:policy/aws-service-role/AmazonGuardDutyServiceRolePolicy"]
  max_session_duration = "3600"
  name                 = "AWSServiceRoleForAmazonGuardDuty"
  path                 = "/aws-service-role/guardduty.amazonaws.com/"
}

resource "aws_iam_role" "tfer--AWSServiceRoleForApplicationAutoScaling_DynamoDBTable" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "dynamodb.application-autoscaling.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  managed_policy_arns  = ["arn:aws:iam::aws:policy/aws-service-role/AWSApplicationAutoscalingDynamoDBTablePolicy"]
  max_session_duration = "3600"
  name                 = "AWSServiceRoleForApplicationAutoScaling_DynamoDBTable"
  path                 = "/aws-service-role/dynamodb.application-autoscaling.amazonaws.com/"
}

resource "aws_iam_role" "tfer--AWSServiceRoleForCloudFormationStackSetsOrgMember" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "member.org.stacksets.cloudformation.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  description          = "Service linked role for CloudFormation StackSets (Organization Member)"
  managed_policy_arns  = ["arn:aws:iam::aws:policy/aws-service-role/CloudFormationStackSetsOrgMemberServiceRolePolicy"]
  max_session_duration = "3600"
  name                 = "AWSServiceRoleForCloudFormationStackSetsOrgMember"
  path                 = "/aws-service-role/member.org.stacksets.cloudformation.amazonaws.com/"
}

resource "aws_iam_role" "tfer--AWSServiceRoleForCloudTrail" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "cloudtrail.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  managed_policy_arns  = ["arn:aws:iam::aws:policy/aws-service-role/CloudTrailServiceRolePolicy"]
  max_session_duration = "3600"
  name                 = "AWSServiceRoleForCloudTrail"
  path                 = "/aws-service-role/cloudtrail.amazonaws.com/"
}

resource "aws_iam_role" "tfer--AWSServiceRoleForConfig" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "config.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  managed_policy_arns  = ["arn:aws:iam::aws:policy/aws-service-role/AWSConfigServiceRolePolicy"]
  max_session_duration = "3600"
  name                 = "AWSServiceRoleForConfig"
  path                 = "/aws-service-role/config.amazonaws.com/"
}

resource "aws_iam_role" "tfer--AWSServiceRoleForECS" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "ecs.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  description          = "Role to enable Amazon ECS to manage your cluster."
  managed_policy_arns  = ["arn:aws:iam::aws:policy/aws-service-role/AmazonECSServiceRolePolicy"]
  max_session_duration = "3600"
  name                 = "AWSServiceRoleForECS"
  path                 = "/aws-service-role/ecs.amazonaws.com/"
}

resource "aws_iam_role" "tfer--AWSServiceRoleForElasticLoadBalancing" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "elasticloadbalancing.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  description          = "Allows ELB to call AWS services on your behalf."
  managed_policy_arns  = ["arn:aws:iam::aws:policy/aws-service-role/AWSElasticLoadBalancingServiceRolePolicy"]
  max_session_duration = "3600"
  name                 = "AWSServiceRoleForElasticLoadBalancing"
  path                 = "/aws-service-role/elasticloadbalancing.amazonaws.com/"
}

resource "aws_iam_role" "tfer--AWSServiceRoleForOrganizations" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "organizations.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  description          = "Service-linked role used by AWS Organizations to enable integration of other AWS services with Organizations."
  managed_policy_arns  = ["arn:aws:iam::aws:policy/aws-service-role/AWSOrganizationsServiceTrustPolicy"]
  max_session_duration = "3600"
  name                 = "AWSServiceRoleForOrganizations"
  path                 = "/aws-service-role/organizations.amazonaws.com/"
}

resource "aws_iam_role" "tfer--AWSServiceRoleForSSO" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "sso.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  description          = "Service-linked role used by AWS SSO to manage AWS resources, including IAM roles, policies and SAML IdP on your behalf."
  managed_policy_arns  = ["arn:aws:iam::aws:policy/aws-service-role/AWSSSOServiceRolePolicy"]
  max_session_duration = "3600"
  name                 = "AWSServiceRoleForSSO"
  path                 = "/aws-service-role/sso.amazonaws.com/"
}

resource "aws_iam_role" "tfer--AWSServiceRoleForSecurityHub" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "securityhub.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  managed_policy_arns  = ["arn:aws:iam::aws:policy/aws-service-role/AWSSecurityHubServiceRolePolicy"]
  max_session_duration = "3600"
  name                 = "AWSServiceRoleForSecurityHub"
  path                 = "/aws-service-role/securityhub.amazonaws.com/"
}

resource "aws_iam_role" "tfer--AWSServiceRoleForSupport" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "support.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  description          = "Enables resource access for AWS to provide billing, administrative and support services"
  managed_policy_arns  = ["arn:aws:iam::aws:policy/aws-service-role/AWSSupportServiceRolePolicy"]
  max_session_duration = "3600"
  name                 = "AWSServiceRoleForSupport"
  path                 = "/aws-service-role/support.amazonaws.com/"
}

resource "aws_iam_role" "tfer--AWSServiceRoleForTrustedAdvisor" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "trustedadvisor.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  description          = "Access for the AWS Trusted Advisor Service to help reduce cost, increase performance, and improve security of your AWS environment."
  managed_policy_arns  = ["arn:aws:iam::aws:policy/aws-service-role/AWSTrustedAdvisorServiceRolePolicy"]
  max_session_duration = "3600"
  name                 = "AWSServiceRoleForTrustedAdvisor"
  path                 = "/aws-service-role/trustedadvisor.amazonaws.com/"
}

resource "aws_iam_role" "tfer--SBOM-Management-Api-APIKeyAuthorizerServiceRole4E8-IBAT5I6C9PNY" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "APIKeyAuthorizerServiceRoleDefaultPolicyB4371CAE"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"dynamodb:BatchGetItem\",\"dynamodb:GetRecords\",\"dynamodb:GetShardIterator\",\"dynamodb:Query\",\"dynamodb:GetItem\",\"dynamodb:Scan\",\"dynamodb:ConditionCheckItem\",\"dynamodb:BatchWriteItem\",\"dynamodb:PutItem\",\"dynamodb:UpdateItem\",\"dynamodb:DeleteItem\",\"dynamodb:DescribeTable\"],\"Resource\":[\"arn:aws:dynamodb:us-east-2:393419659647:table/HarborTeamsTable\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "SBOM-Management-Api-APIKeyAuthorizerServiceRole4E8-IBAT5I6C9PNY"
  path                 = "/"
}

resource "aws_iam_role" "tfer--SBOM-Management-Api-LoginLambdaServiceRoleBF816334-W7XUB9TKESGU" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "LoginLambdaServiceRoleDefaultPolicyFF0411F9"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"cognito-idp:AdminGetUser\",\"cognito-idp:AdminEnableUser\",\"cognito-idp:AdminDisableUser\",\"cognito-idp:AdminInitiateAuth\"],\"Resource\":\"*\",\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "SBOM-Management-Api-LoginLambdaServiceRoleBF816334-W7XUB9TKESGU"
  path                 = "/"
}

resource "aws_iam_role" "tfer--SBOM-Management-Api-SBOMAPIAuthorizerAuthorizerSer-LP0PPL9842PZ" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "SBOMAPIAuthorizerAuthorizerServiceRoleDefaultPolicy14415F12"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"cognito-idp:AdminDisableUser\",\"cognito-idp:AdminEnableUser\",\"cognito-idp:AdminGetUser\",\"cognito-idp:ListUsers\"],\"Resource\":\"*\",\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "SBOM-Management-Api-SBOMAPIAuthorizerAuthorizerSer-LP0PPL9842PZ"
  path                 = "/"
}

resource "aws_iam_role" "tfer--SBOM-Management-Api-SBOMHarborCodebaseLambdaServic-FNMSV0WLE7SY" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "SBOMHarborCodebaseLambdaServiceRoleDefaultPolicy941BCA2E"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"dynamodb:BatchGetItem\",\"dynamodb:GetRecords\",\"dynamodb:GetShardIterator\",\"dynamodb:Query\",\"dynamodb:GetItem\",\"dynamodb:Scan\",\"dynamodb:ConditionCheckItem\",\"dynamodb:BatchWriteItem\",\"dynamodb:PutItem\",\"dynamodb:UpdateItem\",\"dynamodb:DeleteItem\",\"dynamodb:DescribeTable\"],\"Resource\":[\"arn:aws:dynamodb:us-east-2:393419659647:table/HarborTeamsTable\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "SBOM-Management-Api-SBOMHarborCodebaseLambdaServic-FNMSV0WLE7SY"
  path                 = "/"
}

resource "aws_iam_role" "tfer--SBOM-Management-Api-SBOMHarborCodebasePOSTLambdaSe-WTFVTQUCTUWN" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "SBOMHarborCodebasePOSTLambdaServiceRoleDefaultPolicyBFDE5298"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"dynamodb:BatchGetItem\",\"dynamodb:GetRecords\",\"dynamodb:GetShardIterator\",\"dynamodb:Query\",\"dynamodb:GetItem\",\"dynamodb:Scan\",\"dynamodb:ConditionCheckItem\",\"dynamodb:BatchWriteItem\",\"dynamodb:PutItem\",\"dynamodb:UpdateItem\",\"dynamodb:DeleteItem\",\"dynamodb:DescribeTable\"],\"Resource\":[\"arn:aws:dynamodb:us-east-2:393419659647:table/HarborTeamsTable\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "SBOM-Management-Api-SBOMHarborCodebasePOSTLambdaSe-WTFVTQUCTUWN"
  path                 = "/"
}

resource "aws_iam_role" "tfer--SBOM-Management-Api-SBOMHarborCodebasesLambdaServi-5CJU7F7V2602" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "SBOMHarborCodebasesLambdaServiceRoleDefaultPolicy4394E45F"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"dynamodb:BatchGetItem\",\"dynamodb:GetRecords\",\"dynamodb:GetShardIterator\",\"dynamodb:Query\",\"dynamodb:GetItem\",\"dynamodb:Scan\",\"dynamodb:ConditionCheckItem\",\"dynamodb:BatchWriteItem\",\"dynamodb:PutItem\",\"dynamodb:UpdateItem\",\"dynamodb:DeleteItem\",\"dynamodb:DescribeTable\"],\"Resource\":[\"arn:aws:dynamodb:us-east-2:393419659647:table/HarborTeamsTable\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "SBOM-Management-Api-SBOMHarborCodebasesLambdaServi-5CJU7F7V2602"
  path                 = "/"
}

resource "aws_iam_role" "tfer--SBOM-Management-Api-SBOMHarborMemberLambdaServiceR-1SE719MXDVF9V" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "SBOMHarborMemberLambdaServiceRoleDefaultPolicy9DBA396F"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"dynamodb:BatchGetItem\",\"dynamodb:GetRecords\",\"dynamodb:GetShardIterator\",\"dynamodb:Query\",\"dynamodb:GetItem\",\"dynamodb:Scan\",\"dynamodb:ConditionCheckItem\",\"dynamodb:BatchWriteItem\",\"dynamodb:PutItem\",\"dynamodb:UpdateItem\",\"dynamodb:DeleteItem\",\"dynamodb:DescribeTable\"],\"Resource\":[\"arn:aws:dynamodb:us-east-2:393419659647:table/HarborTeamsTable\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "SBOM-Management-Api-SBOMHarborMemberLambdaServiceR-1SE719MXDVF9V"
  path                 = "/"
}

resource "aws_iam_role" "tfer--SBOM-Management-Api-SBOMHarborMemberPOSTLambdaServ-146TE0NUMMGBI" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "SBOMHarborMemberPOSTLambdaServiceRoleDefaultPolicy87BF556E"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"dynamodb:BatchGetItem\",\"dynamodb:GetRecords\",\"dynamodb:GetShardIterator\",\"dynamodb:Query\",\"dynamodb:GetItem\",\"dynamodb:Scan\",\"dynamodb:ConditionCheckItem\",\"dynamodb:BatchWriteItem\",\"dynamodb:PutItem\",\"dynamodb:UpdateItem\",\"dynamodb:DeleteItem\",\"dynamodb:DescribeTable\"],\"Resource\":[\"arn:aws:dynamodb:us-east-2:393419659647:table/HarborTeamsTable\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "SBOM-Management-Api-SBOMHarborMemberPOSTLambdaServ-146TE0NUMMGBI"
  path                 = "/"
}

resource "aws_iam_role" "tfer--SBOM-Management-Api-SBOMHarborMembersLambdaService-19LN7SS31SOV6" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "SBOMHarborMembersLambdaServiceRoleDefaultPolicy463F955A"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"dynamodb:BatchGetItem\",\"dynamodb:GetRecords\",\"dynamodb:GetShardIterator\",\"dynamodb:Query\",\"dynamodb:GetItem\",\"dynamodb:Scan\",\"dynamodb:ConditionCheckItem\",\"dynamodb:BatchWriteItem\",\"dynamodb:PutItem\",\"dynamodb:UpdateItem\",\"dynamodb:DeleteItem\",\"dynamodb:DescribeTable\"],\"Resource\":[\"arn:aws:dynamodb:us-east-2:393419659647:table/HarborTeamsTable\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "SBOM-Management-Api-SBOMHarborMembersLambdaService-19LN7SS31SOV6"
  path                 = "/"
}

resource "aws_iam_role" "tfer--SBOM-Management-Api-SBOMHarborProjectLambdaService-1DA4GKY55NY7C" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "SBOMHarborProjectLambdaServiceRoleDefaultPolicy2B2C1CD9"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"dynamodb:BatchGetItem\",\"dynamodb:GetRecords\",\"dynamodb:GetShardIterator\",\"dynamodb:Query\",\"dynamodb:GetItem\",\"dynamodb:Scan\",\"dynamodb:ConditionCheckItem\",\"dynamodb:BatchWriteItem\",\"dynamodb:PutItem\",\"dynamodb:UpdateItem\",\"dynamodb:DeleteItem\",\"dynamodb:DescribeTable\"],\"Resource\":[\"arn:aws:dynamodb:us-east-2:393419659647:table/HarborTeamsTable\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "SBOM-Management-Api-SBOMHarborProjectLambdaService-1DA4GKY55NY7C"
  path                 = "/"
}

resource "aws_iam_role" "tfer--SBOM-Management-Api-SBOMHarborProjectPOSTLambdaSer-70D8PDMD1BZM" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "SBOMHarborProjectPOSTLambdaServiceRoleDefaultPolicy396C40D7"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"dynamodb:BatchGetItem\",\"dynamodb:GetRecords\",\"dynamodb:GetShardIterator\",\"dynamodb:Query\",\"dynamodb:GetItem\",\"dynamodb:Scan\",\"dynamodb:ConditionCheckItem\",\"dynamodb:BatchWriteItem\",\"dynamodb:PutItem\",\"dynamodb:UpdateItem\",\"dynamodb:DeleteItem\",\"dynamodb:DescribeTable\"],\"Resource\":[\"arn:aws:dynamodb:us-east-2:393419659647:table/HarborTeamsTable\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "SBOM-Management-Api-SBOMHarborProjectPOSTLambdaSer-70D8PDMD1BZM"
  path                 = "/"
}

resource "aws_iam_role" "tfer--SBOM-Management-Api-SBOMHarborProjectsLambdaServic-8HDO9BSLPW0" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "SBOMHarborProjectsLambdaServiceRoleDefaultPolicy907A292B"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"dynamodb:BatchGetItem\",\"dynamodb:GetRecords\",\"dynamodb:GetShardIterator\",\"dynamodb:Query\",\"dynamodb:GetItem\",\"dynamodb:Scan\",\"dynamodb:ConditionCheckItem\",\"dynamodb:BatchWriteItem\",\"dynamodb:PutItem\",\"dynamodb:UpdateItem\",\"dynamodb:DeleteItem\",\"dynamodb:DescribeTable\"],\"Resource\":[\"arn:aws:dynamodb:us-east-2:393419659647:table/HarborTeamsTable\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "SBOM-Management-Api-SBOMHarborProjectsLambdaServic-8HDO9BSLPW0"
  path                 = "/"
}

resource "aws_iam_role" "tfer--SBOM-Management-Api-SBOMHarborTeamLambdaServiceRol-2YGJRNR2P7J3" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "SBOMHarborTeamLambdaServiceRoleDefaultPolicy2C54D764"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"dynamodb:BatchGetItem\",\"dynamodb:GetRecords\",\"dynamodb:GetShardIterator\",\"dynamodb:Query\",\"dynamodb:GetItem\",\"dynamodb:Scan\",\"dynamodb:ConditionCheckItem\",\"dynamodb:BatchWriteItem\",\"dynamodb:PutItem\",\"dynamodb:UpdateItem\",\"dynamodb:DeleteItem\",\"dynamodb:DescribeTable\"],\"Resource\":[\"arn:aws:dynamodb:us-east-2:393419659647:table/HarborTeamsTable\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "SBOM-Management-Api-SBOMHarborTeamLambdaServiceRol-2YGJRNR2P7J3"
  path                 = "/"
}

resource "aws_iam_role" "tfer--SBOM-Management-Api-SBOMHarborTeamPOSTLambdaServic-1H1JYUAR9SQEV" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "SBOMHarborTeamPOSTLambdaServiceRoleDefaultPolicy7ED973BA"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"dynamodb:BatchGetItem\",\"dynamodb:GetRecords\",\"dynamodb:GetShardIterator\",\"dynamodb:Query\",\"dynamodb:GetItem\",\"dynamodb:Scan\",\"dynamodb:ConditionCheckItem\",\"dynamodb:BatchWriteItem\",\"dynamodb:PutItem\",\"dynamodb:UpdateItem\",\"dynamodb:DeleteItem\",\"dynamodb:DescribeTable\"],\"Resource\":[\"arn:aws:dynamodb:us-east-2:393419659647:table/HarborTeamsTable\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "SBOM-Management-Api-SBOMHarborTeamPOSTLambdaServic-1H1JYUAR9SQEV"
  path                 = "/"
}

resource "aws_iam_role" "tfer--SBOM-Management-Api-SBOMHarborTeamsLambdaServiceRo-1I6C3OHL8WMED" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "SBOMHarborTeamsLambdaServiceRoleDefaultPolicyE611D150"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"dynamodb:BatchGetItem\",\"dynamodb:GetRecords\",\"dynamodb:GetShardIterator\",\"dynamodb:Query\",\"dynamodb:GetItem\",\"dynamodb:Scan\",\"dynamodb:ConditionCheckItem\",\"dynamodb:BatchWriteItem\",\"dynamodb:PutItem\",\"dynamodb:UpdateItem\",\"dynamodb:DeleteItem\",\"dynamodb:DescribeTable\"],\"Resource\":[\"arn:aws:dynamodb:us-east-2:393419659647:table/HarborTeamsTable\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "SBOM-Management-Api-SBOMHarborTeamsLambdaServiceRo-1I6C3OHL8WMED"
  path                 = "/"
}

resource "aws_iam_role" "tfer--SBOM-Management-Api-SBOMHarborTokenLambdaServiceRo-THV24GLM4RZG" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "SBOMHarborTokenLambdaServiceRoleDefaultPolicyCA88A994"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"dynamodb:BatchGetItem\",\"dynamodb:GetRecords\",\"dynamodb:GetShardIterator\",\"dynamodb:Query\",\"dynamodb:GetItem\",\"dynamodb:Scan\",\"dynamodb:ConditionCheckItem\",\"dynamodb:BatchWriteItem\",\"dynamodb:PutItem\",\"dynamodb:UpdateItem\",\"dynamodb:DeleteItem\",\"dynamodb:DescribeTable\"],\"Resource\":[\"arn:aws:dynamodb:us-east-2:393419659647:table/HarborTeamsTable\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "SBOM-Management-Api-SBOMHarborTokenLambdaServiceRo-THV24GLM4RZG"
  path                 = "/"
}

resource "aws_iam_role" "tfer--SBOM-Management-Api-SBOMHarborTokenPOSTLambdaServi-GVF6EL2RI93Q" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "SBOMHarborTokenPOSTLambdaServiceRoleDefaultPolicyBF441CA5"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"dynamodb:BatchGetItem\",\"dynamodb:GetRecords\",\"dynamodb:GetShardIterator\",\"dynamodb:Query\",\"dynamodb:GetItem\",\"dynamodb:Scan\",\"dynamodb:ConditionCheckItem\",\"dynamodb:BatchWriteItem\",\"dynamodb:PutItem\",\"dynamodb:UpdateItem\",\"dynamodb:DeleteItem\",\"dynamodb:DescribeTable\"],\"Resource\":[\"arn:aws:dynamodb:us-east-2:393419659647:table/HarborTeamsTable\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "SBOM-Management-Api-SBOMHarborTokenPOSTLambdaServi-GVF6EL2RI93Q"
  path                 = "/"
}

resource "aws_iam_role" "tfer--SBOM-Management-Api-SBOMHarborTokensLambdaServiceR-1MN0Z6PRZMWV7" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "SBOMHarborTokensLambdaServiceRoleDefaultPolicy5EB0FA46"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"dynamodb:BatchGetItem\",\"dynamodb:GetRecords\",\"dynamodb:GetShardIterator\",\"dynamodb:Query\",\"dynamodb:GetItem\",\"dynamodb:Scan\",\"dynamodb:ConditionCheckItem\",\"dynamodb:BatchWriteItem\",\"dynamodb:PutItem\",\"dynamodb:UpdateItem\",\"dynamodb:DeleteItem\",\"dynamodb:DescribeTable\"],\"Resource\":[\"arn:aws:dynamodb:us-east-2:393419659647:table/HarborTeamsTable\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "SBOM-Management-Api-SBOMHarborTokensLambdaServiceR-1MN0Z6PRZMWV7"
  path                 = "/"
}

resource "aws_iam_role" "tfer--SBOM-Management-Api-SBOMIngressLambdaServiceRole0D-1IEGFYDRHCHMU" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "SBOMIngressLambdaServiceRoleDefaultPolicy4A3C2501"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"s3:PutObject\",\"s3:PutObjectLegalHold\",\"s3:PutObjectRetention\",\"s3:PutObjectTagging\",\"s3:PutObjectVersionTagging\",\"s3:Abort*\"],\"Resource\":\"arn:aws:s3:::sbom.bucket.393419659647/*\",\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "SBOM-Management-Api-SBOMIngressLambdaServiceRole0D-1IEGFYDRHCHMU"
  path                 = "/"
}

resource "aws_iam_role" "tfer--SBOM-Management-Api-UserSearchLambdaServiceRoleB24-1I6PJQYXXW6FO" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "UserSearchLambdaServiceRoleDefaultPolicy1F80E6C6"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":\"cognito-idp:ListUsers\",\"Resource\":\"*\",\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "SBOM-Management-Api-UserSearchLambdaServiceRoleB24-1I6PJQYXXW6FO"
  path                 = "/"
}

resource "aws_iam_role" "tfer--SBOMApi-Enrichment-BucketNotificationsHandler050a0-WORBW5Y8PI4O" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "BucketNotificationsHandler050a0587b7544547bf325f094a3db834RoleDefaultPolicy2CF63D36"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":\"s3:PutBucketNotification\",\"Resource\":\"*\",\"Effect\":\"Allow\"},{\"Action\":\"s3:GetBucketNotification\",\"Resource\":\"*\",\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"]
  max_session_duration = "3600"
  name                 = "SBOMApi-Enrichment-BucketNotificationsHandler050a0-WORBW5Y8PI4O"
  path                 = "/"
}

resource "aws_iam_role" "tfer--SBOMApi-Enrichment-CustomS3AutoDeleteObjectsCustom-VOKAN1K8PZEV" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"]
  max_session_duration = "3600"
  name                 = "SBOMApi-Enrichment-CustomS3AutoDeleteObjectsCustom-VOKAN1K8PZEV"
  path                 = "/"
}

resource "aws_iam_role" "tfer--SBOMApi-Enrichment-DTFargateClusterdtTaskDefinitio-1ADE4X95U9SYU" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "ecs-tasks.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "DTFargateClusterdtTaskDefinitionExecutionRoleDefaultPolicy10A3D394"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"logs:CreateLogStream\",\"logs:PutLogEvents\"],\"Resource\":\"arn:aws:logs:us-east-2:393419659647:log-group:SBOMApi-Enrichment-DTFargateClusterdtTaskDefinitiondtContainerLogGroup2CD7A010-SEfr032VcIYV:*\",\"Effect\":\"Allow\"}]}"
  }

  max_session_duration = "3600"
  name                 = "SBOMApi-Enrichment-DTFargateClusterdtTaskDefinitio-1ADE4X95U9SYU"
  path                 = "/"
}

resource "aws_iam_role" "tfer--SBOMApi-Enrichment-DTFargateClusterdtTaskDefinitio-MN2AU0Q2QUUS" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "ecs-tasks.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  max_session_duration = "3600"
  name                 = "SBOMApi-Enrichment-DTFargateClusterdtTaskDefinitio-MN2AU0Q2QUUS"
  path                 = "/"
}

resource "aws_iam_role" "tfer--SBOMApi-Enrichment-DefaultEnrichmentInterfaceLambd-VKGSNN0CAP2F" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "DefaultEnrichmentInterfaceLambdaServiceRoleDefaultPolicy433111C8"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":\"events:PutEvents\",\"Resource\":\"arn:aws:events:us-east-2:393419659647:event-bus/SBOMEnrichmentEventBus\",\"Effect\":\"Allow\"},{\"Action\":[\"s3:PutObject\",\"s3:PutObjectLegalHold\",\"s3:PutObjectRetention\",\"s3:PutObjectTagging\",\"s3:PutObjectVersionTagging\",\"s3:Abort*\"],\"Resource\":\"arn:aws:s3:::sbom.bucket.393419659647/*\",\"Effect\":\"Allow\"},{\"Action\":[\"s3:GetObject*\",\"s3:GetBucket*\",\"s3:List*\",\"s3:DeleteObject*\",\"s3:PutObject\",\"s3:PutObjectLegalHold\",\"s3:PutObjectRetention\",\"s3:PutObjectTagging\",\"s3:PutObjectVersionTagging\",\"s3:Abort*\"],\"Resource\":[\"arn:aws:s3:::sbom.bucket.393419659647\",\"arn:aws:s3:::sbom.bucket.393419659647/*\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "SBOMApi-Enrichment-DefaultEnrichmentInterfaceLambd-VKGSNN0CAP2F"
  path                 = "/"
}

resource "aws_iam_role" "tfer--SBOMApi-Enrichment-DependencyTrackInterfaceLambdaS-153Y3Q0VCTZJH" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "DependencyTrackInterfaceLambdaServiceRoleDefaultPolicy9B4EB588"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":\"events:PutEvents\",\"Resource\":\"arn:aws:events:us-east-2:393419659647:event-bus/SBOMEnrichmentEventBus\",\"Effect\":\"Allow\"},{\"Action\":[\"s3:PutObject\",\"s3:PutObjectLegalHold\",\"s3:PutObjectRetention\",\"s3:PutObjectTagging\",\"s3:PutObjectVersionTagging\",\"s3:Abort*\"],\"Resource\":\"arn:aws:s3:::sbom.bucket.393419659647/*\",\"Effect\":\"Allow\"},{\"Action\":[\"s3:GetObject*\",\"s3:GetBucket*\",\"s3:List*\",\"s3:DeleteObject*\",\"s3:PutObject\",\"s3:PutObjectLegalHold\",\"s3:PutObjectRetention\",\"s3:PutObjectTagging\",\"s3:PutObjectVersionTagging\",\"s3:Abort*\"],\"Resource\":[\"arn:aws:s3:::sbom.bucket.393419659647\",\"arn:aws:s3:::sbom.bucket.393419659647/*\"],\"Effect\":\"Allow\"},{\"Action\":[\"ssm:DescribeParameters\",\"ssm:GetParameters\",\"ssm:GetParameter\",\"ssm:GetParameterHistory\"],\"Resource\":\"arn:aws:ssm:us-east-2:393419659647:parameter/DT_ROOT_PWD\",\"Effect\":\"Allow\"},{\"Action\":\"ssm:PutParameter\",\"Resource\":\"arn:aws:ssm:us-east-2:393419659647:parameter/DT_ROOT_PWD\",\"Effect\":\"Allow\"},{\"Action\":[\"ssm:DescribeParameters\",\"ssm:GetParameters\",\"ssm:GetParameter\",\"ssm:GetParameterHistory\"],\"Resource\":\"arn:aws:ssm:us-east-2:393419659647:parameter/DT_API_KEY\",\"Effect\":\"Allow\"},{\"Action\":\"ssm:PutParameter\",\"Resource\":\"arn:aws:ssm:us-east-2:393419659647:parameter/DT_API_KEY\",\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "SBOMApi-Enrichment-DependencyTrackInterfaceLambdaS-153Y3Q0VCTZJH"
  path                 = "/"
}

resource "aws_iam_role" "tfer--SBOMApi-Enrichment-ENRICHMENTSTATEMACHINEEventsRol-QLRDKM2C3E3A" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "events.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "ENRICHMENTSTATEMACHINEEventsRoleDefaultPolicyC7BD5AE4"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":\"states:StartExecution\",\"Resource\":\"arn:aws:states:us-east-2:393419659647:stateMachine:ENRICHMENT_STATE_MACHINE\",\"Effect\":\"Allow\"}]}"
  }

  max_session_duration = "3600"
  name                 = "SBOMApi-Enrichment-ENRICHMENTSTATEMACHINEEventsRol-QLRDKM2C3E3A"
  path                 = "/"
}

resource "aws_iam_role" "tfer--SBOMApi-Enrichment-ENRICHMENTSTATEMACHINERoleA5B02-19AOYJNM8DJNR" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "states.us-east-2.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "ENRICHMENTSTATEMACHINERoleDefaultPolicyA998D563"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":\"lambda:InvokeFunction\",\"Resource\":[\"arn:aws:lambda:us-east-2:393419659647:function:SummarizerLambda\",\"arn:aws:lambda:us-east-2:393419659647:function:SummarizerLambda:*\"],\"Effect\":\"Allow\"},{\"Action\":\"lambda:InvokeFunction\",\"Resource\":[\"arn:aws:lambda:us-east-2:393419659647:function:DependencyTrackInterfaceLambda\",\"arn:aws:lambda:us-east-2:393419659647:function:DependencyTrackInterfaceLambda:*\"],\"Effect\":\"Allow\"},{\"Action\":\"lambda:InvokeFunction\",\"Resource\":[\"arn:aws:lambda:us-east-2:393419659647:function:DefaultEnrichmentInterfaceLambda\",\"arn:aws:lambda:us-east-2:393419659647:function:DefaultEnrichmentInterfaceLambda:*\"],\"Effect\":\"Allow\"}]}"
  }

  max_session_duration = "3600"
  name                 = "SBOMApi-Enrichment-ENRICHMENTSTATEMACHINERoleA5B02-19AOYJNM8DJNR"
  path                 = "/"
}

resource "aws_iam_role" "tfer--SBOMApi-Enrichment-SBOMEnrichmentIngressLambdaServ-GC5O9QX4V5H1" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "SBOMEnrichmentIngressLambdaServiceRoleDefaultPolicy48134E1F"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"s3:GetObject*\",\"s3:GetBucket*\",\"s3:List*\"],\"Resource\":[\"arn:aws:s3:::sbom.bucket.393419659647\",\"arn:aws:s3:::sbom.bucket.393419659647/*\"],\"Effect\":\"Allow\"},{\"Action\":\"events:PutEvents\",\"Resource\":\"arn:aws:events:us-east-2:393419659647:event-bus/SBOMEnrichmentEventBus\",\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "SBOMApi-Enrichment-SBOMEnrichmentIngressLambdaServ-GC5O9QX4V5H1"
  path                 = "/"
}

resource "aws_iam_role" "tfer--SBOMApi-Enrichment-SummarizerLambdaServiceRoleEEF5-1V7DA30FJTF4X" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "SummarizerLambdaServiceRoleDefaultPolicy6A39C251"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":\"events:PutEvents\",\"Resource\":\"arn:aws:events:us-east-2:393419659647:event-bus/SBOMEnrichmentEventBus\",\"Effect\":\"Allow\"},{\"Action\":[\"s3:PutObject\",\"s3:PutObjectLegalHold\",\"s3:PutObjectRetention\",\"s3:PutObjectTagging\",\"s3:PutObjectVersionTagging\",\"s3:Abort*\"],\"Resource\":\"arn:aws:s3:::sbom.bucket.393419659647/*\",\"Effect\":\"Allow\"},{\"Action\":[\"s3:GetObject*\",\"s3:GetBucket*\",\"s3:List*\",\"s3:DeleteObject*\",\"s3:PutObject\",\"s3:PutObjectLegalHold\",\"s3:PutObjectRetention\",\"s3:PutObjectTagging\",\"s3:PutObjectVersionTagging\",\"s3:Abort*\"],\"Resource\":[\"arn:aws:s3:::sbom.bucket.393419659647\",\"arn:aws:s3:::sbom.bucket.393419659647/*\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "SBOMApi-Enrichment-SummarizerLambdaServiceRoleEEF5-1V7DA30FJTF4X"
  path                 = "/"
}

resource "aws_iam_role" "tfer--SBOMApi-Shared-Resource-BucketNotificationsHandler-931RZ9L19VYI" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "BucketNotificationsHandler050a0587b7544547bf325f094a3db834RoleDefaultPolicy2CF63D36"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":\"s3:PutBucketNotification\",\"Resource\":\"*\",\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"]
  max_session_duration = "3600"
  name                 = "SBOMApi-Shared-Resource-BucketNotificationsHandler-931RZ9L19VYI"
  path                 = "/"
}

resource "aws_iam_role" "tfer--SBOMApi-Shared-Resource-CustomS3AutoDeleteObjectsC-1CYVYQLWTF64P" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"]
  max_session_duration = "3600"
  name                 = "SBOMApi-Shared-Resource-CustomS3AutoDeleteObjectsC-1CYVYQLWTF64P"
  path                 = "/"
}

resource "aws_iam_role" "tfer--SBOMUserRole" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Condition": {
        "ForAnyValue:StringLike": {
          "cognito-identity.amazonaws.com:amr": "authenticated"
        },
        "StringEquals": {
          "cognito-identity.amazonaws.com:aud": "us-east-2_SE1Irip0Q"
        }
      },
      "Effect": "Allow",
      "Principal": {
        "Federated": "cognito-identity.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  description          = "Default role for authenticated users"
  managed_policy_arns  = ["arn:aws:iam::aws:policy/AmazonS3FullAccess", "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"]
  max_session_duration = "3600"
  name                 = "SBOMUserRole"
  path                 = "/"
}

resource "aws_iam_role" "tfer--aws-controltower-AdministratorExecutionRole" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "AWS": "arn:aws:iam::678602588155:role/aws-controltower-AuditAdministratorRole"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  managed_policy_arns  = ["arn:aws:iam::aws:policy/AdministratorAccess"]
  max_session_duration = "3600"
  name                 = "aws-controltower-AdministratorExecutionRole"
  path                 = "/"
}

resource "aws_iam_role" "tfer--aws-controltower-ConfigRecorderRole" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "config.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  managed_policy_arns  = ["arn:aws:iam::aws:policy/ReadOnlyAccess", "arn:aws:iam::aws:policy/service-role/AWS_ConfigRole"]
  max_session_duration = "3600"
  name                 = "aws-controltower-ConfigRecorderRole"
  path                 = "/"
}

resource "aws_iam_role" "tfer--aws-controltower-ForwardSnsNotificationRole" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "sns"
    policy = "{\"Statement\":[{\"Action\":[\"sns:publish\"],\"Resource\":\"arn:aws:sns:*:678602588155:aws-controltower-AggregateSecurityNotifications\",\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"]
  max_session_duration = "3600"
  name                 = "aws-controltower-ForwardSnsNotificationRole"
  path                 = "/"
}

resource "aws_iam_role" "tfer--aws-controltower-ReadOnlyExecutionRole" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "AWS": "arn:aws:iam::678602588155:role/aws-controltower-AuditReadOnlyRole"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  managed_policy_arns  = ["arn:aws:iam::aws:policy/ReadOnlyAccess"]
  max_session_duration = "3600"
  name                 = "aws-controltower-ReadOnlyExecutionRole"
  path                 = "/"
}

resource "aws_iam_role" "tfer--cdk-hnb659fds-cfn-exec-role-393419659647-us-east-2" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "cloudformation.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  managed_policy_arns  = ["arn:aws:iam::aws:policy/AdministratorAccess"]
  max_session_duration = "3600"
  name                 = "cdk-hnb659fds-cfn-exec-role-393419659647-us-east-2"
  path                 = "/"
}

resource "aws_iam_role" "tfer--cdk-hnb659fds-cfn-exec-role-393419659647-us-west-2" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "cloudformation.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  managed_policy_arns  = ["arn:aws:iam::aws:policy/AdministratorAccess"]
  max_session_duration = "3600"
  name                 = "cdk-hnb659fds-cfn-exec-role-393419659647-us-west-2"
  path                 = "/"
}

resource "aws_iam_role" "tfer--cdk-hnb659fds-deploy-role-393419659647-us-east-2" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "AWS": "arn:aws:iam::393419659647:root"
      }
    }
  ],
  "Version": "2008-10-17"
}
POLICY

  inline_policy {
    name   = "default"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"cloudformation:CreateChangeSet\",\"cloudformation:DeleteChangeSet\",\"cloudformation:DescribeChangeSet\",\"cloudformation:DescribeStacks\",\"cloudformation:ExecuteChangeSet\",\"cloudformation:CreateStack\",\"cloudformation:UpdateStack\"],\"Resource\":\"*\",\"Effect\":\"Allow\",\"Sid\":\"CloudFormationPermissions\"},{\"Condition\":{\"StringNotEquals\":{\"s3:ResourceAccount\":\"393419659647\"}},\"Action\":[\"s3:GetObject*\",\"s3:GetBucket*\",\"s3:List*\",\"s3:Abort*\",\"s3:DeleteObject*\",\"s3:PutObject*\"],\"Resource\":\"*\",\"Effect\":\"Allow\",\"Sid\":\"PipelineCrossAccountArtifactsBucket\"},{\"Condition\":{\"StringEquals\":{\"kms:ViaService\":\"s3.us-east-2.amazonaws.com\"}},\"Action\":[\"kms:Decrypt\",\"kms:DescribeKey\",\"kms:Encrypt\",\"kms:ReEncrypt*\",\"kms:GenerateDataKey*\"],\"Resource\":\"*\",\"Effect\":\"Allow\",\"Sid\":\"PipelineCrossAccountArtifactsKey\"},{\"Action\":\"iam:PassRole\",\"Resource\":\"arn:aws:iam::393419659647:role/cdk-hnb659fds-cfn-exec-role-393419659647-us-east-2\",\"Effect\":\"Allow\"},{\"Action\":[\"cloudformation:DescribeStackEvents\",\"cloudformation:GetTemplate\",\"cloudformation:DeleteStack\",\"cloudformation:UpdateTerminationProtection\",\"sts:GetCallerIdentity\",\"cloudformation:GetTemplateSummary\"],\"Resource\":\"*\",\"Effect\":\"Allow\",\"Sid\":\"CliPermissions\"},{\"Action\":[\"s3:GetObject*\",\"s3:GetBucket*\",\"s3:List*\"],\"Resource\":[\"arn:aws:s3:::cdk-hnb659fds-assets-393419659647-us-east-2\",\"arn:aws:s3:::cdk-hnb659fds-assets-393419659647-us-east-2/*\"],\"Effect\":\"Allow\",\"Sid\":\"CliStagingBucket\"},{\"Action\":[\"ssm:GetParameter\"],\"Resource\":[\"arn:aws:ssm:us-east-2:393419659647:parameter/cdk-bootstrap/hnb659fds/version\"],\"Effect\":\"Allow\",\"Sid\":\"ReadVersion\"}]}"
  }

  max_session_duration = "3600"
  name                 = "cdk-hnb659fds-deploy-role-393419659647-us-east-2"
  path                 = "/"

  tags = {
    "aws-cdk:bootstrap-role" = "deploy"
  }

  tags_all = {
    "aws-cdk:bootstrap-role" = "deploy"
  }
}

resource "aws_iam_role" "tfer--cdk-hnb659fds-deploy-role-393419659647-us-west-2" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "AWS": "arn:aws:iam::393419659647:root"
      }
    }
  ],
  "Version": "2008-10-17"
}
POLICY

  inline_policy {
    name   = "default"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"cloudformation:CreateChangeSet\",\"cloudformation:DeleteChangeSet\",\"cloudformation:DescribeChangeSet\",\"cloudformation:DescribeStacks\",\"cloudformation:ExecuteChangeSet\",\"cloudformation:CreateStack\",\"cloudformation:UpdateStack\"],\"Resource\":\"*\",\"Effect\":\"Allow\",\"Sid\":\"CloudFormationPermissions\"},{\"Condition\":{\"StringNotEquals\":{\"s3:ResourceAccount\":\"393419659647\"}},\"Action\":[\"s3:GetObject*\",\"s3:GetBucket*\",\"s3:List*\",\"s3:Abort*\",\"s3:DeleteObject*\",\"s3:PutObject*\"],\"Resource\":\"*\",\"Effect\":\"Allow\",\"Sid\":\"PipelineCrossAccountArtifactsBucket\"},{\"Condition\":{\"StringEquals\":{\"kms:ViaService\":\"s3.us-west-2.amazonaws.com\"}},\"Action\":[\"kms:Decrypt\",\"kms:DescribeKey\",\"kms:Encrypt\",\"kms:ReEncrypt*\",\"kms:GenerateDataKey*\"],\"Resource\":\"*\",\"Effect\":\"Allow\",\"Sid\":\"PipelineCrossAccountArtifactsKey\"},{\"Action\":\"iam:PassRole\",\"Resource\":\"arn:aws:iam::393419659647:role/cdk-hnb659fds-cfn-exec-role-393419659647-us-west-2\",\"Effect\":\"Allow\"},{\"Action\":[\"cloudformation:DescribeStackEvents\",\"cloudformation:GetTemplate\",\"cloudformation:DeleteStack\",\"cloudformation:UpdateTerminationProtection\",\"sts:GetCallerIdentity\",\"cloudformation:GetTemplateSummary\"],\"Resource\":\"*\",\"Effect\":\"Allow\",\"Sid\":\"CliPermissions\"},{\"Action\":[\"s3:GetObject*\",\"s3:GetBucket*\",\"s3:List*\"],\"Resource\":[\"arn:aws:s3:::cdk-hnb659fds-assets-393419659647-us-west-2\",\"arn:aws:s3:::cdk-hnb659fds-assets-393419659647-us-west-2/*\"],\"Effect\":\"Allow\",\"Sid\":\"CliStagingBucket\"},{\"Action\":[\"ssm:GetParameter\"],\"Resource\":[\"arn:aws:ssm:us-west-2:393419659647:parameter/cdk-bootstrap/hnb659fds/version\"],\"Effect\":\"Allow\",\"Sid\":\"ReadVersion\"}]}"
  }

  max_session_duration = "3600"
  name                 = "cdk-hnb659fds-deploy-role-393419659647-us-west-2"
  path                 = "/"

  tags = {
    "aws-cdk:bootstrap-role" = "deploy"
  }

  tags_all = {
    "aws-cdk:bootstrap-role" = "deploy"
  }
}

resource "aws_iam_role" "tfer--cdk-hnb659fds-file-publishing-role-393419659647-us-east-2" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "AWS": "arn:aws:iam::393419659647:root"
      }
    }
  ],
  "Version": "2008-10-17"
}
POLICY

  inline_policy {
    name   = "cdk-hnb659fds-file-publishing-role-default-policy-393419659647-us-east-2"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"s3:GetObject*\",\"s3:GetBucket*\",\"s3:GetEncryptionConfiguration\",\"s3:List*\",\"s3:DeleteObject*\",\"s3:PutObject*\",\"s3:Abort*\"],\"Resource\":[\"arn:aws:s3:::cdk-hnb659fds-assets-393419659647-us-east-2\",\"arn:aws:s3:::cdk-hnb659fds-assets-393419659647-us-east-2/*\"],\"Effect\":\"Allow\"},{\"Action\":[\"kms:Decrypt\",\"kms:DescribeKey\",\"kms:Encrypt\",\"kms:ReEncrypt*\",\"kms:GenerateDataKey*\"],\"Resource\":\"arn:aws:kms:us-east-2:393419659647:key/AWS_MANAGED_KEY\",\"Effect\":\"Allow\"}]}"
  }

  max_session_duration = "3600"
  name                 = "cdk-hnb659fds-file-publishing-role-393419659647-us-east-2"
  path                 = "/"

  tags = {
    "aws-cdk:bootstrap-role" = "file-publishing"
  }

  tags_all = {
    "aws-cdk:bootstrap-role" = "file-publishing"
  }
}

resource "aws_iam_role" "tfer--cdk-hnb659fds-file-publishing-role-393419659647-us-west-2" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "AWS": "arn:aws:iam::393419659647:root"
      }
    }
  ],
  "Version": "2008-10-17"
}
POLICY

  inline_policy {
    name   = "cdk-hnb659fds-file-publishing-role-default-policy-393419659647-us-west-2"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"s3:GetObject*\",\"s3:GetBucket*\",\"s3:GetEncryptionConfiguration\",\"s3:List*\",\"s3:DeleteObject*\",\"s3:PutObject*\",\"s3:Abort*\"],\"Resource\":[\"arn:aws:s3:::cdk-hnb659fds-assets-393419659647-us-west-2\",\"arn:aws:s3:::cdk-hnb659fds-assets-393419659647-us-west-2/*\"],\"Effect\":\"Allow\"},{\"Action\":[\"kms:Decrypt\",\"kms:DescribeKey\",\"kms:Encrypt\",\"kms:ReEncrypt*\",\"kms:GenerateDataKey*\"],\"Resource\":\"arn:aws:kms:us-west-2:393419659647:key/AWS_MANAGED_KEY\",\"Effect\":\"Allow\"}]}"
  }

  max_session_duration = "3600"
  name                 = "cdk-hnb659fds-file-publishing-role-393419659647-us-west-2"
  path                 = "/"

  tags = {
    "aws-cdk:bootstrap-role" = "file-publishing"
  }

  tags_all = {
    "aws-cdk:bootstrap-role" = "file-publishing"
  }
}

resource "aws_iam_role" "tfer--cdk-hnb659fds-image-publishing-role-393419659647-us-east-2" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "AWS": "arn:aws:iam::393419659647:root"
      }
    }
  ],
  "Version": "2008-10-17"
}
POLICY

  inline_policy {
    name   = "cdk-hnb659fds-image-publishing-role-default-policy-393419659647-us-east-2"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"ecr:PutImage\",\"ecr:InitiateLayerUpload\",\"ecr:UploadLayerPart\",\"ecr:CompleteLayerUpload\",\"ecr:BatchCheckLayerAvailability\",\"ecr:DescribeRepositories\",\"ecr:DescribeImages\",\"ecr:BatchGetImage\",\"ecr:GetDownloadUrlForLayer\"],\"Resource\":\"arn:aws:ecr:us-east-2:393419659647:repository/cdk-hnb659fds-container-assets-393419659647-us-east-2\",\"Effect\":\"Allow\"},{\"Action\":[\"ecr:GetAuthorizationToken\"],\"Resource\":\"*\",\"Effect\":\"Allow\"}]}"
  }

  max_session_duration = "3600"
  name                 = "cdk-hnb659fds-image-publishing-role-393419659647-us-east-2"
  path                 = "/"

  tags = {
    "aws-cdk:bootstrap-role" = "image-publishing"
  }

  tags_all = {
    "aws-cdk:bootstrap-role" = "image-publishing"
  }
}

resource "aws_iam_role" "tfer--cdk-hnb659fds-image-publishing-role-393419659647-us-west-2" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "AWS": "arn:aws:iam::393419659647:root"
      }
    }
  ],
  "Version": "2008-10-17"
}
POLICY

  inline_policy {
    name   = "cdk-hnb659fds-image-publishing-role-default-policy-393419659647-us-west-2"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"ecr:PutImage\",\"ecr:InitiateLayerUpload\",\"ecr:UploadLayerPart\",\"ecr:CompleteLayerUpload\",\"ecr:BatchCheckLayerAvailability\",\"ecr:DescribeRepositories\",\"ecr:DescribeImages\",\"ecr:BatchGetImage\",\"ecr:GetDownloadUrlForLayer\"],\"Resource\":\"arn:aws:ecr:us-west-2:393419659647:repository/cdk-hnb659fds-container-assets-393419659647-us-west-2\",\"Effect\":\"Allow\"},{\"Action\":[\"ecr:GetAuthorizationToken\"],\"Resource\":\"*\",\"Effect\":\"Allow\"}]}"
  }

  max_session_duration = "3600"
  name                 = "cdk-hnb659fds-image-publishing-role-393419659647-us-west-2"
  path                 = "/"

  tags = {
    "aws-cdk:bootstrap-role" = "image-publishing"
  }

  tags_all = {
    "aws-cdk:bootstrap-role" = "image-publishing"
  }
}

resource "aws_iam_role" "tfer--cdk-hnb659fds-lookup-role-393419659647-us-east-2" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "AWS": "arn:aws:iam::393419659647:root"
      }
    }
  ],
  "Version": "2008-10-17"
}
POLICY

  inline_policy {
    name   = "LookupRolePolicy"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"kms:Decrypt\"],\"Resource\":\"*\",\"Effect\":\"Deny\",\"Sid\":\"DontReadSecrets\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/ReadOnlyAccess"]
  max_session_duration = "3600"
  name                 = "cdk-hnb659fds-lookup-role-393419659647-us-east-2"
  path                 = "/"

  tags = {
    "aws-cdk:bootstrap-role" = "lookup"
  }

  tags_all = {
    "aws-cdk:bootstrap-role" = "lookup"
  }
}

resource "aws_iam_role" "tfer--cdk-hnb659fds-lookup-role-393419659647-us-west-2" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "AWS": "arn:aws:iam::393419659647:root"
      }
    }
  ],
  "Version": "2008-10-17"
}
POLICY

  inline_policy {
    name   = "LookupRolePolicy"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"kms:Decrypt\"],\"Resource\":\"*\",\"Effect\":\"Deny\",\"Sid\":\"DontReadSecrets\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/ReadOnlyAccess"]
  max_session_duration = "3600"
  name                 = "cdk-hnb659fds-lookup-role-393419659647-us-west-2"
  path                 = "/"

  tags = {
    "aws-cdk:bootstrap-role" = "lookup"
  }

  tags_all = {
    "aws-cdk:bootstrap-role" = "lookup"
  }
}

resource "aws_iam_role" "tfer--sandbox-CognitoUser-usw2" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Condition": {
        "ForAnyValue:StringLike": {
          "cognito-identity.amazonaws.com:amr": "authenticated"
        },
        "StringEquals": {
          "cognito-identity.amazonaws.com:aud": "us-west-2_XPCOGDlGl"
        }
      },
      "Effect": "Allow",
      "Principal": {
        "Federated": "cognito-identity.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  description          = "Default role for authenticated users"
  managed_policy_arns  = ["arn:aws:iam::aws:policy/AmazonS3FullAccess", "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"]
  max_session_duration = "3600"
  name                 = "sandbox-CognitoUser-usw2"
  path                 = "/"
}

resource "aws_iam_role" "tfer--sandbox-harbor-backend-us-sandboxAPIKeyAuthorizeru-J6G3ARPEMAB8" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "sandboxAPIKeyAuthorizerusw2ServiceRoleDefaultPolicy6CD3C6A3"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"dynamodb:BatchGetItem\",\"dynamodb:GetRecords\",\"dynamodb:GetShardIterator\",\"dynamodb:Query\",\"dynamodb:GetItem\",\"dynamodb:Scan\",\"dynamodb:ConditionCheckItem\",\"dynamodb:BatchWriteItem\",\"dynamodb:PutItem\",\"dynamodb:UpdateItem\",\"dynamodb:DeleteItem\",\"dynamodb:DescribeTable\"],\"Resource\":[\"arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "sandbox-harbor-backend-us-sandboxAPIKeyAuthorizeru-J6G3ARPEMAB8"
  path                 = "/"
}

resource "aws_iam_role" "tfer--sandbox-harbor-backend-us-sandboxHarborCodebaseLam-JV3C9SPDAZND" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "sandboxHarborCodebaseLambdausw2ServiceRoleDefaultPolicy3030D26E"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"cognito-idp:AdminDisableUser\",\"cognito-idp:AdminEnableUser\",\"cognito-idp:AdminGetUser\",\"cognito-idp:ListUsers\",\"cognito-idp:AdminUpdateUserAttributes\"],\"Resource\":\"*\",\"Effect\":\"Allow\"},{\"Action\":[\"dynamodb:BatchGetItem\",\"dynamodb:GetRecords\",\"dynamodb:GetShardIterator\",\"dynamodb:Query\",\"dynamodb:GetItem\",\"dynamodb:Scan\",\"dynamodb:ConditionCheckItem\",\"dynamodb:BatchWriteItem\",\"dynamodb:PutItem\",\"dynamodb:UpdateItem\",\"dynamodb:DeleteItem\",\"dynamodb:DescribeTable\"],\"Resource\":[\"arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "sandbox-harbor-backend-us-sandboxHarborCodebaseLam-JV3C9SPDAZND"
  path                 = "/"
}

resource "aws_iam_role" "tfer--sandbox-harbor-backend-us-sandboxHarborCodebasePOS-1KEMFXHFN45G9" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "sandboxHarborCodebasePOSTLambdausw2ServiceRoleDefaultPolicy83B38555"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"cognito-idp:AdminDisableUser\",\"cognito-idp:AdminEnableUser\",\"cognito-idp:AdminGetUser\",\"cognito-idp:ListUsers\",\"cognito-idp:AdminUpdateUserAttributes\"],\"Resource\":\"*\",\"Effect\":\"Allow\"},{\"Action\":[\"dynamodb:BatchGetItem\",\"dynamodb:GetRecords\",\"dynamodb:GetShardIterator\",\"dynamodb:Query\",\"dynamodb:GetItem\",\"dynamodb:Scan\",\"dynamodb:ConditionCheckItem\",\"dynamodb:BatchWriteItem\",\"dynamodb:PutItem\",\"dynamodb:UpdateItem\",\"dynamodb:DeleteItem\",\"dynamodb:DescribeTable\"],\"Resource\":[\"arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "sandbox-harbor-backend-us-sandboxHarborCodebasePOS-1KEMFXHFN45G9"
  path                 = "/"
}

resource "aws_iam_role" "tfer--sandbox-harbor-backend-us-sandboxHarborCodebasesLa-1GXA10VAOSWZ8" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "sandboxHarborCodebasesLambdausw2ServiceRoleDefaultPolicy4CCAF7F7"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"cognito-idp:AdminDisableUser\",\"cognito-idp:AdminEnableUser\",\"cognito-idp:AdminGetUser\",\"cognito-idp:ListUsers\",\"cognito-idp:AdminUpdateUserAttributes\"],\"Resource\":\"*\",\"Effect\":\"Allow\"},{\"Action\":[\"dynamodb:BatchGetItem\",\"dynamodb:GetRecords\",\"dynamodb:GetShardIterator\",\"dynamodb:Query\",\"dynamodb:GetItem\",\"dynamodb:Scan\",\"dynamodb:ConditionCheckItem\",\"dynamodb:BatchWriteItem\",\"dynamodb:PutItem\",\"dynamodb:UpdateItem\",\"dynamodb:DeleteItem\",\"dynamodb:DescribeTable\"],\"Resource\":[\"arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "sandbox-harbor-backend-us-sandboxHarborCodebasesLa-1GXA10VAOSWZ8"
  path                 = "/"
}

resource "aws_iam_role" "tfer--sandbox-harbor-backend-us-sandboxHarborMemberLambd-IX1FUENAP5PG" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "sandboxHarborMemberLambdausw2ServiceRoleDefaultPolicyD607E11E"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"cognito-idp:AdminDisableUser\",\"cognito-idp:AdminEnableUser\",\"cognito-idp:AdminGetUser\",\"cognito-idp:ListUsers\",\"cognito-idp:AdminUpdateUserAttributes\"],\"Resource\":\"*\",\"Effect\":\"Allow\"},{\"Action\":[\"dynamodb:BatchGetItem\",\"dynamodb:GetRecords\",\"dynamodb:GetShardIterator\",\"dynamodb:Query\",\"dynamodb:GetItem\",\"dynamodb:Scan\",\"dynamodb:ConditionCheckItem\",\"dynamodb:BatchWriteItem\",\"dynamodb:PutItem\",\"dynamodb:UpdateItem\",\"dynamodb:DeleteItem\",\"dynamodb:DescribeTable\"],\"Resource\":[\"arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "sandbox-harbor-backend-us-sandboxHarborMemberLambd-IX1FUENAP5PG"
  path                 = "/"
}

resource "aws_iam_role" "tfer--sandbox-harbor-backend-us-sandboxHarborMemberPOSTL-R9N72L26UAYS" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "sandboxHarborMemberPOSTLambdausw2ServiceRoleDefaultPolicy48D96B89"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"cognito-idp:AdminDisableUser\",\"cognito-idp:AdminEnableUser\",\"cognito-idp:AdminGetUser\",\"cognito-idp:ListUsers\",\"cognito-idp:AdminUpdateUserAttributes\"],\"Resource\":\"*\",\"Effect\":\"Allow\"},{\"Action\":[\"dynamodb:BatchGetItem\",\"dynamodb:GetRecords\",\"dynamodb:GetShardIterator\",\"dynamodb:Query\",\"dynamodb:GetItem\",\"dynamodb:Scan\",\"dynamodb:ConditionCheckItem\",\"dynamodb:BatchWriteItem\",\"dynamodb:PutItem\",\"dynamodb:UpdateItem\",\"dynamodb:DeleteItem\",\"dynamodb:DescribeTable\"],\"Resource\":[\"arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "sandbox-harbor-backend-us-sandboxHarborMemberPOSTL-R9N72L26UAYS"
  path                 = "/"
}

resource "aws_iam_role" "tfer--sandbox-harbor-backend-us-sandboxHarborMembersLamb-1I0LXSVO9QGWI" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "sandboxHarborMembersLambdausw2ServiceRoleDefaultPolicy38CDE6CC"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"cognito-idp:AdminDisableUser\",\"cognito-idp:AdminEnableUser\",\"cognito-idp:AdminGetUser\",\"cognito-idp:ListUsers\",\"cognito-idp:AdminUpdateUserAttributes\"],\"Resource\":\"*\",\"Effect\":\"Allow\"},{\"Action\":[\"dynamodb:BatchGetItem\",\"dynamodb:GetRecords\",\"dynamodb:GetShardIterator\",\"dynamodb:Query\",\"dynamodb:GetItem\",\"dynamodb:Scan\",\"dynamodb:ConditionCheckItem\",\"dynamodb:BatchWriteItem\",\"dynamodb:PutItem\",\"dynamodb:UpdateItem\",\"dynamodb:DeleteItem\",\"dynamodb:DescribeTable\"],\"Resource\":[\"arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "sandbox-harbor-backend-us-sandboxHarborMembersLamb-1I0LXSVO9QGWI"
  path                 = "/"
}

resource "aws_iam_role" "tfer--sandbox-harbor-backend-us-sandboxHarborProjectLamb-DXYP13V3MR7Y" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "sandboxHarborProjectLambdausw2ServiceRoleDefaultPolicyD03AB49F"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"cognito-idp:AdminDisableUser\",\"cognito-idp:AdminEnableUser\",\"cognito-idp:AdminGetUser\",\"cognito-idp:ListUsers\",\"cognito-idp:AdminUpdateUserAttributes\"],\"Resource\":\"*\",\"Effect\":\"Allow\"},{\"Action\":[\"dynamodb:BatchGetItem\",\"dynamodb:GetRecords\",\"dynamodb:GetShardIterator\",\"dynamodb:Query\",\"dynamodb:GetItem\",\"dynamodb:Scan\",\"dynamodb:ConditionCheckItem\",\"dynamodb:BatchWriteItem\",\"dynamodb:PutItem\",\"dynamodb:UpdateItem\",\"dynamodb:DeleteItem\",\"dynamodb:DescribeTable\"],\"Resource\":[\"arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "sandbox-harbor-backend-us-sandboxHarborProjectLamb-DXYP13V3MR7Y"
  path                 = "/"
}

resource "aws_iam_role" "tfer--sandbox-harbor-backend-us-sandboxHarborProjectPOST-NCR778JYL4FJ" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "sandboxHarborProjectPOSTLambdausw2ServiceRoleDefaultPolicy3843939F"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"cognito-idp:AdminDisableUser\",\"cognito-idp:AdminEnableUser\",\"cognito-idp:AdminGetUser\",\"cognito-idp:ListUsers\",\"cognito-idp:AdminUpdateUserAttributes\"],\"Resource\":\"*\",\"Effect\":\"Allow\"},{\"Action\":[\"dynamodb:BatchGetItem\",\"dynamodb:GetRecords\",\"dynamodb:GetShardIterator\",\"dynamodb:Query\",\"dynamodb:GetItem\",\"dynamodb:Scan\",\"dynamodb:ConditionCheckItem\",\"dynamodb:BatchWriteItem\",\"dynamodb:PutItem\",\"dynamodb:UpdateItem\",\"dynamodb:DeleteItem\",\"dynamodb:DescribeTable\"],\"Resource\":[\"arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "sandbox-harbor-backend-us-sandboxHarborProjectPOST-NCR778JYL4FJ"
  path                 = "/"
}

resource "aws_iam_role" "tfer--sandbox-harbor-backend-us-sandboxHarborProjectsLam-IVSYK5W51ZY7" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "sandboxHarborProjectsLambdausw2ServiceRoleDefaultPolicy8C0DDC62"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"cognito-idp:AdminDisableUser\",\"cognito-idp:AdminEnableUser\",\"cognito-idp:AdminGetUser\",\"cognito-idp:ListUsers\",\"cognito-idp:AdminUpdateUserAttributes\"],\"Resource\":\"*\",\"Effect\":\"Allow\"},{\"Action\":[\"dynamodb:BatchGetItem\",\"dynamodb:GetRecords\",\"dynamodb:GetShardIterator\",\"dynamodb:Query\",\"dynamodb:GetItem\",\"dynamodb:Scan\",\"dynamodb:ConditionCheckItem\",\"dynamodb:BatchWriteItem\",\"dynamodb:PutItem\",\"dynamodb:UpdateItem\",\"dynamodb:DeleteItem\",\"dynamodb:DescribeTable\"],\"Resource\":[\"arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "sandbox-harbor-backend-us-sandboxHarborProjectsLam-IVSYK5W51ZY7"
  path                 = "/"
}

resource "aws_iam_role" "tfer--sandbox-harbor-backend-us-sandboxHarborTeamLambdau-1SUMZV3BOCS2O" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "sandboxHarborTeamLambdausw2ServiceRoleDefaultPolicyFFFA7801"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"cognito-idp:AdminDisableUser\",\"cognito-idp:AdminEnableUser\",\"cognito-idp:AdminGetUser\",\"cognito-idp:ListUsers\",\"cognito-idp:AdminUpdateUserAttributes\"],\"Resource\":\"*\",\"Effect\":\"Allow\"},{\"Action\":[\"dynamodb:BatchGetItem\",\"dynamodb:GetRecords\",\"dynamodb:GetShardIterator\",\"dynamodb:Query\",\"dynamodb:GetItem\",\"dynamodb:Scan\",\"dynamodb:ConditionCheckItem\",\"dynamodb:BatchWriteItem\",\"dynamodb:PutItem\",\"dynamodb:UpdateItem\",\"dynamodb:DeleteItem\",\"dynamodb:DescribeTable\"],\"Resource\":[\"arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "sandbox-harbor-backend-us-sandboxHarborTeamLambdau-1SUMZV3BOCS2O"
  path                 = "/"
}

resource "aws_iam_role" "tfer--sandbox-harbor-backend-us-sandboxHarborTeamPOSTLam-1M9DCBMYMUTZ9" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "sandboxHarborTeamPOSTLambdausw2ServiceRoleDefaultPolicy2DD1D4F7"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"cognito-idp:AdminDisableUser\",\"cognito-idp:AdminEnableUser\",\"cognito-idp:AdminGetUser\",\"cognito-idp:ListUsers\",\"cognito-idp:AdminUpdateUserAttributes\"],\"Resource\":\"*\",\"Effect\":\"Allow\"},{\"Action\":[\"dynamodb:BatchGetItem\",\"dynamodb:GetRecords\",\"dynamodb:GetShardIterator\",\"dynamodb:Query\",\"dynamodb:GetItem\",\"dynamodb:Scan\",\"dynamodb:ConditionCheckItem\",\"dynamodb:BatchWriteItem\",\"dynamodb:PutItem\",\"dynamodb:UpdateItem\",\"dynamodb:DeleteItem\",\"dynamodb:DescribeTable\"],\"Resource\":[\"arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "sandbox-harbor-backend-us-sandboxHarborTeamPOSTLam-1M9DCBMYMUTZ9"
  path                 = "/"
}

resource "aws_iam_role" "tfer--sandbox-harbor-backend-us-sandboxHarborTeamsLambda-97ORIE58LYQK" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "sandboxHarborTeamsLambdausw2ServiceRoleDefaultPolicyD0FADD25"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"cognito-idp:AdminDisableUser\",\"cognito-idp:AdminEnableUser\",\"cognito-idp:AdminGetUser\",\"cognito-idp:ListUsers\",\"cognito-idp:AdminUpdateUserAttributes\"],\"Resource\":\"*\",\"Effect\":\"Allow\"},{\"Action\":[\"dynamodb:BatchGetItem\",\"dynamodb:GetRecords\",\"dynamodb:GetShardIterator\",\"dynamodb:Query\",\"dynamodb:GetItem\",\"dynamodb:Scan\",\"dynamodb:ConditionCheckItem\",\"dynamodb:BatchWriteItem\",\"dynamodb:PutItem\",\"dynamodb:UpdateItem\",\"dynamodb:DeleteItem\",\"dynamodb:DescribeTable\"],\"Resource\":[\"arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "sandbox-harbor-backend-us-sandboxHarborTeamsLambda-97ORIE58LYQK"
  path                 = "/"
}

resource "aws_iam_role" "tfer--sandbox-harbor-backend-us-sandboxHarborTokenLambda-4J703PBNIS81" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "sandboxHarborTokenLambdausw2ServiceRoleDefaultPolicyA589C9EC"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"cognito-idp:AdminDisableUser\",\"cognito-idp:AdminEnableUser\",\"cognito-idp:AdminGetUser\",\"cognito-idp:ListUsers\",\"cognito-idp:AdminUpdateUserAttributes\"],\"Resource\":\"*\",\"Effect\":\"Allow\"},{\"Action\":[\"dynamodb:BatchGetItem\",\"dynamodb:GetRecords\",\"dynamodb:GetShardIterator\",\"dynamodb:Query\",\"dynamodb:GetItem\",\"dynamodb:Scan\",\"dynamodb:ConditionCheckItem\",\"dynamodb:BatchWriteItem\",\"dynamodb:PutItem\",\"dynamodb:UpdateItem\",\"dynamodb:DeleteItem\",\"dynamodb:DescribeTable\"],\"Resource\":[\"arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "sandbox-harbor-backend-us-sandboxHarborTokenLambda-4J703PBNIS81"
  path                 = "/"
}

resource "aws_iam_role" "tfer--sandbox-harbor-backend-us-sandboxHarborTokenPOSTLa-1WNCMTFZ75DLF" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "sandboxHarborTokenPOSTLambdausw2ServiceRoleDefaultPolicy8BDAC0DF"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"cognito-idp:AdminDisableUser\",\"cognito-idp:AdminEnableUser\",\"cognito-idp:AdminGetUser\",\"cognito-idp:ListUsers\",\"cognito-idp:AdminUpdateUserAttributes\"],\"Resource\":\"*\",\"Effect\":\"Allow\"},{\"Action\":[\"dynamodb:BatchGetItem\",\"dynamodb:GetRecords\",\"dynamodb:GetShardIterator\",\"dynamodb:Query\",\"dynamodb:GetItem\",\"dynamodb:Scan\",\"dynamodb:ConditionCheckItem\",\"dynamodb:BatchWriteItem\",\"dynamodb:PutItem\",\"dynamodb:UpdateItem\",\"dynamodb:DeleteItem\",\"dynamodb:DescribeTable\"],\"Resource\":[\"arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "sandbox-harbor-backend-us-sandboxHarborTokenPOSTLa-1WNCMTFZ75DLF"
  path                 = "/"
}

resource "aws_iam_role" "tfer--sandbox-harbor-backend-us-sandboxHarborTokensLambd-3BH0LJLJN4F6" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "sandboxHarborTokensLambdausw2ServiceRoleDefaultPolicyA587B751"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"cognito-idp:AdminDisableUser\",\"cognito-idp:AdminEnableUser\",\"cognito-idp:AdminGetUser\",\"cognito-idp:ListUsers\",\"cognito-idp:AdminUpdateUserAttributes\"],\"Resource\":\"*\",\"Effect\":\"Allow\"},{\"Action\":[\"dynamodb:BatchGetItem\",\"dynamodb:GetRecords\",\"dynamodb:GetShardIterator\",\"dynamodb:Query\",\"dynamodb:GetItem\",\"dynamodb:Scan\",\"dynamodb:ConditionCheckItem\",\"dynamodb:BatchWriteItem\",\"dynamodb:PutItem\",\"dynamodb:UpdateItem\",\"dynamodb:DeleteItem\",\"dynamodb:DescribeTable\"],\"Resource\":[\"arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "sandbox-harbor-backend-us-sandboxHarborTokensLambd-3BH0LJLJN4F6"
  path                 = "/"
}

resource "aws_iam_role" "tfer--sandbox-harbor-backend-us-sandboxJwtTokenAuthorize-1MM2C42NKJCYH" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "sandboxJwtTokenAuthorizerusw2ServiceRoleDefaultPolicyF29C585C"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"cognito-idp:AdminDisableUser\",\"cognito-idp:AdminEnableUser\",\"cognito-idp:AdminGetUser\",\"cognito-idp:ListUsers\"],\"Resource\":\"*\",\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "sandbox-harbor-backend-us-sandboxJwtTokenAuthorize-1MM2C42NKJCYH"
  path                 = "/"
}

resource "aws_iam_role" "tfer--sandbox-harbor-backend-us-sandboxLoginusw2ServiceR-D69CUROSBKHL" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "sandboxLoginusw2ServiceRoleDefaultPolicy6E70030D"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"cognito-idp:AdminGetUser\",\"cognito-idp:AdminEnableUser\",\"cognito-idp:AdminDisableUser\",\"cognito-idp:AdminInitiateAuth\"],\"Resource\":\"*\",\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "sandbox-harbor-backend-us-sandboxLoginusw2ServiceR-D69CUROSBKHL"
  path                 = "/"
}

resource "aws_iam_role" "tfer--sandbox-harbor-backend-us-sandboxSBOMIngressusw2Se-WRDZ6YKTOSC0" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "sandboxSBOMIngressusw2ServiceRoleDefaultPolicyBEBAC792"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"s3:PutObject\",\"s3:PutObjectLegalHold\",\"s3:PutObjectRetention\",\"s3:PutObjectTagging\",\"s3:PutObjectVersionTagging\",\"s3:Abort*\"],\"Resource\":\"arn:aws:s3:::sandbox-harbor-sbom-uploads-enrichment-393419659647-usw2/*\",\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "sandbox-harbor-backend-us-sandboxSBOMIngressusw2Se-WRDZ6YKTOSC0"
  path                 = "/"
}

resource "aws_iam_role" "tfer--sandbox-harbor-backend-us-sandboxUserSearchusw2Ser-EZE8AIWUFWN7" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "sandboxUserSearchusw2ServiceRoleDefaultPolicy4FFCD120"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":\"cognito-idp:ListUsers\",\"Resource\":\"*\",\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "sandbox-harbor-backend-us-sandboxUserSearchusw2Ser-EZE8AIWUFWN7"
  path                 = "/"
}

resource "aws_iam_role" "tfer--sandbox-harbor-enrichment-BucketNotificationsHandl-1HERPSPAMUYO4" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "BucketNotificationsHandler050a0587b7544547bf325f094a3db834RoleDefaultPolicy2CF63D36"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":\"s3:PutBucketNotification\",\"Resource\":\"*\",\"Effect\":\"Allow\"},{\"Action\":\"s3:GetBucketNotification\",\"Resource\":\"*\",\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"]
  max_session_duration = "3600"
  name                 = "sandbox-harbor-enrichment-BucketNotificationsHandl-1HERPSPAMUYO4"
  path                 = "/"
}

resource "aws_iam_role" "tfer--sandbox-harbor-enrichment-ENRICHMENTSTATEMACHINEEv-1DCSHZUSIB298" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "events.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "ENRICHMENTSTATEMACHINEEventsRoleDefaultPolicyC7BD5AE4"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":\"states:StartExecution\",\"Resource\":\"arn:aws:states:us-west-2:393419659647:stateMachine:sandbox_Enrichment_usw2\",\"Effect\":\"Allow\"}]}"
  }

  max_session_duration = "3600"
  name                 = "sandbox-harbor-enrichment-ENRICHMENTSTATEMACHINEEv-1DCSHZUSIB298"
  path                 = "/"
}

resource "aws_iam_role" "tfer--sandbox-harbor-enrichment-ENRICHMENTSTATEMACHINERo-1TKL415Y7O63T" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "states.us-west-2.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "ENRICHMENTSTATEMACHINERoleDefaultPolicyA998D563"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":\"lambda:InvokeFunction\",\"Resource\":[\"arn:aws:lambda:us-west-2:393419659647:function:sandbox_Summarizer_usw2\",\"arn:aws:lambda:us-west-2:393419659647:function:sandbox_Summarizer_usw2:*\"],\"Effect\":\"Allow\"},{\"Action\":\"lambda:InvokeFunction\",\"Resource\":[\"arn:aws:lambda:us-west-2:393419659647:function:sandbox_DependencyTrackInterface_usw2\",\"arn:aws:lambda:us-west-2:393419659647:function:sandbox_DependencyTrackInterface_usw2:*\"],\"Effect\":\"Allow\"},{\"Action\":\"lambda:InvokeFunction\",\"Resource\":[\"arn:aws:lambda:us-west-2:393419659647:function:sandbox_IonChannelInterface_usw2\",\"arn:aws:lambda:us-west-2:393419659647:function:sandbox_IonChannelInterface_usw2:*\"],\"Effect\":\"Allow\"}]}"
  }

  max_session_duration = "3600"
  name                 = "sandbox-harbor-enrichment-ENRICHMENTSTATEMACHINERo-1TKL415Y7O63T"
  path                 = "/"
}

resource "aws_iam_role" "tfer--sandbox-harbor-enrichment-HarborFargateClusterDepe-FD1S9ZI389P3" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "ecs-tasks.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  max_session_duration = "3600"
  name                 = "sandbox-harbor-enrichment-HarborFargateClusterDepe-FD1S9ZI389P3"
  path                 = "/"
}

resource "aws_iam_role" "tfer--sandbox-harbor-enrichment-HarborFargateClusterDepe-IKOTZ3G2IJRS" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "ecs-tasks.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "HarborFargateClusterDependencyTrackTaskDefinitionExecutionRoleDefaultPolicy8425CBAD"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"logs:CreateLogStream\",\"logs:PutLogEvents\"],\"Resource\":\"arn:aws:logs:us-west-2:393419659647:log-group:sandbox-harbor-enrichment-usw2-HarborFargateClusterDependencyTrackTaskDefinitiondtContainerLogGroup7E08ADFB-PYqt9clwLgx5:*\",\"Effect\":\"Allow\"}]}"
  }

  max_session_duration = "3600"
  name                 = "sandbox-harbor-enrichment-HarborFargateClusterDepe-IKOTZ3G2IJRS"
  path                 = "/"
}

resource "aws_iam_role" "tfer--sandbox-harbor-enrichment-sandboxDependencyTrackIn-16OFNW74A35SH" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "sandboxDependencyTrackInterfaceusw2ServiceRoleDefaultPolicyFA87F6A0"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":\"events:PutEvents\",\"Resource\":\"arn:aws:events:us-west-2:393419659647:event-bus/sandbox-HarborEnrichments-usw2\",\"Effect\":\"Allow\"},{\"Action\":[\"s3:PutObject\",\"s3:PutObjectLegalHold\",\"s3:PutObjectRetention\",\"s3:PutObjectTagging\",\"s3:PutObjectVersionTagging\",\"s3:Abort*\"],\"Resource\":\"arn:aws:s3:::sandbox-harbor-sbom-uploads-enrichment-393419659647-usw2/*\",\"Effect\":\"Allow\"},{\"Action\":[\"s3:GetObject*\",\"s3:GetBucket*\",\"s3:List*\",\"s3:DeleteObject*\",\"s3:PutObject\",\"s3:PutObjectLegalHold\",\"s3:PutObjectRetention\",\"s3:PutObjectTagging\",\"s3:PutObjectVersionTagging\",\"s3:Abort*\"],\"Resource\":[\"arn:aws:s3:::sandbox-harbor-sbom-uploads-enrichment-393419659647-usw2\",\"arn:aws:s3:::sandbox-harbor-sbom-uploads-enrichment-393419659647-usw2/*\"],\"Effect\":\"Allow\"},{\"Action\":[\"ssm:DescribeParameters\",\"ssm:GetParameters\",\"ssm:GetParameter\",\"ssm:GetParameterHistory\"],\"Resource\":\"arn:aws:ssm:us-west-2:393419659647:parameter/sandbox_DT_ROOT_PWD_usw2\",\"Effect\":\"Allow\"},{\"Action\":\"ssm:PutParameter\",\"Resource\":\"arn:aws:ssm:us-west-2:393419659647:parameter/sandbox_DT_ROOT_PWD_usw2\",\"Effect\":\"Allow\"},{\"Action\":[\"ssm:DescribeParameters\",\"ssm:GetParameters\",\"ssm:GetParameter\",\"ssm:GetParameterHistory\"],\"Resource\":\"arn:aws:ssm:us-west-2:393419659647:parameter/sandbox_DT_API_KEY_usw2\",\"Effect\":\"Allow\"},{\"Action\":\"ssm:PutParameter\",\"Resource\":\"arn:aws:ssm:us-west-2:393419659647:parameter/sandbox_DT_API_KEY_usw2\",\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "sandbox-harbor-enrichment-sandboxDependencyTrackIn-16OFNW74A35SH"
  path                 = "/"
}

resource "aws_iam_role" "tfer--sandbox-harbor-enrichment-sandboxIonChannelInterfa-1XZPOHEU8TXD2" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "sandboxIonChannelInterfaceusw2ServiceRoleDefaultPolicyC8A0B4DF"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":\"events:PutEvents\",\"Resource\":\"arn:aws:events:us-west-2:393419659647:event-bus/sandbox-HarborEnrichments-usw2\",\"Effect\":\"Allow\"},{\"Action\":[\"s3:PutObject\",\"s3:PutObjectLegalHold\",\"s3:PutObjectRetention\",\"s3:PutObjectTagging\",\"s3:PutObjectVersionTagging\",\"s3:Abort*\"],\"Resource\":\"arn:aws:s3:::sandbox-harbor-sbom-uploads-enrichment-393419659647-usw2/*\",\"Effect\":\"Allow\"},{\"Action\":[\"s3:GetObject*\",\"s3:GetBucket*\",\"s3:List*\",\"s3:DeleteObject*\",\"s3:PutObject\",\"s3:PutObjectLegalHold\",\"s3:PutObjectRetention\",\"s3:PutObjectTagging\",\"s3:PutObjectVersionTagging\",\"s3:Abort*\"],\"Resource\":[\"arn:aws:s3:::sandbox-harbor-sbom-uploads-enrichment-393419659647-usw2\",\"arn:aws:s3:::sandbox-harbor-sbom-uploads-enrichment-393419659647-usw2/*\"],\"Effect\":\"Allow\"},{\"Action\":[\"ssm:DescribeParameters\",\"ssm:GetParameters\",\"ssm:GetParameter\",\"ssm:GetParameterHistory\"],\"Resource\":\"arn:aws:ssm:us-west-2:393419659647:parameter/sandbox_IC_API_KEY_usw2\",\"Effect\":\"Allow\"},{\"Action\":[\"ssm:DescribeParameters\",\"ssm:GetParameters\",\"ssm:GetParameter\",\"ssm:GetParameterHistory\"],\"Resource\":\"arn:aws:ssm:us-west-2:393419659647:parameter/sandbox_IC_API_BASE_usw2\",\"Effect\":\"Allow\"},{\"Action\":[\"ssm:DescribeParameters\",\"ssm:GetParameters\",\"ssm:GetParameter\",\"ssm:GetParameterHistory\"],\"Resource\":\"arn:aws:ssm:us-west-2:393419659647:parameter/sandbox_IC_RULESET_TEAM_ID_usw2\",\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "sandbox-harbor-enrichment-sandboxIonChannelInterfa-1XZPOHEU8TXD2"
  path                 = "/"
}

resource "aws_iam_role" "tfer--sandbox-harbor-enrichment-sandboxSBOMEnrichmentIng-1EZW673RQD559" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "sandboxSBOMEnrichmentIngressusw2ServiceRoleDefaultPolicyB2F4F671"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"s3:GetObject*\",\"s3:GetBucket*\",\"s3:List*\"],\"Resource\":[\"arn:aws:s3:::sandbox-harbor-sbom-uploads-enrichment-393419659647-usw2\",\"arn:aws:s3:::sandbox-harbor-sbom-uploads-enrichment-393419659647-usw2/*\"],\"Effect\":\"Allow\"},{\"Action\":\"events:PutEvents\",\"Resource\":\"arn:aws:events:us-west-2:393419659647:event-bus/sandbox-HarborEnrichments-usw2\",\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "sandbox-harbor-enrichment-sandboxSBOMEnrichmentIng-1EZW673RQD559"
  path                 = "/"
}

resource "aws_iam_role" "tfer--sandbox-harbor-enrichment-sandboxSummarizerusw2Ser-N73UKG5RFO78" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "sandboxSummarizerusw2ServiceRoleDefaultPolicy007FC4FF"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":\"events:PutEvents\",\"Resource\":\"arn:aws:events:us-west-2:393419659647:event-bus/sandbox-HarborEnrichments-usw2\",\"Effect\":\"Allow\"},{\"Action\":[\"s3:PutObject\",\"s3:PutObjectLegalHold\",\"s3:PutObjectRetention\",\"s3:PutObjectTagging\",\"s3:PutObjectVersionTagging\",\"s3:Abort*\"],\"Resource\":\"arn:aws:s3:::sandbox-harbor-sbom-uploads-enrichment-393419659647-usw2/*\",\"Effect\":\"Allow\"},{\"Action\":[\"s3:GetObject*\",\"s3:GetBucket*\",\"s3:List*\",\"s3:DeleteObject*\",\"s3:PutObject\",\"s3:PutObjectLegalHold\",\"s3:PutObjectRetention\",\"s3:PutObjectTagging\",\"s3:PutObjectVersionTagging\",\"s3:Abort*\"],\"Resource\":[\"arn:aws:s3:::sandbox-harbor-sbom-uploads-enrichment-393419659647-usw2\",\"arn:aws:s3:::sandbox-harbor-sbom-uploads-enrichment-393419659647-usw2/*\"],\"Effect\":\"Allow\"},{\"Action\":[\"dynamodb:BatchGetItem\",\"dynamodb:GetRecords\",\"dynamodb:GetShardIterator\",\"dynamodb:Query\",\"dynamodb:GetItem\",\"dynamodb:Scan\",\"dynamodb:ConditionCheckItem\",\"dynamodb:BatchWriteItem\",\"dynamodb:PutItem\",\"dynamodb:UpdateItem\",\"dynamodb:DeleteItem\",\"dynamodb:DescribeTable\"],\"Resource\":[\"arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2\"],\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole", "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"]
  max_session_duration = "3600"
  name                 = "sandbox-harbor-enrichment-sandboxSummarizerusw2Ser-N73UKG5RFO78"
  path                 = "/"
}

resource "aws_iam_role" "tfer--sandbox-harbor-pilot-usw2-sandboxPilotusw2ServiceR-MSTMBB1ZMJQY" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"]
  max_session_duration = "3600"
  name                 = "sandbox-harbor-pilot-usw2-sandboxPilotusw2ServiceR-MSTMBB1ZMJQY"
  path                 = "/"
}

resource "aws_iam_role" "tfer--sandbox-harbor-shared-res-BucketNotificationsHandl-1OE2Y4QUIM3MH" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "BucketNotificationsHandler050a0587b7544547bf325f094a3db834RoleDefaultPolicy2CF63D36"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":\"s3:PutBucketNotification\",\"Resource\":\"*\",\"Effect\":\"Allow\"}]}"
  }

  managed_policy_arns  = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"]
  max_session_duration = "3600"
  name                 = "sandbox-harbor-shared-res-BucketNotificationsHandl-1OE2Y4QUIM3MH"
  path                 = "/"
}

resource "aws_iam_role" "tfer--sandbox-harbor-shared-reso-ReplicationRoleCE149CEC-T5KI4MILWWFJ" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "s3.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  inline_policy {
    name   = "ReplicationRoleDefaultPolicy80AD15BB"
    policy = "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"s3:GetReplicationConfiguration\",\"s3:ListBucket\"],\"Resource\":\"arn:aws:s3:::sandbox-harbor-sbom-uploads-enrichment-393419659647-usw2\",\"Effect\":\"Allow\"},{\"Action\":[\"s3:GetObjectVersion\",\"s3:GetObjectVersionAcl\",\"s3:GetObjectVersionForReplication\",\"s3:GetObjectLegalHold\",\"s3:GetObjectVersionTagging\",\"s3:GetObjectRetention\"],\"Resource\":\"arn:aws:s3:::sandbox-harbor-sbom-uploads-enrichment-393419659647-usw2/*\",\"Effect\":\"Allow\"},{\"Action\":[\"s3:ReplicateObject\",\"s3:ReplicateDelete\",\"s3:ReplicateTags\",\"s3:GetObjectVersionTagging\",\"s3:ObjectOwnerOverrideToBucketOwner\"],\"Resource\":\"arn:aws:s3:::dev-harbor-sbom-summary-share-557147098836-use1/*\",\"Effect\":\"Allow\"}]}"
  }

  max_session_duration = "3600"
  name                 = "sandbox-harbor-shared-reso-ReplicationRoleCE149CEC-T5KI4MILWWFJ"
  path                 = "/service-role/"
}

resource "aws_iam_role" "tfer--stacksets-exec-d760af5b917175709402d1ec524a2012" {
  assume_role_policy = <<POLICY
{
  "Id": "stacksets-exec-d760af5b917175709402d1ec524a2012-assume-role-policy",
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "AWS": "arn:aws:iam::803783872674:role/aws-service-role/stacksets.cloudformation.amazonaws.com/AWSServiceRoleForCloudFormationStackSetsOrgAdmin"
      },
      "Sid": "1"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  description          = "Role created by AWSCloudFormation StackSets"
  managed_policy_arns  = ["arn:aws:iam::aws:policy/AdministratorAccess"]
  max_session_duration = "3600"
  name                 = "stacksets-exec-d760af5b917175709402d1ec524a2012"
  name_prefix          = "stacksets-exec-d760af"
  path                 = "/"
}
