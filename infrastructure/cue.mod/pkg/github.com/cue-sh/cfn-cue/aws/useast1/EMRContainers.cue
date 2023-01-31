package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#EMRContainers: {
	#VirtualCluster: {
		Type: "AWS::EMRContainers::VirtualCluster"
		Properties: {
			ContainerProvider: *{
				Id:   *(strings.MinRunes(1) & strings.MaxRunes(100) & (=~#"^[0-9A-Za-z][A-Za-z0-9\-_]*"#)) | fn.#Fn
				Info: *{
					EksInfo: *{
						Namespace: *(strings.MinRunes(1) & strings.MaxRunes(63) & (=~#"[a-z0-9]([-a-z0-9]*[a-z0-9])?"#)) | fn.#Fn
					} | fn.#If
				} | fn.#If
				Type: *string | fn.#Fn
			} | fn.#If
			Name:  *(strings.MinRunes(1) & strings.MaxRunes(64) & (=~#"[\.\-_/#A-Za-z0-9]+"#)) | fn.#Fn
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
}
