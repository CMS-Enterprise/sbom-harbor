resource "aws_nat_gateway" "tfer--nat-0ca584c069e06819d" {
  allocation_id     = "eipalloc-0bde129b22cf23ffa"
  connectivity_type = "public"
  private_ip        = "10.0.0.239"
  subnet_id         = "subnet-04661abd803c98fc9"

  tags = {
    Name = "sandbox-harbor-shared-resources-usw2/sandbox-HarborNetwork-usw2/HarborNetworkVpc/PublicSubnet2"
  }

  tags_all = {
    Name = "sandbox-harbor-shared-resources-usw2/sandbox-HarborNetwork-usw2/HarborNetworkVpc/PublicSubnet2"
  }
}

resource "aws_nat_gateway" "tfer--nat-0d5af2c8f8e52d6ad" {
  allocation_id     = "eipalloc-05badc8783ce0e393"
  connectivity_type = "public"
  private_ip        = "10.0.0.164"
  subnet_id         = "subnet-0e57d259d4ff194b1"

  tags = {
    Name = "sandbox-harbor-shared-resources-usw2/sandbox-HarborNetwork-usw2/HarborNetworkVpc/PublicSubnet1"
  }

  tags_all = {
    Name = "sandbox-harbor-shared-resources-usw2/sandbox-HarborNetwork-usw2/HarborNetworkVpc/PublicSubnet1"
  }
}
