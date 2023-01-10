variable "region" {
  type = string
  default = "us-east-1"
}

variable "profile" {
  type = string
  default = "sandbox"
}

variable "okta_secret_arn" {
  type = string
  default = "arn:aws:secretsmanager:us-east-1:951774275151:secret:Okta-uftbHP"
}
