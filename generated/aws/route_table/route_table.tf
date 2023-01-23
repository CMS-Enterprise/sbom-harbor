resource "aws_route_table" "tfer--rtb-00b4bdb1108190321" {
  route {
    cidr_block = "0.0.0.0/0"
    gateway_id = "igw-089143981ae909a31"
  }

  tags = {
    Name = "sandbox-harbor-shared-resources-usw2/sandbox-HarborNetwork-usw2/HarborNetworkVpc/PublicSubnet2"
  }

  tags_all = {
    Name = "sandbox-harbor-shared-resources-usw2/sandbox-HarborNetwork-usw2/HarborNetworkVpc/PublicSubnet2"
  }

  vpc_id = "${data.terraform_remote_state.vpc.outputs.aws_vpc_tfer--vpc-0254d1f2cb183a8dc_id}"
}

resource "aws_route_table" "tfer--rtb-0156c096007c963e3" {
  route {
    cidr_block = "0.0.0.0/0"
    gateway_id = "igw-089143981ae909a31"
  }

  tags = {
    Name = "sandbox-harbor-shared-resources-usw2/sandbox-HarborNetwork-usw2/HarborNetworkVpc/PublicSubnet1"
  }

  tags_all = {
    Name = "sandbox-harbor-shared-resources-usw2/sandbox-HarborNetwork-usw2/HarborNetworkVpc/PublicSubnet1"
  }

  vpc_id = "${data.terraform_remote_state.vpc.outputs.aws_vpc_tfer--vpc-0254d1f2cb183a8dc_id}"
}

resource "aws_route_table" "tfer--rtb-02ede89fbfd58e17d" {
  vpc_id = "${data.terraform_remote_state.vpc.outputs.aws_vpc_tfer--vpc-045a1b5658854587f_id}"
}

resource "aws_route_table" "tfer--rtb-0379b18cd2cfa8969" {
  route {
    cidr_block     = "0.0.0.0/0"
    nat_gateway_id = "nat-0d5af2c8f8e52d6ad"
  }

  tags = {
    Name = "sandbox-harbor-shared-resources-usw2/sandbox-HarborNetwork-usw2/HarborNetworkVpc/PrivateSubnet1"
  }

  tags_all = {
    Name = "sandbox-harbor-shared-resources-usw2/sandbox-HarborNetwork-usw2/HarborNetworkVpc/PrivateSubnet1"
  }

  vpc_id = "${data.terraform_remote_state.vpc.outputs.aws_vpc_tfer--vpc-0254d1f2cb183a8dc_id}"
}

resource "aws_route_table" "tfer--rtb-0488e4a4fac2255db" {
  vpc_id = "${data.terraform_remote_state.vpc.outputs.aws_vpc_tfer--vpc-0254d1f2cb183a8dc_id}"
}

resource "aws_route_table" "tfer--rtb-058dc25a82a8bd3f4" {
  tags = {
    Name    = "aws-controltower-PrivateSubnet3ARouteTable"
    Network = "Private"
  }

  tags_all = {
    Name    = "aws-controltower-PrivateSubnet3ARouteTable"
    Network = "Private"
  }

  vpc_id = "${data.terraform_remote_state.vpc.outputs.aws_vpc_tfer--vpc-045a1b5658854587f_id}"
}

resource "aws_route_table" "tfer--rtb-069a911f014f31680" {
  tags = {
    Name    = "aws-controltower-PrivateSubnet1ARouteTable"
    Network = "Private"
  }

  tags_all = {
    Name    = "aws-controltower-PrivateSubnet1ARouteTable"
    Network = "Private"
  }

  vpc_id = "${data.terraform_remote_state.vpc.outputs.aws_vpc_tfer--vpc-045a1b5658854587f_id}"
}

resource "aws_route_table" "tfer--rtb-07f69a2f51703e840" {
  tags = {
    Name    = "aws-controltower-PrivateSubnet2ARouteTable"
    Network = "Private"
  }

  tags_all = {
    Name    = "aws-controltower-PrivateSubnet2ARouteTable"
    Network = "Private"
  }

  vpc_id = "${data.terraform_remote_state.vpc.outputs.aws_vpc_tfer--vpc-045a1b5658854587f_id}"
}

resource "aws_route_table" "tfer--rtb-095b5fb8bfbec50f2" {
  route {
    cidr_block     = "0.0.0.0/0"
    nat_gateway_id = "nat-0ca584c069e06819d"
  }

  tags = {
    Name = "sandbox-harbor-shared-resources-usw2/sandbox-HarborNetwork-usw2/HarborNetworkVpc/PrivateSubnet2"
  }

  tags_all = {
    Name = "sandbox-harbor-shared-resources-usw2/sandbox-HarborNetwork-usw2/HarborNetworkVpc/PrivateSubnet2"
  }

  vpc_id = "${data.terraform_remote_state.vpc.outputs.aws_vpc_tfer--vpc-0254d1f2cb183a8dc_id}"
}
