resource "aws_ecs_service" "tfer--sandbox-Harbor-usw2_sandbox-harbor-enrichment-usw2-HarborFargateClustersandboxDTFargateServiceusw2Service96D0FFC6-uhTC83H8gPz0" {
  cluster = "sandbox-Harbor-usw2"

  deployment_circuit_breaker {
    enable   = "false"
    rollback = "false"
  }

  deployment_controller {
    type = "ECS"
  }

  deployment_maximum_percent         = "200"
  deployment_minimum_healthy_percent = "50"
  desired_count                      = "1"
  enable_ecs_managed_tags            = "false"
  enable_execute_command             = "false"
  health_check_grace_period_seconds  = "60"
  launch_type                        = "FARGATE"

  load_balancer {
    container_name   = "dtContainer"
    container_port   = "8080"
    target_group_arn = "arn:aws:elasticloadbalancing:us-west-2:393419659647:targetgroup/sandbo-DEPEN-FPYUXF9IXARC/fb8f32e9d5dbb90e"
  }

  name = "sandbox-harbor-enrichment-usw2-HarborFargateClustersandboxDTFargateServiceusw2Service96D0FFC6-uhTC83H8gPz0"

  network_configuration {
    assign_public_ip = "false"
    security_groups  = ["${data.terraform_remote_state.sg.outputs.aws_security_group_tfer--sandbox-harbor-enrichment-usw2-HarborFargateClusterALLOW8080SGA923B51F-1G7B19M1MHMW2_sg-07277d8669bb21226_id}"]
    subnets          = ["${data.terraform_remote_state.subnet.outputs.aws_subnet_tfer--subnet-01084e563225fed6e_id}", "${data.terraform_remote_state.subnet.outputs.aws_subnet_tfer--subnet-07adf8b6e59adf416_id}"]
  }

  platform_version    = "1.4.0"
  scheduling_strategy = "REPLICA"
  task_definition     = "arn:aws:ecs:us-west-2:393419659647:task-definition/sandboxharborenrichmentusw2HarborFargateClusterDependencyTrackTaskDefinition1604FC01:2"
}
