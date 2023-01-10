provider "aws" {
  region = var.region
}

data "aws_secretsmanager_secret" "secrets" {
  arn = var.okta_secret_arn
}

data "aws_secretsmanager_secret_version" "current" {
  secret_id = data.aws_secretsmanager_secret.secrets.id
}

data "okta" "config" {
  client_id = jsondecode(data.aws_secretsmanager_secret_version.current.secret_string)['client_id']
  client_secret = jsondecode(data.aws_secretsmanager_secret_version.current.secret_string)['client_secret']
  issuer = jsondecode(data.aws_secretsmanager_secret_version.current.secret_string)['issuer']
}

resource "aws_cognito_user_pool" "harbor" {
  name                     = "harbor"
  auto_verified_attributes = ["email"]
  username_attributes = ["email"]

  account_recovery_setting {
    recovery_mechanism {
      name     = "verified_email"
      priority = 1
    }
  }

  username_configuration {
    case_sensitive = false
  }

  password_policy {
    minimum_length = 8
    require_symbols = true
    require_numbers = true
    require_lowercase = true
    require_uppercase = true
  }
}

resource "aws_cognito_user_pool_client" "okta_client" {
  name = "cognito-client"

  user_pool_id = aws_cognito_user_pool.harbor.id
  generate_secret = false
  refresh_token_validity = 90
  prevent_user_existence_errors = "ENABLED"
  explicit_auth_flows = [
    "ALLOW_REFRESH_TOKEN_AUTH",
    "ALLOW_USER_PASSWORD_AUTH",
    "ALLOW_ADMIN_USER_PASSWORD_AUTH"
  ]

}

resource "aws_cognito_user_pool_domain" "harbor_domain" {
  domain       = "sbom-harbor"
  user_pool_id = aws_cognito_user_pool.harbor.id
}

resource "aws_cognito_identity_provider" "okta_idp" {
  user_pool_id  = aws_cognito_user_pool.harbor.id
  provider_name = "Okta"
  provider_type = "OIDC"

  provider_details = {
    authorize_scopes = ["openid", "email", "profile"]
    client_id        = data.okta.config.client_id
    client_secret    = data.okta.config.client_secret
    oidc_issuer       = data.okta.config.issuer
  }

  attribute_mapping = {
    username = "sub"
  }
}
