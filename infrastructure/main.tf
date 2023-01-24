terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.49"
    }
  }

  required_version = ">= 1.3.7"
}

provider "aws" {
  region = "us-west-2"
}

module "common" {
  source           = "./modules/common"
  environment      = var.environment
  aws_region_short = local.aws_region_short
}
