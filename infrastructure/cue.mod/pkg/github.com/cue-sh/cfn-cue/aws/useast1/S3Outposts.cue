package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#S3Outposts: {
	#AccessPoint: {
		Type: "AWS::S3Outposts::AccessPoint"
		Properties: {
			Bucket:  *(strings.MinRunes(20) & strings.MaxRunes(2048) & (=~#"^arn:[^:]+:s3-outposts:[a-zA-Z0-9\-]+:\d{12}:outpost\/[^:]+\/bucket\/[^:]+$"#)) | fn.#Fn
			Name:    *(strings.MinRunes(3) & strings.MaxRunes(50) & (=~#"^[a-z0-9]([a-z0-9\\-]*[a-z0-9])?$"#)) | fn.#Fn
			Policy?: *{
				[string]: _
			} | fn.#Fn
			VpcConfiguration: *{
				VpcId?: *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Bucket: {
		Type: "AWS::S3Outposts::Bucket"
		Properties: {
			BucketName:              *(strings.MinRunes(3) & strings.MaxRunes(63) & (=~#"(?=^.{3,63}$)(?!^(\d+\.)+\d+$)(^(([a-z0-9]|[a-z0-9][a-z0-9\-]*[a-z0-9])\.)*([a-z0-9]|[a-z0-9][a-z0-9\-]*[a-z0-9])$)"#)) | fn.#Fn
			LifecycleConfiguration?: *{
				Rules: *[...{
					AbortIncompleteMultipartUpload?: *{
						DaysAfterInitiation: *int | fn.#Fn
					} | fn.#If
					ExpirationDate?:   *(=~#"^([0-2]\d{3})-(0[0-9]|1[0-2])-([0-2]\d|3[01])T([01]\d|2[0-4]):([0-5]\d):([0-6]\d)((\.\d{3})?)Z$"#) | fn.#Fn
					ExpirationInDays?: *int | fn.#Fn
					Filter?:           *{
						[string]: _
					} | fn.#Fn
					Id?:     *string | fn.#Fn
					Status?: *("Enabled" | "Disabled") | fn.#Fn
				}] | fn.#If
			} | fn.#If
			OutpostId: *(=~#"^(op-[a-f0-9]{17}|\d{12}|ec2)$"#) | fn.#Fn
			Tags?:     *[...{
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
	#BucketPolicy: {
		Type: "AWS::S3Outposts::BucketPolicy"
		Properties: {
			Bucket:         *(strings.MinRunes(20) & strings.MaxRunes(2048) & (=~#"^arn:[^:]+:s3-outposts:[a-zA-Z0-9\-]+:\d{12}:outpost\/[^:]+\/bucket\/[^:]+$"#)) | fn.#Fn
			PolicyDocument: *{
				{
					[string]: _
				}
				Version: string | *"2012-10-17"
			} | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Endpoint: {
		Type: "AWS::S3Outposts::Endpoint"
		Properties: {
			AccessType?:            *("CustomerOwnedIp" | "Private") | fn.#Fn
			CustomerOwnedIpv4Pool?: *(=~#"^ipv4pool-coip-([0-9a-f]{17})$"#) | fn.#Fn
			OutpostId:              *(=~#"^(op-[a-f0-9]{17}|\d{12}|ec2)$"#) | fn.#Fn
			SecurityGroupId:        *(strings.MinRunes(1) & strings.MaxRunes(100) & (=~#"^sg-([0-9a-f]{8}|[0-9a-f]{17})$"#)) | fn.#Fn
			SubnetId:               *(strings.MinRunes(1) & strings.MaxRunes(100) & (=~#"^subnet-([0-9a-f]{8}|[0-9a-f]{17})$"#)) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
