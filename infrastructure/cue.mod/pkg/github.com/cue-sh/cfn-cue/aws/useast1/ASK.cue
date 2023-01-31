package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#ASK: {
	#Skill: {
		Type: "Alexa::ASK::Skill"
		Properties: {
			AuthenticationConfiguration: *{
				ClientId:     *string | fn.#Fn
				ClientSecret: *string | fn.#Fn
				RefreshToken: *string | fn.#Fn
			} | fn.#If
			SkillPackage: *{
				Overrides?: *{
					Manifest?: *{
						[string]: _
					} | fn.#Fn
				} | fn.#If
				S3Bucket:         *string | fn.#Fn
				S3BucketRole?:    *string | fn.#Fn
				S3Key:            *string | fn.#Fn
				S3ObjectVersion?: *string | fn.#Fn
			} | fn.#If
			VendorId: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
