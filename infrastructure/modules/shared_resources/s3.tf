# Used to upload SBOMS and store enrichment data. Accessed by CMS SDL via Snowflake
# TODO: add bucket notifications
resource "aws_s3_bucket" "datalake" {
  bucket              = "${var.environment}-harbor-datalake-${var.aws_account_id}-${var.aws_region_short}"
  force_destroy       = "false"
  object_lock_enabled = "false"
}

# Used as the origin to CloudFront for serving UI assets to the browser
resource "aws_s3_bucket" "web_assets" {
  bucket              = "${var.environment}-harbor-web-assets-${var.aws_account_id}-${var.aws_region_short}"
  force_destroy       = "false"
  object_lock_enabled = "false"
}

resource "aws_s3_bucket_website_configuration" "web_assets" {
  # index_document = "index.html"
  bucket = aws_s3_bucket.web_assets.bucket
}

resource "aws_s3_bucket_policy" "web_assets" {
  bucket = aws_s3_bucket.web_assets.bucket
  # TODO: fix policy once cloudfront resources are imported
  policy = <<POLICY
{
  "Statement":
  [
    {
      "Action":"s3:GetObject",
      "Effect":"Allow",
      "Principal":{
        "AWS":"arn:aws:iam::cloudfront:user/CloudFront Origin Access Identity E1YIHLRHW85ZFJ"
      },
      "Resource":"${aws_s3_bucket.web_assets.arn}"
    }
  ],
  "Version":"2012-10-17"
}
POLICY
}

output "web_assets_bucket_name" {
  description = "value"
  value       = aws_s3_bucket.web_assets.bucket
}

output "datalake_bucket_name" {
  description = "value"
  value       = aws_s3_bucket.datalake.bucket
}
