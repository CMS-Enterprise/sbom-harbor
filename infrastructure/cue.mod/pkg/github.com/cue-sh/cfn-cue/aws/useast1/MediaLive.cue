package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#MediaLive: {
	#Channel: {
		Type: "AWS::MediaLive::Channel"
		Properties: {
			CdiInputSpecification?: *{
				Resolution?: *string | fn.#Fn
			} | fn.#If
			ChannelClass?: *string | fn.#Fn
			Destinations?: *[...{
				Id?:                   *string | fn.#Fn
				MediaPackageSettings?: *[...{
					ChannelId?: *string | fn.#Fn
				}] | fn.#If
				MultiplexSettings?: *{
					MultiplexId?: *string | fn.#Fn
					ProgramName?: *string | fn.#Fn
				} | fn.#If
				Settings?: *[...{
					PasswordParam?: *string | fn.#Fn
					StreamName?:    *string | fn.#Fn
					Url?:           *string | fn.#Fn
					Username?:      *string | fn.#Fn
				}] | fn.#If
			}] | fn.#If
			EncoderSettings?: *{
				AudioDescriptions?: *[...{
					AudioNormalizationSettings?: *{
						Algorithm?:        *string | fn.#Fn
						AlgorithmControl?: *string | fn.#Fn
						TargetLkfs?:       *number | fn.#Fn
					} | fn.#If
					AudioSelectorName?:         *string | fn.#Fn
					AudioType?:                 *string | fn.#Fn
					AudioTypeControl?:          *string | fn.#Fn
					AudioWatermarkingSettings?: *{
						NielsenWatermarksSettings?: *{
							NielsenCbetSettings?: *{
								CbetCheckDigitString?: *string | fn.#Fn
								CbetStepaside?:        *string | fn.#Fn
								Csid?:                 *string | fn.#Fn
							} | fn.#If
							NielsenDistributionType?: *string | fn.#Fn
							NielsenNaesIiNwSettings?: *{
								CheckDigitString?: *string | fn.#Fn
								Sid?:              *number | fn.#Fn
							} | fn.#If
						} | fn.#If
					} | fn.#If
					CodecSettings?: *{
						AacSettings?: *{
							Bitrate?:         *number | fn.#Fn
							CodingMode?:      *string | fn.#Fn
							InputType?:       *string | fn.#Fn
							Profile?:         *string | fn.#Fn
							RateControlMode?: *string | fn.#Fn
							RawFormat?:       *string | fn.#Fn
							SampleRate?:      *number | fn.#Fn
							Spec?:            *string | fn.#Fn
							VbrQuality?:      *string | fn.#Fn
						} | fn.#If
						Ac3Settings?: *{
							Bitrate?:         *number | fn.#Fn
							BitstreamMode?:   *string | fn.#Fn
							CodingMode?:      *string | fn.#Fn
							Dialnorm?:        *int | fn.#Fn
							DrcProfile?:      *string | fn.#Fn
							LfeFilter?:       *string | fn.#Fn
							MetadataControl?: *string | fn.#Fn
						} | fn.#If
						Eac3Settings?: *{
							AttenuationControl?:   *string | fn.#Fn
							Bitrate?:              *number | fn.#Fn
							BitstreamMode?:        *string | fn.#Fn
							CodingMode?:           *string | fn.#Fn
							DcFilter?:             *string | fn.#Fn
							Dialnorm?:             *int | fn.#Fn
							DrcLine?:              *string | fn.#Fn
							DrcRf?:                *string | fn.#Fn
							LfeControl?:           *string | fn.#Fn
							LfeFilter?:            *string | fn.#Fn
							LoRoCenterMixLevel?:   *number | fn.#Fn
							LoRoSurroundMixLevel?: *number | fn.#Fn
							LtRtCenterMixLevel?:   *number | fn.#Fn
							LtRtSurroundMixLevel?: *number | fn.#Fn
							MetadataControl?:      *string | fn.#Fn
							PassthroughControl?:   *string | fn.#Fn
							PhaseControl?:         *string | fn.#Fn
							StereoDownmix?:        *string | fn.#Fn
							SurroundExMode?:       *string | fn.#Fn
							SurroundMode?:         *string | fn.#Fn
						} | fn.#If
						Mp2Settings?: *{
							Bitrate?:    *number | fn.#Fn
							CodingMode?: *string | fn.#Fn
							SampleRate?: *number | fn.#Fn
						} | fn.#If
						PassThroughSettings?: *{} | fn.#If
						WavSettings?:         *{
							BitDepth?:   *number | fn.#Fn
							CodingMode?: *string | fn.#Fn
							SampleRate?: *number | fn.#Fn
						} | fn.#If
					} | fn.#If
					LanguageCode?:        *string | fn.#Fn
					LanguageCodeControl?: *string | fn.#Fn
					Name?:                *string | fn.#Fn
					RemixSettings?:       *{
						ChannelMappings?: *[...{
							InputChannelLevels?: *[...{
								Gain?:         *int | fn.#Fn
								InputChannel?: *int | fn.#Fn
							}] | fn.#If
							OutputChannel?: *int | fn.#Fn
						}] | fn.#If
						ChannelsIn?:  *int | fn.#Fn
						ChannelsOut?: *int | fn.#Fn
					} | fn.#If
					StreamName?: *string | fn.#Fn
				}] | fn.#If
				AvailBlanking?: *{
					AvailBlankingImage?: *{
						PasswordParam?: *string | fn.#Fn
						Uri?:           *string | fn.#Fn
						Username?:      *string | fn.#Fn
					} | fn.#If
					State?: *string | fn.#Fn
				} | fn.#If
				AvailConfiguration?: *{
					AvailSettings?: *{
						Scte35SpliceInsert?: *{
							AdAvailOffset?:          *int | fn.#Fn
							NoRegionalBlackoutFlag?: *string | fn.#Fn
							WebDeliveryAllowedFlag?: *string | fn.#Fn
						} | fn.#If
						Scte35TimeSignalApos?: *{
							AdAvailOffset?:          *int | fn.#Fn
							NoRegionalBlackoutFlag?: *string | fn.#Fn
							WebDeliveryAllowedFlag?: *string | fn.#Fn
						} | fn.#If
					} | fn.#If
				} | fn.#If
				BlackoutSlate?: *{
					BlackoutSlateImage?: *{
						PasswordParam?: *string | fn.#Fn
						Uri?:           *string | fn.#Fn
						Username?:      *string | fn.#Fn
					} | fn.#If
					NetworkEndBlackout?:      *string | fn.#Fn
					NetworkEndBlackoutImage?: *{
						PasswordParam?: *string | fn.#Fn
						Uri?:           *string | fn.#Fn
						Username?:      *string | fn.#Fn
					} | fn.#If
					NetworkId?: *string | fn.#Fn
					State?:     *string | fn.#Fn
				} | fn.#If
				CaptionDescriptions?: *[...{
					CaptionSelectorName?: *string | fn.#Fn
					DestinationSettings?: *{
						AribDestinationSettings?:   *{} | fn.#If
						BurnInDestinationSettings?: *{
							Alignment?:         *string | fn.#Fn
							BackgroundColor?:   *string | fn.#Fn
							BackgroundOpacity?: *int | fn.#Fn
							Font?:              *{
								PasswordParam?: *string | fn.#Fn
								Uri?:           *string | fn.#Fn
								Username?:      *string | fn.#Fn
							} | fn.#If
							FontColor?:           *string | fn.#Fn
							FontOpacity?:         *int | fn.#Fn
							FontResolution?:      *int | fn.#Fn
							FontSize?:            *string | fn.#Fn
							OutlineColor?:        *string | fn.#Fn
							OutlineSize?:         *int | fn.#Fn
							ShadowColor?:         *string | fn.#Fn
							ShadowOpacity?:       *int | fn.#Fn
							ShadowXOffset?:       *int | fn.#Fn
							ShadowYOffset?:       *int | fn.#Fn
							TeletextGridControl?: *string | fn.#Fn
							XPosition?:           *int | fn.#Fn
							YPosition?:           *int | fn.#Fn
						} | fn.#If
						DvbSubDestinationSettings?: *{
							Alignment?:         *string | fn.#Fn
							BackgroundColor?:   *string | fn.#Fn
							BackgroundOpacity?: *int | fn.#Fn
							Font?:              *{
								PasswordParam?: *string | fn.#Fn
								Uri?:           *string | fn.#Fn
								Username?:      *string | fn.#Fn
							} | fn.#If
							FontColor?:           *string | fn.#Fn
							FontOpacity?:         *int | fn.#Fn
							FontResolution?:      *int | fn.#Fn
							FontSize?:            *string | fn.#Fn
							OutlineColor?:        *string | fn.#Fn
							OutlineSize?:         *int | fn.#Fn
							ShadowColor?:         *string | fn.#Fn
							ShadowOpacity?:       *int | fn.#Fn
							ShadowXOffset?:       *int | fn.#Fn
							ShadowYOffset?:       *int | fn.#Fn
							TeletextGridControl?: *string | fn.#Fn
							XPosition?:           *int | fn.#Fn
							YPosition?:           *int | fn.#Fn
						} | fn.#If
						EbuTtDDestinationSettings?: *{
							CopyrightHolder?: *string | fn.#Fn
							FillLineGap?:     *string | fn.#Fn
							FontFamily?:      *string | fn.#Fn
							StyleControl?:    *string | fn.#Fn
						} | fn.#If
						EmbeddedDestinationSettings?:           *{} | fn.#If
						EmbeddedPlusScte20DestinationSettings?: *{} | fn.#If
						RtmpCaptionInfoDestinationSettings?:    *{} | fn.#If
						Scte20PlusEmbeddedDestinationSettings?: *{} | fn.#If
						Scte27DestinationSettings?:             *{} | fn.#If
						SmpteTtDestinationSettings?:            *{} | fn.#If
						TeletextDestinationSettings?:           *{} | fn.#If
						TtmlDestinationSettings?:               *{
							StyleControl?: *string | fn.#Fn
						} | fn.#If
						WebvttDestinationSettings?: *{
							StyleControl?: *string | fn.#Fn
						} | fn.#If
					} | fn.#If
					LanguageCode?:        *string | fn.#Fn
					LanguageDescription?: *string | fn.#Fn
					Name?:                *string | fn.#Fn
				}] | fn.#If
				FeatureActivations?: *{
					InputPrepareScheduleActions?: *string | fn.#Fn
				} | fn.#If
				GlobalConfiguration?: *{
					InitialAudioGain?:  *int | fn.#Fn
					InputEndAction?:    *string | fn.#Fn
					InputLossBehavior?: *{
						BlackFrameMsec?:      *int | fn.#Fn
						InputLossImageColor?: *string | fn.#Fn
						InputLossImageSlate?: *{
							PasswordParam?: *string | fn.#Fn
							Uri?:           *string | fn.#Fn
							Username?:      *string | fn.#Fn
						} | fn.#If
						InputLossImageType?: *string | fn.#Fn
						RepeatFrameMsec?:    *int | fn.#Fn
					} | fn.#If
					OutputLockingMode?:         *string | fn.#Fn
					OutputTimingSource?:        *string | fn.#Fn
					SupportLowFramerateInputs?: *string | fn.#Fn
				} | fn.#If
				MotionGraphicsConfiguration?: *{
					MotionGraphicsInsertion?: *string | fn.#Fn
					MotionGraphicsSettings?:  *{
						HtmlMotionGraphicsSettings?: *{} | fn.#If
					} | fn.#If
				} | fn.#If
				NielsenConfiguration?: *{
					DistributorId?:          *string | fn.#Fn
					NielsenPcmToId3Tagging?: *string | fn.#Fn
				} | fn.#If
				OutputGroups?: *[...{
					Name?:                *string | fn.#Fn
					OutputGroupSettings?: *{
						ArchiveGroupSettings?: *{
							ArchiveCdnSettings?: *{
								ArchiveS3Settings?: *{
									CannedAcl?: *string | fn.#Fn
								} | fn.#If
							} | fn.#If
							Destination?: *{
								DestinationRefId?: *string | fn.#Fn
							} | fn.#If
							RolloverInterval?: *int | fn.#Fn
						} | fn.#If
						FrameCaptureGroupSettings?: *{
							Destination?: *{
								DestinationRefId?: *string | fn.#Fn
							} | fn.#If
							FrameCaptureCdnSettings?: *{
								FrameCaptureS3Settings?: *{
									CannedAcl?: *string | fn.#Fn
								} | fn.#If
							} | fn.#If
						} | fn.#If
						HlsGroupSettings?: *{
							AdMarkers?:               [...(*string | fn.#Fn)] | (*string | fn.#Fn)
							BaseUrlContent?:          *string | fn.#Fn
							BaseUrlContent1?:         *string | fn.#Fn
							BaseUrlManifest?:         *string | fn.#Fn
							BaseUrlManifest1?:        *string | fn.#Fn
							CaptionLanguageMappings?: *[...{
								CaptionChannel?:      *int | fn.#Fn
								LanguageCode?:        *string | fn.#Fn
								LanguageDescription?: *string | fn.#Fn
							}] | fn.#If
							CaptionLanguageSetting?: *string | fn.#Fn
							ClientCache?:            *string | fn.#Fn
							CodecSpecification?:     *string | fn.#Fn
							ConstantIv?:             *string | fn.#Fn
							Destination?:            *{
								DestinationRefId?: *string | fn.#Fn
							} | fn.#If
							DirectoryStructure?: *string | fn.#Fn
							DiscontinuityTags?:  *string | fn.#Fn
							EncryptionType?:     *string | fn.#Fn
							HlsCdnSettings?:     *{
								HlsAkamaiSettings?: *{
									ConnectionRetryInterval?: *int | fn.#Fn
									FilecacheDuration?:       *int | fn.#Fn
									HttpTransferMode?:        *string | fn.#Fn
									NumRetries?:              *int | fn.#Fn
									RestartDelay?:            *int | fn.#Fn
									Salt?:                    *string | fn.#Fn
									Token?:                   *string | fn.#Fn
								} | fn.#If
								HlsBasicPutSettings?: *{
									ConnectionRetryInterval?: *int | fn.#Fn
									FilecacheDuration?:       *int | fn.#Fn
									NumRetries?:              *int | fn.#Fn
									RestartDelay?:            *int | fn.#Fn
								} | fn.#If
								HlsMediaStoreSettings?: *{
									ConnectionRetryInterval?: *int | fn.#Fn
									FilecacheDuration?:       *int | fn.#Fn
									MediaStoreStorageClass?:  *string | fn.#Fn
									NumRetries?:              *int | fn.#Fn
									RestartDelay?:            *int | fn.#Fn
								} | fn.#If
								HlsS3Settings?: *{
									CannedAcl?: *string | fn.#Fn
								} | fn.#If
								HlsWebdavSettings?: *{
									ConnectionRetryInterval?: *int | fn.#Fn
									FilecacheDuration?:       *int | fn.#Fn
									HttpTransferMode?:        *string | fn.#Fn
									NumRetries?:              *int | fn.#Fn
									RestartDelay?:            *int | fn.#Fn
								} | fn.#If
							} | fn.#If
							HlsId3SegmentTagging?:      *string | fn.#Fn
							IFrameOnlyPlaylists?:       *string | fn.#Fn
							IncompleteSegmentBehavior?: *string | fn.#Fn
							IndexNSegments?:            *int | fn.#Fn
							InputLossAction?:           *string | fn.#Fn
							IvInManifest?:              *string | fn.#Fn
							IvSource?:                  *string | fn.#Fn
							KeepSegments?:              *int | fn.#Fn
							KeyFormat?:                 *string | fn.#Fn
							KeyFormatVersions?:         *string | fn.#Fn
							KeyProviderSettings?:       *{
								StaticKeySettings?: *{
									KeyProviderServer?: *{
										PasswordParam?: *string | fn.#Fn
										Uri?:           *string | fn.#Fn
										Username?:      *string | fn.#Fn
									} | fn.#If
									StaticKeyValue?: *string | fn.#Fn
								} | fn.#If
							} | fn.#If
							ManifestCompression?:        *string | fn.#Fn
							ManifestDurationFormat?:     *string | fn.#Fn
							MinSegmentLength?:           *int | fn.#Fn
							Mode?:                       *string | fn.#Fn
							OutputSelection?:            *string | fn.#Fn
							ProgramDateTime?:            *string | fn.#Fn
							ProgramDateTimePeriod?:      *int | fn.#Fn
							RedundantManifest?:          *string | fn.#Fn
							SegmentLength?:              *int | fn.#Fn
							SegmentationMode?:           *string | fn.#Fn
							SegmentsPerSubdirectory?:    *int | fn.#Fn
							StreamInfResolution?:        *string | fn.#Fn
							TimedMetadataId3Frame?:      *string | fn.#Fn
							TimedMetadataId3Period?:     *int | fn.#Fn
							TimestampDeltaMilliseconds?: *int | fn.#Fn
							TsFileMode?:                 *string | fn.#Fn
						} | fn.#If
						MediaPackageGroupSettings?: *{
							Destination?: *{
								DestinationRefId?: *string | fn.#Fn
							} | fn.#If
						} | fn.#If
						MsSmoothGroupSettings?: *{
							AcquisitionPointId?:       *string | fn.#Fn
							AudioOnlyTimecodeControl?: *string | fn.#Fn
							CertificateMode?:          *string | fn.#Fn
							ConnectionRetryInterval?:  *int | fn.#Fn
							Destination?:              *{
								DestinationRefId?: *string | fn.#Fn
							} | fn.#If
							EventId?:                *string | fn.#Fn
							EventIdMode?:            *string | fn.#Fn
							EventStopBehavior?:      *string | fn.#Fn
							FilecacheDuration?:      *int | fn.#Fn
							FragmentLength?:         *int | fn.#Fn
							InputLossAction?:        *string | fn.#Fn
							NumRetries?:             *int | fn.#Fn
							RestartDelay?:           *int | fn.#Fn
							SegmentationMode?:       *string | fn.#Fn
							SendDelayMs?:            *int | fn.#Fn
							SparseTrackType?:        *string | fn.#Fn
							StreamManifestBehavior?: *string | fn.#Fn
							TimestampOffset?:        *string | fn.#Fn
							TimestampOffsetMode?:    *string | fn.#Fn
						} | fn.#If
						MultiplexGroupSettings?: *{} | fn.#If
						RtmpGroupSettings?:      *{
							AdMarkers?:            [...(*string | fn.#Fn)] | (*string | fn.#Fn)
							AuthenticationScheme?: *string | fn.#Fn
							CacheFullBehavior?:    *string | fn.#Fn
							CacheLength?:          *int | fn.#Fn
							CaptionData?:          *string | fn.#Fn
							InputLossAction?:      *string | fn.#Fn
							RestartDelay?:         *int | fn.#Fn
						} | fn.#If
						UdpGroupSettings?: *{
							InputLossAction?:        *string | fn.#Fn
							TimedMetadataId3Frame?:  *string | fn.#Fn
							TimedMetadataId3Period?: *int | fn.#Fn
						} | fn.#If
					} | fn.#If
					Outputs?: *[...{
						AudioDescriptionNames?:   [...(*string | fn.#Fn)] | (*string | fn.#Fn)
						CaptionDescriptionNames?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
						OutputName?:              *string | fn.#Fn
						OutputSettings?:          *{
							ArchiveOutputSettings?: *{
								ContainerSettings?: *{
									M2tsSettings?: *{
										AbsentInputAudioBehavior?: *string | fn.#Fn
										Arib?:                     *string | fn.#Fn
										AribCaptionsPid?:          *string | fn.#Fn
										AribCaptionsPidControl?:   *string | fn.#Fn
										AudioBufferModel?:         *string | fn.#Fn
										AudioFramesPerPes?:        *int | fn.#Fn
										AudioPids?:                *string | fn.#Fn
										AudioStreamType?:          *string | fn.#Fn
										Bitrate?:                  *int | fn.#Fn
										BufferModel?:              *string | fn.#Fn
										CcDescriptor?:             *string | fn.#Fn
										DvbNitSettings?:           *{
											NetworkId?:   *int | fn.#Fn
											NetworkName?: *string | fn.#Fn
											RepInterval?: *int | fn.#Fn
										} | fn.#If
										DvbSdtSettings?: *{
											OutputSdt?:           *string | fn.#Fn
											RepInterval?:         *int | fn.#Fn
											ServiceName?:         *string | fn.#Fn
											ServiceProviderName?: *string | fn.#Fn
										} | fn.#If
										DvbSubPids?:     *string | fn.#Fn
										DvbTdtSettings?: *{
											RepInterval?: *int | fn.#Fn
										} | fn.#If
										DvbTeletextPid?:        *string | fn.#Fn
										Ebif?:                  *string | fn.#Fn
										EbpAudioInterval?:      *string | fn.#Fn
										EbpLookaheadMs?:        *int | fn.#Fn
										EbpPlacement?:          *string | fn.#Fn
										EcmPid?:                *string | fn.#Fn
										EsRateInPes?:           *string | fn.#Fn
										EtvPlatformPid?:        *string | fn.#Fn
										EtvSignalPid?:          *string | fn.#Fn
										FragmentTime?:          *number | fn.#Fn
										Klv?:                   *string | fn.#Fn
										KlvDataPids?:           *string | fn.#Fn
										NielsenId3Behavior?:    *string | fn.#Fn
										NullPacketBitrate?:     *number | fn.#Fn
										PatInterval?:           *int | fn.#Fn
										PcrControl?:            *string | fn.#Fn
										PcrPeriod?:             *int | fn.#Fn
										PcrPid?:                *string | fn.#Fn
										PmtInterval?:           *int | fn.#Fn
										PmtPid?:                *string | fn.#Fn
										ProgramNum?:            *int | fn.#Fn
										RateMode?:              *string | fn.#Fn
										Scte27Pids?:            *string | fn.#Fn
										Scte35Control?:         *string | fn.#Fn
										Scte35Pid?:             *string | fn.#Fn
										SegmentationMarkers?:   *string | fn.#Fn
										SegmentationStyle?:     *string | fn.#Fn
										SegmentationTime?:      *number | fn.#Fn
										TimedMetadataBehavior?: *string | fn.#Fn
										TimedMetadataPid?:      *string | fn.#Fn
										TransportStreamId?:     *int | fn.#Fn
										VideoPid?:              *string | fn.#Fn
									} | fn.#If
									RawSettings?: *{} | fn.#If
								} | fn.#If
								Extension?:    *string | fn.#Fn
								NameModifier?: *string | fn.#Fn
							} | fn.#If
							FrameCaptureOutputSettings?: *{
								NameModifier?: *string | fn.#Fn
							} | fn.#If
							HlsOutputSettings?: *{
								H265PackagingType?: *string | fn.#Fn
								HlsSettings?:       *{
									AudioOnlyHlsSettings?: *{
										AudioGroupId?:   *string | fn.#Fn
										AudioOnlyImage?: *{
											PasswordParam?: *string | fn.#Fn
											Uri?:           *string | fn.#Fn
											Username?:      *string | fn.#Fn
										} | fn.#If
										AudioTrackType?: *string | fn.#Fn
										SegmentType?:    *string | fn.#Fn
									} | fn.#If
									Fmp4HlsSettings?: *{
										AudioRenditionSets?:    *string | fn.#Fn
										NielsenId3Behavior?:    *string | fn.#Fn
										TimedMetadataBehavior?: *string | fn.#Fn
									} | fn.#If
									FrameCaptureHlsSettings?: *{} | fn.#If
									StandardHlsSettings?:     *{
										AudioRenditionSets?: *string | fn.#Fn
										M3u8Settings?:       *{
											AudioFramesPerPes?:     *int | fn.#Fn
											AudioPids?:             *string | fn.#Fn
											EcmPid?:                *string | fn.#Fn
											NielsenId3Behavior?:    *string | fn.#Fn
											PatInterval?:           *int | fn.#Fn
											PcrControl?:            *string | fn.#Fn
											PcrPeriod?:             *int | fn.#Fn
											PcrPid?:                *string | fn.#Fn
											PmtInterval?:           *int | fn.#Fn
											PmtPid?:                *string | fn.#Fn
											ProgramNum?:            *int | fn.#Fn
											Scte35Behavior?:        *string | fn.#Fn
											Scte35Pid?:             *string | fn.#Fn
											TimedMetadataBehavior?: *string | fn.#Fn
											TimedMetadataPid?:      *string | fn.#Fn
											TransportStreamId?:     *int | fn.#Fn
											VideoPid?:              *string | fn.#Fn
										} | fn.#If
									} | fn.#If
								} | fn.#If
								NameModifier?:    *string | fn.#Fn
								SegmentModifier?: *string | fn.#Fn
							} | fn.#If
							MediaPackageOutputSettings?: *{} | fn.#If
							MsSmoothOutputSettings?:     *{
								H265PackagingType?: *string | fn.#Fn
								NameModifier?:      *string | fn.#Fn
							} | fn.#If
							MultiplexOutputSettings?: *{
								Destination?: *{
									DestinationRefId?: *string | fn.#Fn
								} | fn.#If
							} | fn.#If
							RtmpOutputSettings?: *{
								CertificateMode?:         *string | fn.#Fn
								ConnectionRetryInterval?: *int | fn.#Fn
								Destination?:             *{
									DestinationRefId?: *string | fn.#Fn
								} | fn.#If
								NumRetries?: *int | fn.#Fn
							} | fn.#If
							UdpOutputSettings?: *{
								BufferMsec?:        *int | fn.#Fn
								ContainerSettings?: *{
									M2tsSettings?: *{
										AbsentInputAudioBehavior?: *string | fn.#Fn
										Arib?:                     *string | fn.#Fn
										AribCaptionsPid?:          *string | fn.#Fn
										AribCaptionsPidControl?:   *string | fn.#Fn
										AudioBufferModel?:         *string | fn.#Fn
										AudioFramesPerPes?:        *int | fn.#Fn
										AudioPids?:                *string | fn.#Fn
										AudioStreamType?:          *string | fn.#Fn
										Bitrate?:                  *int | fn.#Fn
										BufferModel?:              *string | fn.#Fn
										CcDescriptor?:             *string | fn.#Fn
										DvbNitSettings?:           *{
											NetworkId?:   *int | fn.#Fn
											NetworkName?: *string | fn.#Fn
											RepInterval?: *int | fn.#Fn
										} | fn.#If
										DvbSdtSettings?: *{
											OutputSdt?:           *string | fn.#Fn
											RepInterval?:         *int | fn.#Fn
											ServiceName?:         *string | fn.#Fn
											ServiceProviderName?: *string | fn.#Fn
										} | fn.#If
										DvbSubPids?:     *string | fn.#Fn
										DvbTdtSettings?: *{
											RepInterval?: *int | fn.#Fn
										} | fn.#If
										DvbTeletextPid?:        *string | fn.#Fn
										Ebif?:                  *string | fn.#Fn
										EbpAudioInterval?:      *string | fn.#Fn
										EbpLookaheadMs?:        *int | fn.#Fn
										EbpPlacement?:          *string | fn.#Fn
										EcmPid?:                *string | fn.#Fn
										EsRateInPes?:           *string | fn.#Fn
										EtvPlatformPid?:        *string | fn.#Fn
										EtvSignalPid?:          *string | fn.#Fn
										FragmentTime?:          *number | fn.#Fn
										Klv?:                   *string | fn.#Fn
										KlvDataPids?:           *string | fn.#Fn
										NielsenId3Behavior?:    *string | fn.#Fn
										NullPacketBitrate?:     *number | fn.#Fn
										PatInterval?:           *int | fn.#Fn
										PcrControl?:            *string | fn.#Fn
										PcrPeriod?:             *int | fn.#Fn
										PcrPid?:                *string | fn.#Fn
										PmtInterval?:           *int | fn.#Fn
										PmtPid?:                *string | fn.#Fn
										ProgramNum?:            *int | fn.#Fn
										RateMode?:              *string | fn.#Fn
										Scte27Pids?:            *string | fn.#Fn
										Scte35Control?:         *string | fn.#Fn
										Scte35Pid?:             *string | fn.#Fn
										SegmentationMarkers?:   *string | fn.#Fn
										SegmentationStyle?:     *string | fn.#Fn
										SegmentationTime?:      *number | fn.#Fn
										TimedMetadataBehavior?: *string | fn.#Fn
										TimedMetadataPid?:      *string | fn.#Fn
										TransportStreamId?:     *int | fn.#Fn
										VideoPid?:              *string | fn.#Fn
									} | fn.#If
								} | fn.#If
								Destination?: *{
									DestinationRefId?: *string | fn.#Fn
								} | fn.#If
								FecOutputSettings?: *{
									ColumnDepth?: *int | fn.#Fn
									IncludeFec?:  *string | fn.#Fn
									RowLength?:   *int | fn.#Fn
								} | fn.#If
							} | fn.#If
						} | fn.#If
						VideoDescriptionName?: *string | fn.#Fn
					}] | fn.#If
				}] | fn.#If
				TimecodeConfig?: *{
					Source?:        *string | fn.#Fn
					SyncThreshold?: *int | fn.#Fn
				} | fn.#If
				VideoDescriptions?: *[...{
					CodecSettings?: *{
						FrameCaptureSettings?: *{
							CaptureInterval?:      *int | fn.#Fn
							CaptureIntervalUnits?: *string | fn.#Fn
						} | fn.#If
						H264Settings?: *{
							AdaptiveQuantization?: *string | fn.#Fn
							AfdSignaling?:         *string | fn.#Fn
							Bitrate?:              *int | fn.#Fn
							BufFillPct?:           *int | fn.#Fn
							BufSize?:              *int | fn.#Fn
							ColorMetadata?:        *string | fn.#Fn
							ColorSpaceSettings?:   *{
								ColorSpacePassthroughSettings?: *{} | fn.#If
								Rec601Settings?:                *{} | fn.#If
								Rec709Settings?:                *{} | fn.#If
							} | fn.#If
							EntropyEncoding?: *string | fn.#Fn
							FilterSettings?:  *{
								TemporalFilterSettings?: *{
									PostFilterSharpening?: *string | fn.#Fn
									Strength?:             *string | fn.#Fn
								} | fn.#If
							} | fn.#If
							FixedAfd?:             *string | fn.#Fn
							FlickerAq?:            *string | fn.#Fn
							ForceFieldPictures?:   *string | fn.#Fn
							FramerateControl?:     *string | fn.#Fn
							FramerateDenominator?: *int | fn.#Fn
							FramerateNumerator?:   *int | fn.#Fn
							GopBReference?:        *string | fn.#Fn
							GopClosedCadence?:     *int | fn.#Fn
							GopNumBFrames?:        *int | fn.#Fn
							GopSize?:              *number | fn.#Fn
							GopSizeUnits?:         *string | fn.#Fn
							Level?:                *string | fn.#Fn
							LookAheadRateControl?: *string | fn.#Fn
							MaxBitrate?:           *int | fn.#Fn
							MinIInterval?:         *int | fn.#Fn
							NumRefFrames?:         *int | fn.#Fn
							ParControl?:           *string | fn.#Fn
							ParDenominator?:       *int | fn.#Fn
							ParNumerator?:         *int | fn.#Fn
							Profile?:              *string | fn.#Fn
							QualityLevel?:         *string | fn.#Fn
							QvbrQualityLevel?:     *int | fn.#Fn
							RateControlMode?:      *string | fn.#Fn
							ScanType?:             *string | fn.#Fn
							SceneChangeDetect?:    *string | fn.#Fn
							Slices?:               *int | fn.#Fn
							Softness?:             *int | fn.#Fn
							SpatialAq?:            *string | fn.#Fn
							SubgopLength?:         *string | fn.#Fn
							Syntax?:               *string | fn.#Fn
							TemporalAq?:           *string | fn.#Fn
							TimecodeInsertion?:    *string | fn.#Fn
						} | fn.#If
						H265Settings?: *{
							AdaptiveQuantization?:        *string | fn.#Fn
							AfdSignaling?:                *string | fn.#Fn
							AlternativeTransferFunction?: *string | fn.#Fn
							Bitrate?:                     *int | fn.#Fn
							BufSize?:                     *int | fn.#Fn
							ColorMetadata?:               *string | fn.#Fn
							ColorSpaceSettings?:          *{
								ColorSpacePassthroughSettings?: *{} | fn.#If
								Hdr10Settings?:                 *{
									MaxCll?:  *int | fn.#Fn
									MaxFall?: *int | fn.#Fn
								} | fn.#If
								Rec601Settings?: *{} | fn.#If
								Rec709Settings?: *{} | fn.#If
							} | fn.#If
							FilterSettings?: *{
								TemporalFilterSettings?: *{
									PostFilterSharpening?: *string | fn.#Fn
									Strength?:             *string | fn.#Fn
								} | fn.#If
							} | fn.#If
							FixedAfd?:             *string | fn.#Fn
							FlickerAq?:            *string | fn.#Fn
							FramerateDenominator?: *int | fn.#Fn
							FramerateNumerator?:   *int | fn.#Fn
							GopClosedCadence?:     *int | fn.#Fn
							GopSize?:              *number | fn.#Fn
							GopSizeUnits?:         *string | fn.#Fn
							Level?:                *string | fn.#Fn
							LookAheadRateControl?: *string | fn.#Fn
							MaxBitrate?:           *int | fn.#Fn
							MinIInterval?:         *int | fn.#Fn
							ParDenominator?:       *int | fn.#Fn
							ParNumerator?:         *int | fn.#Fn
							Profile?:              *string | fn.#Fn
							QvbrQualityLevel?:     *int | fn.#Fn
							RateControlMode?:      *string | fn.#Fn
							ScanType?:             *string | fn.#Fn
							SceneChangeDetect?:    *string | fn.#Fn
							Slices?:               *int | fn.#Fn
							Tier?:                 *string | fn.#Fn
							TimecodeInsertion?:    *string | fn.#Fn
						} | fn.#If
						Mpeg2Settings?: *{
							AdaptiveQuantization?: *string | fn.#Fn
							AfdSignaling?:         *string | fn.#Fn
							ColorMetadata?:        *string | fn.#Fn
							ColorSpace?:           *string | fn.#Fn
							DisplayAspectRatio?:   *string | fn.#Fn
							FilterSettings?:       *{
								TemporalFilterSettings?: *{
									PostFilterSharpening?: *string | fn.#Fn
									Strength?:             *string | fn.#Fn
								} | fn.#If
							} | fn.#If
							FixedAfd?:             *string | fn.#Fn
							FramerateDenominator?: *int | fn.#Fn
							FramerateNumerator?:   *int | fn.#Fn
							GopClosedCadence?:     *int | fn.#Fn
							GopNumBFrames?:        *int | fn.#Fn
							GopSize?:              *number | fn.#Fn
							GopSizeUnits?:         *string | fn.#Fn
							ScanType?:             *string | fn.#Fn
							SubgopLength?:         *string | fn.#Fn
							TimecodeInsertion?:    *string | fn.#Fn
						} | fn.#If
					} | fn.#If
					Height?:          *int | fn.#Fn
					Name?:            *string | fn.#Fn
					RespondToAfd?:    *string | fn.#Fn
					ScalingBehavior?: *string | fn.#Fn
					Sharpness?:       *int | fn.#Fn
					Width?:           *int | fn.#Fn
				}] | fn.#If
			} | fn.#If
			InputAttachments?: *[...{
				AutomaticInputFailoverSettings?: *{
					ErrorClearTimeMsec?: *int | fn.#Fn
					FailoverConditions?: *[...{
						FailoverConditionSettings?: *{
							AudioSilenceSettings?: *{
								AudioSelectorName?:         *string | fn.#Fn
								AudioSilenceThresholdMsec?: *int | fn.#Fn
							} | fn.#If
							InputLossSettings?: *{
								InputLossThresholdMsec?: *int | fn.#Fn
							} | fn.#If
							VideoBlackSettings?: *{
								BlackDetectThreshold?:    *number | fn.#Fn
								VideoBlackThresholdMsec?: *int | fn.#Fn
							} | fn.#If
						} | fn.#If
					}] | fn.#If
					InputPreference?:  *string | fn.#Fn
					SecondaryInputId?: *string | fn.#Fn
				} | fn.#If
				InputAttachmentName?: *string | fn.#Fn
				InputId?:             *string | fn.#Fn
				InputSettings?:       *{
					AudioSelectors?: *[...{
						Name?:             *string | fn.#Fn
						SelectorSettings?: *{
							AudioHlsRenditionSelection?: *{
								GroupId?: *string | fn.#Fn
								Name?:    *string | fn.#Fn
							} | fn.#If
							AudioLanguageSelection?: *{
								LanguageCode?:            *string | fn.#Fn
								LanguageSelectionPolicy?: *string | fn.#Fn
							} | fn.#If
							AudioPidSelection?: *{
								Pid?: *int | fn.#Fn
							} | fn.#If
							AudioTrackSelection?: *{
								Tracks?: *[...{
									Track?: *int | fn.#Fn
								}] | fn.#If
							} | fn.#If
						} | fn.#If
					}] | fn.#If
					CaptionSelectors?: *[...{
						LanguageCode?:     *string | fn.#Fn
						Name?:             *string | fn.#Fn
						SelectorSettings?: *{
							AncillarySourceSettings?: *{
								SourceAncillaryChannelNumber?: *int | fn.#Fn
							} | fn.#If
							AribSourceSettings?:   *{} | fn.#If
							DvbSubSourceSettings?: *{
								OcrLanguage?: *string | fn.#Fn
								Pid?:         *int | fn.#Fn
							} | fn.#If
							EmbeddedSourceSettings?: *{
								Convert608To708?:        *string | fn.#Fn
								Scte20Detection?:        *string | fn.#Fn
								Source608ChannelNumber?: *int | fn.#Fn
								Source608TrackNumber?:   *int | fn.#Fn
							} | fn.#If
							Scte20SourceSettings?: *{
								Convert608To708?:        *string | fn.#Fn
								Source608ChannelNumber?: *int | fn.#Fn
							} | fn.#If
							Scte27SourceSettings?: *{
								OcrLanguage?: *string | fn.#Fn
								Pid?:         *int | fn.#Fn
							} | fn.#If
							TeletextSourceSettings?: *{
								OutputRectangle?: *{
									Height?:     *number | fn.#Fn
									LeftOffset?: *number | fn.#Fn
									TopOffset?:  *number | fn.#Fn
									Width?:      *number | fn.#Fn
								} | fn.#If
								PageNumber?: *string | fn.#Fn
							} | fn.#If
						} | fn.#If
					}] | fn.#If
					DeblockFilter?:        *string | fn.#Fn
					DenoiseFilter?:        *string | fn.#Fn
					FilterStrength?:       *int | fn.#Fn
					InputFilter?:          *string | fn.#Fn
					NetworkInputSettings?: *{
						HlsInputSettings?: *{
							Bandwidth?:      *int | fn.#Fn
							BufferSegments?: *int | fn.#Fn
							Retries?:        *int | fn.#Fn
							RetryInterval?:  *int | fn.#Fn
							Scte35Source?:   *string | fn.#Fn
						} | fn.#If
						ServerValidation?: *string | fn.#Fn
					} | fn.#If
					Smpte2038DataPreference?: *string | fn.#Fn
					SourceEndBehavior?:       *string | fn.#Fn
					VideoSelector?:           *{
						ColorSpace?:         *string | fn.#Fn
						ColorSpaceSettings?: *{
							Hdr10Settings?: *{
								MaxCll?:  *int | fn.#Fn
								MaxFall?: *int | fn.#Fn
							} | fn.#If
						} | fn.#If
						ColorSpaceUsage?:  *string | fn.#Fn
						SelectorSettings?: *{
							VideoSelectorPid?: *{
								Pid?: *int | fn.#Fn
							} | fn.#If
							VideoSelectorProgramId?: *{
								ProgramId?: *int | fn.#Fn
							} | fn.#If
						} | fn.#If
					} | fn.#If
				} | fn.#If
			}] | fn.#If
			InputSpecification?: *{
				Codec?:          *string | fn.#Fn
				MaximumBitrate?: *string | fn.#Fn
				Resolution?:     *string | fn.#Fn
			} | fn.#If
			LogLevel?: *string | fn.#Fn
			Name?:     *string | fn.#Fn
			RoleArn?:  *string | fn.#Fn
			Tags?:     *{
				[string]: _
			} | fn.#Fn
			Vpc?: *{
				PublicAddressAllocationIds?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				SecurityGroupIds?:           [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				SubnetIds?:                  [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Input: {
		Type: "AWS::MediaLive::Input"
		Properties: {
			Destinations?: *[...{
				StreamName?: *string | fn.#Fn
			}] | fn.#If
			InputDevices?: *[...{
				Id?: *string | fn.#Fn
			}] | fn.#If
			InputSecurityGroups?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			MediaConnectFlows?:   *[...{
				FlowArn?: *string | fn.#Fn
			}] | fn.#If
			Name?:    *string | fn.#Fn
			RoleArn?: *string | fn.#Fn
			Sources?: *[...{
				PasswordParam?: *string | fn.#Fn
				Url?:           *string | fn.#Fn
				Username?:      *string | fn.#Fn
			}] | fn.#If
			Tags?: *{
				[string]: _
			} | fn.#Fn
			Type?: *string | fn.#Fn
			Vpc?:  *{
				SecurityGroupIds?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				SubnetIds?:        [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#InputSecurityGroup: {
		Type: "AWS::MediaLive::InputSecurityGroup"
		Properties: {
			Tags?: *{
				[string]: _
			} | fn.#Fn
			WhitelistRules?: *[...{
				Cidr?: *(=~#"^(([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5]).){3}([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])(\/([0-9]|[1-2][0-9]|3[0-2]))$"#) | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
