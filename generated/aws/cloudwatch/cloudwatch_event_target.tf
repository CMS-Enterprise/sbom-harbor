resource "aws_cloudwatch_event_target" "tfer--aws-controltower-ConfigComplianceChangeEventRule-002F-Compliance-Change-Topic" {
  arn       = "arn:aws:sns:us-west-2:393419659647:aws-controltower-SecurityNotifications"
  rule      = "aws-controltower-ConfigComplianceChangeEventRule"
  target_id = "Compliance-Change-Topic"
}
