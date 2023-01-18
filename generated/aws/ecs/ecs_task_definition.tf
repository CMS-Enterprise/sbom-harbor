resource "aws_ecs_task_definition" "tfer--task-definition-002F-sandboxharborenrichmentusw2HarborFargateClusterDependencyTrackTaskDefinition1604FC01" {
  container_definitions    = "[{\"command\":[],\"cpu\":4096,\"dnsSearchDomains\":[],\"dnsServers\":[],\"dockerLabels\":{},\"dockerSecurityOptions\":[],\"entryPoint\":[],\"environment\":[],\"environmentFiles\":[],\"essential\":true,\"extraHosts\":[],\"image\":\"dependencytrack/apiserver\",\"links\":[],\"logConfiguration\":{\"logDriver\":\"awslogs\",\"options\":{\"awslogs-group\":\"sandbox-harbor-enrichment-usw2-HarborFargateClusterDependencyTrackTaskDefinitiondtContainerLogGroup7E08ADFB-PYqt9clwLgx5\",\"awslogs-region\":\"us-west-2\",\"awslogs-stream-prefix\":\"dependencyTrackApi\"},\"secretOptions\":[]},\"memoryReservation\":8192,\"mountPoints\":[{\"containerPath\":\"/apiserver\",\"readOnly\":false,\"sourceVolume\":\"dtApiStorage\"}],\"name\":\"dtContainer\",\"portMappings\":[{\"containerPort\":8080,\"hostPort\":8080,\"protocol\":\"tcp\"}],\"readonlyRootFilesystem\":true,\"secrets\":[],\"systemControls\":[],\"ulimits\":[],\"volumesFrom\":[]}]"
  cpu                      = "4096"
  execution_role_arn       = "arn:aws:iam::393419659647:role/sandbox-harbor-enrichment-HarborFargateClusterDepe-IKOTZ3G2IJRS"
  family                   = "sandboxharborenrichmentusw2HarborFargateClusterDependencyTrackTaskDefinition1604FC01"
  memory                   = "8192"
  network_mode             = "awsvpc"
  requires_compatibilities = ["FARGATE"]
  task_role_arn            = "arn:aws:iam::393419659647:role/sandbox-harbor-enrichment-HarborFargateClusterDepe-FD1S9ZI389P3"

  volume {
    efs_volume_configuration {
      file_system_id          = "fs-0954adb46534caae7"
      root_directory          = "/"
      transit_encryption_port = "0"
    }

    name = "dtApiStorage"
  }
}
