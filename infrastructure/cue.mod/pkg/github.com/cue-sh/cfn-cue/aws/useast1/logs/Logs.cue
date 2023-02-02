package logs

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#Destination: {
	Type: "AWS::Logs::Destination"
	Properties: {
		DestinationName:   *string | fn.#Fn
		DestinationPolicy: *string | fn.#Fn
		RoleArn:           *string | fn.#Fn
		TargetArn:         *string | fn.#Fn
	}
	DependsOn?:           string | [...string]
	DeletionPolicy?:      "Delete" | "Retain"
	UpdateReplacePolicy?: "Delete" | "Retain"
	Metadata?: [string]: _
	Condition?: string
}
#LogGroup: {
	Type: "AWS::Logs::LogGroup"
	Properties: {
		KmsKeyId?:        *(=~#"^arn:[a-z0-9-]+:kms:[a-z0-9-]+:\d{12}:(key|alias)/.+\Z"#) | fn.#Fn
		LogGroupName?:    *(strings.MinRunes(1) & strings.MaxRunes(512) & (=~#"^[.\-_/#A-Za-z0-9]{1,512}"#)) | fn.#Fn
		RetentionInDays?: *(1 | 3 | 5 | 7 | 14 | 30 | 60 | 90 | 120 | 150 | 180 | 365 | 400 | 545 | 731 | 1827 | 3653) | fn.#Fn
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
#LogStream: {
	Type: "AWS::Logs::LogStream"
	Properties: {
		LogGroupName:   *string | fn.#Fn
		LogStreamName?: *string | fn.#Fn
	}
	DependsOn?:           string | [...string]
	DeletionPolicy?:      "Delete" | "Retain"
	UpdateReplacePolicy?: "Delete" | "Retain"
	Metadata?: [string]: _
	Condition?: string
}
#MetricFilter: {
	Type: "AWS::Logs::MetricFilter"
	Properties: {
		FilterPattern:         *string | fn.#Fn
		LogGroupName:          *string | fn.#Fn
		MetricTransformations: *[...{
			DefaultValue?:   *number | fn.#Fn
			MetricName:      *string | fn.#Fn
			MetricNamespace: *string | fn.#Fn
			MetricValue:     *(=~#"^(([0-9]*)|(\$.*))$"#) | fn.#Fn
		}] | fn.#If
	}
	DependsOn?:           string | [...string]
	DeletionPolicy?:      "Delete" | "Retain"
	UpdateReplacePolicy?: "Delete" | "Retain"
	Metadata?: [string]: _
	Condition?: string
}
#QueryDefinition: {
	Type: "AWS::Logs::QueryDefinition"
	Properties: {
		LogGroupNames?: [...(*(strings.MinRunes(1) & strings.MaxRunes(512) & (=~#"[\.\-_/#A-Za-z0-9]+"#)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(512) & (=~#"[\.\-_/#A-Za-z0-9]+"#)) | fn.#Fn)
		Name:           *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"^([^:*\/]+\/?)*[^:*\/]+$"#)) | fn.#Fn
		QueryString:    *(strings.MinRunes(1) & strings.MaxRunes(10000)) | fn.#Fn
	}
	DependsOn?:           string | [...string]
	DeletionPolicy?:      "Delete" | "Retain"
	UpdateReplacePolicy?: "Delete" | "Retain"
	Metadata?: [string]: _
	Condition?: string
}
#ResourcePolicy: {
	Type: "AWS::Logs::ResourcePolicy"
	Properties: {
		PolicyDocument: *(strings.MinRunes(1) & strings.MaxRunes(5120) & (=~#"[\u0009\u000A\u000D\u0020-\u00FF]+"#)) | fn.#Fn
		PolicyName:     *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"^([^:*\/]+\/?)*[^:*\/]+$"#)) | fn.#Fn
	}
	DependsOn?:           string | [...string]
	DeletionPolicy?:      "Delete" | "Retain"
	UpdateReplacePolicy?: "Delete" | "Retain"
	Metadata?: [string]: _
	Condition?: string
}
#SubscriptionFilter: {
	Type: "AWS::Logs::SubscriptionFilter"
	Properties: {
		DestinationArn: *string | fn.#Fn
		FilterPattern:  *string | fn.#Fn
		LogGroupName:   *string | fn.#Fn
		RoleArn?:       *string | fn.#Fn
	}
	DependsOn?:           string | [...string]
	DeletionPolicy?:      "Delete" | "Retain"
	UpdateReplacePolicy?: "Delete" | "Retain"
	Metadata?: [string]: _
	Condition?: string
}
