resource "aws_default_network_acl" "tfer--acl-04520ca52f6d42f1b" {
  egress {
    action     = "allow"
    cidr_block = "0.0.0.0/0"
    from_port  = "0"
    icmp_code  = "0"
    icmp_type  = "0"
    protocol   = "-1"
    rule_no    = "100"
    to_port    = "0"
  }

  ingress {
    action     = "allow"
    cidr_block = "0.0.0.0/0"
    from_port  = "0"
    icmp_code  = "0"
    icmp_type  = "0"
    protocol   = "-1"
    rule_no    = "100"
    to_port    = "0"
  }

  subnet_ids = ["${data.terraform_remote_state.subnet.outputs.aws_subnet_tfer--subnet-04ab449e25c4f5687_id}", "${data.terraform_remote_state.subnet.outputs.aws_subnet_tfer--subnet-05d3f20f94caaf36a_id}", "${data.terraform_remote_state.subnet.outputs.aws_subnet_tfer--subnet-07a25d44c8f3f54c9_id}"]
}

resource "aws_default_network_acl" "tfer--acl-0a5d8404f29ced5eb" {
  egress {
    action     = "allow"
    cidr_block = "0.0.0.0/0"
    from_port  = "0"
    icmp_code  = "0"
    icmp_type  = "0"
    protocol   = "-1"
    rule_no    = "100"
    to_port    = "0"
  }

  ingress {
    action     = "allow"
    cidr_block = "0.0.0.0/0"
    from_port  = "0"
    icmp_code  = "0"
    icmp_type  = "0"
    protocol   = "-1"
    rule_no    = "100"
    to_port    = "0"
  }

  subnet_ids = ["${data.terraform_remote_state.subnet.outputs.aws_subnet_tfer--subnet-01084e563225fed6e_id}", "${data.terraform_remote_state.subnet.outputs.aws_subnet_tfer--subnet-04661abd803c98fc9_id}", "${data.terraform_remote_state.subnet.outputs.aws_subnet_tfer--subnet-07adf8b6e59adf416_id}", "${data.terraform_remote_state.subnet.outputs.aws_subnet_tfer--subnet-0e57d259d4ff194b1_id}"]
}
