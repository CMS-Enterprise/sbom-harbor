resource "aws_dynamodb_table" "harbor_teams" {
  attribute {
    name = "EntityKey"
    type = "S"
  }

  attribute {
    name = "TeamId"
    type = "S"
  }

  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "TeamId"
  name         = "${var.environment}-harbor-teams"

  point_in_time_recovery {
    enabled = "false"
  }

  range_key      = "EntityKey"
  stream_enabled = "false"
}

output "dynamodb_table_teams_id" {
  value = aws_dynamodb_table.harbor_teams.id
}

output "dynamodb_table_teams_arn" {
  value = aws_dynamodb_table.harbor_teams.arn
}
