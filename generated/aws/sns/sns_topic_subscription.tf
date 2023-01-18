resource "aws_sns_topic_subscription" "tfer--subscription-cca46aea-dc50-493a-ae9e-953e8fb84132" {
  endpoint             = "arn:aws:lambda:us-west-2:393419659647:function:aws-controltower-NotificationForwarder"
  protocol             = "lambda"
  raw_message_delivery = "false"
  topic_arn            = "${data.terraform_remote_state.sns.outputs.aws_sns_topic_tfer--aws-controltower-SecurityNotifications_id}"
}
