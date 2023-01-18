resource "aws_ecr_repository" "tfer--cdk-hnb659fds-container-assets-393419659647-us-west-2" {
  encryption_configuration {
    encryption_type = "AES256"
  }

  image_scanning_configuration {
    scan_on_push = "false"
  }

  image_tag_mutability = "IMMUTABLE"
  name                 = "cdk-hnb659fds-container-assets-393419659647-us-west-2"
}
