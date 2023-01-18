resource "aws_s3_bucket_policy" "tfer--cdk-hnb659fds-assets-393419659647-us-west-2" {
  bucket = "cdk-hnb659fds-assets-393419659647-us-west-2"
  policy = "{\"Id\":\"AccessControl\",\"Statement\":[{\"Action\":\"s3:*\",\"Condition\":{\"Bool\":{\"aws:SecureTransport\":\"false\"}},\"Effect\":\"Deny\",\"Principal\":\"*\",\"Resource\":[\"arn:aws:s3:::cdk-hnb659fds-assets-393419659647-us-west-2\",\"arn:aws:s3:::cdk-hnb659fds-assets-393419659647-us-west-2/*\"],\"Sid\":\"AllowSSLRequestsOnly\"}],\"Version\":\"2012-10-17\"}"
}

resource "aws_s3_bucket_policy" "tfer--sandbox-harbor-enrichmen-dependencytrackloadbalan-5e1o9pg62tuq" {
  bucket = "sandbox-harbor-enrichmen-dependencytrackloadbalan-5e1o9pg62tuq"
  policy = "{\"Statement\":[{\"Action\":[\"s3:PutObject\",\"s3:PutObjectLegalHold\",\"s3:PutObjectRetention\",\"s3:PutObjectTagging\",\"s3:PutObjectVersionTagging\",\"s3:Abort*\"],\"Effect\":\"Allow\",\"Principal\":{\"AWS\":\"arn:aws:iam::797873946194:root\"},\"Resource\":\"arn:aws:s3:::sandbox-harbor-enrichmen-dependencytrackloadbalan-5e1o9pg62tuq/AWSLogs/393419659647/*\"},{\"Action\":\"s3:PutObject\",\"Condition\":{\"StringEquals\":{\"s3:x-amz-acl\":\"bucket-owner-full-control\"}},\"Effect\":\"Allow\",\"Principal\":{\"Service\":\"delivery.logs.amazonaws.com\"},\"Resource\":\"arn:aws:s3:::sandbox-harbor-enrichmen-dependencytrackloadbalan-5e1o9pg62tuq/AWSLogs/393419659647/*\"},{\"Action\":\"s3:GetBucketAcl\",\"Effect\":\"Allow\",\"Principal\":{\"Service\":\"delivery.logs.amazonaws.com\"},\"Resource\":\"arn:aws:s3:::sandbox-harbor-enrichmen-dependencytrackloadbalan-5e1o9pg62tuq\"}],\"Version\":\"2012-10-17\"}"
}

resource "aws_s3_bucket_policy" "tfer--sandbox-harbor-web-assets-393419659647-usw2" {
  bucket = "sandbox-harbor-web-assets-393419659647-usw2"
  policy = "{\"Statement\":[{\"Action\":\"s3:GetObject\",\"Effect\":\"Allow\",\"Principal\":{\"AWS\":\"arn:aws:iam::cloudfront:user/CloudFront Origin Access Identity E1YIHLRHW85ZFJ\"},\"Resource\":\"arn:aws:s3:::sandbox-harbor-web-assets-393419659647-usw2/*\"}],\"Version\":\"2012-10-17\"}"
}
