package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#MediaPackage: {
	#Asset: {
		Type: "AWS::MediaPackage::Asset"
		Properties: {
			Id:               *string | fn.#Fn
			PackagingGroupId: *string | fn.#Fn
			ResourceId?:      *string | fn.#Fn
			SourceArn:        *string | fn.#Fn
			SourceRoleArn:    *string | fn.#Fn
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
	#Channel: {
		Type: "AWS::MediaPackage::Channel"
		Properties: {
			Description?:      *string | fn.#Fn
			EgressAccessLogs?: *{
				LogGroupName?: *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"\A^(\/aws\/MediaPackage\/)[a-zA-Z0-9_-]+\Z"#)) | fn.#Fn
			} | fn.#If
			Id:                 *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"\A[0-9a-zA-Z-_]+\Z"#)) | fn.#Fn
			IngressAccessLogs?: *{
				LogGroupName?: *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"\A^(\/aws\/MediaPackage\/)[a-zA-Z0-9_-]+\Z"#)) | fn.#Fn
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
	#OriginEndpoint: {
		Type: "AWS::MediaPackage::OriginEndpoint"
		Properties: {
			Authorization?: *{
				CdnIdentifierSecret: *string | fn.#Fn
				SecretsRoleArn:      *string | fn.#Fn
			} | fn.#If
			ChannelId:    *string | fn.#Fn
			CmafPackage?: *{
				Encryption?: *{
					ConstantInitializationVector?: *(strings.MinRunes(32) & strings.MaxRunes(32) & (=~#"\A[0-9a-fA-F]+\Z"#)) | fn.#Fn
					KeyRotationIntervalSeconds?:   *int | fn.#Fn
					SpekeKeyProvider:              *{
						CertificateArn?: *string | fn.#Fn
						ResourceId:      *string | fn.#Fn
						RoleArn:         *string | fn.#Fn
						SystemIds:       [...(*string | fn.#Fn)] | (*string | fn.#Fn)
						Url:             *string | fn.#Fn
					} | fn.#If
				} | fn.#If
				HlsManifests?: *[...{
					AdMarkers?:                      *("NONE" | "SCTE35_ENHANCED" | "PASSTHROUGH" | "DATERANGE") | fn.#Fn
					AdTriggers?:                     [...(*("SPLICE_INSERT" | "BREAK" | "PROVIDER_ADVERTISEMENT" | "DISTRIBUTOR_ADVERTISEMENT" | "PROVIDER_PLACEMENT_OPPORTUNITY" | "DISTRIBUTOR_PLACEMENT_OPPORTUNITY" | "PROVIDER_OVERLAY_PLACEMENT_OPPORTUNITY" | "DISTRIBUTOR_OVERLAY_PLACEMENT_OPPORTUNITY") | fn.#Fn)] | (*("SPLICE_INSERT" | "BREAK" | "PROVIDER_ADVERTISEMENT" | "DISTRIBUTOR_ADVERTISEMENT" | "PROVIDER_PLACEMENT_OPPORTUNITY" | "DISTRIBUTOR_PLACEMENT_OPPORTUNITY" | "PROVIDER_OVERLAY_PLACEMENT_OPPORTUNITY" | "DISTRIBUTOR_OVERLAY_PLACEMENT_OPPORTUNITY") | fn.#Fn)
					AdsOnDeliveryRestrictions?:      *("NONE" | "RESTRICTED" | "UNRESTRICTED" | "BOTH") | fn.#Fn
					Id:                              *string | fn.#Fn
					IncludeIframeOnlyStream?:        *bool | fn.#Fn
					ManifestName?:                   *string | fn.#Fn
					PlaylistType?:                   *("NONE" | "EVENT" | "VOD") | fn.#Fn
					PlaylistWindowSeconds?:          *int | fn.#Fn
					ProgramDateTimeIntervalSeconds?: *int | fn.#Fn
					Url?:                            *string | fn.#Fn
				}] | fn.#If
				SegmentDurationSeconds?: *int | fn.#Fn
				SegmentPrefix?:          *string | fn.#Fn
				StreamSelection?:        *{
					MaxVideoBitsPerSecond?: *int | fn.#Fn
					MinVideoBitsPerSecond?: *int | fn.#Fn
					StreamOrder?:           *("ORIGINAL" | "VIDEO_BITRATE_ASCENDING" | "VIDEO_BITRATE_DESCENDING") | fn.#Fn
				} | fn.#If
			} | fn.#If
			DashPackage?: *{
				AdTriggers?:                [...(*("SPLICE_INSERT" | "BREAK" | "PROVIDER_ADVERTISEMENT" | "DISTRIBUTOR_ADVERTISEMENT" | "PROVIDER_PLACEMENT_OPPORTUNITY" | "DISTRIBUTOR_PLACEMENT_OPPORTUNITY" | "PROVIDER_OVERLAY_PLACEMENT_OPPORTUNITY" | "DISTRIBUTOR_OVERLAY_PLACEMENT_OPPORTUNITY") | fn.#Fn)] | (*("SPLICE_INSERT" | "BREAK" | "PROVIDER_ADVERTISEMENT" | "DISTRIBUTOR_ADVERTISEMENT" | "PROVIDER_PLACEMENT_OPPORTUNITY" | "DISTRIBUTOR_PLACEMENT_OPPORTUNITY" | "PROVIDER_OVERLAY_PLACEMENT_OPPORTUNITY" | "DISTRIBUTOR_OVERLAY_PLACEMENT_OPPORTUNITY") | fn.#Fn)
				AdsOnDeliveryRestrictions?: *("NONE" | "RESTRICTED" | "UNRESTRICTED" | "BOTH") | fn.#Fn
				Encryption?:                *{
					KeyRotationIntervalSeconds?: *int | fn.#Fn
					SpekeKeyProvider:            *{
						CertificateArn?: *string | fn.#Fn
						ResourceId:      *string | fn.#Fn
						RoleArn:         *string | fn.#Fn
						SystemIds:       [...(*string | fn.#Fn)] | (*string | fn.#Fn)
						Url:             *string | fn.#Fn
					} | fn.#If
				} | fn.#If
				ManifestLayout?:         *("FULL" | "COMPACT") | fn.#Fn
				ManifestWindowSeconds?:  *int | fn.#Fn
				MinBufferTimeSeconds?:   *int | fn.#Fn
				MinUpdatePeriodSeconds?: *int | fn.#Fn
				PeriodTriggers?:         [...(*("ADS") | fn.#Fn)] | (*("ADS") | fn.#Fn)
				Profile?:                *("NONE" | "HBBTV_1_5") | fn.#Fn
				SegmentDurationSeconds?: *int | fn.#Fn
				SegmentTemplateFormat?:  *("NUMBER_WITH_TIMELINE" | "TIME_WITH_TIMELINE" | "NUMBER_WITH_DURATION") | fn.#Fn
				StreamSelection?:        *{
					MaxVideoBitsPerSecond?: *int | fn.#Fn
					MinVideoBitsPerSecond?: *int | fn.#Fn
					StreamOrder?:           *("ORIGINAL" | "VIDEO_BITRATE_ASCENDING" | "VIDEO_BITRATE_DESCENDING") | fn.#Fn
				} | fn.#If
				SuggestedPresentationDelaySeconds?: *int | fn.#Fn
				UtcTiming?:                         *("HTTP-ISO" | "HTTP-HEAD" | "NONE") | fn.#Fn
				UtcTimingUri?:                      *string | fn.#Fn
			} | fn.#If
			Description?: *string | fn.#Fn
			HlsPackage?:  *{
				AdMarkers?:                 *("NONE" | "SCTE35_ENHANCED" | "PASSTHROUGH" | "DATERANGE") | fn.#Fn
				AdTriggers?:                [...(*("SPLICE_INSERT" | "BREAK" | "PROVIDER_ADVERTISEMENT" | "DISTRIBUTOR_ADVERTISEMENT" | "PROVIDER_PLACEMENT_OPPORTUNITY" | "DISTRIBUTOR_PLACEMENT_OPPORTUNITY" | "PROVIDER_OVERLAY_PLACEMENT_OPPORTUNITY" | "DISTRIBUTOR_OVERLAY_PLACEMENT_OPPORTUNITY") | fn.#Fn)] | (*("SPLICE_INSERT" | "BREAK" | "PROVIDER_ADVERTISEMENT" | "DISTRIBUTOR_ADVERTISEMENT" | "PROVIDER_PLACEMENT_OPPORTUNITY" | "DISTRIBUTOR_PLACEMENT_OPPORTUNITY" | "PROVIDER_OVERLAY_PLACEMENT_OPPORTUNITY" | "DISTRIBUTOR_OVERLAY_PLACEMENT_OPPORTUNITY") | fn.#Fn)
				AdsOnDeliveryRestrictions?: *("NONE" | "RESTRICTED" | "UNRESTRICTED" | "BOTH") | fn.#Fn
				Encryption?:                *{
					ConstantInitializationVector?: *string | fn.#Fn
					EncryptionMethod?:             *("AES_128" | "SAMPLE_AES") | fn.#Fn
					KeyRotationIntervalSeconds?:   *int | fn.#Fn
					RepeatExtXKey?:                *bool | fn.#Fn
					SpekeKeyProvider:              *{
						CertificateArn?: *string | fn.#Fn
						ResourceId:      *string | fn.#Fn
						RoleArn:         *string | fn.#Fn
						SystemIds:       [...(*string | fn.#Fn)] | (*string | fn.#Fn)
						Url:             *string | fn.#Fn
					} | fn.#If
				} | fn.#If
				IncludeIframeOnlyStream?:        *bool | fn.#Fn
				PlaylistType?:                   *("NONE" | "EVENT" | "VOD") | fn.#Fn
				PlaylistWindowSeconds?:          *int | fn.#Fn
				ProgramDateTimeIntervalSeconds?: *int | fn.#Fn
				SegmentDurationSeconds?:         *int | fn.#Fn
				StreamSelection?:                *{
					MaxVideoBitsPerSecond?: *int | fn.#Fn
					MinVideoBitsPerSecond?: *int | fn.#Fn
					StreamOrder?:           *("ORIGINAL" | "VIDEO_BITRATE_ASCENDING" | "VIDEO_BITRATE_DESCENDING") | fn.#Fn
				} | fn.#If
				UseAudioRenditionGroup?: *bool | fn.#Fn
			} | fn.#If
			Id:            *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"\A[0-9a-zA-Z-_]+\Z"#)) | fn.#Fn
			ManifestName?: *string | fn.#Fn
			MssPackage?:   *{
				Encryption?: *{
					SpekeKeyProvider: *{
						CertificateArn?: *string | fn.#Fn
						ResourceId:      *string | fn.#Fn
						RoleArn:         *string | fn.#Fn
						SystemIds:       [...(*string | fn.#Fn)] | (*string | fn.#Fn)
						Url:             *string | fn.#Fn
					} | fn.#If
				} | fn.#If
				ManifestWindowSeconds?:  *int | fn.#Fn
				SegmentDurationSeconds?: *int | fn.#Fn
				StreamSelection?:        *{
					MaxVideoBitsPerSecond?: *int | fn.#Fn
					MinVideoBitsPerSecond?: *int | fn.#Fn
					StreamOrder?:           *("ORIGINAL" | "VIDEO_BITRATE_ASCENDING" | "VIDEO_BITRATE_DESCENDING") | fn.#Fn
				} | fn.#If
			} | fn.#If
			Origination?:            *("ALLOW" | "DENY") | fn.#Fn
			StartoverWindowSeconds?: *int | fn.#Fn
			Tags?:                   *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			TimeDelaySeconds?: *int | fn.#Fn
			Whitelist?:        [...(*string | fn.#Fn)] | (*string | fn.#Fn)
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#PackagingConfiguration: {
		Type: "AWS::MediaPackage::PackagingConfiguration"
		Properties: {
			CmafPackage?: *{
				Encryption?: *{
					SpekeKeyProvider: *{
						RoleArn:   *string | fn.#Fn
						SystemIds: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
						Url:       *string | fn.#Fn
					} | fn.#If
				} | fn.#If
				HlsManifests: *[...{
					AdMarkers?:                      *("NONE" | "SCTE35_ENHANCED" | "PASSTHROUGH") | fn.#Fn
					IncludeIframeOnlyStream?:        *bool | fn.#Fn
					ManifestName?:                   *string | fn.#Fn
					ProgramDateTimeIntervalSeconds?: *int | fn.#Fn
					RepeatExtXKey?:                  *bool | fn.#Fn
					StreamSelection?:                *{
						MaxVideoBitsPerSecond?: *int | fn.#Fn
						MinVideoBitsPerSecond?: *int | fn.#Fn
						StreamOrder?:           *("ORIGINAL" | "VIDEO_BITRATE_ASCENDING" | "VIDEO_BITRATE_DESCENDING") | fn.#Fn
					} | fn.#If
				}] | fn.#If
				IncludeEncoderConfigurationInSegments?: *bool | fn.#Fn
				SegmentDurationSeconds?:                *int | fn.#Fn
			} | fn.#If
			DashPackage?: *{
				DashManifests: *[...{
					ManifestLayout?:       *("FULL" | "COMPACT") | fn.#Fn
					ManifestName?:         *string | fn.#Fn
					MinBufferTimeSeconds?: *int | fn.#Fn
					Profile?:              *("NONE" | "HBBTV_1_5") | fn.#Fn
					StreamSelection?:      *{
						MaxVideoBitsPerSecond?: *int | fn.#Fn
						MinVideoBitsPerSecond?: *int | fn.#Fn
						StreamOrder?:           *("ORIGINAL" | "VIDEO_BITRATE_ASCENDING" | "VIDEO_BITRATE_DESCENDING") | fn.#Fn
					} | fn.#If
				}] | fn.#If
				Encryption?: *{
					SpekeKeyProvider: *{
						RoleArn:   *string | fn.#Fn
						SystemIds: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
						Url:       *string | fn.#Fn
					} | fn.#If
				} | fn.#If
				IncludeEncoderConfigurationInSegments?: *bool | fn.#Fn
				PeriodTriggers?:                        [...(*("ADS") | fn.#Fn)] | (*("ADS") | fn.#Fn)
				SegmentDurationSeconds?:                *int | fn.#Fn
				SegmentTemplateFormat?:                 *("NUMBER_WITH_TIMELINE" | "TIME_WITH_TIMELINE" | "NUMBER_WITH_DURATION") | fn.#Fn
			} | fn.#If
			HlsPackage?: *{
				Encryption?: *{
					ConstantInitializationVector?: *string | fn.#Fn
					EncryptionMethod?:             *("AES_128" | "SAMPLE_AES") | fn.#Fn
					SpekeKeyProvider:              *{
						RoleArn:   *string | fn.#Fn
						SystemIds: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
						Url:       *string | fn.#Fn
					} | fn.#If
				} | fn.#If
				HlsManifests: *[...{
					AdMarkers?:                      *("NONE" | "SCTE35_ENHANCED" | "PASSTHROUGH") | fn.#Fn
					IncludeIframeOnlyStream?:        *bool | fn.#Fn
					ManifestName?:                   *string | fn.#Fn
					ProgramDateTimeIntervalSeconds?: *int | fn.#Fn
					RepeatExtXKey?:                  *bool | fn.#Fn
					StreamSelection?:                *{
						MaxVideoBitsPerSecond?: *int | fn.#Fn
						MinVideoBitsPerSecond?: *int | fn.#Fn
						StreamOrder?:           *("ORIGINAL" | "VIDEO_BITRATE_ASCENDING" | "VIDEO_BITRATE_DESCENDING") | fn.#Fn
					} | fn.#If
				}] | fn.#If
				SegmentDurationSeconds?: *int | fn.#Fn
				UseAudioRenditionGroup?: *bool | fn.#Fn
			} | fn.#If
			Id:          *string | fn.#Fn
			MssPackage?: *{
				Encryption?: *{
					SpekeKeyProvider: *{
						RoleArn:   *string | fn.#Fn
						SystemIds: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
						Url:       *string | fn.#Fn
					} | fn.#If
				} | fn.#If
				MssManifests: *[...{
					ManifestName?:    *string | fn.#Fn
					StreamSelection?: *{
						MaxVideoBitsPerSecond?: *int | fn.#Fn
						MinVideoBitsPerSecond?: *int | fn.#Fn
						StreamOrder?:           *("ORIGINAL" | "VIDEO_BITRATE_ASCENDING" | "VIDEO_BITRATE_DESCENDING") | fn.#Fn
					} | fn.#If
				}] | fn.#If
				SegmentDurationSeconds?: *int | fn.#Fn
			} | fn.#If
			PackagingGroupId: *string | fn.#Fn
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
	#PackagingGroup: {
		Type: "AWS::MediaPackage::PackagingGroup"
		Properties: {
			Authorization?: *{
				CdnIdentifierSecret: *string | fn.#Fn
				SecretsRoleArn:      *string | fn.#Fn
			} | fn.#If
			EgressAccessLogs?: *{
				LogGroupName?: *(strings.MinRunes(1) & strings.MaxRunes(512) & (=~#"\A\/aws\/MediaPackage\/[0-9a-zA-Z-_\/\.#]+\Z"#)) | fn.#Fn
			} | fn.#If
			Id:    *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"\A[0-9a-zA-Z-_]+\Z"#)) | fn.#Fn
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
