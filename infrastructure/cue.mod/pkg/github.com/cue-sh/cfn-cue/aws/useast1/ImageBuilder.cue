package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#ImageBuilder: {
	#Component: {
		Type: "AWS::ImageBuilder::Component"
		Properties: {
			ChangeDescription?:   *string | fn.#Fn
			Data?:                *(strings.MinRunes(1) & strings.MaxRunes(16000)) | fn.#Fn
			Description?:         *string | fn.#Fn
			KmsKeyId?:            *string | fn.#Fn
			Name:                 *string | fn.#Fn
			Platform:             *("Windows" | "Linux") | fn.#Fn
			SupportedOsVersions?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			Tags?:                *{
				[string]: *string | fn.#Fn
			} | fn.#If
			Uri?:    *string | fn.#Fn
			Version: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ContainerRecipe: {
		Type: "AWS::ImageBuilder::ContainerRecipe"
		Properties: {
			Components: *[...{
				ComponentArn?: *string | fn.#Fn
			}] | fn.#If
			ContainerType:           *("DOCKER") | fn.#Fn
			Description?:            *string | fn.#Fn
			DockerfileTemplateData?: *string | fn.#Fn
			DockerfileTemplateUri?:  *string | fn.#Fn
			ImageOsVersionOverride?: *string | fn.#Fn
			InstanceConfiguration?:  *{
				BlockDeviceMappings?: *[...{
					DeviceName?: *string | fn.#Fn
					Ebs?:        *{
						DeleteOnTermination?: *bool | fn.#Fn
						Encrypted?:           *bool | fn.#Fn
						Iops?:                *int | fn.#Fn
						KmsKeyId?:            *string | fn.#Fn
						SnapshotId?:          *string | fn.#Fn
						Throughput?:          *int | fn.#Fn
						VolumeSize?:          *int | fn.#Fn
						VolumeType?:          *("standard" | "io1" | "io2" | "gp2" | "gp3" | "sc1" | "st1") | fn.#Fn
					} | fn.#If
					NoDevice?:    *string | fn.#Fn
					VirtualName?: *string | fn.#Fn
				}] | fn.#If
				Image?: *string | fn.#Fn
			} | fn.#If
			KmsKeyId?:         *string | fn.#Fn
			Name:              *string | fn.#Fn
			ParentImage:       *string | fn.#Fn
			PlatformOverride?: *("Windows" | "Linux") | fn.#Fn
			Tags?:             *{
				[string]: *string | fn.#Fn
			} | fn.#If
			TargetRepository: *{
				RepositoryName?: *string | fn.#Fn
				Service?:        *("ECR") | fn.#Fn
			} | fn.#If
			Version:           *string | fn.#Fn
			WorkingDirectory?: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#DistributionConfiguration: {
		Type: "AWS::ImageBuilder::DistributionConfiguration"
		Properties: {
			Description?:  *string | fn.#Fn
			Distributions: *[...{
				AmiDistributionConfiguration?: *{
					[string]: _
				} | fn.#Fn
				ContainerDistributionConfiguration?: *{
					[string]: _
				} | fn.#Fn
				LaunchTemplateConfigurations?: *[...{
					AccountId?:         *string | fn.#Fn
					LaunchTemplateId?:  *string | fn.#Fn
					SetDefaultVersion?: *bool | fn.#Fn
				}] | fn.#If
				LicenseConfigurationArns?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				Region:                    *string | fn.#Fn
			}] | fn.#If
			Name:  *string | fn.#Fn
			Tags?: *{
				[string]: *string | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Image: {
		Type: "AWS::ImageBuilder::Image"
		Properties: {
			ContainerRecipeArn?:           *string | fn.#Fn
			DistributionConfigurationArn?: *string | fn.#Fn
			EnhancedImageMetadataEnabled?: *bool | fn.#Fn
			ImageRecipeArn?:               *string | fn.#Fn
			ImageTestsConfiguration?:      *{
				ImageTestsEnabled?: *bool | fn.#Fn
				TimeoutMinutes?:    *(>=60 & <=1440) | fn.#Fn
			} | fn.#If
			InfrastructureConfigurationArn: *string | fn.#Fn
			Tags?:                          *{
				[string]: *string | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ImagePipeline: {
		Type: "AWS::ImageBuilder::ImagePipeline"
		Properties: {
			ContainerRecipeArn?:           *string | fn.#Fn
			Description?:                  *string | fn.#Fn
			DistributionConfigurationArn?: *string | fn.#Fn
			EnhancedImageMetadataEnabled?: *bool | fn.#Fn
			ImageRecipeArn?:               *string | fn.#Fn
			ImageTestsConfiguration?:      *{
				ImageTestsEnabled?: *bool | fn.#Fn
				TimeoutMinutes?:    *(>=60 & <=1440) | fn.#Fn
			} | fn.#If
			InfrastructureConfigurationArn: *string | fn.#Fn
			Name:                           *string | fn.#Fn
			Schedule?:                      *{
				PipelineExecutionStartCondition?: *("EXPRESSION_MATCH_ONLY" | "EXPRESSION_MATCH_AND_DEPENDENCY_UPDATES_AVAILABLE") | fn.#Fn
				ScheduleExpression?:              *string | fn.#Fn
			} | fn.#If
			Status?: *("DISABLED" | "ENABLED") | fn.#Fn
			Tags?:   *{
				[string]: *string | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ImageRecipe: {
		Type: "AWS::ImageBuilder::ImageRecipe"
		Properties: {
			AdditionalInstanceConfiguration?: *{
				SystemsManagerAgent?: *{
					UninstallAfterBuild?: *bool | fn.#Fn
				} | fn.#If
				UserDataOverride?: *string | fn.#Fn
			} | fn.#If
			BlockDeviceMappings?: *[...{
				DeviceName?: *string | fn.#Fn
				Ebs?:        *{
					DeleteOnTermination?: *bool | fn.#Fn
					Encrypted?:           *bool | fn.#Fn
					Iops?:                *int | fn.#Fn
					KmsKeyId?:            *string | fn.#Fn
					SnapshotId?:          *string | fn.#Fn
					Throughput?:          *int | fn.#Fn
					VolumeSize?:          *int | fn.#Fn
					VolumeType?:          *("standard" | "io1" | "io2" | "gp2" | "gp3" | "sc1" | "st1") | fn.#Fn
				} | fn.#If
				NoDevice?:    *string | fn.#Fn
				VirtualName?: *string | fn.#Fn
			}] | fn.#If
			Components: *[...{
				ComponentArn?: *string | fn.#Fn
				Parameters?:   *[...{
					Name:  *string | fn.#Fn
					Value: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				}] | fn.#If
			}] | fn.#If
			Description?: *string | fn.#Fn
			Name:         *string | fn.#Fn
			ParentImage:  *string | fn.#Fn
			Tags?:        *{
				[string]: *string | fn.#Fn
			} | fn.#If
			Version:           *string | fn.#Fn
			WorkingDirectory?: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#InfrastructureConfiguration: {
		Type: "AWS::ImageBuilder::InfrastructureConfiguration"
		Properties: {
			Description?:             *string | fn.#Fn
			InstanceMetadataOptions?: *{
				HttpPutResponseHopLimit?: *int | fn.#Fn
				HttpTokens?:              *("required" | "optional") | fn.#Fn
			} | fn.#If
			InstanceProfileName: *string | fn.#Fn
			InstanceTypes?:      [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			KeyPair?:            *string | fn.#Fn
			Logging?:            *{
				S3Logs?: *{
					S3BucketName?: *string | fn.#Fn
					S3KeyPrefix?:  *string | fn.#Fn
				} | fn.#If
			} | fn.#If
			Name:          *string | fn.#Fn
			ResourceTags?: *{
				[string]: *string | fn.#Fn
			} | fn.#If
			SecurityGroupIds?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			SnsTopicArn?:      *string | fn.#Fn
			SubnetId?:         *string | fn.#Fn
			Tags?:             *{
				[string]: *string | fn.#Fn
			} | fn.#If
			TerminateInstanceOnFailure?: *bool | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
