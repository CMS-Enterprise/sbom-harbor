resource "aws_security_group" "tfer--default_sg-0741932c0b37540ed" {
  description = "default VPC security group"

  egress {
    cidr_blocks = ["0.0.0.0/0"]
    from_port   = "0"
    protocol    = "-1"
    self        = "false"
    to_port     = "0"
  }

  ingress {
    from_port = "0"
    protocol  = "-1"
    self      = "true"
    to_port   = "0"
  }

  name   = "default"
  vpc_id = "vpc-045a1b5658854587f"
}

resource "aws_security_group" "tfer--default_sg-08d1dad020c09d2a5" {
  description = "default VPC security group"

  egress {
    cidr_blocks = ["0.0.0.0/0"]
    from_port   = "0"
    protocol    = "-1"
    self        = "false"
    to_port     = "0"
  }

  ingress {
    from_port = "0"
    protocol  = "-1"
    self      = "true"
    to_port   = "0"
  }

  name   = "default"
  vpc_id = "vpc-0254d1f2cb183a8dc"
}

resource "aws_security_group" "tfer--sandbox-harbor-backend-usw2-sandboxAPIKeyAuthorizerusw2SecurityGroupA588B3E0-16J2WURVC3LQS_sg-0e1b7ec8aaf0cdea9" {
  description = "Automatic security group for Lambda Function sandboxharborbackendusw2sandboxAPIKeyAuthorizerusw29C67E07C"

  egress {
    cidr_blocks = ["0.0.0.0/0"]
    description = "Allow all outbound traffic by default"
    from_port   = "0"
    protocol    = "-1"
    self        = "false"
    to_port     = "0"
  }

  name   = "sandbox-harbor-backend-usw2-sandboxAPIKeyAuthorizerusw2SecurityGroupA588B3E0-16J2WURVC3LQS"
  vpc_id = "vpc-0254d1f2cb183a8dc"
}

resource "aws_security_group" "tfer--sandbox-harbor-backend-usw2-sandboxHarborCodebaseLambdausw2SecurityGroup0A34ABEA-12V03MLCLE06B_sg-0a7e8a899d7af1295" {
  description = "Automatic security group for Lambda Function sandboxharborbackendusw2sandboxHarborCodebaseLambdausw2349AD163"

  egress {
    cidr_blocks = ["0.0.0.0/0"]
    description = "Allow all outbound traffic by default"
    from_port   = "0"
    protocol    = "-1"
    self        = "false"
    to_port     = "0"
  }

  name   = "sandbox-harbor-backend-usw2-sandboxHarborCodebaseLambdausw2SecurityGroup0A34ABEA-12V03MLCLE06B"
  vpc_id = "vpc-0254d1f2cb183a8dc"
}

resource "aws_security_group" "tfer--sandbox-harbor-backend-usw2-sandboxHarborCodebasePOSTLambdausw2SecurityGroup3D380A36-1HWP9T4VUAJSG_sg-0281f9d6ca7d7f5a3" {
  description = "Automatic security group for Lambda Function sandboxharborbackendusw2sandboxHarborCodebasePOSTLambdausw2FC476788"

  egress {
    cidr_blocks = ["0.0.0.0/0"]
    description = "Allow all outbound traffic by default"
    from_port   = "0"
    protocol    = "-1"
    self        = "false"
    to_port     = "0"
  }

  name   = "sandbox-harbor-backend-usw2-sandboxHarborCodebasePOSTLambdausw2SecurityGroup3D380A36-1HWP9T4VUAJSG"
  vpc_id = "vpc-0254d1f2cb183a8dc"
}

resource "aws_security_group" "tfer--sandbox-harbor-backend-usw2-sandboxHarborCodebasesLambdausw2SecurityGroup026BC645-1AZJN695N78FK_sg-0abf873864d3cb15d" {
  description = "Automatic security group for Lambda Function sandboxharborbackendusw2sandboxHarborCodebasesLambdausw29E4417F4"

  egress {
    cidr_blocks = ["0.0.0.0/0"]
    description = "Allow all outbound traffic by default"
    from_port   = "0"
    protocol    = "-1"
    self        = "false"
    to_port     = "0"
  }

  name   = "sandbox-harbor-backend-usw2-sandboxHarborCodebasesLambdausw2SecurityGroup026BC645-1AZJN695N78FK"
  vpc_id = "vpc-0254d1f2cb183a8dc"
}

resource "aws_security_group" "tfer--sandbox-harbor-backend-usw2-sandboxHarborMemberLambdausw2SecurityGroup87D4792C-LTLW1ERLW4TH_sg-08b0e7baf38bec05a" {
  description = "Automatic security group for Lambda Function sandboxharborbackendusw2sandboxHarborMemberLambdausw2AE5F6535"

  egress {
    cidr_blocks = ["0.0.0.0/0"]
    description = "Allow all outbound traffic by default"
    from_port   = "0"
    protocol    = "-1"
    self        = "false"
    to_port     = "0"
  }

  name   = "sandbox-harbor-backend-usw2-sandboxHarborMemberLambdausw2SecurityGroup87D4792C-LTLW1ERLW4TH"
  vpc_id = "vpc-0254d1f2cb183a8dc"
}

resource "aws_security_group" "tfer--sandbox-harbor-backend-usw2-sandboxHarborMemberPOSTLambdausw2SecurityGroup3C920516-Q7SJ0ZB7ARPP_sg-0ed04744d3a304471" {
  description = "Automatic security group for Lambda Function sandboxharborbackendusw2sandboxHarborMemberPOSTLambdausw2C0C02B34"

  egress {
    cidr_blocks = ["0.0.0.0/0"]
    description = "Allow all outbound traffic by default"
    from_port   = "0"
    protocol    = "-1"
    self        = "false"
    to_port     = "0"
  }

  name   = "sandbox-harbor-backend-usw2-sandboxHarborMemberPOSTLambdausw2SecurityGroup3C920516-Q7SJ0ZB7ARPP"
  vpc_id = "vpc-0254d1f2cb183a8dc"
}

resource "aws_security_group" "tfer--sandbox-harbor-backend-usw2-sandboxHarborMembersLambdausw2SecurityGroup20212BE9-18KV37DJ4DXAI_sg-068dcba0b8397770f" {
  description = "Automatic security group for Lambda Function sandboxharborbackendusw2sandboxHarborMembersLambdausw24F5A77A4"

  egress {
    cidr_blocks = ["0.0.0.0/0"]
    description = "Allow all outbound traffic by default"
    from_port   = "0"
    protocol    = "-1"
    self        = "false"
    to_port     = "0"
  }

  name   = "sandbox-harbor-backend-usw2-sandboxHarborMembersLambdausw2SecurityGroup20212BE9-18KV37DJ4DXAI"
  vpc_id = "vpc-0254d1f2cb183a8dc"
}

resource "aws_security_group" "tfer--sandbox-harbor-backend-usw2-sandboxHarborProjectLambdausw2SecurityGroup9275CE3E-PWRNFS8046LS_sg-0b4a3b500a0362192" {
  description = "Automatic security group for Lambda Function sandboxharborbackendusw2sandboxHarborProjectLambdausw2B9896C68"

  egress {
    cidr_blocks = ["0.0.0.0/0"]
    description = "Allow all outbound traffic by default"
    from_port   = "0"
    protocol    = "-1"
    self        = "false"
    to_port     = "0"
  }

  name   = "sandbox-harbor-backend-usw2-sandboxHarborProjectLambdausw2SecurityGroup9275CE3E-PWRNFS8046LS"
  vpc_id = "vpc-0254d1f2cb183a8dc"
}

resource "aws_security_group" "tfer--sandbox-harbor-backend-usw2-sandboxHarborProjectPOSTLambdausw2SecurityGroupF1B83E2E-TI7EG0X3ZWQP_sg-038ba9dccbab55c9d" {
  description = "Automatic security group for Lambda Function sandboxharborbackendusw2sandboxHarborProjectPOSTLambdausw27AE061EF"

  egress {
    cidr_blocks = ["0.0.0.0/0"]
    description = "Allow all outbound traffic by default"
    from_port   = "0"
    protocol    = "-1"
    self        = "false"
    to_port     = "0"
  }

  name   = "sandbox-harbor-backend-usw2-sandboxHarborProjectPOSTLambdausw2SecurityGroupF1B83E2E-TI7EG0X3ZWQP"
  vpc_id = "vpc-0254d1f2cb183a8dc"
}

resource "aws_security_group" "tfer--sandbox-harbor-backend-usw2-sandboxHarborProjectsLambdausw2SecurityGroupB20F0A89-1JVWPMJWBID5F_sg-0db29fd9e8b080c1b" {
  description = "Automatic security group for Lambda Function sandboxharborbackendusw2sandboxHarborProjectsLambdausw27E209CEE"

  egress {
    cidr_blocks = ["0.0.0.0/0"]
    description = "Allow all outbound traffic by default"
    from_port   = "0"
    protocol    = "-1"
    self        = "false"
    to_port     = "0"
  }

  name   = "sandbox-harbor-backend-usw2-sandboxHarborProjectsLambdausw2SecurityGroupB20F0A89-1JVWPMJWBID5F"
  vpc_id = "vpc-0254d1f2cb183a8dc"
}

resource "aws_security_group" "tfer--sandbox-harbor-backend-usw2-sandboxHarborTeamLambdausw2SecurityGroup4FC76CF7-RY8OBN3XJW9O_sg-07042daf33b6c5d6f" {
  description = "Automatic security group for Lambda Function sandboxharborbackendusw2sandboxHarborTeamLambdausw2144AB195"

  egress {
    cidr_blocks = ["0.0.0.0/0"]
    description = "Allow all outbound traffic by default"
    from_port   = "0"
    protocol    = "-1"
    self        = "false"
    to_port     = "0"
  }

  name   = "sandbox-harbor-backend-usw2-sandboxHarborTeamLambdausw2SecurityGroup4FC76CF7-RY8OBN3XJW9O"
  vpc_id = "vpc-0254d1f2cb183a8dc"
}

resource "aws_security_group" "tfer--sandbox-harbor-backend-usw2-sandboxHarborTeamPOSTLambdausw2SecurityGroupA1EDD63E-1ANKWS6XVNRL0_sg-0162618c8c5b12964" {
  description = "Automatic security group for Lambda Function sandboxharborbackendusw2sandboxHarborTeamPOSTLambdausw20BE126D2"

  egress {
    cidr_blocks = ["0.0.0.0/0"]
    description = "Allow all outbound traffic by default"
    from_port   = "0"
    protocol    = "-1"
    self        = "false"
    to_port     = "0"
  }

  name   = "sandbox-harbor-backend-usw2-sandboxHarborTeamPOSTLambdausw2SecurityGroupA1EDD63E-1ANKWS6XVNRL0"
  vpc_id = "vpc-0254d1f2cb183a8dc"
}

resource "aws_security_group" "tfer--sandbox-harbor-backend-usw2-sandboxHarborTeamsLambdausw2SecurityGroupEC98022C-1JMGN921ZSP7W_sg-09768da74014838ea" {
  description = "Automatic security group for Lambda Function sandboxharborbackendusw2sandboxHarborTeamsLambdausw261BDCCF7"

  egress {
    cidr_blocks = ["0.0.0.0/0"]
    description = "Allow all outbound traffic by default"
    from_port   = "0"
    protocol    = "-1"
    self        = "false"
    to_port     = "0"
  }

  name   = "sandbox-harbor-backend-usw2-sandboxHarborTeamsLambdausw2SecurityGroupEC98022C-1JMGN921ZSP7W"
  vpc_id = "vpc-0254d1f2cb183a8dc"
}

resource "aws_security_group" "tfer--sandbox-harbor-backend-usw2-sandboxHarborTokenLambdausw2SecurityGroup8B803048-1198U9PLKR4DF_sg-08eff3434ed91757f" {
  description = "Automatic security group for Lambda Function sandboxharborbackendusw2sandboxHarborTokenLambdausw20A500E0A"

  egress {
    cidr_blocks = ["0.0.0.0/0"]
    description = "Allow all outbound traffic by default"
    from_port   = "0"
    protocol    = "-1"
    self        = "false"
    to_port     = "0"
  }

  name   = "sandbox-harbor-backend-usw2-sandboxHarborTokenLambdausw2SecurityGroup8B803048-1198U9PLKR4DF"
  vpc_id = "vpc-0254d1f2cb183a8dc"
}

resource "aws_security_group" "tfer--sandbox-harbor-backend-usw2-sandboxHarborTokenPOSTLambdausw2SecurityGroup13FEAC48-1SJT3008NJUO6_sg-06bc0090e300d1433" {
  description = "Automatic security group for Lambda Function sandboxharborbackendusw2sandboxHarborTokenPOSTLambdausw2E7B54607"

  egress {
    cidr_blocks = ["0.0.0.0/0"]
    description = "Allow all outbound traffic by default"
    from_port   = "0"
    protocol    = "-1"
    self        = "false"
    to_port     = "0"
  }

  name   = "sandbox-harbor-backend-usw2-sandboxHarborTokenPOSTLambdausw2SecurityGroup13FEAC48-1SJT3008NJUO6"
  vpc_id = "vpc-0254d1f2cb183a8dc"
}

resource "aws_security_group" "tfer--sandbox-harbor-backend-usw2-sandboxHarborTokensLambdausw2SecurityGroup36BC7EC0-1OYUHD3U01JO7_sg-063eb402b673689ce" {
  description = "Automatic security group for Lambda Function sandboxharborbackendusw2sandboxHarborTokensLambdausw2E1E1D111"

  egress {
    cidr_blocks = ["0.0.0.0/0"]
    description = "Allow all outbound traffic by default"
    from_port   = "0"
    protocol    = "-1"
    self        = "false"
    to_port     = "0"
  }

  name   = "sandbox-harbor-backend-usw2-sandboxHarborTokensLambdausw2SecurityGroup36BC7EC0-1OYUHD3U01JO7"
  vpc_id = "vpc-0254d1f2cb183a8dc"
}

resource "aws_security_group" "tfer--sandbox-harbor-backend-usw2-sandboxJwtTokenAuthorizerusw2SecurityGroup54D487E2-1JEWVOWC7FK2L_sg-05a051d939389afd8" {
  description = "Automatic security group for Lambda Function sandboxharborbackendusw2sandboxJwtTokenAuthorizerusw23092B9B0"

  egress {
    cidr_blocks = ["0.0.0.0/0"]
    description = "Allow all outbound traffic by default"
    from_port   = "0"
    protocol    = "-1"
    self        = "false"
    to_port     = "0"
  }

  name   = "sandbox-harbor-backend-usw2-sandboxJwtTokenAuthorizerusw2SecurityGroup54D487E2-1JEWVOWC7FK2L"
  vpc_id = "vpc-0254d1f2cb183a8dc"
}

resource "aws_security_group" "tfer--sandbox-harbor-backend-usw2-sandboxLoginusw2SecurityGroup0637C3EB-QKVVCO0V6PB2_sg-0a52ef40545b235b6" {
  description = "Automatic security group for Lambda Function sandboxharborbackendusw2sandboxLoginusw2A10CB7E1"

  egress {
    cidr_blocks = ["0.0.0.0/0"]
    description = "Allow all outbound traffic by default"
    from_port   = "0"
    protocol    = "-1"
    self        = "false"
    to_port     = "0"
  }

  name   = "sandbox-harbor-backend-usw2-sandboxLoginusw2SecurityGroup0637C3EB-QKVVCO0V6PB2"
  vpc_id = "vpc-0254d1f2cb183a8dc"
}

resource "aws_security_group" "tfer--sandbox-harbor-backend-usw2-sandboxSBOMIngressusw2SecurityGroup5D9C4E3A-1MLJRXPG2IBKY_sg-035910c3f2696083f" {
  description = "Automatic security group for Lambda Function sandboxharborbackendusw2sandboxSBOMIngressusw25B6E84B1"

  egress {
    cidr_blocks = ["0.0.0.0/0"]
    description = "Allow all outbound traffic by default"
    from_port   = "0"
    protocol    = "-1"
    self        = "false"
    to_port     = "0"
  }

  name   = "sandbox-harbor-backend-usw2-sandboxSBOMIngressusw2SecurityGroup5D9C4E3A-1MLJRXPG2IBKY"
  vpc_id = "vpc-0254d1f2cb183a8dc"
}

resource "aws_security_group" "tfer--sandbox-harbor-backend-usw2-sandboxUserSearchusw2SecurityGroup5F223F4F-ZIYYX5PKZZ2I_sg-079e04c0ab035b95e" {
  description = "Automatic security group for Lambda Function sandboxharborbackendusw2sandboxUserSearchusw292C9A803"

  egress {
    cidr_blocks = ["0.0.0.0/0"]
    description = "Allow all outbound traffic by default"
    from_port   = "0"
    protocol    = "-1"
    self        = "false"
    to_port     = "0"
  }

  name   = "sandbox-harbor-backend-usw2-sandboxUserSearchusw2SecurityGroup5F223F4F-ZIYYX5PKZZ2I"
  vpc_id = "vpc-0254d1f2cb183a8dc"
}

resource "aws_security_group" "tfer--sandbox-harbor-enrichment-usw2-DEPENDENCYTRACKLOADBALANCERDEPENDENCYTRACKLOADBALANCERSECURITYGROUP8CB2E209-UW3J8975T63V_sg-04a63d76aba331aab" {
  description = "sandbox-harbor-enrichment-usw2/DEPENDENCY-TRACK-LOAD-BALANCER/DEPENDENCY-TRACK-LOAD-BALANCER-SECURITY-GROUP"

  egress {
    cidr_blocks = ["0.0.0.0/0"]
    description = "Allow all outbound traffic by default"
    from_port   = "0"
    protocol    = "-1"
    self        = "false"
    to_port     = "0"
  }

  ingress {
    cidr_blocks = ["0.0.0.0/0"]
    description = "from 0.0.0.0/0:8080"
    from_port   = "8080"
    protocol    = "tcp"
    self        = "false"
    to_port     = "8080"
  }

  name   = "sandbox-harbor-enrichment-usw2-DEPENDENCYTRACKLOADBALANCERDEPENDENCYTRACKLOADBALANCERSECURITYGROUP8CB2E209-UW3J8975T63V"
  vpc_id = "vpc-0254d1f2cb183a8dc"
}

resource "aws_security_group" "tfer--sandbox-harbor-enrichment-usw2-HarborFargateClusterALLOW8080SGA923B51F-1G7B19M1MHMW2_sg-07277d8669bb21226" {
  description = "sandbox-harbor-enrichment-usw2/HarborFargateCluster/ALLOW_8080_SG"

  egress {
    cidr_blocks = ["0.0.0.0/0"]
    description = "Allow all outbound traffic by default"
    from_port   = "0"
    protocol    = "-1"
    self        = "false"
    to_port     = "0"
  }

  ingress {
    cidr_blocks = ["0.0.0.0/0"]
    description = "from 0.0.0.0/0:8080"
    from_port   = "8080"
    protocol    = "tcp"
    self        = "false"
    to_port     = "8080"
  }

  ingress {
    description     = "Load balancer to target"
    from_port       = "8080"
    protocol        = "tcp"
    security_groups = ["${data.terraform_remote_state.sg.outputs.aws_security_group_tfer--sandbox-harbor-enrichment-usw2-DEPENDENCYTRACKLOADBALANCERDEPENDENCYTRACKLOADBALANCERSECURITYGROUP8CB2E209-UW3J8975T63V_sg-04a63d76aba331aab_id}"]
    self            = "false"
    to_port         = "8080"
  }

  name   = "sandbox-harbor-enrichment-usw2-HarborFargateClusterALLOW8080SGA923B51F-1G7B19M1MHMW2"
  vpc_id = "vpc-0254d1f2cb183a8dc"
}

resource "aws_security_group" "tfer--sandbox-harbor-enrichment-usw2-HarborFargateClusterdtApiStorageEfsSecurityGroupB840FBB2-1VSQXAGGZQEQL_sg-0c5bcf59f841c929c" {
  description = "sandbox-harbor-enrichment-usw2/HarborFargateCluster/dtApiStorage/EfsSecurityGroup"

  egress {
    cidr_blocks = ["0.0.0.0/0"]
    description = "Allow all outbound traffic by default"
    from_port   = "0"
    protocol    = "-1"
    self        = "false"
    to_port     = "0"
  }

  ingress {
    description     = "from sandboxharborenrichmentusw2HarborFargateClusterALLOW8080SGDFB2B8EE:2049"
    from_port       = "2049"
    protocol        = "tcp"
    security_groups = ["${data.terraform_remote_state.sg.outputs.aws_security_group_tfer--sandbox-harbor-enrichment-usw2-HarborFargateClusterALLOW8080SGA923B51F-1G7B19M1MHMW2_sg-07277d8669bb21226_id}"]
    self            = "false"
    to_port         = "2049"
  }

  name = "sandbox-harbor-enrichment-usw2-HarborFargateClusterdtApiStorageEfsSecurityGroupB840FBB2-1VSQXAGGZQEQL"

  tags = {
    Name = "sandbox-harbor-enrichment-usw2/HarborFargateCluster/dtApiStorage"
  }

  tags_all = {
    Name = "sandbox-harbor-enrichment-usw2/HarborFargateCluster/dtApiStorage"
  }

  vpc_id = "vpc-0254d1f2cb183a8dc"
}

resource "aws_security_group" "tfer--sandbox-harbor-enrichment-usw2-sandboxDependencyTrackInterfaceusw2LaunchTemplateSG5E9EC21C-1WJZMA8O08S4O_sg-04be8e9c0e99edd02" {
  description = "sandbox-harbor-enrichment-usw2/sandbox_DependencyTrackInterface_usw2/LaunchTemplateSG"

  egress {
    cidr_blocks = ["0.0.0.0/0"]
    description = "Allow all outbound traffic by default"
    from_port   = "0"
    protocol    = "-1"
    self        = "false"
    to_port     = "0"
  }

  name   = "sandbox-harbor-enrichment-usw2-sandboxDependencyTrackInterfaceusw2LaunchTemplateSG5E9EC21C-1WJZMA8O08S4O"
  vpc_id = "vpc-0254d1f2cb183a8dc"
}

resource "aws_security_group" "tfer--sandbox-harbor-enrichment-usw2-sandboxIonChannelInterfaceusw2LaunchTemplateSGF5257E50-XVV74RVKEO88_sg-0a09feaf5fdc0dda9" {
  description = "sandbox-harbor-enrichment-usw2/sandbox_IonChannelInterface_usw2/LaunchTemplateSG"

  egress {
    cidr_blocks = ["0.0.0.0/0"]
    description = "Allow all outbound traffic by default"
    from_port   = "0"
    protocol    = "-1"
    self        = "false"
    to_port     = "0"
  }

  name   = "sandbox-harbor-enrichment-usw2-sandboxIonChannelInterfaceusw2LaunchTemplateSGF5257E50-XVV74RVKEO88"
  vpc_id = "vpc-0254d1f2cb183a8dc"
}

resource "aws_security_group" "tfer--sandbox-harbor-enrichment-usw2-sandboxSBOMEnrichmentIngressusw2SecurityGroupCA51E0E5-11X5F6126HILG_sg-0e83a5181353b01a0" {
  description = "Automatic security group for Lambda Function sandboxharborenrichmentusw2sandboxSBOMEnrichmentIngressusw2ACAEB089"

  egress {
    cidr_blocks = ["0.0.0.0/0"]
    description = "Allow all outbound traffic by default"
    from_port   = "0"
    protocol    = "-1"
    self        = "false"
    to_port     = "0"
  }

  name   = "sandbox-harbor-enrichment-usw2-sandboxSBOMEnrichmentIngressusw2SecurityGroupCA51E0E5-11X5F6126HILG"
  vpc_id = "vpc-0254d1f2cb183a8dc"
}

resource "aws_security_group" "tfer--sandbox-harbor-enrichment-usw2-sandboxSummarizerusw2LaunchTemplateSG011B28C7-15ABI34BQUEZA_sg-0f49ea6a983a186ef" {
  description = "sandbox-harbor-enrichment-usw2/sandbox_Summarizer_usw2/LaunchTemplateSG"

  egress {
    cidr_blocks = ["0.0.0.0/0"]
    description = "Allow all outbound traffic by default"
    from_port   = "0"
    protocol    = "-1"
    self        = "false"
    to_port     = "0"
  }

  name   = "sandbox-harbor-enrichment-usw2-sandboxSummarizerusw2LaunchTemplateSG011B28C7-15ABI34BQUEZA"
  vpc_id = "vpc-0254d1f2cb183a8dc"
}
