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

module "shared_resources" {
  source           = "./modules/shared_resources"
  environment      = var.environment
  aws_region_short = local.aws_region_short
  aws_account_id   = data.aws_caller_identity.current.account_id
}

module "network" {
  source                 = "./modules/network"
  environment            = var.environment
  aws_region_short       = local.aws_region_short
  aws_account_id         = data.aws_caller_identity.current.account_id
  aws_availability_zones = data.aws_availability_zones.available.names
}
