package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#S3: {
	#AccessPoint: {
		Type: "AWS::S3::AccessPoint"
		Properties: {
			Bucket:  *(strings.MinRunes(3) & strings.MaxRunes(255)) | fn.#Fn
			Name?:   *(strings.MinRunes(3) & strings.MaxRunes(50) & (=~#"^[a-z0-9]([a-z0-9\-]*[a-z0-9])?$"#)) | fn.#Fn
			Policy?: *{
				[string]: _
			} | fn.#Fn
			PolicyStatus?: *{
				[string]: _
			} | fn.#Fn
			PublicAccessBlockConfiguration?: *{
				BlockPublicAcls?:       *bool | fn.#Fn
				BlockPublicPolicy?:     *bool | fn.#Fn
				IgnorePublicAcls?:      *bool | fn.#Fn
				RestrictPublicBuckets?: *bool | fn.#Fn
			} | fn.#If
			VpcConfiguration?: *{
				VpcId?: *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Bucket: {
		Type: "AWS::S3::Bucket"
		Properties: {
			AccelerateConfiguration?: *{
				AccelerationStatus: *("Enabled" | "Suspended") | fn.#Fn
			} | fn.#If
			AccessControl?:           *("AuthenticatedRead" | "AwsExecRead" | "BucketOwnerFullControl" | "BucketOwnerRead" | "LogDeliveryWrite" | "Private" | "PublicRead" | "PublicReadWrite") | fn.#Fn
			AnalyticsConfigurations?: *[...{
				Id:                   *string | fn.#Fn
				Prefix?:              *string | fn.#Fn
				StorageClassAnalysis: *{
					DataExport?: *{
						Destination: *{
							BucketAccountId?: *string | fn.#Fn
							BucketArn:        *string | fn.#Fn
							Format:           *("CSV" | "ORC" | "Parquet") | fn.#Fn
							Prefix?:          *string | fn.#Fn
						} | fn.#If
						OutputSchemaVersion: *string | fn.#Fn
					} | fn.#If
				} | fn.#If
				TagFilters?: *[...{
					Key:   *string | fn.#Fn
					Value: *string | fn.#Fn
				}] | fn.#If
			}] | fn.#If
			BucketEncryption?: *{
				ServerSideEncryptionConfiguration: *[...{
					BucketKeyEnabled?:              *bool | fn.#Fn
					ServerSideEncryptionByDefault?: *{
						KMSMasterKeyID?: *string | fn.#Fn
						SSEAlgorithm:    *("aws:kms" | "AES256") | fn.#Fn
					} | fn.#If
				}] | fn.#If
			} | fn.#If
			BucketName?:        *(strings.MinRunes(3) & strings.MaxRunes(63) & (=~#"^[a-z0-9][a-z0-9//.//-]*[a-z0-9]$"#)) | fn.#Fn
			CorsConfiguration?: *{
				CorsRules: *[...{
					AllowedHeaders?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					AllowedMethods:  [...(*("GET" | "PUT" | "HEAD" | "POST" | "DELETE") | fn.#Fn)] | (*("GET" | "PUT" | "HEAD" | "POST" | "DELETE") | fn.#Fn)
					AllowedOrigins:  [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					ExposedHeaders?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					Id?:             *string | fn.#Fn
					MaxAge?:         *int | fn.#Fn
				}] | fn.#If
			} | fn.#If
			IntelligentTieringConfigurations?: *[...{
				Id:          *string | fn.#Fn
				Prefix?:     *string | fn.#Fn
				Status:      *("Disabled" | "Enabled") | fn.#Fn
				TagFilters?: *[...{
					Key:   *string | fn.#Fn
					Value: *string | fn.#Fn
				}] | fn.#If
				Tierings: *[...{
					AccessTier: *("ARCHIVE_ACCESS" | "DEEP_ARCHIVE_ACCESS") | fn.#Fn
					Days:       *int | fn.#Fn
				}] | fn.#If
			}] | fn.#If
			InventoryConfigurations?: *[...{
				Destination: *{
					BucketAccountId?: *string | fn.#Fn
					BucketArn:        *string | fn.#Fn
					Format:           *("CSV" | "ORC" | "Parquet") | fn.#Fn
					Prefix?:          *string | fn.#Fn
				} | fn.#If
				Enabled:                *bool | fn.#Fn
				Id:                     *string | fn.#Fn
				IncludedObjectVersions: *("All" | "Current") | fn.#Fn
				OptionalFields?:        [...(*("Size" | "LastModifiedDate" | "StorageClass" | "ETag" | "IsMultipartUploaded" | "ReplicationStatus" | "EncryptionStatus" | "ObjectLockRetainUntilDate" | "ObjectLockMode" | "ObjectLockLegalHoldStatus" | "IntelligentTieringAccessTier" | "BucketKeyStatus") | fn.#Fn)] | (*("Size" | "LastModifiedDate" | "StorageClass" | "ETag" | "IsMultipartUploaded" | "ReplicationStatus" | "EncryptionStatus" | "ObjectLockRetainUntilDate" | "ObjectLockMode" | "ObjectLockLegalHoldStatus" | "IntelligentTieringAccessTier" | "BucketKeyStatus") | fn.#Fn)
				Prefix?:                *string | fn.#Fn
				ScheduleFrequency:      *("Daily" | "Weekly") | fn.#Fn
			}] | fn.#If
			LifecycleConfiguration?: *{
				Rules: *[...{
					AbortIncompleteMultipartUpload?: *{
						DaysAfterInitiation: *int | fn.#Fn
					} | fn.#If
					ExpirationDate?:              *(=~#"^([0-2]\d{3})-(0[0-9]|1[0-2])-([0-2]\d|3[01])T([01]\d|2[0-4]):([0-5]\d):([0-6]\d)((\.\d{3})?)Z$"#) | fn.#Fn
					ExpirationInDays?:            *int | fn.#Fn
					ExpiredObjectDeleteMarker?:   *bool | fn.#Fn
					Id?:                          *string | fn.#Fn
					NoncurrentVersionExpiration?: *{
						NewerNoncurrentVersions?: *int | fn.#Fn
						NoncurrentDays:           *int | fn.#Fn
					} | fn.#If
					NoncurrentVersionExpirationInDays?: *int | fn.#Fn
					NoncurrentVersionTransition?:       *{
						NewerNoncurrentVersions?: *int | fn.#Fn
						StorageClass:             *("DEEP_ARCHIVE" | "GLACIER" | "Glacier" | "INTELLIGENT_TIERING" | "ONEZONE_IA" | "STANDARD_IA") | fn.#Fn
						TransitionInDays:         *int | fn.#Fn
					} | fn.#If
					NoncurrentVersionTransitions?: *[...{
						NewerNoncurrentVersions?: *int | fn.#Fn
						StorageClass:             *("DEEP_ARCHIVE" | "GLACIER" | "Glacier" | "INTELLIGENT_TIERING" | "ONEZONE_IA" | "STANDARD_IA") | fn.#Fn
						TransitionInDays:         *int | fn.#Fn
					}] | fn.#If
					ObjectSizeGreaterThan?: *int | fn.#Fn
					ObjectSizeLessThan?:    *int | fn.#Fn
					Prefix?:                *string | fn.#Fn
					Status:                 *("Enabled" | "Disabled") | fn.#Fn
					TagFilters?:            *[...{
						Key:   *string | fn.#Fn
						Value: *string | fn.#Fn
					}] | fn.#If
					Transition?: *{
						StorageClass:      *("DEEP_ARCHIVE" | "GLACIER" | "Glacier" | "INTELLIGENT_TIERING" | "ONEZONE_IA" | "STANDARD_IA") | fn.#Fn
						TransitionDate?:   *(=~#"^([0-2]\d{3})-(0[0-9]|1[0-2])-([0-2]\d|3[01])T([01]\d|2[0-4]):([0-5]\d):([0-6]\d)((\.\d{3})?)Z$"#) | fn.#Fn
						TransitionInDays?: *int | fn.#Fn
					} | fn.#If
					Transitions?: *[...{
						StorageClass:      *("DEEP_ARCHIVE" | "GLACIER" | "Glacier" | "INTELLIGENT_TIERING" | "ONEZONE_IA" | "STANDARD_IA") | fn.#Fn
						TransitionDate?:   *(=~#"^([0-2]\d{3})-(0[0-9]|1[0-2])-([0-2]\d|3[01])T([01]\d|2[0-4]):([0-5]\d):([0-6]\d)((\.\d{3})?)Z$"#) | fn.#Fn
						TransitionInDays?: *int | fn.#Fn
					}] | fn.#If
				}] | fn.#If
			} | fn.#If
			LoggingConfiguration?: *{
				DestinationBucketName?: *string | fn.#Fn
				LogFilePrefix?:         *string | fn.#Fn
			} | fn.#If
			MetricsConfigurations?: *[...{
				AccessPointArn?: *string | fn.#Fn
				Id:              *string | fn.#Fn
				Prefix?:         *string | fn.#Fn
				TagFilters?:     *[...{
					Key:   *string | fn.#Fn
					Value: *string | fn.#Fn
				}] | fn.#If
			}] | fn.#If
			NotificationConfiguration?: *{
				EventBridgeConfiguration?: *{
					EventBridgeEnabled?: *bool | fn.#Fn
				} | fn.#If
				LambdaConfigurations?: *[...{
					Event:   *string | fn.#Fn
					Filter?: *{
						S3Key: *{
							Rules: *[...{
								Name:  *string | fn.#Fn
								Value: *string | fn.#Fn
							}] | fn.#If
						} | fn.#If
					} | fn.#If
					Function: *string | fn.#Fn
				}] | fn.#If
				QueueConfigurations?: *[...{
					Event:   *string | fn.#Fn
					Filter?: *{
						S3Key: *{
							Rules: *[...{
								Name:  *string | fn.#Fn
								Value: *string | fn.#Fn
							}] | fn.#If
						} | fn.#If
					} | fn.#If
					Queue: *string | fn.#Fn
				}] | fn.#If
				TopicConfigurations?: *[...{
					Event:   *string | fn.#Fn
					Filter?: *{
						S3Key: *{
							Rules: *[...{
								Name:  *string | fn.#Fn
								Value: *string | fn.#Fn
							}] | fn.#If
						} | fn.#If
					} | fn.#If
					Topic: *string | fn.#Fn
				}] | fn.#If
			} | fn.#If
			ObjectLockConfiguration?: *{
				ObjectLockEnabled?: *string | fn.#Fn
				Rule?:              *{
					DefaultRetention?: *{
						Days?:  *int | fn.#Fn
						Mode?:  *("COMPLIANCE" | "GOVERNANCE") | fn.#Fn
						Years?: *int | fn.#Fn
					} | fn.#If
				} | fn.#If
			} | fn.#If
			ObjectLockEnabled?: *bool | fn.#Fn
			OwnershipControls?: *{
				Rules: *[...{
					ObjectOwnership?: *("ObjectWriter" | "BucketOwnerPreferred") | fn.#Fn
				}] | fn.#If
			} | fn.#If
			PublicAccessBlockConfiguration?: *{
				BlockPublicAcls?:       *bool | fn.#Fn
				BlockPublicPolicy?:     *bool | fn.#Fn
				IgnorePublicAcls?:      *bool | fn.#Fn
				RestrictPublicBuckets?: *bool | fn.#Fn
			} | fn.#If
			ReplicationConfiguration?: *{
				Role:  *(=~#"arn:(aws[a-zA-Z-]*)?:iam::\d{12}:role/[a-zA-Z_0-9+=,.@\-_/]+"#) | fn.#Fn
				Rules: *[...{
					DeleteMarkerReplication?: *{
						Status?: *("Disabled" | "Enabled") | fn.#Fn
					} | fn.#If
					Destination: *{
						AccessControlTranslation?: *{
							Owner: *string | fn.#Fn
						} | fn.#If
						Account?:                 *string | fn.#Fn
						Bucket:                   *string | fn.#Fn
						EncryptionConfiguration?: *{
							ReplicaKmsKeyID: *string | fn.#Fn
						} | fn.#If
						Metrics?: *{
							EventThreshold?: *{
								Minutes: *int | fn.#Fn
							} | fn.#If
							Status: *("Disabled" | "Enabled") | fn.#Fn
						} | fn.#If
						ReplicationTime?: *{
							Status: *("Disabled" | "Enabled") | fn.#Fn
							Time:   *{
								Minutes: *int | fn.#Fn
							} | fn.#If
						} | fn.#If
						StorageClass?: *("DEEP_ARCHIVE" | "GLACIER" | "INTELLIGENT_TIERING" | "ONEZONE_IA" | "REDUCED_REDUNDANCY" | "STANDARD" | "STANDARD_IA") | fn.#Fn
					} | fn.#If
					Filter?: *{
						And?: *{
							Prefix?:     *string | fn.#Fn
							TagFilters?: *[...{
								Key:   *string | fn.#Fn
								Value: *string | fn.#Fn
							}] | fn.#If
						} | fn.#If
						Prefix?:    *string | fn.#Fn
						TagFilter?: *{
							Key:   *string | fn.#Fn
							Value: *string | fn.#Fn
						} | fn.#If
					} | fn.#If
					Id?:                      *string | fn.#Fn
					Prefix?:                  *string | fn.#Fn
					Priority?:                *int | fn.#Fn
					SourceSelectionCriteria?: *{
						ReplicaModifications?: *{
							Status: *("Enabled" | "Disabled") | fn.#Fn
						} | fn.#If
						SseKmsEncryptedObjects?: *{
							Status: *("Disabled" | "Enabled") | fn.#Fn
						} | fn.#If
					} | fn.#If
					Status: *("Disabled" | "Enabled") | fn.#Fn
				}] | fn.#If
			} | fn.#If
			Tags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			VersioningConfiguration?: *{
				Status: *("Enabled" | "Suspended") | fn.#Fn
			} | fn.#If
			WebsiteConfiguration?: *{
				ErrorDocument?:         *string | fn.#Fn
				IndexDocument?:         *string | fn.#Fn
				RedirectAllRequestsTo?: *{
					HostName:  *string | fn.#Fn
					Protocol?: *("http" | "https") | fn.#Fn
				} | fn.#If
				RoutingRules?: *[...{
					RedirectRule: *{
						HostName?:             *string | fn.#Fn
						HttpRedirectCode?:     *string | fn.#Fn
						Protocol?:             *("http" | "https") | fn.#Fn
						ReplaceKeyPrefixWith?: *string | fn.#Fn
						ReplaceKeyWith?:       *string | fn.#Fn
					} | fn.#If
					RoutingRuleCondition?: *{
						HttpErrorCodeReturnedEquals?: *string | fn.#Fn
						KeyPrefixEquals?:             *string | fn.#Fn
					} | fn.#If
				}] | fn.#If
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#BucketPolicy: {
		Type: "AWS::S3::BucketPolicy"
		Properties: {
			Bucket:         *string | fn.#Fn
			PolicyDocument: *{
				{
					[string]: _
				}
				Version: string | *"2012-10-17"
			} | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#MultiRegionAccessPoint: {
		Type: "AWS::S3::MultiRegionAccessPoint"
		Properties: {
			Name?:                           *(strings.MinRunes(3) & strings.MaxRunes(50) & (=~#"^[a-z0-9][-a-z0-9]{1,48}[a-z0-9]$"#)) | fn.#Fn
			PublicAccessBlockConfiguration?: *{
				BlockPublicAcls?:       *bool | fn.#Fn
				BlockPublicPolicy?:     *bool | fn.#Fn
				IgnorePublicAcls?:      *bool | fn.#Fn
				RestrictPublicBuckets?: *bool | fn.#Fn
			} | fn.#If
			Regions: *[...{
				Bucket: *(strings.MinRunes(3) & strings.MaxRunes(63) & (=~#"^[a-z0-9][a-z0-9//.//-]*[a-z0-9]$"#)) | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#MultiRegionAccessPointPolicy: {
		Type: "AWS::S3::MultiRegionAccessPointPolicy"
		Properties: {
			MrapName: *(strings.MinRunes(3) & strings.MaxRunes(50) & (=~#"^[a-z0-9][-a-z0-9]{1,48}[a-z0-9]$"#)) | fn.#Fn
			Policy:   *{
				[string]: _
			} | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#StorageLens: {
		Type: "AWS::S3::StorageLens"
		Properties: {
			StorageLensConfiguration: *{
				AccountLevel: *{
					ActivityMetrics?: *{
						IsEnabled?: *bool | fn.#Fn
					} | fn.#If
					BucketLevel: *{
						ActivityMetrics?: *{
							IsEnabled?: *bool | fn.#Fn
						} | fn.#If
						PrefixLevel?: *{
							StorageMetrics: *{
								IsEnabled?:         *bool | fn.#Fn
								SelectionCriteria?: *{
									Delimiter?:                 *string | fn.#Fn
									MaxDepth?:                  *int | fn.#Fn
									MinStorageBytesPercentage?: *number | fn.#Fn
								} | fn.#If
							} | fn.#If
						} | fn.#If
					} | fn.#If
				} | fn.#If
				AwsOrg?: *{
					Arn: *string | fn.#Fn
				} | fn.#If
				DataExport?: *{
					CloudWatchMetrics?: *{
						IsEnabled: *bool | fn.#Fn
					} | fn.#If
					S3BucketDestination?: *{
						AccountId:           *string | fn.#Fn
						Arn:                 *string | fn.#Fn
						Encryption?:         *{} | fn.#If
						Format:              *("CSV" | "Parquet") | fn.#Fn
						OutputSchemaVersion: *("V_1") | fn.#Fn
						Prefix?:             *string | fn.#Fn
					} | fn.#If
				} | fn.#If
				Exclude?: *{
					Buckets?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					Regions?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				} | fn.#If
				Id:       *(strings.MinRunes(1) & strings.MaxRunes(64) & (=~#"^[a-zA-Z0-9\-_.]+$"#)) | fn.#Fn
				Include?: *{
					Buckets?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					Regions?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				} | fn.#If
				IsEnabled:       *bool | fn.#Fn
				StorageLensArn?: *string | fn.#Fn
			} | fn.#If
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
