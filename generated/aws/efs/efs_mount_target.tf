resource "aws_efs_mount_target" "tfer--fsmt-0d89f96fa6db8fff7" {
  file_system_id  = "fs-0954adb46534caae7"
  ip_address      = "10.0.0.107"
  security_groups = ["sg-0c5bcf59f841c929c"]
  subnet_id       = "subnet-07adf8b6e59adf416"
}

resource "aws_efs_mount_target" "tfer--fsmt-0f65a5b8492239fa3" {
  file_system_id  = "fs-0954adb46534caae7"
  ip_address      = "10.0.0.5"
  security_groups = ["sg-0c5bcf59f841c929c"]
  subnet_id       = "subnet-01084e563225fed6e"
}
