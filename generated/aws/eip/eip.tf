resource "aws_eip" "tfer--eipalloc-05badc8783ce0e393" {
  network_border_group = "us-west-2"
  network_interface    = "eni-04031a2373ee27dbf"
  public_ipv4_pool     = "amazon"

  tags = {
    Name = "sandbox-harbor-shared-resources-usw2/sandbox-HarborNetwork-usw2/HarborNetworkVpc/PublicSubnet1"
  }

  tags_all = {
    Name = "sandbox-harbor-shared-resources-usw2/sandbox-HarborNetwork-usw2/HarborNetworkVpc/PublicSubnet1"
  }

  vpc = "true"
}

resource "aws_eip" "tfer--eipalloc-0bde129b22cf23ffa" {
  network_border_group = "us-west-2"
  network_interface    = "eni-05f80454f757ad989"
  public_ipv4_pool     = "amazon"

  tags = {
    Name = "sandbox-harbor-shared-resources-usw2/sandbox-HarborNetwork-usw2/HarborNetworkVpc/PublicSubnet2"
  }

  tags_all = {
    Name = "sandbox-harbor-shared-resources-usw2/sandbox-HarborNetwork-usw2/HarborNetworkVpc/PublicSubnet2"
  }

  vpc = "true"
}
