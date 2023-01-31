package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#AccessAnalyzer: {
	#Analyzer: {
		Type: "AWS::AccessAnalyzer::Analyzer"
		Properties: {
			AnalyzerName?: *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
			ArchiveRules?: *[...{
				Filter: *[...{
					Contains?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					Eq?:       [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					Exists?:   *bool | fn.#Fn
					Neq?:      [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					Property:  *string | fn.#Fn
				}] | fn.#If
				RuleName: *string | fn.#Fn
			}] | fn.#If
			Tags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			Type: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
