data "terraform_remote_state" "sns" {
  backend = "local"

  config = {
    path = "../../../generated/aws/sns/terraform.tfstate"
  }
}

data "terraform_remote_state" "sqs" {
  backend = "local"

  config = {
    path = "../../../generated/aws/sqs/terraform.tfstate"
  }
}
