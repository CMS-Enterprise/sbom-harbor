package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#DynamoDB: {
	#GlobalTable: {
		Type: "AWS::DynamoDB::GlobalTable"
		Properties: {
			AttributeDefinitions: *[...{
				AttributeName: *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
				AttributeType: *string | fn.#Fn
			}] | fn.#If
			BillingMode?:            *string | fn.#Fn
			GlobalSecondaryIndexes?: *[...{
				IndexName: *(strings.MinRunes(3) & strings.MaxRunes(255)) | fn.#Fn
				KeySchema: *[...{
					AttributeName: *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
					KeyType:       *string | fn.#Fn
				}] | fn.#If
				Projection: *{
					NonKeyAttributes?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					ProjectionType?:   *string | fn.#Fn
				} | fn.#If
				WriteProvisionedThroughputSettings?: *{
					WriteCapacityAutoScalingSettings?: *{
						MaxCapacity:                              *int | fn.#Fn
						MinCapacity:                              *int | fn.#Fn
						SeedCapacity?:                            *int | fn.#Fn
						TargetTrackingScalingPolicyConfiguration: *{
							DisableScaleIn?:   *bool | fn.#Fn
							ScaleInCooldown?:  *int | fn.#Fn
							ScaleOutCooldown?: *int | fn.#Fn
							TargetValue:       *number | fn.#Fn
						} | fn.#If
					} | fn.#If
				} | fn.#If
			}] | fn.#If
			KeySchema: *[...{
				AttributeName: *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
				KeyType:       *string | fn.#Fn
			}] | fn.#If
			LocalSecondaryIndexes?: *[...{
				IndexName: *(strings.MinRunes(3) & strings.MaxRunes(255)) | fn.#Fn
				KeySchema: *[...{
					AttributeName: *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
					KeyType:       *string | fn.#Fn
				}] | fn.#If
				Projection: *{
					NonKeyAttributes?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					ProjectionType?:   *string | fn.#Fn
				} | fn.#If
			}] | fn.#If
			Replicas: *[...{
				ContributorInsightsSpecification?: *{
					Enabled: *bool | fn.#Fn
				} | fn.#If
				GlobalSecondaryIndexes?: *[...{
					ContributorInsightsSpecification?: *{
						Enabled: *bool | fn.#Fn
					} | fn.#If
					IndexName:                          *(strings.MinRunes(3) & strings.MaxRunes(255)) | fn.#Fn
					ReadProvisionedThroughputSettings?: *{
						ReadCapacityAutoScalingSettings?: *{
							MaxCapacity:                              *int | fn.#Fn
							MinCapacity:                              *int | fn.#Fn
							SeedCapacity?:                            *int | fn.#Fn
							TargetTrackingScalingPolicyConfiguration: *{
								DisableScaleIn?:   *bool | fn.#Fn
								ScaleInCooldown?:  *int | fn.#Fn
								ScaleOutCooldown?: *int | fn.#Fn
								TargetValue:       *number | fn.#Fn
							} | fn.#If
						} | fn.#If
						ReadCapacityUnits?: *int | fn.#Fn
					} | fn.#If
				}] | fn.#If
				PointInTimeRecoverySpecification?: *{
					PointInTimeRecoveryEnabled?: *bool | fn.#Fn
				} | fn.#If
				ReadProvisionedThroughputSettings?: *{
					ReadCapacityAutoScalingSettings?: *{
						MaxCapacity:                              *int | fn.#Fn
						MinCapacity:                              *int | fn.#Fn
						SeedCapacity?:                            *int | fn.#Fn
						TargetTrackingScalingPolicyConfiguration: *{
							DisableScaleIn?:   *bool | fn.#Fn
							ScaleInCooldown?:  *int | fn.#Fn
							ScaleOutCooldown?: *int | fn.#Fn
							TargetValue:       *number | fn.#Fn
						} | fn.#If
					} | fn.#If
					ReadCapacityUnits?: *int | fn.#Fn
				} | fn.#If
				Region:            *string | fn.#Fn
				SSESpecification?: *{
					KMSMasterKeyId: *string | fn.#Fn
				} | fn.#If
				Tags?: *[...{
					Key:   *string | fn.#Fn
					Value: *string | fn.#Fn
				}] | fn.#If
			}] | fn.#If
			SSESpecification?: *{
				SSEEnabled: *bool | fn.#Fn
				SSEType?:   *string | fn.#Fn
			} | fn.#If
			StreamSpecification?: *{
				StreamViewType: *string | fn.#Fn
			} | fn.#If
			TableName?:               *string | fn.#Fn
			TimeToLiveSpecification?: *{
				AttributeName?: *string | fn.#Fn
				Enabled:        *bool | fn.#Fn
			} | fn.#If
			WriteProvisionedThroughputSettings?: *{
				WriteCapacityAutoScalingSettings?: *{
					MaxCapacity:                              *int | fn.#Fn
					MinCapacity:                              *int | fn.#Fn
					SeedCapacity?:                            *int | fn.#Fn
					TargetTrackingScalingPolicyConfiguration: *{
						DisableScaleIn?:   *bool | fn.#Fn
						ScaleInCooldown?:  *int | fn.#Fn
						ScaleOutCooldown?: *int | fn.#Fn
						TargetValue:       *number | fn.#Fn
					} | fn.#If
				} | fn.#If
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Table: {
		Type: "AWS::DynamoDB::Table"
		Properties: {
			AttributeDefinitions: *[...{
				AttributeName: *string | fn.#Fn
				AttributeType: *("B" | "N" | "S") | fn.#Fn
			}] | fn.#If
			BillingMode?:                      *("PAY_PER_REQUEST" | "PROVISIONED") | fn.#Fn
			ContributorInsightsSpecification?: *{
				Enabled: *bool | fn.#Fn
			} | fn.#If
			GlobalSecondaryIndexes?: *[...{
				ContributorInsightsSpecification?: *{
					Enabled: *bool | fn.#Fn
				} | fn.#If
				IndexName: *string | fn.#Fn
				KeySchema: *[...{
					AttributeName: *string | fn.#Fn
					KeyType:       *("HASH" | "RANGE") | fn.#Fn
				}] | fn.#If
				Projection: *{
					NonKeyAttributes?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					ProjectionType?:   *("ALL" | "INCLUDE" | "KEYS_ONLY") | fn.#Fn
				} | fn.#If
				ProvisionedThroughput?: *{
					ReadCapacityUnits:  *int | fn.#Fn
					WriteCapacityUnits: *int | fn.#Fn
				} | fn.#If
			}] | fn.#If
			KeySchema: *[...{
				AttributeName: *string | fn.#Fn
				KeyType:       *("HASH" | "RANGE") | fn.#Fn
			}] | fn.#If
			KinesisStreamSpecification?: *{
				StreamArn: *string | fn.#Fn
			} | fn.#If
			LocalSecondaryIndexes?: *[...{
				IndexName: *string | fn.#Fn
				KeySchema: *[...{
					AttributeName: *string | fn.#Fn
					KeyType:       *("HASH" | "RANGE") | fn.#Fn
				}] | fn.#If
				Projection: *{
					NonKeyAttributes?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					ProjectionType?:   *("ALL" | "INCLUDE" | "KEYS_ONLY") | fn.#Fn
				} | fn.#If
			}] | fn.#If
			PointInTimeRecoverySpecification?: *{
				PointInTimeRecoveryEnabled?: *bool | fn.#Fn
			} | fn.#If
			ProvisionedThroughput?: *{
				ReadCapacityUnits:  *int | fn.#Fn
				WriteCapacityUnits: *int | fn.#Fn
			} | fn.#If
			SSESpecification?: *{
				KMSMasterKeyId?: *string | fn.#Fn
				SSEEnabled:      *bool | fn.#Fn
				SSEType?:        *string | fn.#Fn
			} | fn.#If
			StreamSpecification?: *{
				StreamViewType: *("KEYS_ONLY" | "NEW_AND_OLD_IMAGES" | "NEW_IMAGE" | "OLD_IMAGE") | fn.#Fn
			} | fn.#If
			TableClass?: *string | fn.#Fn
			TableName?:  *string | fn.#Fn
			Tags?:       *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			TimeToLiveSpecification?: *{
				AttributeName: *string | fn.#Fn
				Enabled:       *bool | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
