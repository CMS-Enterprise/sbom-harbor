resource "aws_ssm_parameter" "tfer---002F-cdk-bootstrap-002F-hnb659fds-002F-version" {
  arn       = "arn:aws:ssm:us-west-2:393419659647:parameter/cdk-bootstrap/hnb659fds/version"
  data_type = "text"
  name      = "/cdk-bootstrap/hnb659fds/version"
  tier      = "Standard"
  type      = "String"
  value     = "14"
}

resource "aws_ssm_parameter" "tfer--sandbox_DT_API_KEY_usw2" {
  arn       = "arn:aws:ssm:us-west-2:393419659647:parameter/sandbox_DT_API_KEY_usw2"
  data_type = "text"
  name      = "sandbox_DT_API_KEY_usw2"
  tier      = "Standard"
  type      = "String"
  value     = "EMPTY"
}

resource "aws_ssm_parameter" "tfer--sandbox_DT_ROOT_PWD_usw2" {
  arn       = "arn:aws:ssm:us-west-2:393419659647:parameter/sandbox_DT_ROOT_PWD_usw2"
  data_type = "text"
  name      = "sandbox_DT_ROOT_PWD_usw2"
  tier      = "Standard"
  type      = "String"
  value     = "EMPTY"
}

resource "aws_ssm_parameter" "tfer--sandbox_IC_API_BASE_usw2" {
  arn       = "arn:aws:ssm:us-west-2:393419659647:parameter/sandbox_IC_API_BASE_usw2"
  data_type = "text"
  name      = "sandbox_IC_API_BASE_usw2"
  tier      = "Standard"
  type      = "String"
  value     = "api.ionchannel.io"
}

resource "aws_ssm_parameter" "tfer--sandbox_IC_API_KEY_usw2" {
  arn       = "arn:aws:ssm:us-west-2:393419659647:parameter/sandbox_IC_API_KEY_usw2"
  data_type = "text"
  name      = "sandbox_IC_API_KEY_usw2"
  tier      = "Standard"
  type      = "String"
  value     = "IonChannelDummyToken"
}

resource "aws_ssm_parameter" "tfer--sandbox_IC_RULESET_TEAM_ID_usw2" {
  arn       = "arn:aws:ssm:us-west-2:393419659647:parameter/sandbox_IC_RULESET_TEAM_ID_usw2"
  data_type = "text"
  name      = "sandbox_IC_RULESET_TEAM_ID_usw2"
  tier      = "Standard"
  type      = "String"
  value     = "232a5775-9231-4083-9422-c2333cecb7da"
}
