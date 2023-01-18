resource "aws_lb_listener" "tfer--arn-003A-aws-003A-elasticloadbalancing-003A-us-west-2-003A-393419659647-003A-listener-002F-app-002F-sandbox-DependencyTrack-usw2-002F-e243a104b51f6849-002F-d4f654352b88824d" {
  default_action {
    target_group_arn = "arn:aws:elasticloadbalancing:us-west-2:393419659647:targetgroup/sandbo-DEPEN-FPYUXF9IXARC/fb8f32e9d5dbb90e"
    type             = "forward"
  }

  load_balancer_arn = "${data.terraform_remote_state.alb.outputs.aws_lb_tfer--sandbox-DependencyTrack-usw2_id}"
  port              = "8080"
  protocol          = "HTTP"
}
