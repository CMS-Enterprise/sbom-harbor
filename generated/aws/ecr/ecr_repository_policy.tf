resource "aws_ecr_repository_policy" "tfer--cdk-hnb659fds-container-assets-393419659647-us-west-2" {
  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "ecr:BatchGetImage",
        "ecr:GetDownloadUrlForLayer"
      ],
      "Condition": {
        "StringLike": {
          "aws:sourceArn": "arn:aws:lambda:us-west-2:393419659647:function:*"
        }
      },
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      },
      "Sid": "LambdaECRImageRetrievalPolicy"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  repository = "cdk-hnb659fds-container-assets-393419659647-us-west-2"
}
