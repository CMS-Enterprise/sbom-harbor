package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#IVS: {
	#Channel: {
		Type: "AWS::IVS::Channel"
		Properties: {
			Authorized?:                *bool | fn.#Fn
			LatencyMode?:               *("NORMAL" | "LOW") | fn.#Fn
			Name?:                      *(=~#"^[a-zA-Z0-9-_]*$"#) | fn.#Fn
			RecordingConfigurationArn?: *(=~#"^$|arn:aws:ivs:[a-z0-9-]+:[0-9]+:recording-configuration/[a-zA-Z0-9-]+$"#) | fn.#Fn
			Tags?:                      *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			Type?: *("STANDARD" | "BASIC") | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#PlaybackKeyPair: {
		Type: "AWS::IVS::PlaybackKeyPair"
		Properties: {
			Name?:             *(=~#"^[a-zA-Z0-9-_]*$"#) | fn.#Fn
			PublicKeyMaterial: *string | fn.#Fn
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
	#RecordingConfiguration: {
		Type: "AWS::IVS::RecordingConfiguration"
		Properties: {
			DestinationConfiguration: *{
				S3: *{
					BucketName: *(strings.MinRunes(3) & strings.MaxRunes(63) & (=~#"^[a-z0-9-.]+$"#)) | fn.#Fn
				} | fn.#If
			} | fn.#If
			Name?: *(=~#"^[a-zA-Z0-9-_]*$"#) | fn.#Fn
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
	#StreamKey: {
		Type: "AWS::IVS::StreamKey"
		Properties: {
			ChannelArn: *(=~#"^arn:aws:ivs:[a-z0-9-]+:[0-9]+:channel/[a-zA-Z0-9-]+$"#) | fn.#Fn
			Tags?:      *[...{
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
