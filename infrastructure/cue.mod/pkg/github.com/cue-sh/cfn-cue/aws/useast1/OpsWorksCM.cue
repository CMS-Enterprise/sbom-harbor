package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#OpsWorksCM: {
	#Server: {
		Type: "AWS::OpsWorksCM::Server"
		Properties: {
			AssociatePublicIpAddress?: *bool | fn.#Fn
			BackupId?:                 *(=~#"[a-zA-Z][a-zA-Z0-9\-\.\:]*"#) | fn.#Fn
			BackupRetentionCount?:     *int | fn.#Fn
			CustomCertificate?:        *(=~#"(?s)\s*-----BEGIN CERTIFICATE-----.+-----END CERTIFICATE-----\s*"#) | fn.#Fn
			CustomDomain?:             *(=~#"^(((?!-)[A-Za-z0-9-]{0,62}[A-Za-z0-9])\.)+((?!-)[A-Za-z0-9-]{1,62}[A-Za-z0-9])$"#) | fn.#Fn
			CustomPrivateKey?:         *string | fn.#Fn
			DisableAutomatedBackup?:   *bool | fn.#Fn
			Engine?:                   *string | fn.#Fn
			EngineAttributes?:         *[...{
				Name?:  *(=~#"(?s).*"#) | fn.#Fn
				Value?: *(=~#"(?s).*"#) | fn.#Fn
			}] | fn.#If
			EngineModel?:                *string | fn.#Fn
			EngineVersion?:              *string | fn.#Fn
			InstanceProfileArn:          *(=~#"arn:aws:iam::[0-9]{12}:instance-profile/.*"#) | fn.#Fn
			InstanceType:                *string | fn.#Fn
			KeyPair?:                    *(=~#".*"#) | fn.#Fn
			PreferredBackupWindow?:      *(=~#"^((Mon|Tue|Wed|Thu|Fri|Sat|Sun):)?([0-1][0-9]|2[0-3]):[0-5][0-9]$"#) | fn.#Fn
			PreferredMaintenanceWindow?: *(=~#"^((Mon|Tue|Wed|Thu|Fri|Sat|Sun):)?([0-1][0-9]|2[0-3]):[0-5][0-9]$"#) | fn.#Fn
			SecurityGroupIds?:           [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			ServerName?:                 *(strings.MinRunes(1) & strings.MaxRunes(40) & (=~#"[a-zA-Z][a-zA-Z0-9\-]*"#)) | fn.#Fn
			ServiceRoleArn:              *(=~#"arn:aws:iam::[0-9]{12}:role/.*"#) | fn.#Fn
			SubnetIds?:                  [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			Tags?:                       *[...{
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
