package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#Lightsail: {
	#Database: {
		Type: "AWS::Lightsail::Database"
		Properties: {
			AvailabilityZone?:             *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
			BackupRetention?:              *bool | fn.#Fn
			CaCertificateIdentifier?:      *string | fn.#Fn
			MasterDatabaseName:            *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
			MasterUserPassword?:           *(strings.MinRunes(1) & strings.MaxRunes(63)) | fn.#Fn
			MasterUsername:                *(strings.MinRunes(1) & strings.MaxRunes(63)) | fn.#Fn
			PreferredBackupWindow?:        *string | fn.#Fn
			PreferredMaintenanceWindow?:   *string | fn.#Fn
			PubliclyAccessible?:           *bool | fn.#Fn
			RelationalDatabaseBlueprintId: *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
			RelationalDatabaseBundleId:    *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
			RelationalDatabaseName:        *(strings.MinRunes(2) & strings.MaxRunes(255) & (=~#"\w[\w\-]*\w"#)) | fn.#Fn
			RelationalDatabaseParameters?: *[...{
				AllowedValues?:  *string | fn.#Fn
				ApplyMethod?:    *string | fn.#Fn
				ApplyType?:      *string | fn.#Fn
				DataType?:       *string | fn.#Fn
				Description?:    *string | fn.#Fn
				IsModifiable?:   *bool | fn.#Fn
				ParameterName?:  *string | fn.#Fn
				ParameterValue?: *string | fn.#Fn
			}] | fn.#If
			RotateMasterUserPassword?: *bool | fn.#Fn
			Tags?:                     *[...{
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
	#Disk: {
		Type: "AWS::Lightsail::Disk"
		Properties: {
			AddOns?: *[...{
				AddOnType:                 *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
				AutoSnapshotAddOnRequest?: *{
					SnapshotTimeOfDay?: *(=~#"^[0-9]{2}:00$"#) | fn.#Fn
				} | fn.#If
				Status?: *("Enabling" | "Disabling" | "Enabled" | "Terminating" | "Terminated" | "Disabled" | "Failed") | fn.#Fn
			}] | fn.#If
			AvailabilityZone?: *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
			DiskName:          *(strings.MinRunes(1) & strings.MaxRunes(254) & (=~#"^[a-zA-Z0-9][\w\-.]*[a-zA-Z0-9]$"#)) | fn.#Fn
			SizeInGb:          *int | fn.#Fn
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
	#Instance: {
		Type: "AWS::Lightsail::Instance"
		Properties: {
			AddOns?: *[...{
				AddOnType:                 *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
				AutoSnapshotAddOnRequest?: *{
					SnapshotTimeOfDay?: *(=~#"^[0-9]{2}:00$"#) | fn.#Fn
				} | fn.#If
				Status?: *("Enabling" | "Disabling" | "Enabled" | "Terminating" | "Terminated" | "Disabled" | "Failed") | fn.#Fn
			}] | fn.#If
			AvailabilityZone?: *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
			BlueprintId:       *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
			BundleId:          *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
			Hardware?:         *{
				CpuCount?: *int | fn.#Fn
				Disks?:    *[...{
					AttachedTo?:      *string | fn.#Fn
					AttachmentState?: *string | fn.#Fn
					DiskName:         *(strings.MinRunes(1) & strings.MaxRunes(254) & (=~#"^[a-zA-Z0-9][\w\-.]*[a-zA-Z0-9]$"#)) | fn.#Fn
					IOPS?:            *int | fn.#Fn
					IsSystemDisk?:    *bool | fn.#Fn
					Path:             *string | fn.#Fn
					SizeInGb?:        *string | fn.#Fn
				}] | fn.#If
				RamSizeInGb?: *int | fn.#Fn
			} | fn.#If
			InstanceName: *(strings.MinRunes(1) & strings.MaxRunes(254) & (=~#"^[a-zA-Z0-9][\w\-.]*[a-zA-Z0-9]$"#)) | fn.#Fn
			KeyPairName?: *string | fn.#Fn
			Networking?:  *{
				MonthlyTransfer?: *{
					GbPerMonthAllocated?: *string | fn.#Fn
				} | fn.#If
				Ports: *[...{
					AccessDirection?: *string | fn.#Fn
					AccessFrom?:      *string | fn.#Fn
					AccessType?:      *string | fn.#Fn
					CidrListAliases?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					Cidrs?:           [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					CommonName?:      *string | fn.#Fn
					FromPort?:        *int | fn.#Fn
					Ipv6Cidrs?:       [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					Protocol?:        *string | fn.#Fn
					ToPort?:          *int | fn.#Fn
				}] | fn.#If
			} | fn.#If
			Tags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			UserData?: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#StaticIp: {
		Type: "AWS::Lightsail::StaticIp"
		Properties: {
			AttachedTo?:  *string | fn.#Fn
			StaticIpName: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
