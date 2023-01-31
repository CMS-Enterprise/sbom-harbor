package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#CodeGuruProfiler: {
	#ProfilingGroup: {
		Type: "AWS::CodeGuruProfiler::ProfilingGroup"
		Properties: {
			AgentPermissions?: *{
				[string]: _
			} | fn.#Fn
			AnomalyDetectionNotificationConfiguration?: *[...{
				channelId?: *(=~#"[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}"#) | fn.#Fn
				channelUri: *(=~#"^arn:aws([-\w]*):[a-z-]+:(([a-z]+-)+[0-9]+)?:([0-9]{12}):[^.]+$"#) | fn.#Fn
			}] | fn.#If
			ComputePlatform?:   *("Default" | "AWSLambda") | fn.#Fn
			ProfilingGroupName: *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"^[\w-]+$"#)) | fn.#Fn
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
}
