resource "aws_cloudwatch_event_rule" "tfer--aws-controltower-ConfigComplianceChangeEventRule" {
  description    = "CloudWatch Event Rule to send notification on Config Rule compliance changes."
  event_bus_name = "default"
  event_pattern  = "{\"detail-type\":[\"Config Rules Compliance Change\"],\"source\":[\"aws.config\"]}"
  is_enabled     = "true"
  name           = "aws-controltower-ConfigComplianceChangeEventRule"
}
