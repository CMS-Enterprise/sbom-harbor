package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#Macie: {
	#CustomDataIdentifier: {
		Type: "AWS::Macie::CustomDataIdentifier"
		Properties: {
			Description?:          *string | fn.#Fn
			IgnoreWords?:          [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			Keywords?:             [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			MaximumMatchDistance?: *int | fn.#Fn
			Name:                  *string | fn.#Fn
			Regex:                 *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#FindingsFilter: {
		Type: "AWS::Macie::FindingsFilter"
		Properties: {
			Action?:         *("ARCHIVE" | "NOOP") | fn.#Fn
			Description?:    *string | fn.#Fn
			FindingCriteria: *{
				Criterion?: *{} | fn.#If
			} | fn.#If
			Name:      *string | fn.#Fn
			Position?: *int | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Session: {
		Type: "AWS::Macie::Session"
		Properties: {
			FindingPublishingFrequency?: *("FIFTEEN_MINUTES" | "ONE_HOUR" | "SIX_HOURS") | fn.#Fn
			Status?:                     *("ENABLED" | "PAUSED") | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
