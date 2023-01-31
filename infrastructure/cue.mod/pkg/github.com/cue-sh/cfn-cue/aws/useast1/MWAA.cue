package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#MWAA: {
	#Environment: {
		Type: "AWS::MWAA::Environment"
		Properties: {
			AirflowConfigurationOptions?: *{
				[string]: _
			} | fn.#Fn
			AirflowVersion?:       *(=~#"^[0-9a-z.]+$"#) | fn.#Fn
			DagS3Path?:            *(=~#".*"#) | fn.#Fn
			EnvironmentClass?:     *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
			ExecutionRoleArn?:     *(=~#"^arn:(aws|aws-us-gov|aws-cn|aws-iso|aws-iso-b)(-[a-z]+)?:iam::\d{12}:role/?[a-zA-Z_0-9+=,.@\-_/]+$"#) | fn.#Fn
			KmsKey?:               *(=~#"^(((arn:(aws|aws-us-gov|aws-cn|aws-iso|aws-iso-b)(-[a-z]+)?:kms:[a-z]{2}-[a-z]+-\d:\d+:)?key\/)?[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}|(arn:(aws|aws-us-gov|aws-cn|aws-iso|aws-iso-b):kms:[a-z]{2}-[a-z]+-\d:\d+:)?alias/.+)$"#) | fn.#Fn
			LoggingConfiguration?: *{
				DagProcessingLogs?: *{
					CloudWatchLogGroupArn?: *(=~#"^arn:(aws|aws-us-gov|aws-cn|aws-iso|aws-iso-b)(-[a-z]+)?:logs:[a-z0-9\-]+:\d{12}:log-group:\w+"#) | fn.#Fn
					Enabled?:               *bool | fn.#Fn
					LogLevel?:              *("CRITICAL" | "ERROR" | "WARNING" | "INFO" | "DEBUG") | fn.#Fn
				} | fn.#If
				SchedulerLogs?: *{
					CloudWatchLogGroupArn?: *(=~#"^arn:(aws|aws-us-gov|aws-cn|aws-iso|aws-iso-b)(-[a-z]+)?:logs:[a-z0-9\-]+:\d{12}:log-group:\w+"#) | fn.#Fn
					Enabled?:               *bool | fn.#Fn
					LogLevel?:              *("CRITICAL" | "ERROR" | "WARNING" | "INFO" | "DEBUG") | fn.#Fn
				} | fn.#If
				TaskLogs?: *{
					CloudWatchLogGroupArn?: *(=~#"^arn:(aws|aws-us-gov|aws-cn|aws-iso|aws-iso-b)(-[a-z]+)?:logs:[a-z0-9\-]+:\d{12}:log-group:\w+"#) | fn.#Fn
					Enabled?:               *bool | fn.#Fn
					LogLevel?:              *("CRITICAL" | "ERROR" | "WARNING" | "INFO" | "DEBUG") | fn.#Fn
				} | fn.#If
				WebserverLogs?: *{
					CloudWatchLogGroupArn?: *(=~#"^arn:(aws|aws-us-gov|aws-cn|aws-iso|aws-iso-b)(-[a-z]+)?:logs:[a-z0-9\-]+:\d{12}:log-group:\w+"#) | fn.#Fn
					Enabled?:               *bool | fn.#Fn
					LogLevel?:              *("CRITICAL" | "ERROR" | "WARNING" | "INFO" | "DEBUG") | fn.#Fn
				} | fn.#If
				WorkerLogs?: *{
					CloudWatchLogGroupArn?: *(=~#"^arn:(aws|aws-us-gov|aws-cn|aws-iso|aws-iso-b)(-[a-z]+)?:logs:[a-z0-9\-]+:\d{12}:log-group:\w+"#) | fn.#Fn
					Enabled?:               *bool | fn.#Fn
					LogLevel?:              *("CRITICAL" | "ERROR" | "WARNING" | "INFO" | "DEBUG") | fn.#Fn
				} | fn.#If
			} | fn.#If
			MaxWorkers?:           *int | fn.#Fn
			MinWorkers?:           *int | fn.#Fn
			Name:                  *(strings.MinRunes(1) & strings.MaxRunes(80) & (=~#"^[a-zA-Z][0-9a-zA-Z\-_]*$"#)) | fn.#Fn
			NetworkConfiguration?: *{
				SecurityGroupIds?: [...(*(strings.MinRunes(1) & strings.MaxRunes(1024) & (=~#"^sg-[a-zA-Z0-9\-._]+$"#)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(1024) & (=~#"^sg-[a-zA-Z0-9\-._]+$"#)) | fn.#Fn)
				SubnetIds?:        [...(*(=~#"^subnet-[a-zA-Z0-9\-._]+$"#) | fn.#Fn)] | (*(=~#"^subnet-[a-zA-Z0-9\-._]+$"#) | fn.#Fn)
			} | fn.#If
			PluginsS3ObjectVersion?:      *string | fn.#Fn
			PluginsS3Path?:               *(=~#".*"#) | fn.#Fn
			RequirementsS3ObjectVersion?: *string | fn.#Fn
			RequirementsS3Path?:          *(=~#".*"#) | fn.#Fn
			Schedulers?:                  *int | fn.#Fn
			SourceBucketArn?:             *(strings.MinRunes(1) & strings.MaxRunes(1224) & (=~#"^arn:(aws|aws-us-gov|aws-cn|aws-iso|aws-iso-b)(-[a-z]+)?:s3:::[a-z0-9.\-]+$"#)) | fn.#Fn
			Tags?:                        *{
				[string]: _
			} | fn.#Fn
			WebserverAccessMode?:          *("PRIVATE_ONLY" | "PUBLIC_ONLY") | fn.#Fn
			WeeklyMaintenanceWindowStart?: *(=~#"(MON|TUE|WED|THU|FRI|SAT|SUN):([01]\d|2[0-3]):(00|30)"#) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
