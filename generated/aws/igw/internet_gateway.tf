resource "aws_internet_gateway" "tfer--igw-089143981ae909a31" {
  tags = {
    Name = "sandbox-HarborNetwork-usw2"
  }

  tags_all = {
    Name = "sandbox-HarborNetwork-usw2"
  }

  vpc_id = "${data.terraform_remote_state.vpc.outputs.aws_vpc_tfer--vpc-0254d1f2cb183a8dc_id}"
}
