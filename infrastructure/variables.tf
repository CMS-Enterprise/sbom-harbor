variable "environment" {
  type        = string
  description = "Name of the environment. dev | stage | prod | e<jira ticket id>"
  default     = "sandbox"
}

variable "aws_region_short_codes" {
  type = map(any)
  default = {
    "us-east-1" = "use1",
    "us-east-2" = "use2",
    "us-west-1" = "usw1",
    "us-west-2" = "usw2",
  }
}

locals {
  aws_region_short = var.aws_region_short_codes[data.aws_region.current.name]
}
