resource "aws_s3_bucket" "tfer--cdk-hnb659fds-assets-393419659647-us-west-2" {
  bucket        = "cdk-hnb659fds-assets-393419659647-us-west-2"
  force_destroy = "false"

  grant {
    id          = "6bc004f2fbba5d3eafef31a5b137fca55943fa3949b21501a2192613b4ded36d"
    permissions = ["FULL_CONTROL"]
    type        = "CanonicalUser"
  }

  object_lock_enabled = "false"

  policy = <<POLICY
{
  "Id": "AccessControl",
  "Statement": [
    {
      "Action": "s3:*",
      "Condition": {
        "Bool": {
          "aws:SecureTransport": "false"
        }
      },
      "Effect": "Deny",
      "Principal": "*",
      "Resource": [
        "arn:aws:s3:::cdk-hnb659fds-assets-393419659647-us-west-2",
        "arn:aws:s3:::cdk-hnb659fds-assets-393419659647-us-west-2/*"
      ],
      "Sid": "AllowSSLRequestsOnly"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  request_payer = "BucketOwner"

  server_side_encryption_configuration {
    rule {
      apply_server_side_encryption_by_default {
        sse_algorithm = "aws:kms"
      }

      bucket_key_enabled = "false"
    }
  }

  versioning {
    enabled    = "true"
    mfa_delete = "false"
  }
}

resource "aws_s3_bucket" "tfer--cf-templates-ilrr419x6bk8-us-west-2" {
  bucket        = "cf-templates-ilrr419x6bk8-us-west-2"
  force_destroy = "false"

  grant {
    id          = "6bc004f2fbba5d3eafef31a5b137fca55943fa3949b21501a2192613b4ded36d"
    permissions = ["FULL_CONTROL"]
    type        = "CanonicalUser"
  }

  object_lock_enabled = "false"
  request_payer       = "BucketOwner"

  server_side_encryption_configuration {
    rule {
      apply_server_side_encryption_by_default {
        sse_algorithm = "AES256"
      }

      bucket_key_enabled = "false"
    }
  }

  versioning {
    enabled    = "false"
    mfa_delete = "false"
  }
}

resource "aws_s3_bucket" "tfer--sandbox-harbor-enrichmen-dependencytrackloadbalan-5e1o9pg62tuq" {
  bucket        = "sandbox-harbor-enrichmen-dependencytrackloadbalan-5e1o9pg62tuq"
  force_destroy = "false"

  grant {
    id          = "6bc004f2fbba5d3eafef31a5b137fca55943fa3949b21501a2192613b4ded36d"
    permissions = ["FULL_CONTROL"]
    type        = "CanonicalUser"
  }

  object_lock_enabled = "false"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": [
        "s3:PutObject",
        "s3:PutObjectLegalHold",
        "s3:PutObjectRetention",
        "s3:PutObjectTagging",
        "s3:PutObjectVersionTagging",
        "s3:Abort*"
      ],
      "Effect": "Allow",
      "Principal": {
        "AWS": "arn:aws:iam::797873946194:root"
      },
      "Resource": "arn:aws:s3:::sandbox-harbor-enrichmen-dependencytrackloadbalan-5e1o9pg62tuq/AWSLogs/393419659647/*"
    },
    {
      "Action": "s3:PutObject",
      "Condition": {
        "StringEquals": {
          "s3:x-amz-acl": "bucket-owner-full-control"
        }
      },
      "Effect": "Allow",
      "Principal": {
        "Service": "delivery.logs.amazonaws.com"
      },
      "Resource": "arn:aws:s3:::sandbox-harbor-enrichmen-dependencytrackloadbalan-5e1o9pg62tuq/AWSLogs/393419659647/*"
    },
    {
      "Action": "s3:GetBucketAcl",
      "Effect": "Allow",
      "Principal": {
        "Service": "delivery.logs.amazonaws.com"
      },
      "Resource": "arn:aws:s3:::sandbox-harbor-enrichmen-dependencytrackloadbalan-5e1o9pg62tuq"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  request_payer = "BucketOwner"

  versioning {
    enabled    = "false"
    mfa_delete = "false"
  }
}

resource "aws_s3_bucket" "tfer--sandbox-harbor-sbom-uploads-enrichment-393419659647-usw2" {
  bucket        = "sandbox-harbor-sbom-uploads-enrichment-393419659647-usw2"
  force_destroy = "false"

  grant {
    id          = "6bc004f2fbba5d3eafef31a5b137fca55943fa3949b21501a2192613b4ded36d"
    permissions = ["FULL_CONTROL"]
    type        = "CanonicalUser"
  }

  object_lock_enabled = "false"

  replication_configuration {
    role = "arn:aws:iam::393419659647:role/service-role/sandbox-harbor-shared-reso-ReplicationRoleCE149CEC-T5KI4MILWWFJ"

    rules {
      destination {
        bucket = "arn:aws:s3:::dev-harbor-sbom-summary-share-557147098836-use1"
      }

      id       = "57032449-d3cc-4b3f-b3cf-08d25d4314a1"
      prefix   = "harbor-data-summary/"
      priority = "0"
      status   = "Enabled"
    }
  }

  request_payer = "BucketOwner"

  versioning {
    enabled    = "true"
    mfa_delete = "false"
  }
}

resource "aws_s3_bucket" "tfer--sandbox-harbor-web-assets-393419659647-usw2" {
  bucket        = "sandbox-harbor-web-assets-393419659647-usw2"
  force_destroy = "false"

  grant {
    id          = "6bc004f2fbba5d3eafef31a5b137fca55943fa3949b21501a2192613b4ded36d"
    permissions = ["FULL_CONTROL"]
    type        = "CanonicalUser"
  }

  object_lock_enabled = "false"

  policy = <<POLICY
{
  "Statement": [
    {
      "Action": "s3:GetObject",
      "Effect": "Allow",
      "Principal": {
        "AWS": "arn:aws:iam::cloudfront:user/CloudFront Origin Access Identity E1YIHLRHW85ZFJ"
      },
      "Resource": "arn:aws:s3:::sandbox-harbor-web-assets-393419659647-usw2/*"
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  request_payer = "BucketOwner"

  versioning {
    enabled    = "false"
    mfa_delete = "false"
  }

  website {
    index_document = "index.html"
  }
}
