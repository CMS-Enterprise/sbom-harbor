package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#NimbleStudio: {
	#LaunchProfile: {
		Type: "AWS::NimbleStudio::LaunchProfile"
		Properties: {
			Description?:                  *string | fn.#Fn
			Ec2SubnetIds:                  [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			LaunchProfileProtocolVersions: [...(*(=~#"^2021\-03\-31$"#) | fn.#Fn)] | (*(=~#"^2021\-03\-31$"#) | fn.#Fn)
			Name:                          *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
			StreamConfiguration:           *{
				ClipboardMode:              *("ENABLED" | "DISABLED") | fn.#Fn
				Ec2InstanceTypes:           [...(*("g4dn.xlarge" | "g4dn.2xlarge" | "g4dn.4xlarge" | "g4dn.8xlarge" | "g4dn.12xlarge" | "g4dn.16xlarge") | fn.#Fn)] | (*("g4dn.xlarge" | "g4dn.2xlarge" | "g4dn.4xlarge" | "g4dn.8xlarge" | "g4dn.12xlarge" | "g4dn.16xlarge") | fn.#Fn)
				MaxSessionLengthInMinutes?: *(>=1 & <=690) | fn.#Fn
				StreamingImageIds:          [...(*(=~#"^[a-zA-Z0-9-_]*$"#) | fn.#Fn)] | (*(=~#"^[a-zA-Z0-9-_]*$"#) | fn.#Fn)
			} | fn.#If
			StudioComponentIds: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			StudioId:           *string | fn.#Fn
			Tags?:              *{
				[string]: *string | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#StreamingImage: {
		Type: "AWS::NimbleStudio::StreamingImage"
		Properties: {
			Description?: *string | fn.#Fn
			Ec2ImageId:   *(=~#"^ami-[0-9A-z]+$"#) | fn.#Fn
			Name:         *string | fn.#Fn
			StudioId:     *string | fn.#Fn
			Tags?:        *{
				[string]: *string | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Studio: {
		Type: "AWS::NimbleStudio::Studio"
		Properties: {
			AdminRoleArn:                   *string | fn.#Fn
			DisplayName:                    *string | fn.#Fn
			StudioEncryptionConfiguration?: *{
				KeyArn?: *(=~#"^arn:.*"#) | fn.#Fn
				KeyType: *("AWS_OWNED_KEY" | "CUSTOMER_MANAGED_KEY") | fn.#Fn
			} | fn.#If
			StudioName: *(strings.MinRunes(3) & strings.MaxRunes(64) & (=~#"^[a-z0-9]*$"#)) | fn.#Fn
			Tags?:      *{
				[string]: *string | fn.#Fn
			} | fn.#If
			UserRoleArn: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#StudioComponent: {
		Type: "AWS::NimbleStudio::StudioComponent"
		Properties: {
			Configuration?: *{
				ActiveDirectoryConfiguration?: *{
					ComputerAttributes?: *[...{
						Name?:  *(strings.MinRunes(1) & strings.MaxRunes(40)) | fn.#Fn
						Value?: *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
					}] | fn.#If
					DirectoryId?:                         *string | fn.#Fn
					OrganizationalUnitDistinguishedName?: *(strings.MinRunes(1) & strings.MaxRunes(2000)) | fn.#Fn
				} | fn.#If
				ComputeFarmConfiguration?: *{
					ActiveDirectoryUser?: *string | fn.#Fn
					Endpoint?:            *string | fn.#Fn
				} | fn.#If
				LicenseServiceConfiguration?: *{
					Endpoint?: *string | fn.#Fn
				} | fn.#If
				SharedFileSystemConfiguration?: *{
					Endpoint?:          *string | fn.#Fn
					FileSystemId?:      *string | fn.#Fn
					LinuxMountPoint?:   *(=~#"^(/?|(\$HOME)?(/[^/\n\s\\]+)*)$"#) | fn.#Fn
					ShareName?:         *string | fn.#Fn
					WindowsMountDrive?: *(=~#"^[A-Z]$"#) | fn.#Fn
				} | fn.#If
			} | fn.#If
			Description?:           *string | fn.#Fn
			Ec2SecurityGroupIds?:   [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			InitializationScripts?: *[...{
				LaunchProfileProtocolVersion?: *(=~#"^2021\-03\-31$"#) | fn.#Fn
				Platform?:                     *("LINUX" | "WINDOWS") | fn.#Fn
				RunContext?:                   *("SYSTEM_INITIALIZATION" | "USER_INITIALIZATION") | fn.#Fn
				Script?:                       *(strings.MinRunes(1) & strings.MaxRunes(5120)) | fn.#Fn
			}] | fn.#If
			Name:              *string | fn.#Fn
			ScriptParameters?: *[...{
				Key?:   *(strings.MinRunes(1) & strings.MaxRunes(64) & (=~#"^[a-zA-Z_][a-zA-Z0-9_]+$"#)) | fn.#Fn
				Value?: *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
			}] | fn.#If
			StudioId: *string | fn.#Fn
			Subtype?: *("AWS_MANAGED_MICROSOFT_AD" | "AMAZON_FSX_FOR_WINDOWS" | "AMAZON_FSX_FOR_LUSTRE" | "CUSTOM") | fn.#Fn
			Tags?:    *{
				[string]: *string | fn.#Fn
			} | fn.#If
			Type: *("ACTIVE_DIRECTORY" | "SHARED_FILE_SYSTEM" | "COMPUTE_FARM" | "LICENSE_SERVICE" | "CUSTOM") | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
