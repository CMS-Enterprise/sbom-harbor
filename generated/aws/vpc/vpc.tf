resource "aws_vpc" "tfer--vpc-0254d1f2cb183a8dc" {
  assign_generated_ipv6_cidr_block     = "false"
  cidr_block                           = "10.0.0.0/16"
  enable_classiclink                   = "false"
  enable_classiclink_dns_support       = "false"
  enable_dns_hostnames                 = "true"
  enable_dns_support                   = "true"
  enable_network_address_usage_metrics = "false"
  instance_tenancy                     = "default"
  ipv6_netmask_length                  = "0"

  tags = {
    Name = "sandbox-HarborNetwork-usw2"
  }

  tags_all = {
    Name = "sandbox-HarborNetwork-usw2"
  }
}

resource "aws_vpc" "tfer--vpc-045a1b5658854587f" {
  assign_generated_ipv6_cidr_block     = "false"
  cidr_block                           = "172.31.0.0/16"
  enable_classiclink                   = "false"
  enable_classiclink_dns_support       = "false"
  enable_dns_hostnames                 = "true"
  enable_dns_support                   = "true"
  enable_network_address_usage_metrics = "false"
  instance_tenancy                     = "default"
  ipv6_netmask_length                  = "0"

  tags = {
    Name = "aws-controltower-VPC"
  }

  tags_all = {
    Name = "aws-controltower-VPC"
  }
}
