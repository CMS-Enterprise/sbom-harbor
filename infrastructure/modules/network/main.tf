module "vpc" {
  source = "terraform-aws-modules/vpc/aws"

  name = "${var.environment}-harbor"
  cidr = "${local.cidr_prefix}.0.0/16"

  azs             = [var.aws_availability_zones[0], var.aws_availability_zones[1]]
  private_subnets = ["${local.cidr_prefix}.32.0/19", "${local.cidr_prefix}.64.0/19"]
  public_subnets  = ["${local.cidr_prefix}.0.0/24", "${local.cidr_prefix}.0.0/24"]

  enable_nat_gateway = true
  enable_vpn_gateway = true

  tags = {
    environment = var.environment
  }
}
