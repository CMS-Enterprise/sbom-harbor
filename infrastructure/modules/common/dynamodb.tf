resource "aws_dynamodb_table" "harbor_teams" {
  attribute {
    name = "EntityKey"
    type = "S"
  }

  attribute {
    name = "TeamId"
    type = "S"
  }

  billing_mode = "PROVISIONED"
  hash_key     = "TeamId"
  name         = "${var.environment}-harbor-teams-${var.aws_region_short}"

  point_in_time_recovery {
    enabled = "false"
  }

  range_key      = "EntityKey"
  read_capacity  = "1"
  stream_enabled = "false"
  write_capacity = "5"
}

output "dynamo_db_table_teams_id" {
  value = aws_dynamodb_table.harbor_teams.id
}

output "dynamo_db_table_teams_arn" {
  value = aws_dynamodb_table.harbor_teams.arn
}
