resource "aws_cloudfront_distribution" "tfer--E279NYFZ8FBZ2O" {
  default_cache_behavior {
    allowed_methods = ["DELETE", "GET", "HEAD", "OPTIONS", "PATCH", "POST", "PUT"]
    cached_methods  = ["GET", "HEAD"]
    compress        = "false"
    default_ttl     = "1800"

    forwarded_values {
      cookies {
        forward = "all"
      }

      headers      = ["Authorization"]
      query_string = "true"
    }

    max_ttl                = "31536000"
    min_ttl                = "0"
    smooth_streaming       = "false"
    target_origin_id       = "origin2"
    viewer_protocol_policy = "redirect-to-https"
  }

  default_root_object = "index.html"
  enabled             = "true"
  http_version        = "http2"
  is_ipv6_enabled     = "true"

  ordered_cache_behavior {
    allowed_methods = ["DELETE", "GET", "HEAD", "OPTIONS", "PATCH", "POST", "PUT"]
    cached_methods  = ["GET", "HEAD"]
    compress        = "true"
    default_ttl     = "5"

    forwarded_values {
      cookies {
        forward = "none"
      }

      headers      = ["Authorization"]
      query_string = "true"
    }

    max_ttl                = "31536000"
    min_ttl                = "0"
    path_pattern           = "/api/*"
    smooth_streaming       = "false"
    target_origin_id       = "origin1"
    viewer_protocol_policy = "redirect-to-https"
  }

  origin {
    connection_attempts = "3"
    connection_timeout  = "10"

    custom_origin_config {
      http_port                = "80"
      https_port               = "443"
      origin_keepalive_timeout = "5"
      origin_protocol_policy   = "https-only"
      origin_read_timeout      = "30"
      origin_ssl_protocols     = ["TLSv1.2"]
    }

    domain_name = "97dif4nvsi.execute-api.us-west-2.amazonaws.com"
    origin_id   = "origin1"
  }

  origin {
    connection_attempts = "3"
    connection_timeout  = "10"
    domain_name         = "sandbox-harbor-web-assets-393419659647-usw2.s3.us-west-2.amazonaws.com"
    origin_id           = "origin2"

    s3_origin_config {
      origin_access_identity = "origin-access-identity/cloudfront/E1YIHLRHW85ZFJ"
    }
  }

  price_class = "PriceClass_100"

  restrictions {
    geo_restriction {
      restriction_type = "none"
    }
  }

  retain_on_delete = "false"

  viewer_certificate {
    cloudfront_default_certificate = "true"
    minimum_protocol_version       = "TLSv1"
  }
}
