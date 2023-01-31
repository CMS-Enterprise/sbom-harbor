package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#DataSync: {
	#Agent: {
		Type: "AWS::DataSync::Agent"
		Properties: {
			ActivationKey:      *(=~#"[A-Z0-9]{5}(-[A-Z0-9]{5}){4}"#) | fn.#Fn
			AgentName?:         *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"^[a-zA-Z0-9\s+=._:@/-]+$"#)) | fn.#Fn
			SecurityGroupArns?: [...(*(=~#"^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):ec2:[a-z\-0-9]*:[0-9]{12}:security-group/.*$"#) | fn.#Fn)] | (*(=~#"^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):ec2:[a-z\-0-9]*:[0-9]{12}:security-group/.*$"#) | fn.#Fn)
			SubnetArns?:        [...(*(=~#"^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):ec2:[a-z\-0-9]*:[0-9]{12}:subnet/.*$"#) | fn.#Fn)] | (*(=~#"^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):ec2:[a-z\-0-9]*:[0-9]{12}:subnet/.*$"#) | fn.#Fn)
			Tags?:              *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			VpcEndpointId?: *(=~#"^vpce-[0-9a-f]{17}$"#) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#LocationEFS: {
		Type: "AWS::DataSync::LocationEFS"
		Properties: {
			Ec2Config: *{
				SecurityGroupArns: [...(*(=~#"^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):ec2:[a-z\-0-9]*:[0-9]{12}:security-group/.*$"#) | fn.#Fn)] | (*(=~#"^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):ec2:[a-z\-0-9]*:[0-9]{12}:security-group/.*$"#) | fn.#Fn)
				SubnetArn:         *(=~#"^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):ec2:[a-z\-0-9]*:[0-9]{12}:subnet/.*$"#) | fn.#Fn
			} | fn.#If
			EfsFilesystemArn: *(=~#"^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):elasticfilesystem:[a-z\-0-9]*:[0-9]{12}:file-system/fs-.*$"#) | fn.#Fn
			Subdirectory?:    *string | fn.#Fn
			Tags?:            *[...{
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
	#LocationFSxWindows: {
		Type: "AWS::DataSync::LocationFSxWindows"
		Properties: {
			Domain?:           *(=~#"^([A-Za-z0-9]+[A-Za-z0-9-.]*)*[A-Za-z0-9-]*[A-Za-z0-9]$"#) | fn.#Fn
			FsxFilesystemArn:  *(=~#"^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):fsx:[a-z\-0-9]*:[0-9]{12}:file-system/fs-.*$"#) | fn.#Fn
			Password:          *(=~#"^.{0,104}$"#) | fn.#Fn
			SecurityGroupArns: [...(*(=~#"^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):ec2:[a-z\-0-9]*:[0-9]{12}:security-group/.*$"#) | fn.#Fn)] | (*(=~#"^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):ec2:[a-z\-0-9]*:[0-9]{12}:security-group/.*$"#) | fn.#Fn)
			Subdirectory?:     *string | fn.#Fn
			Tags?:             *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			User: *(=~#"^[^\x5B\x5D\\/:;|=,+*?]{1,104}$"#) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#LocationHDFS: {
		Type: "AWS::DataSync::LocationHDFS"
		Properties: {
			AgentArns:          [...(*(=~#"^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):datasync:[a-z\-0-9]+:[0-9]{12}:agent/agent-[0-9a-z]{17}$"#) | fn.#Fn)] | (*(=~#"^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):datasync:[a-z\-0-9]+:[0-9]{12}:agent/agent-[0-9a-z]{17}$"#) | fn.#Fn)
			AuthenticationType: *("SIMPLE" | "KERBEROS") | fn.#Fn
			BlockSize?:         *(>=1048576 & <=1073741824) | fn.#Fn
			KerberosKeytab?:    *string | fn.#Fn
			KerberosKrb5Conf?:  *string | fn.#Fn
			KerberosPrincipal?: *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"^.+$"#)) | fn.#Fn
			KmsKeyProviderUri?: *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"^kms:\/\/http[s]?@(([a-zA-Z0-9\-]*[a-zA-Z0-9])\.)*([A-Za-z0-9\-]*[A-Za-z0-9])(;(([a-zA-Z0-9\-]*[a-zA-Z0-9])\.)*([A-Za-z0-9\-]*[A-Za-z0-9]))*:[0-9]{1,5}\/kms$"#)) | fn.#Fn
			NameNodes:          *[...{
				Hostname: *(=~#"^(([a-zA-Z0-9\-]*[a-zA-Z0-9])\.)*([A-Za-z0-9\-]*[A-Za-z0-9])$"#) | fn.#Fn
				Port:     *(>=1 & <=65536) | fn.#Fn
			}] | fn.#If
			QopConfiguration?: *{
				DataTransferProtection?: *("AUTHENTICATION" | "INTEGRITY" | "PRIVACY" | "DISABLED") | fn.#Fn
				RpcProtection?:          *("AUTHENTICATION" | "INTEGRITY" | "PRIVACY" | "DISABLED") | fn.#Fn
			} | fn.#If
			ReplicationFactor?: *(>=1 & <=512) | fn.#Fn
			SimpleUser?:        *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"^[_.A-Za-z0-9][-_.A-Za-z0-9]*$"#)) | fn.#Fn
			Subdirectory?:      *string | fn.#Fn
			Tags?:              *[...{
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
	#LocationNFS: {
		Type: "AWS::DataSync::LocationNFS"
		Properties: {
			MountOptions?: *{
				Version?: *("AUTOMATIC" | "NFS3" | "NFS4_0" | "NFS4_1") | fn.#Fn
			} | fn.#If
			OnPremConfig: *{
				AgentArns: [...(*(=~#"^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):datasync:[a-z\-0-9]+:[0-9]{12}:agent/agent-[0-9a-z]{17}$"#) | fn.#Fn)] | (*(=~#"^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):datasync:[a-z\-0-9]+:[0-9]{12}:agent/agent-[0-9a-z]{17}$"#) | fn.#Fn)
			} | fn.#If
			ServerHostname: *(=~#"^(([a-zA-Z0-9\-]*[a-zA-Z0-9])\.)*([A-Za-z0-9\-]*[A-Za-z0-9])$"#) | fn.#Fn
			Subdirectory:   *string | fn.#Fn
			Tags?:          *[...{
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
	#LocationObjectStorage: {
		Type: "AWS::DataSync::LocationObjectStorage"
		Properties: {
			AccessKey?:      *(strings.MinRunes(8) & strings.MaxRunes(200) & (=~#"^.+$"#)) | fn.#Fn
			AgentArns:       [...(*(=~#"^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):datasync:[a-z\-0-9]+:[0-9]{12}:agent/agent-[0-9a-z]{17}$"#) | fn.#Fn)] | (*(=~#"^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):datasync:[a-z\-0-9]+:[0-9]{12}:agent/agent-[0-9a-z]{17}$"#) | fn.#Fn)
			BucketName:      *string | fn.#Fn
			SecretKey?:      *(strings.MinRunes(8) & strings.MaxRunes(200) & (=~#"^.+$"#)) | fn.#Fn
			ServerHostname:  *(=~#"^(([a-zA-Z0-9\-]*[a-zA-Z0-9])\.)*([A-Za-z0-9\-]*[A-Za-z0-9])$"#) | fn.#Fn
			ServerPort?:     *(>=1 & <=65536) | fn.#Fn
			ServerProtocol?: *("HTTPS" | "HTTP") | fn.#Fn
			Subdirectory?:   *string | fn.#Fn
			Tags?:           *[...{
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
	#LocationS3: {
		Type: "AWS::DataSync::LocationS3"
		Properties: {
			S3BucketArn: *(=~#"^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):s3:[a-z\-0-9]*:[0-9]*:.*$"#) | fn.#Fn
			S3Config:    *{
				BucketAccessRoleArn: *(=~#"^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):iam::[0-9]{12}:role/.*$"#) | fn.#Fn
			} | fn.#If
			S3StorageClass?: *("STANDARD" | "STANDARD_IA" | "ONEZONE_IA" | "INTELLIGENT_TIERING" | "GLACIER" | "DEEP_ARCHIVE") | fn.#Fn
			Subdirectory?:   *string | fn.#Fn
			Tags?:           *[...{
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
	#LocationSMB: {
		Type: "AWS::DataSync::LocationSMB"
		Properties: {
			AgentArns:     [...(*(=~#"^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):datasync:[a-z\-0-9]+:[0-9]{12}:agent/agent-[0-9a-z]{17}$"#) | fn.#Fn)] | (*(=~#"^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):datasync:[a-z\-0-9]+:[0-9]{12}:agent/agent-[0-9a-z]{17}$"#) | fn.#Fn)
			Domain?:       *(=~#"^([A-Za-z0-9]+[A-Za-z0-9-.]*)*[A-Za-z0-9-]*[A-Za-z0-9]$"#) | fn.#Fn
			MountOptions?: *{
				Version?: *("AUTOMATIC" | "SMB2" | "SMB3") | fn.#Fn
			} | fn.#If
			Password:       *(=~#"^.{0,104}$"#) | fn.#Fn
			ServerHostname: *(=~#"^(([a-zA-Z0-9\-]*[a-zA-Z0-9])\.)*([A-Za-z0-9\-]*[A-Za-z0-9])$"#) | fn.#Fn
			Subdirectory:   *string | fn.#Fn
			Tags?:          *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			User: *(=~#"^[^\x5B\x5D\\/:;|=,+*?]{1,104}$"#) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Task: {
		Type: "AWS::DataSync::Task"
		Properties: {
			CloudWatchLogGroupArn?: *(=~#"^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):logs:[a-z\-0-9]*:[0-9]{12}:log-group:([^:\*]*)(:\*)?$"#) | fn.#Fn
			DestinationLocationArn: *(=~#"^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):datasync:[a-z\-0-9]+:[0-9]{12}:location/loc-[0-9a-z]{17}$"#) | fn.#Fn
			Excludes?:              *[...{
				FilterType?: *("SIMPLE_PATTERN" & (=~#"^[A-Z0-9_]+$"#)) | fn.#Fn
				Value?:      *(=~#"^[^\x00]+$"#) | fn.#Fn
			}] | fn.#If
			Includes?: *[...{
				FilterType?: *("SIMPLE_PATTERN" & (=~#"^[A-Z0-9_]+$"#)) | fn.#Fn
				Value?:      *(=~#"^[^\x00]+$"#) | fn.#Fn
			}] | fn.#If
			Name?:    *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"^[a-zA-Z0-9\s+=._:@/-]+$"#)) | fn.#Fn
			Options?: *{
				Atime?:                       *("NONE" | "BEST_EFFORT") | fn.#Fn
				BytesPerSecond?:              *int | fn.#Fn
				Gid?:                         *("NONE" | "INT_VALUE" | "NAME" | "BOTH") | fn.#Fn
				LogLevel?:                    *("OFF" | "BASIC" | "TRANSFER") | fn.#Fn
				Mtime?:                       *("NONE" | "PRESERVE") | fn.#Fn
				OverwriteMode?:               *("ALWAYS" | "NEVER") | fn.#Fn
				PosixPermissions?:            *("NONE" | "PRESERVE") | fn.#Fn
				PreserveDeletedFiles?:        *("PRESERVE" | "REMOVE") | fn.#Fn
				PreserveDevices?:             *("NONE" | "PRESERVE") | fn.#Fn
				SecurityDescriptorCopyFlags?: *("NONE" | "OWNER_DACL" | "OWNER_DACL_SACL") | fn.#Fn
				TaskQueueing?:                *("ENABLED" | "DISABLED") | fn.#Fn
				TransferMode?:                *("CHANGED" | "ALL") | fn.#Fn
				Uid?:                         *("NONE" | "INT_VALUE" | "NAME" | "BOTH") | fn.#Fn
				VerifyMode?:                  *("POINT_IN_TIME_CONSISTENT" | "ONLY_FILES_TRANSFERRED" | "NONE") | fn.#Fn
			} | fn.#If
			Schedule?: *{
				ScheduleExpression: *(=~#"^[a-zA-Z0-9\ \_\*\?\,\|\^\-\/\#\s\(\)\+]*$"#) | fn.#Fn
			} | fn.#If
			SourceLocationArn: *(=~#"^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):datasync:[a-z\-0-9]+:[0-9]{12}:location/loc-[0-9a-z]{17}$"#) | fn.#Fn
			Tags?:             *[...{
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
}
