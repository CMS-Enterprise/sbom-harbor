resource "aws_cognito_user_pool" "harbor_user_pool" {
  account_recovery_setting {
    recovery_mechanism {
      name     = "verified_email"
      priority = "1"
    }
  }

  admin_create_user_config {
    allow_admin_create_user_only = "false"
  }

  auto_verified_attributes = ["email"]
  deletion_protection      = "INACTIVE"

  email_configuration {
    email_sending_account = "COGNITO_DEFAULT"
  }

  mfa_configuration = "OFF"
  name              = "${var.environment}-harbor-user-pool"

  password_policy {
    minimum_length                   = "8"
    require_lowercase                = "true"
    require_numbers                  = "true"
    require_symbols                  = "true"
    require_uppercase                = "true"
    temporary_password_validity_days = "7"
  }

  schema {
    attribute_data_type      = "String"
    developer_only_attribute = "false"
    mutable                  = "false"
    name                     = "email"
    required                 = "true"

    string_attribute_constraints {
      max_length = "2048"
      min_length = "0"
    }
  }

  schema {
    attribute_data_type      = "String"
    developer_only_attribute = "false"
    mutable                  = "true"
    name                     = "teams"
    required                 = "false"
  }

  username_attributes = ["email"]

  username_configuration {
    case_sensitive = "false"
  }

  verification_message_template {
    default_email_option = "CONFIRM_WITH_CODE"
    email_message        = "The verification code to your new account is {####}"
    email_subject        = "Verify your new account"
    sms_message          = "The verification code to your new account is {####}"
  }
}

output "user_pool_id" {
  description = "Cognito user pool id"
  value       = aws_cognito_user_pool.harbor_user_pool.id
}


resource "aws_cognito_user_pool_client" "harbor_user_pool_client" {
  access_token_validity                         = "0"
  allowed_oauth_flows                           = ["implicit"]
  allowed_oauth_flows_user_pool_client          = "true"
  allowed_oauth_scopes                          = ["aws.cognito.signin.user.admin", "email", "openid", "phone", "profile"]
  auth_session_validity                         = "3"
  callback_urls                                 = ["https:/example.com"]
  enable_propagate_additional_user_context_data = "false"
  enable_token_revocation                       = "true"
  explicit_auth_flows                           = ["ALLOW_ADMIN_USER_PASSWORD_AUTH", "ALLOW_CUSTOM_AUTH", "ALLOW_REFRESH_TOKEN_AUTH", "ALLOW_USER_PASSWORD_AUTH", "ALLOW_USER_SRP_AUTH"]
  id_token_validity                             = "0"
  name                                          = "${var.environment}-harbor-user-pool-client-${var.aws_region_short}"
  prevent_user_existence_errors                 = "ENABLED"
  read_attributes                               = ["custom:teams", "email", "email_verified", "family_name", "given_name", "locale", "name", "phone_number", "phone_number_verified", "preferred_username", "zoneinfo"]
  refresh_token_validity                        = "1"
  supported_identity_providers                  = ["COGNITO"]
  user_pool_id                                  = "us-west-2_XPCOGDlGl"
  write_attributes                              = ["email", "family_name", "given_name", "locale", "name", "phone_number", "preferred_username", "zoneinfo"]
}

output "user_pool_client_id" {
  description = "Cognito user pool client id"
  value       = aws_cognito_user_pool_client.harbor_user_pool_client.id
}

resource "aws_iam_role" "cognito-user" {
  assume_role_policy = <<POLICY
{
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Condition": {
        "ForAnyValue:StringLike": {
          "cognito-identity.amazonaws.com:amr": "authenticated"
        },
        "StringEquals": {
          "cognito-identity.amazonaws.com:aud": ${aws_cognito_user_pool.harbor_user_pool.id}
        }
      },
      "Effect": "Allow",
      "Principal": {
        "Federated": "cognito-identity.amazonaws.com"
      }
    }
  ],
  "Version": "2012-10-17"
}
POLICY

  description          = "Default role for authenticated users"
  managed_policy_arns  = ["arn:aws:iam::aws:policy/AmazonS3FullAccess", "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"]
  max_session_duration = "3600"
  name                 = "${var.environment}-cognito-user"
  path                 = "/"
}


resource "aws_cognito_user_group" "harbor_user_pool_group_admins" {
  name         = "${var.environment}-harbor-admins"
  user_pool_id = aws_cognito_user_pool.harbor_user_pool.id
  description  = "Default group for authenticated administrator users"
  role_arn     = aws_iam_role.cognito-user.arn
}
