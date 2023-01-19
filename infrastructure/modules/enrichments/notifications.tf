resource "aws_s3_bucket_notification" "enrichment-notification" {
  bucket      = module.common.aws_s3_bucket.datalake.bucket
  eventbridge = true
}
