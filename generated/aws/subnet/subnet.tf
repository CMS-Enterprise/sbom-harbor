resource "aws_subnet" "tfer--subnet-01084e563225fed6e" {
  assign_ipv6_address_on_creation                = "false"
  cidr_block                                     = "10.0.0.0/26"
  enable_dns64                                   = "false"
  enable_resource_name_dns_a_record_on_launch    = "false"
  enable_resource_name_dns_aaaa_record_on_launch = "false"
  ipv6_native                                    = "false"
  map_customer_owned_ip_on_launch                = "false"
  map_public_ip_on_launch                        = "false"
  private_dns_hostname_type_on_launch            = "ip-name"

  tags = {
    Name                  = "sandbox-harbor-shared-resources-usw2/sandbox-HarborNetwork-usw2/HarborNetworkVpc/PrivateSubnet1"
    "aws-cdk:subnet-name" = "Private"
    "aws-cdk:subnet-type" = "Private"
  }

  tags_all = {
    Name                  = "sandbox-harbor-shared-resources-usw2/sandbox-HarborNetwork-usw2/HarborNetworkVpc/PrivateSubnet1"
    "aws-cdk:subnet-name" = "Private"
    "aws-cdk:subnet-type" = "Private"
  }

  vpc_id = "${data.terraform_remote_state.vpc.outputs.aws_vpc_tfer--vpc-0254d1f2cb183a8dc_id}"
}

resource "aws_subnet" "tfer--subnet-04661abd803c98fc9" {
  assign_ipv6_address_on_creation                = "false"
  cidr_block                                     = "10.0.0.192/26"
  enable_dns64                                   = "false"
  enable_resource_name_dns_a_record_on_launch    = "false"
  enable_resource_name_dns_aaaa_record_on_launch = "false"
  ipv6_native                                    = "false"
  map_customer_owned_ip_on_launch                = "false"
  map_public_ip_on_launch                        = "true"
  private_dns_hostname_type_on_launch            = "ip-name"

  tags = {
    Name                  = "sandbox-harbor-shared-resources-usw2/sandbox-HarborNetwork-usw2/HarborNetworkVpc/PublicSubnet2"
    "aws-cdk:subnet-name" = "Public"
    "aws-cdk:subnet-type" = "Public"
  }

  tags_all = {
    Name                  = "sandbox-harbor-shared-resources-usw2/sandbox-HarborNetwork-usw2/HarborNetworkVpc/PublicSubnet2"
    "aws-cdk:subnet-name" = "Public"
    "aws-cdk:subnet-type" = "Public"
  }

  vpc_id = "${data.terraform_remote_state.vpc.outputs.aws_vpc_tfer--vpc-0254d1f2cb183a8dc_id}"
}

resource "aws_subnet" "tfer--subnet-04ab449e25c4f5687" {
  assign_ipv6_address_on_creation                = "false"
  cidr_block                                     = "172.31.32.0/20"
  enable_dns64                                   = "false"
  enable_resource_name_dns_a_record_on_launch    = "false"
  enable_resource_name_dns_aaaa_record_on_launch = "false"
  ipv6_native                                    = "false"
  map_customer_owned_ip_on_launch                = "false"
  map_public_ip_on_launch                        = "false"
  private_dns_hostname_type_on_launch            = "ip-name"

  tags = {
    Name    = "aws-controltower-PrivateSubnet2A"
    Network = "Private"
  }

  tags_all = {
    Name    = "aws-controltower-PrivateSubnet2A"
    Network = "Private"
  }

  vpc_id = "${data.terraform_remote_state.vpc.outputs.aws_vpc_tfer--vpc-045a1b5658854587f_id}"
}

resource "aws_subnet" "tfer--subnet-05d3f20f94caaf36a" {
  assign_ipv6_address_on_creation                = "false"
  cidr_block                                     = "172.31.80.0/20"
  enable_dns64                                   = "false"
  enable_resource_name_dns_a_record_on_launch    = "false"
  enable_resource_name_dns_aaaa_record_on_launch = "false"
  ipv6_native                                    = "false"
  map_customer_owned_ip_on_launch                = "false"
  map_public_ip_on_launch                        = "false"
  private_dns_hostname_type_on_launch            = "ip-name"

  tags = {
    Name    = "aws-controltower-PrivateSubnet3A"
    Network = "Private"
  }

  tags_all = {
    Name    = "aws-controltower-PrivateSubnet3A"
    Network = "Private"
  }

  vpc_id = "${data.terraform_remote_state.vpc.outputs.aws_vpc_tfer--vpc-045a1b5658854587f_id}"
}

resource "aws_subnet" "tfer--subnet-07a25d44c8f3f54c9" {
  assign_ipv6_address_on_creation                = "false"
  cidr_block                                     = "172.31.64.0/20"
  enable_dns64                                   = "false"
  enable_resource_name_dns_a_record_on_launch    = "false"
  enable_resource_name_dns_aaaa_record_on_launch = "false"
  ipv6_native                                    = "false"
  map_customer_owned_ip_on_launch                = "false"
  map_public_ip_on_launch                        = "false"
  private_dns_hostname_type_on_launch            = "ip-name"

  tags = {
    Name    = "aws-controltower-PrivateSubnet1A"
    Network = "Private"
  }

  tags_all = {
    Name    = "aws-controltower-PrivateSubnet1A"
    Network = "Private"
  }

  vpc_id = "${data.terraform_remote_state.vpc.outputs.aws_vpc_tfer--vpc-045a1b5658854587f_id}"
}

resource "aws_subnet" "tfer--subnet-07adf8b6e59adf416" {
  assign_ipv6_address_on_creation                = "false"
  cidr_block                                     = "10.0.0.64/26"
  enable_dns64                                   = "false"
  enable_resource_name_dns_a_record_on_launch    = "false"
  enable_resource_name_dns_aaaa_record_on_launch = "false"
  ipv6_native                                    = "false"
  map_customer_owned_ip_on_launch                = "false"
  map_public_ip_on_launch                        = "false"
  private_dns_hostname_type_on_launch            = "ip-name"

  tags = {
    Name                  = "sandbox-harbor-shared-resources-usw2/sandbox-HarborNetwork-usw2/HarborNetworkVpc/PrivateSubnet2"
    "aws-cdk:subnet-name" = "Private"
    "aws-cdk:subnet-type" = "Private"
  }

  tags_all = {
    Name                  = "sandbox-harbor-shared-resources-usw2/sandbox-HarborNetwork-usw2/HarborNetworkVpc/PrivateSubnet2"
    "aws-cdk:subnet-name" = "Private"
    "aws-cdk:subnet-type" = "Private"
  }

  vpc_id = "${data.terraform_remote_state.vpc.outputs.aws_vpc_tfer--vpc-0254d1f2cb183a8dc_id}"
}

resource "aws_subnet" "tfer--subnet-0e57d259d4ff194b1" {
  assign_ipv6_address_on_creation                = "false"
  cidr_block                                     = "10.0.0.128/26"
  enable_dns64                                   = "false"
  enable_resource_name_dns_a_record_on_launch    = "false"
  enable_resource_name_dns_aaaa_record_on_launch = "false"
  ipv6_native                                    = "false"
  map_customer_owned_ip_on_launch                = "false"
  map_public_ip_on_launch                        = "true"
  private_dns_hostname_type_on_launch            = "ip-name"

  tags = {
    Name                  = "sandbox-harbor-shared-resources-usw2/sandbox-HarborNetwork-usw2/HarborNetworkVpc/PublicSubnet1"
    "aws-cdk:subnet-name" = "Public"
    "aws-cdk:subnet-type" = "Public"
  }

  tags_all = {
    Name                  = "sandbox-harbor-shared-resources-usw2/sandbox-HarborNetwork-usw2/HarborNetworkVpc/PublicSubnet1"
    "aws-cdk:subnet-name" = "Public"
    "aws-cdk:subnet-type" = "Public"
  }

  vpc_id = "${data.terraform_remote_state.vpc.outputs.aws_vpc_tfer--vpc-0254d1f2cb183a8dc_id}"
}
