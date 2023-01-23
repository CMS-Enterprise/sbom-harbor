resource "aws_main_route_table_association" "tfer--vpc-0254d1f2cb183a8dc" {
  route_table_id = "${data.terraform_remote_state.route_table.outputs.aws_route_table_tfer--rtb-0488e4a4fac2255db_id}"
  vpc_id         = "${data.terraform_remote_state.vpc.outputs.aws_vpc_tfer--vpc-0254d1f2cb183a8dc_id}"
}

resource "aws_main_route_table_association" "tfer--vpc-045a1b5658854587f" {
  route_table_id = "${data.terraform_remote_state.route_table.outputs.aws_route_table_tfer--rtb-02ede89fbfd58e17d_id}"
  vpc_id         = "${data.terraform_remote_state.vpc.outputs.aws_vpc_tfer--vpc-045a1b5658854587f_id}"
}
