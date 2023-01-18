resource "aws_sns_topic" "tfer--aws-controltower-SecurityNotifications" {
  application_success_feedback_sample_rate = "0"
  content_based_deduplication              = "false"
  display_name                             = "aws-controltower-SecurityNotifications"
  fifo_topic                               = "false"
  firehose_success_feedback_sample_rate    = "0"
  http_success_feedback_sample_rate        = "0"
  lambda_success_feedback_sample_rate      = "0"
  name                                     = "aws-controltower-SecurityNotifications"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "SNS:GetTopicAttributes",
        "SNS:SetTopicAttributes",
        "SNS:AddPermission",
        "SNS:RemovePermission",
        "SNS:DeleteTopic",
        "SNS:Subscribe",
        "SNS:ListSubscriptionsByTopic",
        "SNS:Publish",
        "SNS:Receive"
      ],
      "Condition": {
        "StringEquals": {
          "AWS:SourceOwner": "393419659647"
        }
      },
      "Effect": "Allow",
      "Principal": {
        "AWS": "*"
      },
      "Resource": "arn:aws:sns:us-west-2:393419659647:aws-controltower-SecurityNotifications",
      "Sid": "__default_statement_ID"
    },
    {
      "Action": "sns:Publish",
      "Effect": "Allow",
      "Principal": {
        "Service": "events.amazonaws.com"
      },
      "Resource": "arn:aws:sns:us-west-2:393419659647:aws-controltower-SecurityNotifications",
      "Sid": "TrustCWEToPublishEventsToMyTopic"
    }
  ],
  "Version": "2008-10-17"
}
POLICY

  sqs_success_feedback_sample_rate = "0"
}
