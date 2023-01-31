package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#DLM: {
	#LifecyclePolicy: {
		Type: "AWS::DLM::LifecyclePolicy"
		Properties: {
			Description?:     *string | fn.#Fn
			ExecutionRoleArn: *string | fn.#Fn
			PolicyDetails?:   *{
				Actions?: *[...{
					CrossRegionCopy: *[...{
						EncryptionConfiguration: *{
							CmkArn?:   *string | fn.#Fn
							Encrypted: *bool | fn.#Fn
						} | fn.#If
						RetainRule?: *{
							Interval:     *int | fn.#Fn
							IntervalUnit: *string | fn.#Fn
						} | fn.#If
						Target: *string | fn.#Fn
					}] | fn.#If
					Name: *string | fn.#Fn
				}] | fn.#If
				EventSource?: *{
					Parameters?: *{
						DescriptionRegex?: *string | fn.#Fn
						EventType:         *string | fn.#Fn
						SnapshotOwner:     [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					} | fn.#If
					Type: *string | fn.#Fn
				} | fn.#If
				Parameters?: *{
					ExcludeBootVolume?: *bool | fn.#Fn
					NoReboot?:          *bool | fn.#Fn
				} | fn.#If
				PolicyType?:        *string | fn.#Fn
				ResourceLocations?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				ResourceTypes?:     [...(*("INSTANCE" | "VOLUME") | fn.#Fn)] | (*("INSTANCE" | "VOLUME") | fn.#Fn)
				Schedules?:         *[...{
					CopyTags?:   *bool | fn.#Fn
					CreateRule?: *{
						CronExpression?: *string | fn.#Fn
						Interval?:       *int | fn.#Fn
						IntervalUnit?:   *string | fn.#Fn
						Location?:       *string | fn.#Fn
						Times?:          [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					} | fn.#If
					CrossRegionCopyRules?: *[...{
						CmkArn?:        *string | fn.#Fn
						CopyTags?:      *bool | fn.#Fn
						DeprecateRule?: *{
							Interval:     *int | fn.#Fn
							IntervalUnit: *string | fn.#Fn
						} | fn.#If
						Encrypted:   *bool | fn.#Fn
						RetainRule?: *{
							Interval:     *int | fn.#Fn
							IntervalUnit: *string | fn.#Fn
						} | fn.#If
						Target?:       *string | fn.#Fn
						TargetRegion?: *string | fn.#Fn
					}] | fn.#If
					DeprecateRule?: *{
						Count?:        *int | fn.#Fn
						Interval?:     *int | fn.#Fn
						IntervalUnit?: *string | fn.#Fn
					} | fn.#If
					FastRestoreRule?: *{
						AvailabilityZones?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
						Count?:             *int | fn.#Fn
						Interval?:          *int | fn.#Fn
						IntervalUnit?:      *string | fn.#Fn
					} | fn.#If
					Name?:       *string | fn.#Fn
					RetainRule?: *{
						Count?:        *int | fn.#Fn
						Interval?:     *int | fn.#Fn
						IntervalUnit?: *string | fn.#Fn
					} | fn.#If
					ShareRules?: *[...{
						TargetAccounts?:      [...(*string | fn.#Fn)] | (*string | fn.#Fn)
						UnshareInterval?:     *int | fn.#Fn
						UnshareIntervalUnit?: *string | fn.#Fn
					}] | fn.#If
					TagsToAdd?: *[...{
						Key:   *string | fn.#Fn
						Value: *string | fn.#Fn
					}] | fn.#If
					VariableTags?: *[...{
						Key:   *string | fn.#Fn
						Value: *string | fn.#Fn
					}] | fn.#If
				}] | fn.#If
				TargetTags?: *[...{
					Key:   *string | fn.#Fn
					Value: *string | fn.#Fn
				}] | fn.#If
			} | fn.#If
			State: *("DISABLED" | "ENABLED") | fn.#Fn
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
