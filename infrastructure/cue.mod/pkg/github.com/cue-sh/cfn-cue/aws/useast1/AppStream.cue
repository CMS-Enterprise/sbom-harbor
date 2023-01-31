package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#AppStream: {
	#AppBlock: {
		Type: "AWS::AppStream::AppBlock"
		Properties: {
			Description?:       *string | fn.#Fn
			DisplayName?:       *string | fn.#Fn
			Name:               *string | fn.#Fn
			SetupScriptDetails: *{
				ExecutableParameters?: *string | fn.#Fn
				ExecutablePath:        *string | fn.#Fn
				ScriptS3Location:      *{
					S3Bucket: *string | fn.#Fn
					S3Key:    *string | fn.#Fn
				} | fn.#If
				TimeoutInSeconds: *int | fn.#Fn
			} | fn.#If
			SourceS3Location: *{
				S3Bucket: *string | fn.#Fn
				S3Key:    *string | fn.#Fn
			} | fn.#If
			Tags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Application: {
		Type: "AWS::AppStream::Application"
		Properties: {
			AppBlockArn:         *string | fn.#Fn
			AttributesToDelete?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			Description?:        *string | fn.#Fn
			DisplayName?:        *string | fn.#Fn
			IconS3Location:      *{
				S3Bucket: *string | fn.#Fn
				S3Key:    *string | fn.#Fn
			} | fn.#If
			InstanceFamilies:  [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			LaunchParameters?: *string | fn.#Fn
			LaunchPath:        *string | fn.#Fn
			Name:              *string | fn.#Fn
			Platforms:         [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			Tags?:             *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			WorkingDirectory?: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ApplicationFleetAssociation: {
		Type: "AWS::AppStream::ApplicationFleetAssociation"
		Properties: {
			ApplicationArn: *string | fn.#Fn
			FleetName:      *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#DirectoryConfig: {
		Type: "AWS::AppStream::DirectoryConfig"
		Properties: {
			DirectoryName:                        *string | fn.#Fn
			OrganizationalUnitDistinguishedNames: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			ServiceAccountCredentials:            *{
				AccountName:     *string | fn.#Fn
				AccountPassword: *string | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Fleet: {
		Type: "AWS::AppStream::Fleet"
		Properties: {
			ComputeCapacity?: *{
				DesiredInstances: *int | fn.#Fn
			} | fn.#If
			Description?:                *string | fn.#Fn
			DisconnectTimeoutInSeconds?: *(>=60 & <=360000) | fn.#Fn
			DisplayName?:                *string | fn.#Fn
			DomainJoinInfo?:             *{
				DirectoryName?:                       *string | fn.#Fn
				OrganizationalUnitDistinguishedName?: *string | fn.#Fn
			} | fn.#If
			EnableDefaultInternetAccess?:    *bool | fn.#Fn
			FleetType?:                      *string | fn.#Fn
			IamRoleArn?:                     *string | fn.#Fn
			IdleDisconnectTimeoutInSeconds?: *(>=0 & <=3600) | fn.#Fn
			ImageArn?:                       *string | fn.#Fn
			ImageName?:                      *string | fn.#Fn
			InstanceType:                    *("stream.compute.2xlarge" | "stream.compute.4xlarge" | "stream.compute.8xlarge" | "stream.compute.large" | "stream.compute.xlarge" | "stream.graphics-design.2xlarge" | "stream.graphics-design.4xlarge" | "stream.graphics-design.large" | "stream.graphics-design.xlarge" | "stream.graphics-desktop.2xlarge" | "stream.graphics-pro.16xlarge" | "stream.graphics-pro.4xlarge" | "stream.graphics-pro.8xlarge" | "stream.graphics.g4dn.12xlarge" | "stream.graphics.g4dn.16xlarge" | "stream.graphics.g4dn.2xlarge" | "stream.graphics.g4dn.4xlarge" | "stream.graphics.g4dn.8xlarge" | "stream.graphics.g4dn.xlarge" | "stream.memory.2xlarge" | "stream.memory.4xlarge" | "stream.memory.8xlarge" | "stream.memory.large" | "stream.memory.xlarge" | "stream.memory.z1d.12xlarge" | "stream.memory.z1d.2xlarge" | "stream.memory.z1d.3xlarge" | "stream.memory.z1d.6xlarge" | "stream.memory.z1d.large" | "stream.memory.z1d.xlarge" | "stream.standard.large" | "stream.standard.medium" | "stream.standard.small") | fn.#Fn
			MaxConcurrentSessions?:          *int | fn.#Fn
			MaxUserDurationInSeconds?:       *(>=600 & <=360000) | fn.#Fn
			Name:                            *string | fn.#Fn
			Platform?:                       *string | fn.#Fn
			StreamView?:                     *string | fn.#Fn
			Tags?:                           *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			UsbDeviceFilterStrings?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			VpcConfig?:              *{
				SecurityGroupIds?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				SubnetIds?:        [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ImageBuilder: {
		Type: "AWS::AppStream::ImageBuilder"
		Properties: {
			AccessEndpoints?: *[...{
				EndpointType: *string | fn.#Fn
				VpceId:       *string | fn.#Fn
			}] | fn.#If
			AppstreamAgentVersion?: *string | fn.#Fn
			Description?:           *string | fn.#Fn
			DisplayName?:           *string | fn.#Fn
			DomainJoinInfo?:        *{
				DirectoryName?:                       *string | fn.#Fn
				OrganizationalUnitDistinguishedName?: *string | fn.#Fn
			} | fn.#If
			EnableDefaultInternetAccess?: *bool | fn.#Fn
			IamRoleArn?:                  *string | fn.#Fn
			ImageArn?:                    *string | fn.#Fn
			ImageName?:                   *string | fn.#Fn
			InstanceType:                 *("stream.compute.2xlarge" | "stream.compute.4xlarge" | "stream.compute.8xlarge" | "stream.compute.large" | "stream.compute.xlarge" | "stream.graphics-design.2xlarge" | "stream.graphics-design.4xlarge" | "stream.graphics-design.large" | "stream.graphics-design.xlarge" | "stream.graphics-desktop.2xlarge" | "stream.graphics-pro.16xlarge" | "stream.graphics-pro.4xlarge" | "stream.graphics-pro.8xlarge" | "stream.graphics.g4dn.12xlarge" | "stream.graphics.g4dn.16xlarge" | "stream.graphics.g4dn.2xlarge" | "stream.graphics.g4dn.4xlarge" | "stream.graphics.g4dn.8xlarge" | "stream.graphics.g4dn.xlarge" | "stream.memory.2xlarge" | "stream.memory.4xlarge" | "stream.memory.8xlarge" | "stream.memory.large" | "stream.memory.xlarge" | "stream.memory.z1d.12xlarge" | "stream.memory.z1d.2xlarge" | "stream.memory.z1d.3xlarge" | "stream.memory.z1d.6xlarge" | "stream.memory.z1d.large" | "stream.memory.z1d.xlarge" | "stream.standard.large" | "stream.standard.medium" | "stream.standard.small") | fn.#Fn
			Name:                         *string | fn.#Fn
			Tags?:                        *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			VpcConfig?: *{
				SecurityGroupIds?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				SubnetIds?:        [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Stack: {
		Type: "AWS::AppStream::Stack"
		Properties: {
			AccessEndpoints?: *[...{
				EndpointType: *string | fn.#Fn
				VpceId:       *string | fn.#Fn
			}] | fn.#If
			ApplicationSettings?: *{
				Enabled:        *bool | fn.#Fn
				SettingsGroup?: *string | fn.#Fn
			} | fn.#If
			AttributesToDelete?:      [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			DeleteStorageConnectors?: *bool | fn.#Fn
			Description?:             *string | fn.#Fn
			DisplayName?:             *string | fn.#Fn
			EmbedHostDomains?:        [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			FeedbackURL?:             *string | fn.#Fn
			Name?:                    *string | fn.#Fn
			RedirectURL?:             *string | fn.#Fn
			StorageConnectors?:       *[...{
				ConnectorType:       *string | fn.#Fn
				Domains?:            [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				ResourceIdentifier?: *string | fn.#Fn
			}] | fn.#If
			Tags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			UserSettings?: *[...{
				Action:     *string | fn.#Fn
				Permission: *string | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#StackFleetAssociation: {
		Type: "AWS::AppStream::StackFleetAssociation"
		Properties: {
			FleetName: *string | fn.#Fn
			StackName: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#StackUserAssociation: {
		Type: "AWS::AppStream::StackUserAssociation"
		Properties: {
			AuthenticationType:     *string | fn.#Fn
			SendEmailNotification?: *bool | fn.#Fn
			StackName:              *string | fn.#Fn
			UserName:               *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#User: {
		Type: "AWS::AppStream::User"
		Properties: {
			AuthenticationType: *string | fn.#Fn
			FirstName?:         *string | fn.#Fn
			LastName?:          *string | fn.#Fn
			MessageAction?:     *string | fn.#Fn
			UserName:           *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
