resource "aws_iam_role_policy" "tfer--SBOM-Management-Api-APIKeyAuthorizerServiceRole4E8-IBAT5I6C9PNY_APIKeyAuthorizerServiceRoleDefaultPolicyB4371CAE" {
  name = "APIKeyAuthorizerServiceRoleDefaultPolicyB4371CAE"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "dynamodb:BatchGetItem",
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:Query",
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:ConditionCheckItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:DescribeTable"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:dynamodb:us-east-2:393419659647:table/HarborTeamsTable"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "SBOM-Management-Api-APIKeyAuthorizerServiceRole4E8-IBAT5I6C9PNY"
}

resource "aws_iam_role_policy" "tfer--SBOM-Management-Api-LoginLambdaServiceRoleBF816334-W7XUB9TKESGU_LoginLambdaServiceRoleDefaultPolicyFF0411F9" {
  name = "LoginLambdaServiceRoleDefaultPolicyFF0411F9"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "cognito-idp:AdminGetUser",
        "cognito-idp:AdminEnableUser",
        "cognito-idp:AdminDisableUser",
        "cognito-idp:AdminInitiateAuth"
      ],
      "Effect": "Allow",
      "Resource": "*"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "SBOM-Management-Api-LoginLambdaServiceRoleBF816334-W7XUB9TKESGU"
}

resource "aws_iam_role_policy" "tfer--SBOM-Management-Api-SBOMAPIAuthorizerAuthorizerSer-LP0PPL9842PZ_SBOMAPIAuthorizerAuthorizerServiceRoleDefaultPolicy14415F12" {
  name = "SBOMAPIAuthorizerAuthorizerServiceRoleDefaultPolicy14415F12"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "cognito-idp:AdminDisableUser",
        "cognito-idp:AdminEnableUser",
        "cognito-idp:AdminGetUser",
        "cognito-idp:ListUsers"
      ],
      "Effect": "Allow",
      "Resource": "*"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "SBOM-Management-Api-SBOMAPIAuthorizerAuthorizerSer-LP0PPL9842PZ"
}

resource "aws_iam_role_policy" "tfer--SBOM-Management-Api-SBOMHarborCodebaseLambdaServic-FNMSV0WLE7SY_SBOMHarborCodebaseLambdaServiceRoleDefaultPolicy941BCA2E" {
  name = "SBOMHarborCodebaseLambdaServiceRoleDefaultPolicy941BCA2E"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "dynamodb:BatchGetItem",
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:Query",
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:ConditionCheckItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:DescribeTable"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:dynamodb:us-east-2:393419659647:table/HarborTeamsTable"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "SBOM-Management-Api-SBOMHarborCodebaseLambdaServic-FNMSV0WLE7SY"
}

resource "aws_iam_role_policy" "tfer--SBOM-Management-Api-SBOMHarborCodebasePOSTLambdaSe-WTFVTQUCTUWN_SBOMHarborCodebasePOSTLambdaServiceRoleDefaultPolicyBFDE5298" {
  name = "SBOMHarborCodebasePOSTLambdaServiceRoleDefaultPolicyBFDE5298"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "dynamodb:BatchGetItem",
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:Query",
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:ConditionCheckItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:DescribeTable"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:dynamodb:us-east-2:393419659647:table/HarborTeamsTable"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "SBOM-Management-Api-SBOMHarborCodebasePOSTLambdaSe-WTFVTQUCTUWN"
}

resource "aws_iam_role_policy" "tfer--SBOM-Management-Api-SBOMHarborCodebasesLambdaServi-5CJU7F7V2602_SBOMHarborCodebasesLambdaServiceRoleDefaultPolicy4394E45F" {
  name = "SBOMHarborCodebasesLambdaServiceRoleDefaultPolicy4394E45F"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "dynamodb:BatchGetItem",
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:Query",
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:ConditionCheckItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:DescribeTable"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:dynamodb:us-east-2:393419659647:table/HarborTeamsTable"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "SBOM-Management-Api-SBOMHarborCodebasesLambdaServi-5CJU7F7V2602"
}

resource "aws_iam_role_policy" "tfer--SBOM-Management-Api-SBOMHarborMemberLambdaServiceR-1SE719MXDVF9V_SBOMHarborMemberLambdaServiceRoleDefaultPolicy9DBA396F" {
  name = "SBOMHarborMemberLambdaServiceRoleDefaultPolicy9DBA396F"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "dynamodb:BatchGetItem",
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:Query",
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:ConditionCheckItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:DescribeTable"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:dynamodb:us-east-2:393419659647:table/HarborTeamsTable"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "SBOM-Management-Api-SBOMHarborMemberLambdaServiceR-1SE719MXDVF9V"
}

resource "aws_iam_role_policy" "tfer--SBOM-Management-Api-SBOMHarborMemberPOSTLambdaServ-146TE0NUMMGBI_SBOMHarborMemberPOSTLambdaServiceRoleDefaultPolicy87BF556E" {
  name = "SBOMHarborMemberPOSTLambdaServiceRoleDefaultPolicy87BF556E"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "dynamodb:BatchGetItem",
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:Query",
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:ConditionCheckItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:DescribeTable"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:dynamodb:us-east-2:393419659647:table/HarborTeamsTable"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "SBOM-Management-Api-SBOMHarborMemberPOSTLambdaServ-146TE0NUMMGBI"
}

resource "aws_iam_role_policy" "tfer--SBOM-Management-Api-SBOMHarborMembersLambdaService-19LN7SS31SOV6_SBOMHarborMembersLambdaServiceRoleDefaultPolicy463F955A" {
  name = "SBOMHarborMembersLambdaServiceRoleDefaultPolicy463F955A"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "dynamodb:BatchGetItem",
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:Query",
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:ConditionCheckItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:DescribeTable"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:dynamodb:us-east-2:393419659647:table/HarborTeamsTable"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "SBOM-Management-Api-SBOMHarborMembersLambdaService-19LN7SS31SOV6"
}

resource "aws_iam_role_policy" "tfer--SBOM-Management-Api-SBOMHarborProjectLambdaService-1DA4GKY55NY7C_SBOMHarborProjectLambdaServiceRoleDefaultPolicy2B2C1CD9" {
  name = "SBOMHarborProjectLambdaServiceRoleDefaultPolicy2B2C1CD9"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "dynamodb:BatchGetItem",
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:Query",
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:ConditionCheckItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:DescribeTable"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:dynamodb:us-east-2:393419659647:table/HarborTeamsTable"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "SBOM-Management-Api-SBOMHarborProjectLambdaService-1DA4GKY55NY7C"
}

resource "aws_iam_role_policy" "tfer--SBOM-Management-Api-SBOMHarborProjectPOSTLambdaSer-70D8PDMD1BZM_SBOMHarborProjectPOSTLambdaServiceRoleDefaultPolicy396C40D7" {
  name = "SBOMHarborProjectPOSTLambdaServiceRoleDefaultPolicy396C40D7"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "dynamodb:BatchGetItem",
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:Query",
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:ConditionCheckItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:DescribeTable"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:dynamodb:us-east-2:393419659647:table/HarborTeamsTable"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "SBOM-Management-Api-SBOMHarborProjectPOSTLambdaSer-70D8PDMD1BZM"
}

resource "aws_iam_role_policy" "tfer--SBOM-Management-Api-SBOMHarborProjectsLambdaServic-8HDO9BSLPW0_SBOMHarborProjectsLambdaServiceRoleDefaultPolicy907A292B" {
  name = "SBOMHarborProjectsLambdaServiceRoleDefaultPolicy907A292B"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "dynamodb:BatchGetItem",
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:Query",
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:ConditionCheckItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:DescribeTable"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:dynamodb:us-east-2:393419659647:table/HarborTeamsTable"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "SBOM-Management-Api-SBOMHarborProjectsLambdaServic-8HDO9BSLPW0"
}

resource "aws_iam_role_policy" "tfer--SBOM-Management-Api-SBOMHarborTeamLambdaServiceRol-2YGJRNR2P7J3_SBOMHarborTeamLambdaServiceRoleDefaultPolicy2C54D764" {
  name = "SBOMHarborTeamLambdaServiceRoleDefaultPolicy2C54D764"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "dynamodb:BatchGetItem",
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:Query",
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:ConditionCheckItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:DescribeTable"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:dynamodb:us-east-2:393419659647:table/HarborTeamsTable"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "SBOM-Management-Api-SBOMHarborTeamLambdaServiceRol-2YGJRNR2P7J3"
}

resource "aws_iam_role_policy" "tfer--SBOM-Management-Api-SBOMHarborTeamPOSTLambdaServic-1H1JYUAR9SQEV_SBOMHarborTeamPOSTLambdaServiceRoleDefaultPolicy7ED973BA" {
  name = "SBOMHarborTeamPOSTLambdaServiceRoleDefaultPolicy7ED973BA"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "dynamodb:BatchGetItem",
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:Query",
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:ConditionCheckItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:DescribeTable"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:dynamodb:us-east-2:393419659647:table/HarborTeamsTable"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "SBOM-Management-Api-SBOMHarborTeamPOSTLambdaServic-1H1JYUAR9SQEV"
}

resource "aws_iam_role_policy" "tfer--SBOM-Management-Api-SBOMHarborTeamsLambdaServiceRo-1I6C3OHL8WMED_SBOMHarborTeamsLambdaServiceRoleDefaultPolicyE611D150" {
  name = "SBOMHarborTeamsLambdaServiceRoleDefaultPolicyE611D150"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "dynamodb:BatchGetItem",
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:Query",
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:ConditionCheckItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:DescribeTable"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:dynamodb:us-east-2:393419659647:table/HarborTeamsTable"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "SBOM-Management-Api-SBOMHarborTeamsLambdaServiceRo-1I6C3OHL8WMED"
}

resource "aws_iam_role_policy" "tfer--SBOM-Management-Api-SBOMHarborTokenLambdaServiceRo-THV24GLM4RZG_SBOMHarborTokenLambdaServiceRoleDefaultPolicyCA88A994" {
  name = "SBOMHarborTokenLambdaServiceRoleDefaultPolicyCA88A994"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "dynamodb:BatchGetItem",
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:Query",
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:ConditionCheckItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:DescribeTable"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:dynamodb:us-east-2:393419659647:table/HarborTeamsTable"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "SBOM-Management-Api-SBOMHarborTokenLambdaServiceRo-THV24GLM4RZG"
}

resource "aws_iam_role_policy" "tfer--SBOM-Management-Api-SBOMHarborTokenPOSTLambdaServi-GVF6EL2RI93Q_SBOMHarborTokenPOSTLambdaServiceRoleDefaultPolicyBF441CA5" {
  name = "SBOMHarborTokenPOSTLambdaServiceRoleDefaultPolicyBF441CA5"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "dynamodb:BatchGetItem",
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:Query",
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:ConditionCheckItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:DescribeTable"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:dynamodb:us-east-2:393419659647:table/HarborTeamsTable"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "SBOM-Management-Api-SBOMHarborTokenPOSTLambdaServi-GVF6EL2RI93Q"
}

resource "aws_iam_role_policy" "tfer--SBOM-Management-Api-SBOMHarborTokensLambdaServiceR-1MN0Z6PRZMWV7_SBOMHarborTokensLambdaServiceRoleDefaultPolicy5EB0FA46" {
  name = "SBOMHarborTokensLambdaServiceRoleDefaultPolicy5EB0FA46"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "dynamodb:BatchGetItem",
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:Query",
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:ConditionCheckItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:DescribeTable"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:dynamodb:us-east-2:393419659647:table/HarborTeamsTable"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "SBOM-Management-Api-SBOMHarborTokensLambdaServiceR-1MN0Z6PRZMWV7"
}

resource "aws_iam_role_policy" "tfer--SBOM-Management-Api-SBOMIngressLambdaServiceRole0D-1IEGFYDRHCHMU_SBOMIngressLambdaServiceRoleDefaultPolicy4A3C2501" {
  name = "SBOMIngressLambdaServiceRoleDefaultPolicy4A3C2501"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "s3:PutObject",
        "s3:PutObjectLegalHold",
        "s3:PutObjectRetention",
        "s3:PutObjectTagging",
        "s3:PutObjectVersionTagging",
        "s3:Abort*"
      ],
      "Effect": "Allow",
      "Resource": "arn:aws:s3:::sbom.bucket.393419659647/*"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "SBOM-Management-Api-SBOMIngressLambdaServiceRole0D-1IEGFYDRHCHMU"
}

resource "aws_iam_role_policy" "tfer--SBOM-Management-Api-UserSearchLambdaServiceRoleB24-1I6PJQYXXW6FO_UserSearchLambdaServiceRoleDefaultPolicy1F80E6C6" {
  name = "UserSearchLambdaServiceRoleDefaultPolicy1F80E6C6"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": "cognito-idp:ListUsers",
      "Effect": "Allow",
      "Resource": "*"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "SBOM-Management-Api-UserSearchLambdaServiceRoleB24-1I6PJQYXXW6FO"
}

resource "aws_iam_role_policy" "tfer--SBOMApi-Enrichment-BucketNotificationsHandler050a0-WORBW5Y8PI4O_BucketNotificationsHandler050a0587b7544547bf325f094a3db834RoleDefaultPolicy2CF63D36" {
  name = "BucketNotificationsHandler050a0587b7544547bf325f094a3db834RoleDefaultPolicy2CF63D36"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": "s3:PutBucketNotification",
      "Effect": "Allow",
      "Resource": "*"
    },
    {
      "Action": "s3:GetBucketNotification",
      "Effect": "Allow",
      "Resource": "*"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "SBOMApi-Enrichment-BucketNotificationsHandler050a0-WORBW5Y8PI4O"
}

resource "aws_iam_role_policy" "tfer--SBOMApi-Enrichment-DTFargateClusterdtTaskDefinitio-1ADE4X95U9SYU_DTFargateClusterdtTaskDefinitionExecutionRoleDefaultPolicy10A3D394" {
  name = "DTFargateClusterdtTaskDefinitionExecutionRoleDefaultPolicy10A3D394"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "logs:CreateLogStream",
        "logs:PutLogEvents"
      ],
      "Effect": "Allow",
      "Resource": "arn:aws:logs:us-east-2:393419659647:log-group:SBOMApi-Enrichment-DTFargateClusterdtTaskDefinitiondtContainerLogGroup2CD7A010-SEfr032VcIYV:*"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "SBOMApi-Enrichment-DTFargateClusterdtTaskDefinitio-1ADE4X95U9SYU"
}

resource "aws_iam_role_policy" "tfer--SBOMApi-Enrichment-DefaultEnrichmentInterfaceLambd-VKGSNN0CAP2F_DefaultEnrichmentInterfaceLambdaServiceRoleDefaultPolicy433111C8" {
  name = "DefaultEnrichmentInterfaceLambdaServiceRoleDefaultPolicy433111C8"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": "events:PutEvents",
      "Effect": "Allow",
      "Resource": "arn:aws:events:us-east-2:393419659647:event-bus/SBOMEnrichmentEventBus"
    },
    {
      "Action": [
        "s3:PutObject",
        "s3:PutObjectLegalHold",
        "s3:PutObjectRetention",
        "s3:PutObjectTagging",
        "s3:PutObjectVersionTagging",
        "s3:Abort*"
      ],
      "Effect": "Allow",
      "Resource": "arn:aws:s3:::sbom.bucket.393419659647/*"
    },
    {
      "Action": [
        "s3:GetObject*",
        "s3:GetBucket*",
        "s3:List*",
        "s3:DeleteObject*",
        "s3:PutObject",
        "s3:PutObjectLegalHold",
        "s3:PutObjectRetention",
        "s3:PutObjectTagging",
        "s3:PutObjectVersionTagging",
        "s3:Abort*"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:s3:::sbom.bucket.393419659647",
        "arn:aws:s3:::sbom.bucket.393419659647/*"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "SBOMApi-Enrichment-DefaultEnrichmentInterfaceLambd-VKGSNN0CAP2F"
}

resource "aws_iam_role_policy" "tfer--SBOMApi-Enrichment-DependencyTrackInterfaceLambdaS-153Y3Q0VCTZJH_DependencyTrackInterfaceLambdaServiceRoleDefaultPolicy9B4EB588" {
  name = "DependencyTrackInterfaceLambdaServiceRoleDefaultPolicy9B4EB588"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": "events:PutEvents",
      "Effect": "Allow",
      "Resource": "arn:aws:events:us-east-2:393419659647:event-bus/SBOMEnrichmentEventBus"
    },
    {
      "Action": [
        "s3:PutObject",
        "s3:PutObjectLegalHold",
        "s3:PutObjectRetention",
        "s3:PutObjectTagging",
        "s3:PutObjectVersionTagging",
        "s3:Abort*"
      ],
      "Effect": "Allow",
      "Resource": "arn:aws:s3:::sbom.bucket.393419659647/*"
    },
    {
      "Action": [
        "s3:GetObject*",
        "s3:GetBucket*",
        "s3:List*",
        "s3:DeleteObject*",
        "s3:PutObject",
        "s3:PutObjectLegalHold",
        "s3:PutObjectRetention",
        "s3:PutObjectTagging",
        "s3:PutObjectVersionTagging",
        "s3:Abort*"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:s3:::sbom.bucket.393419659647",
        "arn:aws:s3:::sbom.bucket.393419659647/*"
      ]
    },
    {
      "Action": [
        "ssm:DescribeParameters",
        "ssm:GetParameters",
        "ssm:GetParameter",
        "ssm:GetParameterHistory"
      ],
      "Effect": "Allow",
      "Resource": "arn:aws:ssm:us-east-2:393419659647:parameter/DT_ROOT_PWD"
    },
    {
      "Action": "ssm:PutParameter",
      "Effect": "Allow",
      "Resource": "arn:aws:ssm:us-east-2:393419659647:parameter/DT_ROOT_PWD"
    },
    {
      "Action": [
        "ssm:DescribeParameters",
        "ssm:GetParameters",
        "ssm:GetParameter",
        "ssm:GetParameterHistory"
      ],
      "Effect": "Allow",
      "Resource": "arn:aws:ssm:us-east-2:393419659647:parameter/DT_API_KEY"
    },
    {
      "Action": "ssm:PutParameter",
      "Effect": "Allow",
      "Resource": "arn:aws:ssm:us-east-2:393419659647:parameter/DT_API_KEY"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "SBOMApi-Enrichment-DependencyTrackInterfaceLambdaS-153Y3Q0VCTZJH"
}

resource "aws_iam_role_policy" "tfer--SBOMApi-Enrichment-ENRICHMENTSTATEMACHINEEventsRol-QLRDKM2C3E3A_ENRICHMENTSTATEMACHINEEventsRoleDefaultPolicyC7BD5AE4" {
  name = "ENRICHMENTSTATEMACHINEEventsRoleDefaultPolicyC7BD5AE4"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": "states:StartExecution",
      "Effect": "Allow",
      "Resource": "arn:aws:states:us-east-2:393419659647:stateMachine:ENRICHMENT_STATE_MACHINE"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "SBOMApi-Enrichment-ENRICHMENTSTATEMACHINEEventsRol-QLRDKM2C3E3A"
}

resource "aws_iam_role_policy" "tfer--SBOMApi-Enrichment-ENRICHMENTSTATEMACHINERoleA5B02-19AOYJNM8DJNR_ENRICHMENTSTATEMACHINERoleDefaultPolicyA998D563" {
  name = "ENRICHMENTSTATEMACHINERoleDefaultPolicyA998D563"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": "lambda:InvokeFunction",
      "Effect": "Allow",
      "Resource": [
        "arn:aws:lambda:us-east-2:393419659647:function:SummarizerLambda",
        "arn:aws:lambda:us-east-2:393419659647:function:SummarizerLambda:*"
      ]
    },
    {
      "Action": "lambda:InvokeFunction",
      "Effect": "Allow",
      "Resource": [
        "arn:aws:lambda:us-east-2:393419659647:function:DependencyTrackInterfaceLambda",
        "arn:aws:lambda:us-east-2:393419659647:function:DependencyTrackInterfaceLambda:*"
      ]
    },
    {
      "Action": "lambda:InvokeFunction",
      "Effect": "Allow",
      "Resource": [
        "arn:aws:lambda:us-east-2:393419659647:function:DefaultEnrichmentInterfaceLambda",
        "arn:aws:lambda:us-east-2:393419659647:function:DefaultEnrichmentInterfaceLambda:*"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "SBOMApi-Enrichment-ENRICHMENTSTATEMACHINERoleA5B02-19AOYJNM8DJNR"
}

resource "aws_iam_role_policy" "tfer--SBOMApi-Enrichment-SBOMEnrichmentIngressLambdaServ-GC5O9QX4V5H1_SBOMEnrichmentIngressLambdaServiceRoleDefaultPolicy48134E1F" {
  name = "SBOMEnrichmentIngressLambdaServiceRoleDefaultPolicy48134E1F"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "s3:GetObject*",
        "s3:GetBucket*",
        "s3:List*"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:s3:::sbom.bucket.393419659647",
        "arn:aws:s3:::sbom.bucket.393419659647/*"
      ]
    },
    {
      "Action": "events:PutEvents",
      "Effect": "Allow",
      "Resource": "arn:aws:events:us-east-2:393419659647:event-bus/SBOMEnrichmentEventBus"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "SBOMApi-Enrichment-SBOMEnrichmentIngressLambdaServ-GC5O9QX4V5H1"
}

resource "aws_iam_role_policy" "tfer--SBOMApi-Enrichment-SummarizerLambdaServiceRoleEEF5-1V7DA30FJTF4X_SummarizerLambdaServiceRoleDefaultPolicy6A39C251" {
  name = "SummarizerLambdaServiceRoleDefaultPolicy6A39C251"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": "events:PutEvents",
      "Effect": "Allow",
      "Resource": "arn:aws:events:us-east-2:393419659647:event-bus/SBOMEnrichmentEventBus"
    },
    {
      "Action": [
        "s3:PutObject",
        "s3:PutObjectLegalHold",
        "s3:PutObjectRetention",
        "s3:PutObjectTagging",
        "s3:PutObjectVersionTagging",
        "s3:Abort*"
      ],
      "Effect": "Allow",
      "Resource": "arn:aws:s3:::sbom.bucket.393419659647/*"
    },
    {
      "Action": [
        "s3:GetObject*",
        "s3:GetBucket*",
        "s3:List*",
        "s3:DeleteObject*",
        "s3:PutObject",
        "s3:PutObjectLegalHold",
        "s3:PutObjectRetention",
        "s3:PutObjectTagging",
        "s3:PutObjectVersionTagging",
        "s3:Abort*"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:s3:::sbom.bucket.393419659647",
        "arn:aws:s3:::sbom.bucket.393419659647/*"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "SBOMApi-Enrichment-SummarizerLambdaServiceRoleEEF5-1V7DA30FJTF4X"
}

resource "aws_iam_role_policy" "tfer--SBOMApi-Shared-Resource-BucketNotificationsHandler-931RZ9L19VYI_BucketNotificationsHandler050a0587b7544547bf325f094a3db834RoleDefaultPolicy2CF63D36" {
  name = "BucketNotificationsHandler050a0587b7544547bf325f094a3db834RoleDefaultPolicy2CF63D36"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": "s3:PutBucketNotification",
      "Effect": "Allow",
      "Resource": "*"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "SBOMApi-Shared-Resource-BucketNotificationsHandler-931RZ9L19VYI"
}

resource "aws_iam_role_policy" "tfer--aws-controltower-ForwardSnsNotificationRole_sns" {
  name = "sns"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "sns:publish"
      ],
      "Effect": "Allow",
      "Resource": "arn:aws:sns:*:678602588155:aws-controltower-AggregateSecurityNotifications"
    }
  ]
}
POLICY

  role = "aws-controltower-ForwardSnsNotificationRole"
}

resource "aws_iam_role_policy" "tfer--cdk-hnb659fds-deploy-role-393419659647-us-east-2_default" {
  name = "default"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "cloudformation:CreateChangeSet",
        "cloudformation:DeleteChangeSet",
        "cloudformation:DescribeChangeSet",
        "cloudformation:DescribeStacks",
        "cloudformation:ExecuteChangeSet",
        "cloudformation:CreateStack",
        "cloudformation:UpdateStack"
      ],
      "Effect": "Allow",
      "Resource": "*",
      "Sid": "CloudFormationPermissions"
    },
    {
      "Action": [
        "s3:GetObject*",
        "s3:GetBucket*",
        "s3:List*",
        "s3:Abort*",
        "s3:DeleteObject*",
        "s3:PutObject*"
      ],
      "Condition": {
        "StringNotEquals": {
          "s3:ResourceAccount": "393419659647"
        }
      },
      "Effect": "Allow",
      "Resource": "*",
      "Sid": "PipelineCrossAccountArtifactsBucket"
    },
    {
      "Action": [
        "kms:Decrypt",
        "kms:DescribeKey",
        "kms:Encrypt",
        "kms:ReEncrypt*",
        "kms:GenerateDataKey*"
      ],
      "Condition": {
        "StringEquals": {
          "kms:ViaService": "s3.us-east-2.amazonaws.com"
        }
      },
      "Effect": "Allow",
      "Resource": "*",
      "Sid": "PipelineCrossAccountArtifactsKey"
    },
    {
      "Action": "iam:PassRole",
      "Effect": "Allow",
      "Resource": "arn:aws:iam::393419659647:role/cdk-hnb659fds-cfn-exec-role-393419659647-us-east-2"
    },
    {
      "Action": [
        "cloudformation:DescribeStackEvents",
        "cloudformation:GetTemplate",
        "cloudformation:DeleteStack",
        "cloudformation:UpdateTerminationProtection",
        "sts:GetCallerIdentity",
        "cloudformation:GetTemplateSummary"
      ],
      "Effect": "Allow",
      "Resource": "*",
      "Sid": "CliPermissions"
    },
    {
      "Action": [
        "s3:GetObject*",
        "s3:GetBucket*",
        "s3:List*"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:s3:::cdk-hnb659fds-assets-393419659647-us-east-2",
        "arn:aws:s3:::cdk-hnb659fds-assets-393419659647-us-east-2/*"
      ],
      "Sid": "CliStagingBucket"
    },
    {
      "Action": [
        "ssm:GetParameter"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:ssm:us-east-2:393419659647:parameter/cdk-bootstrap/hnb659fds/version"
      ],
      "Sid": "ReadVersion"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "cdk-hnb659fds-deploy-role-393419659647-us-east-2"
}

resource "aws_iam_role_policy" "tfer--cdk-hnb659fds-deploy-role-393419659647-us-west-2_default" {
  name = "default"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "cloudformation:CreateChangeSet",
        "cloudformation:DeleteChangeSet",
        "cloudformation:DescribeChangeSet",
        "cloudformation:DescribeStacks",
        "cloudformation:ExecuteChangeSet",
        "cloudformation:CreateStack",
        "cloudformation:UpdateStack"
      ],
      "Effect": "Allow",
      "Resource": "*",
      "Sid": "CloudFormationPermissions"
    },
    {
      "Action": [
        "s3:GetObject*",
        "s3:GetBucket*",
        "s3:List*",
        "s3:Abort*",
        "s3:DeleteObject*",
        "s3:PutObject*"
      ],
      "Condition": {
        "StringNotEquals": {
          "s3:ResourceAccount": "393419659647"
        }
      },
      "Effect": "Allow",
      "Resource": "*",
      "Sid": "PipelineCrossAccountArtifactsBucket"
    },
    {
      "Action": [
        "kms:Decrypt",
        "kms:DescribeKey",
        "kms:Encrypt",
        "kms:ReEncrypt*",
        "kms:GenerateDataKey*"
      ],
      "Condition": {
        "StringEquals": {
          "kms:ViaService": "s3.us-west-2.amazonaws.com"
        }
      },
      "Effect": "Allow",
      "Resource": "*",
      "Sid": "PipelineCrossAccountArtifactsKey"
    },
    {
      "Action": "iam:PassRole",
      "Effect": "Allow",
      "Resource": "arn:aws:iam::393419659647:role/cdk-hnb659fds-cfn-exec-role-393419659647-us-west-2"
    },
    {
      "Action": [
        "cloudformation:DescribeStackEvents",
        "cloudformation:GetTemplate",
        "cloudformation:DeleteStack",
        "cloudformation:UpdateTerminationProtection",
        "sts:GetCallerIdentity",
        "cloudformation:GetTemplateSummary"
      ],
      "Effect": "Allow",
      "Resource": "*",
      "Sid": "CliPermissions"
    },
    {
      "Action": [
        "s3:GetObject*",
        "s3:GetBucket*",
        "s3:List*"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:s3:::cdk-hnb659fds-assets-393419659647-us-west-2",
        "arn:aws:s3:::cdk-hnb659fds-assets-393419659647-us-west-2/*"
      ],
      "Sid": "CliStagingBucket"
    },
    {
      "Action": [
        "ssm:GetParameter"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:ssm:us-west-2:393419659647:parameter/cdk-bootstrap/hnb659fds/version"
      ],
      "Sid": "ReadVersion"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "cdk-hnb659fds-deploy-role-393419659647-us-west-2"
}

resource "aws_iam_role_policy" "tfer--cdk-hnb659fds-file-publishing-role-393419659647-us-east-2_cdk-hnb659fds-file-publishing-role-default-policy-393419659647-us-east-2" {
  name = "cdk-hnb659fds-file-publishing-role-default-policy-393419659647-us-east-2"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "s3:GetObject*",
        "s3:GetBucket*",
        "s3:GetEncryptionConfiguration",
        "s3:List*",
        "s3:DeleteObject*",
        "s3:PutObject*",
        "s3:Abort*"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:s3:::cdk-hnb659fds-assets-393419659647-us-east-2",
        "arn:aws:s3:::cdk-hnb659fds-assets-393419659647-us-east-2/*"
      ]
    },
    {
      "Action": [
        "kms:Decrypt",
        "kms:DescribeKey",
        "kms:Encrypt",
        "kms:ReEncrypt*",
        "kms:GenerateDataKey*"
      ],
      "Effect": "Allow",
      "Resource": "arn:aws:kms:us-east-2:393419659647:key/AWS_MANAGED_KEY"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "cdk-hnb659fds-file-publishing-role-393419659647-us-east-2"
}

resource "aws_iam_role_policy" "tfer--cdk-hnb659fds-file-publishing-role-393419659647-us-west-2_cdk-hnb659fds-file-publishing-role-default-policy-393419659647-us-west-2" {
  name = "cdk-hnb659fds-file-publishing-role-default-policy-393419659647-us-west-2"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "s3:GetObject*",
        "s3:GetBucket*",
        "s3:GetEncryptionConfiguration",
        "s3:List*",
        "s3:DeleteObject*",
        "s3:PutObject*",
        "s3:Abort*"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:s3:::cdk-hnb659fds-assets-393419659647-us-west-2",
        "arn:aws:s3:::cdk-hnb659fds-assets-393419659647-us-west-2/*"
      ]
    },
    {
      "Action": [
        "kms:Decrypt",
        "kms:DescribeKey",
        "kms:Encrypt",
        "kms:ReEncrypt*",
        "kms:GenerateDataKey*"
      ],
      "Effect": "Allow",
      "Resource": "arn:aws:kms:us-west-2:393419659647:key/AWS_MANAGED_KEY"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "cdk-hnb659fds-file-publishing-role-393419659647-us-west-2"
}

resource "aws_iam_role_policy" "tfer--cdk-hnb659fds-image-publishing-role-393419659647-us-east-2_cdk-hnb659fds-image-publishing-role-default-policy-393419659647-us-east-2" {
  name = "cdk-hnb659fds-image-publishing-role-default-policy-393419659647-us-east-2"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "ecr:PutImage",
        "ecr:InitiateLayerUpload",
        "ecr:UploadLayerPart",
        "ecr:CompleteLayerUpload",
        "ecr:BatchCheckLayerAvailability",
        "ecr:DescribeRepositories",
        "ecr:DescribeImages",
        "ecr:BatchGetImage",
        "ecr:GetDownloadUrlForLayer"
      ],
      "Effect": "Allow",
      "Resource": "arn:aws:ecr:us-east-2:393419659647:repository/cdk-hnb659fds-container-assets-393419659647-us-east-2"
    },
    {
      "Action": [
        "ecr:GetAuthorizationToken"
      ],
      "Effect": "Allow",
      "Resource": "*"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "cdk-hnb659fds-image-publishing-role-393419659647-us-east-2"
}

resource "aws_iam_role_policy" "tfer--cdk-hnb659fds-image-publishing-role-393419659647-us-west-2_cdk-hnb659fds-image-publishing-role-default-policy-393419659647-us-west-2" {
  name = "cdk-hnb659fds-image-publishing-role-default-policy-393419659647-us-west-2"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "ecr:PutImage",
        "ecr:InitiateLayerUpload",
        "ecr:UploadLayerPart",
        "ecr:CompleteLayerUpload",
        "ecr:BatchCheckLayerAvailability",
        "ecr:DescribeRepositories",
        "ecr:DescribeImages",
        "ecr:BatchGetImage",
        "ecr:GetDownloadUrlForLayer"
      ],
      "Effect": "Allow",
      "Resource": "arn:aws:ecr:us-west-2:393419659647:repository/cdk-hnb659fds-container-assets-393419659647-us-west-2"
    },
    {
      "Action": [
        "ecr:GetAuthorizationToken"
      ],
      "Effect": "Allow",
      "Resource": "*"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "cdk-hnb659fds-image-publishing-role-393419659647-us-west-2"
}

resource "aws_iam_role_policy" "tfer--cdk-hnb659fds-lookup-role-393419659647-us-east-2_LookupRolePolicy" {
  name = "LookupRolePolicy"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "kms:Decrypt"
      ],
      "Effect": "Deny",
      "Resource": "*",
      "Sid": "DontReadSecrets"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "cdk-hnb659fds-lookup-role-393419659647-us-east-2"
}

resource "aws_iam_role_policy" "tfer--cdk-hnb659fds-lookup-role-393419659647-us-west-2_LookupRolePolicy" {
  name = "LookupRolePolicy"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "kms:Decrypt"
      ],
      "Effect": "Deny",
      "Resource": "*",
      "Sid": "DontReadSecrets"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "cdk-hnb659fds-lookup-role-393419659647-us-west-2"
}

resource "aws_iam_role_policy" "tfer--sandbox-harbor-backend-us-sandboxAPIKeyAuthorizeru-J6G3ARPEMAB8_sandboxAPIKeyAuthorizerusw2ServiceRoleDefaultPolicy6CD3C6A3" {
  name = "sandboxAPIKeyAuthorizerusw2ServiceRoleDefaultPolicy6CD3C6A3"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "dynamodb:BatchGetItem",
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:Query",
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:ConditionCheckItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:DescribeTable"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "sandbox-harbor-backend-us-sandboxAPIKeyAuthorizeru-J6G3ARPEMAB8"
}

resource "aws_iam_role_policy" "tfer--sandbox-harbor-backend-us-sandboxHarborCodebaseLam-JV3C9SPDAZND_sandboxHarborCodebaseLambdausw2ServiceRoleDefaultPolicy3030D26E" {
  name = "sandboxHarborCodebaseLambdausw2ServiceRoleDefaultPolicy3030D26E"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "cognito-idp:AdminDisableUser",
        "cognito-idp:AdminEnableUser",
        "cognito-idp:AdminGetUser",
        "cognito-idp:ListUsers",
        "cognito-idp:AdminUpdateUserAttributes"
      ],
      "Effect": "Allow",
      "Resource": "*"
    },
    {
      "Action": [
        "dynamodb:BatchGetItem",
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:Query",
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:ConditionCheckItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:DescribeTable"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "sandbox-harbor-backend-us-sandboxHarborCodebaseLam-JV3C9SPDAZND"
}

resource "aws_iam_role_policy" "tfer--sandbox-harbor-backend-us-sandboxHarborCodebasePOS-1KEMFXHFN45G9_sandboxHarborCodebasePOSTLambdausw2ServiceRoleDefaultPolicy83B38555" {
  name = "sandboxHarborCodebasePOSTLambdausw2ServiceRoleDefaultPolicy83B38555"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "cognito-idp:AdminDisableUser",
        "cognito-idp:AdminEnableUser",
        "cognito-idp:AdminGetUser",
        "cognito-idp:ListUsers",
        "cognito-idp:AdminUpdateUserAttributes"
      ],
      "Effect": "Allow",
      "Resource": "*"
    },
    {
      "Action": [
        "dynamodb:BatchGetItem",
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:Query",
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:ConditionCheckItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:DescribeTable"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "sandbox-harbor-backend-us-sandboxHarborCodebasePOS-1KEMFXHFN45G9"
}

resource "aws_iam_role_policy" "tfer--sandbox-harbor-backend-us-sandboxHarborCodebasesLa-1GXA10VAOSWZ8_sandboxHarborCodebasesLambdausw2ServiceRoleDefaultPolicy4CCAF7F7" {
  name = "sandboxHarborCodebasesLambdausw2ServiceRoleDefaultPolicy4CCAF7F7"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "cognito-idp:AdminDisableUser",
        "cognito-idp:AdminEnableUser",
        "cognito-idp:AdminGetUser",
        "cognito-idp:ListUsers",
        "cognito-idp:AdminUpdateUserAttributes"
      ],
      "Effect": "Allow",
      "Resource": "*"
    },
    {
      "Action": [
        "dynamodb:BatchGetItem",
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:Query",
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:ConditionCheckItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:DescribeTable"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "sandbox-harbor-backend-us-sandboxHarborCodebasesLa-1GXA10VAOSWZ8"
}

resource "aws_iam_role_policy" "tfer--sandbox-harbor-backend-us-sandboxHarborMemberLambd-IX1FUENAP5PG_sandboxHarborMemberLambdausw2ServiceRoleDefaultPolicyD607E11E" {
  name = "sandboxHarborMemberLambdausw2ServiceRoleDefaultPolicyD607E11E"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "cognito-idp:AdminDisableUser",
        "cognito-idp:AdminEnableUser",
        "cognito-idp:AdminGetUser",
        "cognito-idp:ListUsers",
        "cognito-idp:AdminUpdateUserAttributes"
      ],
      "Effect": "Allow",
      "Resource": "*"
    },
    {
      "Action": [
        "dynamodb:BatchGetItem",
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:Query",
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:ConditionCheckItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:DescribeTable"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "sandbox-harbor-backend-us-sandboxHarborMemberLambd-IX1FUENAP5PG"
}

resource "aws_iam_role_policy" "tfer--sandbox-harbor-backend-us-sandboxHarborMemberPOSTL-R9N72L26UAYS_sandboxHarborMemberPOSTLambdausw2ServiceRoleDefaultPolicy48D96B89" {
  name = "sandboxHarborMemberPOSTLambdausw2ServiceRoleDefaultPolicy48D96B89"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "cognito-idp:AdminDisableUser",
        "cognito-idp:AdminEnableUser",
        "cognito-idp:AdminGetUser",
        "cognito-idp:ListUsers",
        "cognito-idp:AdminUpdateUserAttributes"
      ],
      "Effect": "Allow",
      "Resource": "*"
    },
    {
      "Action": [
        "dynamodb:BatchGetItem",
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:Query",
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:ConditionCheckItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:DescribeTable"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "sandbox-harbor-backend-us-sandboxHarborMemberPOSTL-R9N72L26UAYS"
}

resource "aws_iam_role_policy" "tfer--sandbox-harbor-backend-us-sandboxHarborMembersLamb-1I0LXSVO9QGWI_sandboxHarborMembersLambdausw2ServiceRoleDefaultPolicy38CDE6CC" {
  name = "sandboxHarborMembersLambdausw2ServiceRoleDefaultPolicy38CDE6CC"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "cognito-idp:AdminDisableUser",
        "cognito-idp:AdminEnableUser",
        "cognito-idp:AdminGetUser",
        "cognito-idp:ListUsers",
        "cognito-idp:AdminUpdateUserAttributes"
      ],
      "Effect": "Allow",
      "Resource": "*"
    },
    {
      "Action": [
        "dynamodb:BatchGetItem",
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:Query",
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:ConditionCheckItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:DescribeTable"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "sandbox-harbor-backend-us-sandboxHarborMembersLamb-1I0LXSVO9QGWI"
}

resource "aws_iam_role_policy" "tfer--sandbox-harbor-backend-us-sandboxHarborProjectLamb-DXYP13V3MR7Y_sandboxHarborProjectLambdausw2ServiceRoleDefaultPolicyD03AB49F" {
  name = "sandboxHarborProjectLambdausw2ServiceRoleDefaultPolicyD03AB49F"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "cognito-idp:AdminDisableUser",
        "cognito-idp:AdminEnableUser",
        "cognito-idp:AdminGetUser",
        "cognito-idp:ListUsers",
        "cognito-idp:AdminUpdateUserAttributes"
      ],
      "Effect": "Allow",
      "Resource": "*"
    },
    {
      "Action": [
        "dynamodb:BatchGetItem",
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:Query",
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:ConditionCheckItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:DescribeTable"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "sandbox-harbor-backend-us-sandboxHarborProjectLamb-DXYP13V3MR7Y"
}

resource "aws_iam_role_policy" "tfer--sandbox-harbor-backend-us-sandboxHarborProjectPOST-NCR778JYL4FJ_sandboxHarborProjectPOSTLambdausw2ServiceRoleDefaultPolicy3843939F" {
  name = "sandboxHarborProjectPOSTLambdausw2ServiceRoleDefaultPolicy3843939F"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "cognito-idp:AdminDisableUser",
        "cognito-idp:AdminEnableUser",
        "cognito-idp:AdminGetUser",
        "cognito-idp:ListUsers",
        "cognito-idp:AdminUpdateUserAttributes"
      ],
      "Effect": "Allow",
      "Resource": "*"
    },
    {
      "Action": [
        "dynamodb:BatchGetItem",
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:Query",
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:ConditionCheckItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:DescribeTable"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "sandbox-harbor-backend-us-sandboxHarborProjectPOST-NCR778JYL4FJ"
}

resource "aws_iam_role_policy" "tfer--sandbox-harbor-backend-us-sandboxHarborProjectsLam-IVSYK5W51ZY7_sandboxHarborProjectsLambdausw2ServiceRoleDefaultPolicy8C0DDC62" {
  name = "sandboxHarborProjectsLambdausw2ServiceRoleDefaultPolicy8C0DDC62"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "cognito-idp:AdminDisableUser",
        "cognito-idp:AdminEnableUser",
        "cognito-idp:AdminGetUser",
        "cognito-idp:ListUsers",
        "cognito-idp:AdminUpdateUserAttributes"
      ],
      "Effect": "Allow",
      "Resource": "*"
    },
    {
      "Action": [
        "dynamodb:BatchGetItem",
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:Query",
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:ConditionCheckItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:DescribeTable"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "sandbox-harbor-backend-us-sandboxHarborProjectsLam-IVSYK5W51ZY7"
}

resource "aws_iam_role_policy" "tfer--sandbox-harbor-backend-us-sandboxHarborTeamLambdau-1SUMZV3BOCS2O_sandboxHarborTeamLambdausw2ServiceRoleDefaultPolicyFFFA7801" {
  name = "sandboxHarborTeamLambdausw2ServiceRoleDefaultPolicyFFFA7801"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "cognito-idp:AdminDisableUser",
        "cognito-idp:AdminEnableUser",
        "cognito-idp:AdminGetUser",
        "cognito-idp:ListUsers",
        "cognito-idp:AdminUpdateUserAttributes"
      ],
      "Effect": "Allow",
      "Resource": "*"
    },
    {
      "Action": [
        "dynamodb:BatchGetItem",
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:Query",
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:ConditionCheckItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:DescribeTable"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "sandbox-harbor-backend-us-sandboxHarborTeamLambdau-1SUMZV3BOCS2O"
}

resource "aws_iam_role_policy" "tfer--sandbox-harbor-backend-us-sandboxHarborTeamPOSTLam-1M9DCBMYMUTZ9_sandboxHarborTeamPOSTLambdausw2ServiceRoleDefaultPolicy2DD1D4F7" {
  name = "sandboxHarborTeamPOSTLambdausw2ServiceRoleDefaultPolicy2DD1D4F7"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "cognito-idp:AdminDisableUser",
        "cognito-idp:AdminEnableUser",
        "cognito-idp:AdminGetUser",
        "cognito-idp:ListUsers",
        "cognito-idp:AdminUpdateUserAttributes"
      ],
      "Effect": "Allow",
      "Resource": "*"
    },
    {
      "Action": [
        "dynamodb:BatchGetItem",
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:Query",
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:ConditionCheckItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:DescribeTable"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "sandbox-harbor-backend-us-sandboxHarborTeamPOSTLam-1M9DCBMYMUTZ9"
}

resource "aws_iam_role_policy" "tfer--sandbox-harbor-backend-us-sandboxHarborTeamsLambda-97ORIE58LYQK_sandboxHarborTeamsLambdausw2ServiceRoleDefaultPolicyD0FADD25" {
  name = "sandboxHarborTeamsLambdausw2ServiceRoleDefaultPolicyD0FADD25"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "cognito-idp:AdminDisableUser",
        "cognito-idp:AdminEnableUser",
        "cognito-idp:AdminGetUser",
        "cognito-idp:ListUsers",
        "cognito-idp:AdminUpdateUserAttributes"
      ],
      "Effect": "Allow",
      "Resource": "*"
    },
    {
      "Action": [
        "dynamodb:BatchGetItem",
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:Query",
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:ConditionCheckItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:DescribeTable"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "sandbox-harbor-backend-us-sandboxHarborTeamsLambda-97ORIE58LYQK"
}

resource "aws_iam_role_policy" "tfer--sandbox-harbor-backend-us-sandboxHarborTokenLambda-4J703PBNIS81_sandboxHarborTokenLambdausw2ServiceRoleDefaultPolicyA589C9EC" {
  name = "sandboxHarborTokenLambdausw2ServiceRoleDefaultPolicyA589C9EC"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "cognito-idp:AdminDisableUser",
        "cognito-idp:AdminEnableUser",
        "cognito-idp:AdminGetUser",
        "cognito-idp:ListUsers",
        "cognito-idp:AdminUpdateUserAttributes"
      ],
      "Effect": "Allow",
      "Resource": "*"
    },
    {
      "Action": [
        "dynamodb:BatchGetItem",
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:Query",
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:ConditionCheckItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:DescribeTable"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "sandbox-harbor-backend-us-sandboxHarborTokenLambda-4J703PBNIS81"
}

resource "aws_iam_role_policy" "tfer--sandbox-harbor-backend-us-sandboxHarborTokenPOSTLa-1WNCMTFZ75DLF_sandboxHarborTokenPOSTLambdausw2ServiceRoleDefaultPolicy8BDAC0DF" {
  name = "sandboxHarborTokenPOSTLambdausw2ServiceRoleDefaultPolicy8BDAC0DF"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "cognito-idp:AdminDisableUser",
        "cognito-idp:AdminEnableUser",
        "cognito-idp:AdminGetUser",
        "cognito-idp:ListUsers",
        "cognito-idp:AdminUpdateUserAttributes"
      ],
      "Effect": "Allow",
      "Resource": "*"
    },
    {
      "Action": [
        "dynamodb:BatchGetItem",
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:Query",
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:ConditionCheckItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:DescribeTable"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "sandbox-harbor-backend-us-sandboxHarborTokenPOSTLa-1WNCMTFZ75DLF"
}

resource "aws_iam_role_policy" "tfer--sandbox-harbor-backend-us-sandboxHarborTokensLambd-3BH0LJLJN4F6_sandboxHarborTokensLambdausw2ServiceRoleDefaultPolicyA587B751" {
  name = "sandboxHarborTokensLambdausw2ServiceRoleDefaultPolicyA587B751"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "cognito-idp:AdminDisableUser",
        "cognito-idp:AdminEnableUser",
        "cognito-idp:AdminGetUser",
        "cognito-idp:ListUsers",
        "cognito-idp:AdminUpdateUserAttributes"
      ],
      "Effect": "Allow",
      "Resource": "*"
    },
    {
      "Action": [
        "dynamodb:BatchGetItem",
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:Query",
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:ConditionCheckItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:DescribeTable"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "sandbox-harbor-backend-us-sandboxHarborTokensLambd-3BH0LJLJN4F6"
}

resource "aws_iam_role_policy" "tfer--sandbox-harbor-backend-us-sandboxJwtTokenAuthorize-1MM2C42NKJCYH_sandboxJwtTokenAuthorizerusw2ServiceRoleDefaultPolicyF29C585C" {
  name = "sandboxJwtTokenAuthorizerusw2ServiceRoleDefaultPolicyF29C585C"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "cognito-idp:AdminDisableUser",
        "cognito-idp:AdminEnableUser",
        "cognito-idp:AdminGetUser",
        "cognito-idp:ListUsers"
      ],
      "Effect": "Allow",
      "Resource": "*"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "sandbox-harbor-backend-us-sandboxJwtTokenAuthorize-1MM2C42NKJCYH"
}

resource "aws_iam_role_policy" "tfer--sandbox-harbor-backend-us-sandboxLoginusw2ServiceR-D69CUROSBKHL_sandboxLoginusw2ServiceRoleDefaultPolicy6E70030D" {
  name = "sandboxLoginusw2ServiceRoleDefaultPolicy6E70030D"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "cognito-idp:AdminGetUser",
        "cognito-idp:AdminEnableUser",
        "cognito-idp:AdminDisableUser",
        "cognito-idp:AdminInitiateAuth"
      ],
      "Effect": "Allow",
      "Resource": "*"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "sandbox-harbor-backend-us-sandboxLoginusw2ServiceR-D69CUROSBKHL"
}

resource "aws_iam_role_policy" "tfer--sandbox-harbor-backend-us-sandboxSBOMIngressusw2Se-WRDZ6YKTOSC0_sandboxSBOMIngressusw2ServiceRoleDefaultPolicyBEBAC792" {
  name = "sandboxSBOMIngressusw2ServiceRoleDefaultPolicyBEBAC792"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "s3:PutObject",
        "s3:PutObjectLegalHold",
        "s3:PutObjectRetention",
        "s3:PutObjectTagging",
        "s3:PutObjectVersionTagging",
        "s3:Abort*"
      ],
      "Effect": "Allow",
      "Resource": "arn:aws:s3:::sandbox-harbor-sbom-uploads-enrichment-393419659647-usw2/*"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "sandbox-harbor-backend-us-sandboxSBOMIngressusw2Se-WRDZ6YKTOSC0"
}

resource "aws_iam_role_policy" "tfer--sandbox-harbor-backend-us-sandboxUserSearchusw2Ser-EZE8AIWUFWN7_sandboxUserSearchusw2ServiceRoleDefaultPolicy4FFCD120" {
  name = "sandboxUserSearchusw2ServiceRoleDefaultPolicy4FFCD120"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": "cognito-idp:ListUsers",
      "Effect": "Allow",
      "Resource": "*"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "sandbox-harbor-backend-us-sandboxUserSearchusw2Ser-EZE8AIWUFWN7"
}

resource "aws_iam_role_policy" "tfer--sandbox-harbor-enrichment-BucketNotificationsHandl-1HERPSPAMUYO4_BucketNotificationsHandler050a0587b7544547bf325f094a3db834RoleDefaultPolicy2CF63D36" {
  name = "BucketNotificationsHandler050a0587b7544547bf325f094a3db834RoleDefaultPolicy2CF63D36"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": "s3:PutBucketNotification",
      "Effect": "Allow",
      "Resource": "*"
    },
    {
      "Action": "s3:GetBucketNotification",
      "Effect": "Allow",
      "Resource": "*"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "sandbox-harbor-enrichment-BucketNotificationsHandl-1HERPSPAMUYO4"
}

resource "aws_iam_role_policy" "tfer--sandbox-harbor-enrichment-ENRICHMENTSTATEMACHINEEv-1DCSHZUSIB298_ENRICHMENTSTATEMACHINEEventsRoleDefaultPolicyC7BD5AE4" {
  name = "ENRICHMENTSTATEMACHINEEventsRoleDefaultPolicyC7BD5AE4"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": "states:StartExecution",
      "Effect": "Allow",
      "Resource": "arn:aws:states:us-west-2:393419659647:stateMachine:sandbox_Enrichment_usw2"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "sandbox-harbor-enrichment-ENRICHMENTSTATEMACHINEEv-1DCSHZUSIB298"
}

resource "aws_iam_role_policy" "tfer--sandbox-harbor-enrichment-ENRICHMENTSTATEMACHINERo-1TKL415Y7O63T_ENRICHMENTSTATEMACHINERoleDefaultPolicyA998D563" {
  name = "ENRICHMENTSTATEMACHINERoleDefaultPolicyA998D563"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": "lambda:InvokeFunction",
      "Effect": "Allow",
      "Resource": [
        "arn:aws:lambda:us-west-2:393419659647:function:sandbox_Summarizer_usw2",
        "arn:aws:lambda:us-west-2:393419659647:function:sandbox_Summarizer_usw2:*"
      ]
    },
    {
      "Action": "lambda:InvokeFunction",
      "Effect": "Allow",
      "Resource": [
        "arn:aws:lambda:us-west-2:393419659647:function:sandbox_DependencyTrackInterface_usw2",
        "arn:aws:lambda:us-west-2:393419659647:function:sandbox_DependencyTrackInterface_usw2:*"
      ]
    },
    {
      "Action": "lambda:InvokeFunction",
      "Effect": "Allow",
      "Resource": [
        "arn:aws:lambda:us-west-2:393419659647:function:sandbox_IonChannelInterface_usw2",
        "arn:aws:lambda:us-west-2:393419659647:function:sandbox_IonChannelInterface_usw2:*"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "sandbox-harbor-enrichment-ENRICHMENTSTATEMACHINERo-1TKL415Y7O63T"
}

resource "aws_iam_role_policy" "tfer--sandbox-harbor-enrichment-HarborFargateClusterDepe-IKOTZ3G2IJRS_HarborFargateClusterDependencyTrackTaskDefinitionExecutionRoleDefaultPolicy8425CBAD" {
  name = "HarborFargateClusterDependencyTrackTaskDefinitionExecutionRoleDefaultPolicy8425CBAD"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "logs:CreateLogStream",
        "logs:PutLogEvents"
      ],
      "Effect": "Allow",
      "Resource": "arn:aws:logs:us-west-2:393419659647:log-group:sandbox-harbor-enrichment-usw2-HarborFargateClusterDependencyTrackTaskDefinitiondtContainerLogGroup7E08ADFB-PYqt9clwLgx5:*"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "sandbox-harbor-enrichment-HarborFargateClusterDepe-IKOTZ3G2IJRS"
}

resource "aws_iam_role_policy" "tfer--sandbox-harbor-enrichment-sandboxDependencyTrackIn-16OFNW74A35SH_sandboxDependencyTrackInterfaceusw2ServiceRoleDefaultPolicyFA87F6A0" {
  name = "sandboxDependencyTrackInterfaceusw2ServiceRoleDefaultPolicyFA87F6A0"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": "events:PutEvents",
      "Effect": "Allow",
      "Resource": "arn:aws:events:us-west-2:393419659647:event-bus/sandbox-HarborEnrichments-usw2"
    },
    {
      "Action": [
        "s3:PutObject",
        "s3:PutObjectLegalHold",
        "s3:PutObjectRetention",
        "s3:PutObjectTagging",
        "s3:PutObjectVersionTagging",
        "s3:Abort*"
      ],
      "Effect": "Allow",
      "Resource": "arn:aws:s3:::sandbox-harbor-sbom-uploads-enrichment-393419659647-usw2/*"
    },
    {
      "Action": [
        "s3:GetObject*",
        "s3:GetBucket*",
        "s3:List*",
        "s3:DeleteObject*",
        "s3:PutObject",
        "s3:PutObjectLegalHold",
        "s3:PutObjectRetention",
        "s3:PutObjectTagging",
        "s3:PutObjectVersionTagging",
        "s3:Abort*"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:s3:::sandbox-harbor-sbom-uploads-enrichment-393419659647-usw2",
        "arn:aws:s3:::sandbox-harbor-sbom-uploads-enrichment-393419659647-usw2/*"
      ]
    },
    {
      "Action": [
        "ssm:DescribeParameters",
        "ssm:GetParameters",
        "ssm:GetParameter",
        "ssm:GetParameterHistory"
      ],
      "Effect": "Allow",
      "Resource": "arn:aws:ssm:us-west-2:393419659647:parameter/sandbox_DT_ROOT_PWD_usw2"
    },
    {
      "Action": "ssm:PutParameter",
      "Effect": "Allow",
      "Resource": "arn:aws:ssm:us-west-2:393419659647:parameter/sandbox_DT_ROOT_PWD_usw2"
    },
    {
      "Action": [
        "ssm:DescribeParameters",
        "ssm:GetParameters",
        "ssm:GetParameter",
        "ssm:GetParameterHistory"
      ],
      "Effect": "Allow",
      "Resource": "arn:aws:ssm:us-west-2:393419659647:parameter/sandbox_DT_API_KEY_usw2"
    },
    {
      "Action": "ssm:PutParameter",
      "Effect": "Allow",
      "Resource": "arn:aws:ssm:us-west-2:393419659647:parameter/sandbox_DT_API_KEY_usw2"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "sandbox-harbor-enrichment-sandboxDependencyTrackIn-16OFNW74A35SH"
}

resource "aws_iam_role_policy" "tfer--sandbox-harbor-enrichment-sandboxIonChannelInterfa-1XZPOHEU8TXD2_sandboxIonChannelInterfaceusw2ServiceRoleDefaultPolicyC8A0B4DF" {
  name = "sandboxIonChannelInterfaceusw2ServiceRoleDefaultPolicyC8A0B4DF"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": "events:PutEvents",
      "Effect": "Allow",
      "Resource": "arn:aws:events:us-west-2:393419659647:event-bus/sandbox-HarborEnrichments-usw2"
    },
    {
      "Action": [
        "s3:PutObject",
        "s3:PutObjectLegalHold",
        "s3:PutObjectRetention",
        "s3:PutObjectTagging",
        "s3:PutObjectVersionTagging",
        "s3:Abort*"
      ],
      "Effect": "Allow",
      "Resource": "arn:aws:s3:::sandbox-harbor-sbom-uploads-enrichment-393419659647-usw2/*"
    },
    {
      "Action": [
        "s3:GetObject*",
        "s3:GetBucket*",
        "s3:List*",
        "s3:DeleteObject*",
        "s3:PutObject",
        "s3:PutObjectLegalHold",
        "s3:PutObjectRetention",
        "s3:PutObjectTagging",
        "s3:PutObjectVersionTagging",
        "s3:Abort*"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:s3:::sandbox-harbor-sbom-uploads-enrichment-393419659647-usw2",
        "arn:aws:s3:::sandbox-harbor-sbom-uploads-enrichment-393419659647-usw2/*"
      ]
    },
    {
      "Action": [
        "ssm:DescribeParameters",
        "ssm:GetParameters",
        "ssm:GetParameter",
        "ssm:GetParameterHistory"
      ],
      "Effect": "Allow",
      "Resource": "arn:aws:ssm:us-west-2:393419659647:parameter/sandbox_IC_API_KEY_usw2"
    },
    {
      "Action": [
        "ssm:DescribeParameters",
        "ssm:GetParameters",
        "ssm:GetParameter",
        "ssm:GetParameterHistory"
      ],
      "Effect": "Allow",
      "Resource": "arn:aws:ssm:us-west-2:393419659647:parameter/sandbox_IC_API_BASE_usw2"
    },
    {
      "Action": [
        "ssm:DescribeParameters",
        "ssm:GetParameters",
        "ssm:GetParameter",
        "ssm:GetParameterHistory"
      ],
      "Effect": "Allow",
      "Resource": "arn:aws:ssm:us-west-2:393419659647:parameter/sandbox_IC_RULESET_TEAM_ID_usw2"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "sandbox-harbor-enrichment-sandboxIonChannelInterfa-1XZPOHEU8TXD2"
}

resource "aws_iam_role_policy" "tfer--sandbox-harbor-enrichment-sandboxSBOMEnrichmentIng-1EZW673RQD559_sandboxSBOMEnrichmentIngressusw2ServiceRoleDefaultPolicyB2F4F671" {
  name = "sandboxSBOMEnrichmentIngressusw2ServiceRoleDefaultPolicyB2F4F671"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "s3:GetObject*",
        "s3:GetBucket*",
        "s3:List*"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:s3:::sandbox-harbor-sbom-uploads-enrichment-393419659647-usw2",
        "arn:aws:s3:::sandbox-harbor-sbom-uploads-enrichment-393419659647-usw2/*"
      ]
    },
    {
      "Action": "events:PutEvents",
      "Effect": "Allow",
      "Resource": "arn:aws:events:us-west-2:393419659647:event-bus/sandbox-HarborEnrichments-usw2"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "sandbox-harbor-enrichment-sandboxSBOMEnrichmentIng-1EZW673RQD559"
}

resource "aws_iam_role_policy" "tfer--sandbox-harbor-enrichment-sandboxSummarizerusw2Ser-N73UKG5RFO78_sandboxSummarizerusw2ServiceRoleDefaultPolicy007FC4FF" {
  name = "sandboxSummarizerusw2ServiceRoleDefaultPolicy007FC4FF"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": "events:PutEvents",
      "Effect": "Allow",
      "Resource": "arn:aws:events:us-west-2:393419659647:event-bus/sandbox-HarborEnrichments-usw2"
    },
    {
      "Action": [
        "s3:PutObject",
        "s3:PutObjectLegalHold",
        "s3:PutObjectRetention",
        "s3:PutObjectTagging",
        "s3:PutObjectVersionTagging",
        "s3:Abort*"
      ],
      "Effect": "Allow",
      "Resource": "arn:aws:s3:::sandbox-harbor-sbom-uploads-enrichment-393419659647-usw2/*"
    },
    {
      "Action": [
        "s3:GetObject*",
        "s3:GetBucket*",
        "s3:List*",
        "s3:DeleteObject*",
        "s3:PutObject",
        "s3:PutObjectLegalHold",
        "s3:PutObjectRetention",
        "s3:PutObjectTagging",
        "s3:PutObjectVersionTagging",
        "s3:Abort*"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:s3:::sandbox-harbor-sbom-uploads-enrichment-393419659647-usw2",
        "arn:aws:s3:::sandbox-harbor-sbom-uploads-enrichment-393419659647-usw2/*"
      ]
    },
    {
      "Action": [
        "dynamodb:BatchGetItem",
        "dynamodb:GetRecords",
        "dynamodb:GetShardIterator",
        "dynamodb:Query",
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:ConditionCheckItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:DescribeTable"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:dynamodb:us-west-2:393419659647:table/sandbox-HarborTeams-usw2"
      ]
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "sandbox-harbor-enrichment-sandboxSummarizerusw2Ser-N73UKG5RFO78"
}

resource "aws_iam_role_policy" "tfer--sandbox-harbor-shared-res-BucketNotificationsHandl-1OE2Y4QUIM3MH_BucketNotificationsHandler050a0587b7544547bf325f094a3db834RoleDefaultPolicy2CF63D36" {
  name = "BucketNotificationsHandler050a0587b7544547bf325f094a3db834RoleDefaultPolicy2CF63D36"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": "s3:PutBucketNotification",
      "Effect": "Allow",
      "Resource": "*"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "sandbox-harbor-shared-res-BucketNotificationsHandl-1OE2Y4QUIM3MH"
}

resource "aws_iam_role_policy" "tfer--sandbox-harbor-shared-reso-ReplicationRoleCE149CEC-T5KI4MILWWFJ_ReplicationRoleDefaultPolicy80AD15BB" {
  name = "ReplicationRoleDefaultPolicy80AD15BB"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "s3:GetReplicationConfiguration",
        "s3:ListBucket"
      ],
      "Effect": "Allow",
      "Resource": "arn:aws:s3:::sandbox-harbor-sbom-uploads-enrichment-393419659647-usw2"
    },
    {
      "Action": [
        "s3:GetObjectVersion",
        "s3:GetObjectVersionAcl",
        "s3:GetObjectVersionForReplication",
        "s3:GetObjectLegalHold",
        "s3:GetObjectVersionTagging",
        "s3:GetObjectRetention"
      ],
      "Effect": "Allow",
      "Resource": "arn:aws:s3:::sandbox-harbor-sbom-uploads-enrichment-393419659647-usw2/*"
    },
    {
      "Action": [
        "s3:ReplicateObject",
        "s3:ReplicateDelete",
        "s3:ReplicateTags",
        "s3:GetObjectVersionTagging",
        "s3:ObjectOwnerOverrideToBucketOwner"
      ],
      "Effect": "Allow",
      "Resource": "arn:aws:s3:::dev-harbor-sbom-summary-share-557147098836-use1/*"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  role = "sandbox-harbor-shared-reso-ReplicationRoleCE149CEC-T5KI4MILWWFJ"
}
