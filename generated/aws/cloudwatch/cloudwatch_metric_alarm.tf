resource "aws_cloudwatch_metric_alarm" "tfer--TargetTracking-table-002F-sandbox-HarborTeams-usw2-AlarmHigh-71208a04-9fd1-4832-9de7-a09f11cd153d" {
  actions_enabled     = "true"
  alarm_actions       = ["arn:aws:autoscaling:us-west-2:393419659647:scalingPolicy:540623c6-310c-4c3d-aa3f-5f2f43fc99bb:resource/dynamodb/table/sandbox-HarborTeams-usw2:policyName/sandboxharborsharedresourcesusw2sandboxHarborTeamsusw2TeamsDynamoDbTableReadScalingTargetTracking80F04FF7:createdBy/835f5831-c259-4557-8b87-bcbff1384db2"]
  alarm_description   = "DO NOT EDIT OR DELETE. For TargetTrackingScaling policy arn:aws:autoscaling:us-west-2:393419659647:scalingPolicy:540623c6-310c-4c3d-aa3f-5f2f43fc99bb:resource/dynamodb/table/sandbox-HarborTeams-usw2:policyName/sandboxharborsharedresourcesusw2sandboxHarborTeamsusw2TeamsDynamoDbTableReadScalingTargetTracking80F04FF7:createdBy/835f5831-c259-4557-8b87-bcbff1384db2."
  alarm_name          = "TargetTracking-table/sandbox-HarborTeams-usw2-AlarmHigh-71208a04-9fd1-4832-9de7-a09f11cd153d"
  comparison_operator = "GreaterThanThreshold"
  datapoints_to_alarm = "0"

  dimensions = {
    TableName = "sandbox-HarborTeams-usw2"
  }

  evaluation_periods = "2"
  metric_name        = "ConsumedReadCapacityUnits"
  namespace          = "AWS/DynamoDB"
  period             = "60"
  statistic          = "Sum"
  threshold          = "30"
  treat_missing_data = "missing"
}

resource "aws_cloudwatch_metric_alarm" "tfer--TargetTracking-table-002F-sandbox-HarborTeams-usw2-AlarmLow-a8ca1407-f358-4104-acc0-32e2edc88ac2" {
  actions_enabled     = "true"
  alarm_actions       = ["arn:aws:autoscaling:us-west-2:393419659647:scalingPolicy:540623c6-310c-4c3d-aa3f-5f2f43fc99bb:resource/dynamodb/table/sandbox-HarborTeams-usw2:policyName/sandboxharborsharedresourcesusw2sandboxHarborTeamsusw2TeamsDynamoDbTableReadScalingTargetTracking80F04FF7:createdBy/835f5831-c259-4557-8b87-bcbff1384db2"]
  alarm_description   = "DO NOT EDIT OR DELETE. For TargetTrackingScaling policy arn:aws:autoscaling:us-west-2:393419659647:scalingPolicy:540623c6-310c-4c3d-aa3f-5f2f43fc99bb:resource/dynamodb/table/sandbox-HarborTeams-usw2:policyName/sandboxharborsharedresourcesusw2sandboxHarborTeamsusw2TeamsDynamoDbTableReadScalingTargetTracking80F04FF7:createdBy/835f5831-c259-4557-8b87-bcbff1384db2."
  alarm_name          = "TargetTracking-table/sandbox-HarborTeams-usw2-AlarmLow-a8ca1407-f358-4104-acc0-32e2edc88ac2"
  comparison_operator = "LessThanThreshold"
  datapoints_to_alarm = "0"

  dimensions = {
    TableName = "sandbox-HarborTeams-usw2"
  }

  evaluation_periods = "15"
  metric_name        = "ConsumedReadCapacityUnits"
  namespace          = "AWS/DynamoDB"
  period             = "60"
  statistic          = "Sum"
  threshold          = "18"
  treat_missing_data = "missing"
}

resource "aws_cloudwatch_metric_alarm" "tfer--TargetTracking-table-002F-sandbox-HarborTeams-usw2-ProvisionedCapacityHigh-f4d44d2f-27af-4c86-a3b4-cdfaff16f058" {
  actions_enabled     = "true"
  alarm_actions       = ["arn:aws:autoscaling:us-west-2:393419659647:scalingPolicy:540623c6-310c-4c3d-aa3f-5f2f43fc99bb:resource/dynamodb/table/sandbox-HarborTeams-usw2:policyName/sandboxharborsharedresourcesusw2sandboxHarborTeamsusw2TeamsDynamoDbTableReadScalingTargetTracking80F04FF7:createdBy/835f5831-c259-4557-8b87-bcbff1384db2"]
  alarm_description   = "DO NOT EDIT OR DELETE. For TargetTrackingScaling policy arn:aws:autoscaling:us-west-2:393419659647:scalingPolicy:540623c6-310c-4c3d-aa3f-5f2f43fc99bb:resource/dynamodb/table/sandbox-HarborTeams-usw2:policyName/sandboxharborsharedresourcesusw2sandboxHarborTeamsusw2TeamsDynamoDbTableReadScalingTargetTracking80F04FF7:createdBy/835f5831-c259-4557-8b87-bcbff1384db2."
  alarm_name          = "TargetTracking-table/sandbox-HarborTeams-usw2-ProvisionedCapacityHigh-f4d44d2f-27af-4c86-a3b4-cdfaff16f058"
  comparison_operator = "GreaterThanThreshold"
  datapoints_to_alarm = "0"

  dimensions = {
    TableName = "sandbox-HarborTeams-usw2"
  }

  evaluation_periods = "3"
  metric_name        = "ProvisionedReadCapacityUnits"
  namespace          = "AWS/DynamoDB"
  period             = "300"
  statistic          = "Average"
  threshold          = "1"
  treat_missing_data = "missing"
}

resource "aws_cloudwatch_metric_alarm" "tfer--TargetTracking-table-002F-sandbox-HarborTeams-usw2-ProvisionedCapacityLow-ebc8e863-af20-469f-9cce-ab7cdeaa7608" {
  actions_enabled     = "true"
  alarm_actions       = ["arn:aws:autoscaling:us-west-2:393419659647:scalingPolicy:540623c6-310c-4c3d-aa3f-5f2f43fc99bb:resource/dynamodb/table/sandbox-HarborTeams-usw2:policyName/sandboxharborsharedresourcesusw2sandboxHarborTeamsusw2TeamsDynamoDbTableReadScalingTargetTracking80F04FF7:createdBy/835f5831-c259-4557-8b87-bcbff1384db2"]
  alarm_description   = "DO NOT EDIT OR DELETE. For TargetTrackingScaling policy arn:aws:autoscaling:us-west-2:393419659647:scalingPolicy:540623c6-310c-4c3d-aa3f-5f2f43fc99bb:resource/dynamodb/table/sandbox-HarborTeams-usw2:policyName/sandboxharborsharedresourcesusw2sandboxHarborTeamsusw2TeamsDynamoDbTableReadScalingTargetTracking80F04FF7:createdBy/835f5831-c259-4557-8b87-bcbff1384db2."
  alarm_name          = "TargetTracking-table/sandbox-HarborTeams-usw2-ProvisionedCapacityLow-ebc8e863-af20-469f-9cce-ab7cdeaa7608"
  comparison_operator = "LessThanThreshold"
  datapoints_to_alarm = "0"

  dimensions = {
    TableName = "sandbox-HarborTeams-usw2"
  }

  evaluation_periods = "3"
  metric_name        = "ProvisionedReadCapacityUnits"
  namespace          = "AWS/DynamoDB"
  period             = "300"
  statistic          = "Average"
  threshold          = "1"
  treat_missing_data = "missing"
}
