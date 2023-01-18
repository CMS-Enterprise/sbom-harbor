resource "aws_route_table_association" "tfer--subnet-01084e563225fed6e" {
  route_table_id = "${data.terraform_remote_state.route_table.outputs.aws_route_table_tfer--rtb-0379b18cd2cfa8969_id}"
  subnet_id      = "${data.terraform_remote_state.subnet.outputs.aws_subnet_tfer--subnet-01084e563225fed6e_id}"
}

resource "aws_route_table_association" "tfer--subnet-04661abd803c98fc9" {
  route_table_id = "${data.terraform_remote_state.route_table.outputs.aws_route_table_tfer--rtb-00b4bdb1108190321_id}"
  subnet_id      = "${data.terraform_remote_state.subnet.outputs.aws_subnet_tfer--subnet-04661abd803c98fc9_id}"
}

resource "aws_route_table_association" "tfer--subnet-04ab449e25c4f5687" {
  route_table_id = "${data.terraform_remote_state.route_table.outputs.aws_route_table_tfer--rtb-07f69a2f51703e840_id}"
  subnet_id      = "${data.terraform_remote_state.subnet.outputs.aws_subnet_tfer--subnet-04ab449e25c4f5687_id}"
}

resource "aws_route_table_association" "tfer--subnet-05d3f20f94caaf36a" {
  route_table_id = "${data.terraform_remote_state.route_table.outputs.aws_route_table_tfer--rtb-058dc25a82a8bd3f4_id}"
  subnet_id      = "${data.terraform_remote_state.subnet.outputs.aws_subnet_tfer--subnet-05d3f20f94caaf36a_id}"
}

resource "aws_route_table_association" "tfer--subnet-07a25d44c8f3f54c9" {
  route_table_id = "${data.terraform_remote_state.route_table.outputs.aws_route_table_tfer--rtb-069a911f014f31680_id}"
  subnet_id      = "${data.terraform_remote_state.subnet.outputs.aws_subnet_tfer--subnet-07a25d44c8f3f54c9_id}"
}

resource "aws_route_table_association" "tfer--subnet-07adf8b6e59adf416" {
  route_table_id = "${data.terraform_remote_state.route_table.outputs.aws_route_table_tfer--rtb-095b5fb8bfbec50f2_id}"
  subnet_id      = "${data.terraform_remote_state.subnet.outputs.aws_subnet_tfer--subnet-07adf8b6e59adf416_id}"
}

resource "aws_route_table_association" "tfer--subnet-0e57d259d4ff194b1" {
  route_table_id = "${data.terraform_remote_state.route_table.outputs.aws_route_table_tfer--rtb-0156c096007c963e3_id}"
  subnet_id      = "${data.terraform_remote_state.subnet.outputs.aws_subnet_tfer--subnet-0e57d259d4ff194b1_id}"
}
