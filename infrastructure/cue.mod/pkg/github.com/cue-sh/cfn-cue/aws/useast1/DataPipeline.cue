package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#DataPipeline: {
	#Pipeline: {
		Type: "AWS::DataPipeline::Pipeline"
		Properties: {
			Activate?:        *bool | fn.#Fn
			Description?:     *string | fn.#Fn
			Name:             *string | fn.#Fn
			ParameterObjects: *[...{
				Attributes: *[...{
					Key:         *string | fn.#Fn
					StringValue: *string | fn.#Fn
				}] | fn.#If
				Id: *string | fn.#Fn
			}] | fn.#If
			ParameterValues?: *[...{
				Id:          *string | fn.#Fn
				StringValue: *string | fn.#Fn
			}] | fn.#If
			PipelineObjects?: *[...{
				Fields: *[...{
					Key:          *string | fn.#Fn
					RefValue?:    *string | fn.#Fn
					StringValue?: *string | fn.#Fn
				}] | fn.#If
				Id:   *string | fn.#Fn
				Name: *string | fn.#Fn
			}] | fn.#If
			PipelineTags?: *[...{
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
