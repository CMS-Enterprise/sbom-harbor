output "vpc_id" {
  value = module.network.vpc_id
}

ouptut "user_pool_id" {
  value = module.aws_cognito_user_pool.harbor.id
}
