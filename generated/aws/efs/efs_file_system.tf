resource "aws_efs_file_system" "tfer--fs-067e0011f21a62148" {
  creation_token                  = "DTFargateClusterdtApiStorage96B5CA9D-YzDQDGJTLz0h"
  encrypted                       = "true"
  kms_key_id                      = "arn:aws:kms:us-west-2:393419659647:key/c0374bfe-f5cb-41ae-994f-2dc98854b43f"
  performance_mode                = "generalPurpose"
  provisioned_throughput_in_mibps = "0"

  tags = {
    Name = "SBOMApi-Enrichment/DTFargateCluster/dtApiStorage"
  }

  tags_all = {
    Name = "SBOMApi-Enrichment/DTFargateCluster/dtApiStorage"
  }

  throughput_mode = "bursting"
}

resource "aws_efs_file_system" "tfer--fs-0954adb46534caae7" {
  creation_token                  = "HarborFargateClusterdtApiStorage079A9F70-OwoiVzXWJ1HY"
  encrypted                       = "true"
  kms_key_id                      = "arn:aws:kms:us-west-2:393419659647:key/c0374bfe-f5cb-41ae-994f-2dc98854b43f"
  performance_mode                = "generalPurpose"
  provisioned_throughput_in_mibps = "0"

  tags = {
    Name = "sandbox-harbor-enrichment-usw2/HarborFargateCluster/dtApiStorage"
  }

  tags_all = {
    Name = "sandbox-harbor-enrichment-usw2/HarborFargateCluster/dtApiStorage"
  }

  throughput_mode = "bursting"
}

resource "aws_efs_file_system" "tfer--fs-0ccd670d366a76b04" {
  creation_token                  = "DTFargateClusterdtApiStorage96B5CA9D-kB950WZ29Wvb"
  encrypted                       = "true"
  kms_key_id                      = "arn:aws:kms:us-west-2:393419659647:key/c0374bfe-f5cb-41ae-994f-2dc98854b43f"
  performance_mode                = "generalPurpose"
  provisioned_throughput_in_mibps = "0"

  tags = {
    Name = "SBOMApi-Enrichment/DTFargateCluster/dtApiStorage"
  }

  tags_all = {
    Name = "SBOMApi-Enrichment/DTFargateCluster/dtApiStorage"
  }

  throughput_mode = "bursting"
}

resource "aws_efs_file_system" "tfer--fs-0cd89dec138429047" {
  creation_token                  = "HarborFargateClusterdtApiStorage079A9F70-ZArrimbZcVf7"
  encrypted                       = "true"
  kms_key_id                      = "arn:aws:kms:us-west-2:393419659647:key/c0374bfe-f5cb-41ae-994f-2dc98854b43f"
  performance_mode                = "generalPurpose"
  provisioned_throughput_in_mibps = "0"

  tags = {
    Name = "sandbox-harbor-enrichment-usw2/HarborFargateCluster/dtApiStorage"
  }

  tags_all = {
    Name = "sandbox-harbor-enrichment-usw2/HarborFargateCluster/dtApiStorage"
  }

  throughput_mode = "bursting"
}
