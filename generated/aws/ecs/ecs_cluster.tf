resource "aws_ecs_cluster" "tfer--sandbox-Harbor-usw2" {
  name = "sandbox-Harbor-usw2"

  setting {
    name  = "containerInsights"
    value = "disabled"
  }
}
