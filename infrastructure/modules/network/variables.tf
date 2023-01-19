variable "environment" {
  type = string
}

variable "aws_region_short" {
  type = string
}

variable "aws_account_id" {
  type = string
}

variable "aws_availability_zones" {
  type = list(any)
}

variable "cidr_by_region" {
  type = map(any)
  default = {
    "usw1" : 10
    "usw2" : 20
    "use2" : 30
    "use1" : 40
  }
}

variable "cidr_by_environment" {
  type = map(any)
  default = {
    "sandbox" : 0
    "dev" : 1
    "test" : 2
    "stage" : 3
    "prod" : 4
  }
}

locals {
  cidr_prefix = "10.${var.cidr_by_region[var.aws_region_short] + var.cidr_by_environment[var.environment]}"
}
