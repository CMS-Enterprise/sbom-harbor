resource "aws_lb" "tfer--sandbox-DependencyTrack-usw2" {
  access_logs {
    bucket  = "sandbox-harbor-enrichmen-dependencytrackloadbalan-5e1o9pg62tuq"
    enabled = "true"
  }

  desync_mitigation_mode           = "defensive"
  drop_invalid_header_fields       = "false"
  enable_cross_zone_load_balancing = "true"
  enable_deletion_protection       = "false"
  enable_http2                     = "true"
  enable_waf_fail_open             = "false"
  idle_timeout                     = "60"
  internal                         = "true"
  ip_address_type                  = "ipv4"
  load_balancer_type               = "application"
  name                             = "sandbox-DependencyTrack-usw2"
  preserve_host_header             = "false"
  security_groups                  = ["${data.terraform_remote_state.sg.outputs.aws_security_group_tfer--sandbox-harbor-enrichment-usw2-DEPENDENCYTRACKLOADBALANCERDEPENDENCYTRACKLOADBALANCERSECURITYGROUP8CB2E209-UW3J8975T63V_sg-04a63d76aba331aab_id}"]

  subnet_mapping {
    subnet_id = "subnet-01084e563225fed6e"
  }

  subnet_mapping {
    subnet_id = "subnet-07adf8b6e59adf416"
  }

  subnets = ["${data.terraform_remote_state.subnet.outputs.aws_subnet_tfer--subnet-01084e563225fed6e_id}", "${data.terraform_remote_state.subnet.outputs.aws_subnet_tfer--subnet-07adf8b6e59adf416_id}"]
}
