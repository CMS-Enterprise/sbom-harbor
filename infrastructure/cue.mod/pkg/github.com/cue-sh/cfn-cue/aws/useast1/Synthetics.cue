package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#Synthetics: {
	#Canary: {
		Type: "AWS::Synthetics::Canary"
		Properties: {
			ArtifactConfig?: *{
				S3Encryption?: *{
					EncryptionMode?: *string | fn.#Fn
					KmsKeyArn?:      *string | fn.#Fn
				} | fn.#If
			} | fn.#If
			ArtifactS3Location: *(=~#"^(s3|S3)://"#) | fn.#Fn
			Code:               *{
				Handler:          *string | fn.#Fn
				S3Bucket?:        *string | fn.#Fn
				S3Key?:           *string | fn.#Fn
				S3ObjectVersion?: *string | fn.#Fn
				Script?:          *string | fn.#Fn
			} | fn.#If
			ExecutionRoleArn:        *string | fn.#Fn
			FailureRetentionPeriod?: *int | fn.#Fn
			Name:                    *(=~#"^[0-9a-z_\-]{1,21}$"#) | fn.#Fn
			RunConfig?:              *{
				ActiveTracing?:        *bool | fn.#Fn
				EnvironmentVariables?: *{
					[string]: *string | fn.#Fn
				} | fn.#If
				MemoryInMB?:       *int | fn.#Fn
				TimeoutInSeconds?: *int | fn.#Fn
			} | fn.#If
			RuntimeVersion: *string | fn.#Fn
			Schedule:       *{
				DurationInSeconds?: *string | fn.#Fn
				Expression:         *string | fn.#Fn
			} | fn.#If
			StartCanaryAfterCreation: *bool | fn.#Fn
			SuccessRetentionPeriod?:  *int | fn.#Fn
			Tags?:                    *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			VPCConfig?: *{
				SecurityGroupIds: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				SubnetIds:        [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				VpcId?:           *string | fn.#Fn
			} | fn.#If
			VisualReference?: *{
				BaseCanaryRunId:  *string | fn.#Fn
				BaseScreenshots?: *[...{
					IgnoreCoordinates?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					ScreenshotName:     *string | fn.#Fn
				}] | fn.#If
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
