resource "aws_iam_role_policy_attachment" "tfer--AWSControlTowerExecution_AdministratorAccess" {
  policy_arn = "arn:aws:iam::aws:policy/AdministratorAccess"
  role       = "AWSControlTowerExecution"
}

resource "aws_iam_role_policy_attachment" "tfer--AWSControlTower_VPCFlowLogsRole_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "AWSControlTower_VPCFlowLogsRole"
}

resource "aws_iam_role_policy_attachment" "tfer--AWSReservedSSO_AWSAdministratorAccess_c2c07cf33a60ffc0_AdministratorAccess" {
  policy_arn = "arn:aws:iam::aws:policy/AdministratorAccess"
  role       = "AWSReservedSSO_AWSAdministratorAccess_c2c07cf33a60ffc0"
}

resource "aws_iam_role_policy_attachment" "tfer--AWSReservedSSO_AWSOrganizationsFullAccess_41009b386636cd27_AWSOrganizationsFullAccess" {
  policy_arn = "arn:aws:iam::aws:policy/AWSOrganizationsFullAccess"
  role       = "AWSReservedSSO_AWSOrganizationsFullAccess_41009b386636cd27"
}

resource "aws_iam_role_policy_attachment" "tfer--AWSReservedSSO_AWSPowerUserAccess_546815fe1bd97154_PowerUserAccess" {
  policy_arn = "arn:aws:iam::aws:policy/PowerUserAccess"
  role       = "AWSReservedSSO_AWSPowerUserAccess_546815fe1bd97154"
}

resource "aws_iam_role_policy_attachment" "tfer--AWSReservedSSO_AWSReadOnlyAccess_d4aae4dfcf0bc4c2_ViewOnlyAccess" {
  policy_arn = "arn:aws:iam::aws:policy/job-function/ViewOnlyAccess"
  role       = "AWSReservedSSO_AWSReadOnlyAccess_d4aae4dfcf0bc4c2"
}

resource "aws_iam_role_policy_attachment" "tfer--AWSServiceRoleForAmazonElasticFileSystem_AmazonElasticFileSystemServiceRolePolicy" {
  policy_arn = "arn:aws:iam::aws:policy/aws-service-role/AmazonElasticFileSystemServiceRolePolicy"
  role       = "AWSServiceRoleForAmazonElasticFileSystem"
}

resource "aws_iam_role_policy_attachment" "tfer--AWSServiceRoleForAmazonGuardDuty_AmazonGuardDutyServiceRolePolicy" {
  policy_arn = "arn:aws:iam::aws:policy/aws-service-role/AmazonGuardDutyServiceRolePolicy"
  role       = "AWSServiceRoleForAmazonGuardDuty"
}

resource "aws_iam_role_policy_attachment" "tfer--AWSServiceRoleForApplicationAutoScaling_DynamoDBTable_AWSApplicationAutoscalingDynamoDBTablePolicy" {
  policy_arn = "arn:aws:iam::aws:policy/aws-service-role/AWSApplicationAutoscalingDynamoDBTablePolicy"
  role       = "AWSServiceRoleForApplicationAutoScaling_DynamoDBTable"
}

resource "aws_iam_role_policy_attachment" "tfer--AWSServiceRoleForCloudFormationStackSetsOrgMember_CloudFormationStackSetsOrgMemberServiceRolePolicy" {
  policy_arn = "arn:aws:iam::aws:policy/aws-service-role/CloudFormationStackSetsOrgMemberServiceRolePolicy"
  role       = "AWSServiceRoleForCloudFormationStackSetsOrgMember"
}

resource "aws_iam_role_policy_attachment" "tfer--AWSServiceRoleForCloudTrail_CloudTrailServiceRolePolicy" {
  policy_arn = "arn:aws:iam::aws:policy/aws-service-role/CloudTrailServiceRolePolicy"
  role       = "AWSServiceRoleForCloudTrail"
}

resource "aws_iam_role_policy_attachment" "tfer--AWSServiceRoleForConfig_AWSConfigServiceRolePolicy" {
  policy_arn = "arn:aws:iam::aws:policy/aws-service-role/AWSConfigServiceRolePolicy"
  role       = "AWSServiceRoleForConfig"
}

resource "aws_iam_role_policy_attachment" "tfer--AWSServiceRoleForECS_AmazonECSServiceRolePolicy" {
  policy_arn = "arn:aws:iam::aws:policy/aws-service-role/AmazonECSServiceRolePolicy"
  role       = "AWSServiceRoleForECS"
}

resource "aws_iam_role_policy_attachment" "tfer--AWSServiceRoleForElasticLoadBalancing_AWSElasticLoadBalancingServiceRolePolicy" {
  policy_arn = "arn:aws:iam::aws:policy/aws-service-role/AWSElasticLoadBalancingServiceRolePolicy"
  role       = "AWSServiceRoleForElasticLoadBalancing"
}

resource "aws_iam_role_policy_attachment" "tfer--AWSServiceRoleForOrganizations_AWSOrganizationsServiceTrustPolicy" {
  policy_arn = "arn:aws:iam::aws:policy/aws-service-role/AWSOrganizationsServiceTrustPolicy"
  role       = "AWSServiceRoleForOrganizations"
}

resource "aws_iam_role_policy_attachment" "tfer--AWSServiceRoleForSSO_AWSSSOServiceRolePolicy" {
  policy_arn = "arn:aws:iam::aws:policy/aws-service-role/AWSSSOServiceRolePolicy"
  role       = "AWSServiceRoleForSSO"
}

resource "aws_iam_role_policy_attachment" "tfer--AWSServiceRoleForSecurityHub_AWSSecurityHubServiceRolePolicy" {
  policy_arn = "arn:aws:iam::aws:policy/aws-service-role/AWSSecurityHubServiceRolePolicy"
  role       = "AWSServiceRoleForSecurityHub"
}

resource "aws_iam_role_policy_attachment" "tfer--AWSServiceRoleForSupport_AWSSupportServiceRolePolicy" {
  policy_arn = "arn:aws:iam::aws:policy/aws-service-role/AWSSupportServiceRolePolicy"
  role       = "AWSServiceRoleForSupport"
}

resource "aws_iam_role_policy_attachment" "tfer--AWSServiceRoleForTrustedAdvisor_AWSTrustedAdvisorServiceRolePolicy" {
  policy_arn = "arn:aws:iam::aws:policy/aws-service-role/AWSTrustedAdvisorServiceRolePolicy"
  role       = "AWSServiceRoleForTrustedAdvisor"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-APIKeyAuthorizerServiceRole4E8-IBAT5I6C9PNY_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "SBOM-Management-Api-APIKeyAuthorizerServiceRole4E8-IBAT5I6C9PNY"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-APIKeyAuthorizerServiceRole4E8-IBAT5I6C9PNY_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "SBOM-Management-Api-APIKeyAuthorizerServiceRole4E8-IBAT5I6C9PNY"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-LoginLambdaServiceRoleBF816334-W7XUB9TKESGU_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "SBOM-Management-Api-LoginLambdaServiceRoleBF816334-W7XUB9TKESGU"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-LoginLambdaServiceRoleBF816334-W7XUB9TKESGU_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "SBOM-Management-Api-LoginLambdaServiceRoleBF816334-W7XUB9TKESGU"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMAPIAuthorizerAuthorizerSer-LP0PPL9842PZ_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "SBOM-Management-Api-SBOMAPIAuthorizerAuthorizerSer-LP0PPL9842PZ"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMAPIAuthorizerAuthorizerSer-LP0PPL9842PZ_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "SBOM-Management-Api-SBOMAPIAuthorizerAuthorizerSer-LP0PPL9842PZ"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMHarborCodebaseLambdaServic-FNMSV0WLE7SY_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "SBOM-Management-Api-SBOMHarborCodebaseLambdaServic-FNMSV0WLE7SY"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMHarborCodebaseLambdaServic-FNMSV0WLE7SY_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "SBOM-Management-Api-SBOMHarborCodebaseLambdaServic-FNMSV0WLE7SY"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMHarborCodebasePOSTLambdaSe-WTFVTQUCTUWN_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "SBOM-Management-Api-SBOMHarborCodebasePOSTLambdaSe-WTFVTQUCTUWN"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMHarborCodebasePOSTLambdaSe-WTFVTQUCTUWN_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "SBOM-Management-Api-SBOMHarborCodebasePOSTLambdaSe-WTFVTQUCTUWN"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMHarborCodebasesLambdaServi-5CJU7F7V2602_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "SBOM-Management-Api-SBOMHarborCodebasesLambdaServi-5CJU7F7V2602"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMHarborCodebasesLambdaServi-5CJU7F7V2602_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "SBOM-Management-Api-SBOMHarborCodebasesLambdaServi-5CJU7F7V2602"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMHarborMemberLambdaServiceR-1SE719MXDVF9V_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "SBOM-Management-Api-SBOMHarborMemberLambdaServiceR-1SE719MXDVF9V"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMHarborMemberLambdaServiceR-1SE719MXDVF9V_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "SBOM-Management-Api-SBOMHarborMemberLambdaServiceR-1SE719MXDVF9V"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMHarborMemberPOSTLambdaServ-146TE0NUMMGBI_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "SBOM-Management-Api-SBOMHarborMemberPOSTLambdaServ-146TE0NUMMGBI"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMHarborMemberPOSTLambdaServ-146TE0NUMMGBI_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "SBOM-Management-Api-SBOMHarborMemberPOSTLambdaServ-146TE0NUMMGBI"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMHarborMembersLambdaService-19LN7SS31SOV6_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "SBOM-Management-Api-SBOMHarborMembersLambdaService-19LN7SS31SOV6"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMHarborMembersLambdaService-19LN7SS31SOV6_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "SBOM-Management-Api-SBOMHarborMembersLambdaService-19LN7SS31SOV6"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMHarborProjectLambdaService-1DA4GKY55NY7C_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "SBOM-Management-Api-SBOMHarborProjectLambdaService-1DA4GKY55NY7C"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMHarborProjectLambdaService-1DA4GKY55NY7C_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "SBOM-Management-Api-SBOMHarborProjectLambdaService-1DA4GKY55NY7C"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMHarborProjectPOSTLambdaSer-70D8PDMD1BZM_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "SBOM-Management-Api-SBOMHarborProjectPOSTLambdaSer-70D8PDMD1BZM"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMHarborProjectPOSTLambdaSer-70D8PDMD1BZM_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "SBOM-Management-Api-SBOMHarborProjectPOSTLambdaSer-70D8PDMD1BZM"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMHarborProjectsLambdaServic-8HDO9BSLPW0_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "SBOM-Management-Api-SBOMHarborProjectsLambdaServic-8HDO9BSLPW0"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMHarborProjectsLambdaServic-8HDO9BSLPW0_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "SBOM-Management-Api-SBOMHarborProjectsLambdaServic-8HDO9BSLPW0"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMHarborTeamLambdaServiceRol-2YGJRNR2P7J3_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "SBOM-Management-Api-SBOMHarborTeamLambdaServiceRol-2YGJRNR2P7J3"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMHarborTeamLambdaServiceRol-2YGJRNR2P7J3_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "SBOM-Management-Api-SBOMHarborTeamLambdaServiceRol-2YGJRNR2P7J3"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMHarborTeamPOSTLambdaServic-1H1JYUAR9SQEV_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "SBOM-Management-Api-SBOMHarborTeamPOSTLambdaServic-1H1JYUAR9SQEV"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMHarborTeamPOSTLambdaServic-1H1JYUAR9SQEV_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "SBOM-Management-Api-SBOMHarborTeamPOSTLambdaServic-1H1JYUAR9SQEV"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMHarborTeamsLambdaServiceRo-1I6C3OHL8WMED_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "SBOM-Management-Api-SBOMHarborTeamsLambdaServiceRo-1I6C3OHL8WMED"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMHarborTeamsLambdaServiceRo-1I6C3OHL8WMED_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "SBOM-Management-Api-SBOMHarborTeamsLambdaServiceRo-1I6C3OHL8WMED"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMHarborTokenLambdaServiceRo-THV24GLM4RZG_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "SBOM-Management-Api-SBOMHarborTokenLambdaServiceRo-THV24GLM4RZG"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMHarborTokenLambdaServiceRo-THV24GLM4RZG_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "SBOM-Management-Api-SBOMHarborTokenLambdaServiceRo-THV24GLM4RZG"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMHarborTokenPOSTLambdaServi-GVF6EL2RI93Q_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "SBOM-Management-Api-SBOMHarborTokenPOSTLambdaServi-GVF6EL2RI93Q"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMHarborTokenPOSTLambdaServi-GVF6EL2RI93Q_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "SBOM-Management-Api-SBOMHarborTokenPOSTLambdaServi-GVF6EL2RI93Q"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMHarborTokensLambdaServiceR-1MN0Z6PRZMWV7_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "SBOM-Management-Api-SBOMHarborTokensLambdaServiceR-1MN0Z6PRZMWV7"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMHarborTokensLambdaServiceR-1MN0Z6PRZMWV7_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "SBOM-Management-Api-SBOMHarborTokensLambdaServiceR-1MN0Z6PRZMWV7"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMIngressLambdaServiceRole0D-1IEGFYDRHCHMU_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "SBOM-Management-Api-SBOMIngressLambdaServiceRole0D-1IEGFYDRHCHMU"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-SBOMIngressLambdaServiceRole0D-1IEGFYDRHCHMU_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "SBOM-Management-Api-SBOMIngressLambdaServiceRole0D-1IEGFYDRHCHMU"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-UserSearchLambdaServiceRoleB24-1I6PJQYXXW6FO_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "SBOM-Management-Api-UserSearchLambdaServiceRoleB24-1I6PJQYXXW6FO"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOM-Management-Api-UserSearchLambdaServiceRoleB24-1I6PJQYXXW6FO_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "SBOM-Management-Api-UserSearchLambdaServiceRoleB24-1I6PJQYXXW6FO"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOMApi-Enrichment-BucketNotificationsHandler050a0-WORBW5Y8PI4O_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "SBOMApi-Enrichment-BucketNotificationsHandler050a0-WORBW5Y8PI4O"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOMApi-Enrichment-CustomS3AutoDeleteObjectsCustom-VOKAN1K8PZEV_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "SBOMApi-Enrichment-CustomS3AutoDeleteObjectsCustom-VOKAN1K8PZEV"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOMApi-Enrichment-DefaultEnrichmentInterfaceLambd-VKGSNN0CAP2F_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "SBOMApi-Enrichment-DefaultEnrichmentInterfaceLambd-VKGSNN0CAP2F"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOMApi-Enrichment-DefaultEnrichmentInterfaceLambd-VKGSNN0CAP2F_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "SBOMApi-Enrichment-DefaultEnrichmentInterfaceLambd-VKGSNN0CAP2F"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOMApi-Enrichment-DependencyTrackInterfaceLambdaS-153Y3Q0VCTZJH_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "SBOMApi-Enrichment-DependencyTrackInterfaceLambdaS-153Y3Q0VCTZJH"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOMApi-Enrichment-DependencyTrackInterfaceLambdaS-153Y3Q0VCTZJH_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "SBOMApi-Enrichment-DependencyTrackInterfaceLambdaS-153Y3Q0VCTZJH"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOMApi-Enrichment-SBOMEnrichmentIngressLambdaServ-GC5O9QX4V5H1_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "SBOMApi-Enrichment-SBOMEnrichmentIngressLambdaServ-GC5O9QX4V5H1"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOMApi-Enrichment-SBOMEnrichmentIngressLambdaServ-GC5O9QX4V5H1_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "SBOMApi-Enrichment-SBOMEnrichmentIngressLambdaServ-GC5O9QX4V5H1"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOMApi-Enrichment-SummarizerLambdaServiceRoleEEF5-1V7DA30FJTF4X_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "SBOMApi-Enrichment-SummarizerLambdaServiceRoleEEF5-1V7DA30FJTF4X"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOMApi-Enrichment-SummarizerLambdaServiceRoleEEF5-1V7DA30FJTF4X_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "SBOMApi-Enrichment-SummarizerLambdaServiceRoleEEF5-1V7DA30FJTF4X"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOMApi-Shared-Resource-BucketNotificationsHandler-931RZ9L19VYI_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "SBOMApi-Shared-Resource-BucketNotificationsHandler-931RZ9L19VYI"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOMApi-Shared-Resource-CustomS3AutoDeleteObjectsC-1CYVYQLWTF64P_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "SBOMApi-Shared-Resource-CustomS3AutoDeleteObjectsC-1CYVYQLWTF64P"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOMUserRole_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "SBOMUserRole"
}

resource "aws_iam_role_policy_attachment" "tfer--SBOMUserRole_AmazonS3FullAccess" {
  policy_arn = "arn:aws:iam::aws:policy/AmazonS3FullAccess"
  role       = "SBOMUserRole"
}

resource "aws_iam_role_policy_attachment" "tfer--aws-controltower-AdministratorExecutionRole_AdministratorAccess" {
  policy_arn = "arn:aws:iam::aws:policy/AdministratorAccess"
  role       = "aws-controltower-AdministratorExecutionRole"
}

resource "aws_iam_role_policy_attachment" "tfer--aws-controltower-ConfigRecorderRole_AWS_ConfigRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWS_ConfigRole"
  role       = "aws-controltower-ConfigRecorderRole"
}

resource "aws_iam_role_policy_attachment" "tfer--aws-controltower-ConfigRecorderRole_ReadOnlyAccess" {
  policy_arn = "arn:aws:iam::aws:policy/ReadOnlyAccess"
  role       = "aws-controltower-ConfigRecorderRole"
}

resource "aws_iam_role_policy_attachment" "tfer--aws-controltower-ForwardSnsNotificationRole_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "aws-controltower-ForwardSnsNotificationRole"
}

resource "aws_iam_role_policy_attachment" "tfer--aws-controltower-ReadOnlyExecutionRole_ReadOnlyAccess" {
  policy_arn = "arn:aws:iam::aws:policy/ReadOnlyAccess"
  role       = "aws-controltower-ReadOnlyExecutionRole"
}

resource "aws_iam_role_policy_attachment" "tfer--cdk-hnb659fds-cfn-exec-role-393419659647-us-east-2_AdministratorAccess" {
  policy_arn = "arn:aws:iam::aws:policy/AdministratorAccess"
  role       = "cdk-hnb659fds-cfn-exec-role-393419659647-us-east-2"
}

resource "aws_iam_role_policy_attachment" "tfer--cdk-hnb659fds-cfn-exec-role-393419659647-us-west-2_AdministratorAccess" {
  policy_arn = "arn:aws:iam::aws:policy/AdministratorAccess"
  role       = "cdk-hnb659fds-cfn-exec-role-393419659647-us-west-2"
}

resource "aws_iam_role_policy_attachment" "tfer--cdk-hnb659fds-lookup-role-393419659647-us-east-2_ReadOnlyAccess" {
  policy_arn = "arn:aws:iam::aws:policy/ReadOnlyAccess"
  role       = "cdk-hnb659fds-lookup-role-393419659647-us-east-2"
}

resource "aws_iam_role_policy_attachment" "tfer--cdk-hnb659fds-lookup-role-393419659647-us-west-2_ReadOnlyAccess" {
  policy_arn = "arn:aws:iam::aws:policy/ReadOnlyAccess"
  role       = "cdk-hnb659fds-lookup-role-393419659647-us-west-2"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-CognitoUser-usw2_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "sandbox-CognitoUser-usw2"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-CognitoUser-usw2_AmazonS3FullAccess" {
  policy_arn = "arn:aws:iam::aws:policy/AmazonS3FullAccess"
  role       = "sandbox-CognitoUser-usw2"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxAPIKeyAuthorizeru-J6G3ARPEMAB8_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxAPIKeyAuthorizeru-J6G3ARPEMAB8"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxAPIKeyAuthorizeru-J6G3ARPEMAB8_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxAPIKeyAuthorizeru-J6G3ARPEMAB8"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxHarborCodebaseLam-JV3C9SPDAZND_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxHarborCodebaseLam-JV3C9SPDAZND"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxHarborCodebaseLam-JV3C9SPDAZND_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxHarborCodebaseLam-JV3C9SPDAZND"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxHarborCodebasePOS-1KEMFXHFN45G9_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxHarborCodebasePOS-1KEMFXHFN45G9"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxHarborCodebasePOS-1KEMFXHFN45G9_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxHarborCodebasePOS-1KEMFXHFN45G9"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxHarborCodebasesLa-1GXA10VAOSWZ8_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxHarborCodebasesLa-1GXA10VAOSWZ8"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxHarborCodebasesLa-1GXA10VAOSWZ8_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxHarborCodebasesLa-1GXA10VAOSWZ8"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxHarborMemberLambd-IX1FUENAP5PG_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxHarborMemberLambd-IX1FUENAP5PG"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxHarborMemberLambd-IX1FUENAP5PG_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxHarborMemberLambd-IX1FUENAP5PG"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxHarborMemberPOSTL-R9N72L26UAYS_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxHarborMemberPOSTL-R9N72L26UAYS"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxHarborMemberPOSTL-R9N72L26UAYS_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxHarborMemberPOSTL-R9N72L26UAYS"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxHarborMembersLamb-1I0LXSVO9QGWI_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxHarborMembersLamb-1I0LXSVO9QGWI"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxHarborMembersLamb-1I0LXSVO9QGWI_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxHarborMembersLamb-1I0LXSVO9QGWI"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxHarborProjectLamb-DXYP13V3MR7Y_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxHarborProjectLamb-DXYP13V3MR7Y"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxHarborProjectLamb-DXYP13V3MR7Y_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxHarborProjectLamb-DXYP13V3MR7Y"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxHarborProjectPOST-NCR778JYL4FJ_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxHarborProjectPOST-NCR778JYL4FJ"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxHarborProjectPOST-NCR778JYL4FJ_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxHarborProjectPOST-NCR778JYL4FJ"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxHarborProjectsLam-IVSYK5W51ZY7_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxHarborProjectsLam-IVSYK5W51ZY7"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxHarborProjectsLam-IVSYK5W51ZY7_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxHarborProjectsLam-IVSYK5W51ZY7"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxHarborTeamLambdau-1SUMZV3BOCS2O_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxHarborTeamLambdau-1SUMZV3BOCS2O"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxHarborTeamLambdau-1SUMZV3BOCS2O_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxHarborTeamLambdau-1SUMZV3BOCS2O"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxHarborTeamPOSTLam-1M9DCBMYMUTZ9_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxHarborTeamPOSTLam-1M9DCBMYMUTZ9"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxHarborTeamPOSTLam-1M9DCBMYMUTZ9_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxHarborTeamPOSTLam-1M9DCBMYMUTZ9"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxHarborTeamsLambda-97ORIE58LYQK_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxHarborTeamsLambda-97ORIE58LYQK"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxHarborTeamsLambda-97ORIE58LYQK_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxHarborTeamsLambda-97ORIE58LYQK"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxHarborTokenLambda-4J703PBNIS81_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxHarborTokenLambda-4J703PBNIS81"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxHarborTokenLambda-4J703PBNIS81_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxHarborTokenLambda-4J703PBNIS81"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxHarborTokenPOSTLa-1WNCMTFZ75DLF_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxHarborTokenPOSTLa-1WNCMTFZ75DLF"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxHarborTokenPOSTLa-1WNCMTFZ75DLF_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxHarborTokenPOSTLa-1WNCMTFZ75DLF"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxHarborTokensLambd-3BH0LJLJN4F6_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxHarborTokensLambd-3BH0LJLJN4F6"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxHarborTokensLambd-3BH0LJLJN4F6_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxHarborTokensLambd-3BH0LJLJN4F6"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxJwtTokenAuthorize-1MM2C42NKJCYH_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxJwtTokenAuthorize-1MM2C42NKJCYH"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxJwtTokenAuthorize-1MM2C42NKJCYH_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxJwtTokenAuthorize-1MM2C42NKJCYH"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxLoginusw2ServiceR-D69CUROSBKHL_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxLoginusw2ServiceR-D69CUROSBKHL"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxLoginusw2ServiceR-D69CUROSBKHL_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxLoginusw2ServiceR-D69CUROSBKHL"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxSBOMIngressusw2Se-WRDZ6YKTOSC0_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxSBOMIngressusw2Se-WRDZ6YKTOSC0"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxSBOMIngressusw2Se-WRDZ6YKTOSC0_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxSBOMIngressusw2Se-WRDZ6YKTOSC0"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxUserSearchusw2Ser-EZE8AIWUFWN7_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxUserSearchusw2Ser-EZE8AIWUFWN7"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-backend-us-sandboxUserSearchusw2Ser-EZE8AIWUFWN7_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "sandbox-harbor-backend-us-sandboxUserSearchusw2Ser-EZE8AIWUFWN7"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-enrichment-BucketNotificationsHandl-1HERPSPAMUYO4_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "sandbox-harbor-enrichment-BucketNotificationsHandl-1HERPSPAMUYO4"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-enrichment-sandboxDependencyTrackIn-16OFNW74A35SH_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "sandbox-harbor-enrichment-sandboxDependencyTrackIn-16OFNW74A35SH"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-enrichment-sandboxDependencyTrackIn-16OFNW74A35SH_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "sandbox-harbor-enrichment-sandboxDependencyTrackIn-16OFNW74A35SH"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-enrichment-sandboxIonChannelInterfa-1XZPOHEU8TXD2_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "sandbox-harbor-enrichment-sandboxIonChannelInterfa-1XZPOHEU8TXD2"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-enrichment-sandboxIonChannelInterfa-1XZPOHEU8TXD2_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "sandbox-harbor-enrichment-sandboxIonChannelInterfa-1XZPOHEU8TXD2"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-enrichment-sandboxSBOMEnrichmentIng-1EZW673RQD559_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "sandbox-harbor-enrichment-sandboxSBOMEnrichmentIng-1EZW673RQD559"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-enrichment-sandboxSBOMEnrichmentIng-1EZW673RQD559_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "sandbox-harbor-enrichment-sandboxSBOMEnrichmentIng-1EZW673RQD559"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-enrichment-sandboxSummarizerusw2Ser-N73UKG5RFO78_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "sandbox-harbor-enrichment-sandboxSummarizerusw2Ser-N73UKG5RFO78"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-enrichment-sandboxSummarizerusw2Ser-N73UKG5RFO78_AWSLambdaVPCAccessExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = "sandbox-harbor-enrichment-sandboxSummarizerusw2Ser-N73UKG5RFO78"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-pilot-usw2-sandboxPilotusw2ServiceR-MSTMBB1ZMJQY_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "sandbox-harbor-pilot-usw2-sandboxPilotusw2ServiceR-MSTMBB1ZMJQY"
}

resource "aws_iam_role_policy_attachment" "tfer--sandbox-harbor-shared-res-BucketNotificationsHandl-1OE2Y4QUIM3MH_AWSLambdaBasicExecutionRole" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = "sandbox-harbor-shared-res-BucketNotificationsHandl-1OE2Y4QUIM3MH"
}

resource "aws_iam_role_policy_attachment" "tfer--stacksets-exec-d760af5b917175709402d1ec524a2012_AdministratorAccess" {
  policy_arn = "arn:aws:iam::aws:policy/AdministratorAccess"
  role       = "stacksets-exec-d760af5b917175709402d1ec524a2012"
}
