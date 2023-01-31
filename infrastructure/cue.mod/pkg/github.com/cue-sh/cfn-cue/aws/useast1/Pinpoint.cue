package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#Pinpoint: {
	#ADMChannel: {
		Type: "AWS::Pinpoint::ADMChannel"
		Properties: {
			ApplicationId: *string | fn.#Fn
			ClientId:      *string | fn.#Fn
			ClientSecret:  *string | fn.#Fn
			Enabled?:      *bool | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#APNSChannel: {
		Type: "AWS::Pinpoint::APNSChannel"
		Properties: {
			ApplicationId:                *string | fn.#Fn
			BundleId?:                    *string | fn.#Fn
			Certificate?:                 *string | fn.#Fn
			DefaultAuthenticationMethod?: *string | fn.#Fn
			Enabled?:                     *bool | fn.#Fn
			PrivateKey?:                  *string | fn.#Fn
			TeamId?:                      *string | fn.#Fn
			TokenKey?:                    *string | fn.#Fn
			TokenKeyId?:                  *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#APNSSandboxChannel: {
		Type: "AWS::Pinpoint::APNSSandboxChannel"
		Properties: {
			ApplicationId:                *string | fn.#Fn
			BundleId?:                    *string | fn.#Fn
			Certificate?:                 *string | fn.#Fn
			DefaultAuthenticationMethod?: *string | fn.#Fn
			Enabled?:                     *bool | fn.#Fn
			PrivateKey?:                  *string | fn.#Fn
			TeamId?:                      *string | fn.#Fn
			TokenKey?:                    *string | fn.#Fn
			TokenKeyId?:                  *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#APNSVoipChannel: {
		Type: "AWS::Pinpoint::APNSVoipChannel"
		Properties: {
			ApplicationId:                *string | fn.#Fn
			BundleId?:                    *string | fn.#Fn
			Certificate?:                 *string | fn.#Fn
			DefaultAuthenticationMethod?: *string | fn.#Fn
			Enabled?:                     *bool | fn.#Fn
			PrivateKey?:                  *string | fn.#Fn
			TeamId?:                      *string | fn.#Fn
			TokenKey?:                    *string | fn.#Fn
			TokenKeyId?:                  *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#APNSVoipSandboxChannel: {
		Type: "AWS::Pinpoint::APNSVoipSandboxChannel"
		Properties: {
			ApplicationId:                *string | fn.#Fn
			BundleId?:                    *string | fn.#Fn
			Certificate?:                 *string | fn.#Fn
			DefaultAuthenticationMethod?: *string | fn.#Fn
			Enabled?:                     *bool | fn.#Fn
			PrivateKey?:                  *string | fn.#Fn
			TeamId?:                      *string | fn.#Fn
			TokenKey?:                    *string | fn.#Fn
			TokenKeyId?:                  *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#App: {
		Type: "AWS::Pinpoint::App"
		Properties: {
			Name:  *string | fn.#Fn
			Tags?: *{
				[string]: _
			} | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ApplicationSettings: {
		Type: "AWS::Pinpoint::ApplicationSettings"
		Properties: {
			ApplicationId: *string | fn.#Fn
			CampaignHook?: *{
				LambdaFunctionName?: *string | fn.#Fn
				Mode?:               *string | fn.#Fn
				WebUrl?:             *string | fn.#Fn
			} | fn.#If
			CloudWatchMetricsEnabled?: *bool | fn.#Fn
			Limits?:                   *{
				Daily?:             *int | fn.#Fn
				MaximumDuration?:   *int | fn.#Fn
				MessagesPerSecond?: *int | fn.#Fn
				Total?:             *int | fn.#Fn
			} | fn.#If
			QuietTime?: *{
				End:   *string | fn.#Fn
				Start: *string | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#BaiduChannel: {
		Type: "AWS::Pinpoint::BaiduChannel"
		Properties: {
			ApiKey:        *string | fn.#Fn
			ApplicationId: *string | fn.#Fn
			Enabled?:      *bool | fn.#Fn
			SecretKey:     *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Campaign: {
		Type: "AWS::Pinpoint::Campaign"
		Properties: {
			AdditionalTreatments?: *[...{
				MessageConfiguration?: *{
					ADMMessage?: *{
						Action?:            *string | fn.#Fn
						Body?:              *string | fn.#Fn
						ImageIconUrl?:      *string | fn.#Fn
						ImageSmallIconUrl?: *string | fn.#Fn
						ImageUrl?:          *string | fn.#Fn
						JsonBody?:          *string | fn.#Fn
						MediaUrl?:          *string | fn.#Fn
						RawContent?:        *string | fn.#Fn
						SilentPush?:        *bool | fn.#Fn
						TimeToLive?:        *int | fn.#Fn
						Title?:             *string | fn.#Fn
						Url?:               *string | fn.#Fn
					} | fn.#If
					APNSMessage?: *{
						Action?:            *string | fn.#Fn
						Body?:              *string | fn.#Fn
						ImageIconUrl?:      *string | fn.#Fn
						ImageSmallIconUrl?: *string | fn.#Fn
						ImageUrl?:          *string | fn.#Fn
						JsonBody?:          *string | fn.#Fn
						MediaUrl?:          *string | fn.#Fn
						RawContent?:        *string | fn.#Fn
						SilentPush?:        *bool | fn.#Fn
						TimeToLive?:        *int | fn.#Fn
						Title?:             *string | fn.#Fn
						Url?:               *string | fn.#Fn
					} | fn.#If
					BaiduMessage?: *{
						Action?:            *string | fn.#Fn
						Body?:              *string | fn.#Fn
						ImageIconUrl?:      *string | fn.#Fn
						ImageSmallIconUrl?: *string | fn.#Fn
						ImageUrl?:          *string | fn.#Fn
						JsonBody?:          *string | fn.#Fn
						MediaUrl?:          *string | fn.#Fn
						RawContent?:        *string | fn.#Fn
						SilentPush?:        *bool | fn.#Fn
						TimeToLive?:        *int | fn.#Fn
						Title?:             *string | fn.#Fn
						Url?:               *string | fn.#Fn
					} | fn.#If
					DefaultMessage?: *{
						Action?:            *string | fn.#Fn
						Body?:              *string | fn.#Fn
						ImageIconUrl?:      *string | fn.#Fn
						ImageSmallIconUrl?: *string | fn.#Fn
						ImageUrl?:          *string | fn.#Fn
						JsonBody?:          *string | fn.#Fn
						MediaUrl?:          *string | fn.#Fn
						RawContent?:        *string | fn.#Fn
						SilentPush?:        *bool | fn.#Fn
						TimeToLive?:        *int | fn.#Fn
						Title?:             *string | fn.#Fn
						Url?:               *string | fn.#Fn
					} | fn.#If
					EmailMessage?: *{
						Body?:        *string | fn.#Fn
						FromAddress?: *string | fn.#Fn
						HtmlBody?:    *string | fn.#Fn
						Title?:       *string | fn.#Fn
					} | fn.#If
					GCMMessage?: *{
						Action?:            *string | fn.#Fn
						Body?:              *string | fn.#Fn
						ImageIconUrl?:      *string | fn.#Fn
						ImageSmallIconUrl?: *string | fn.#Fn
						ImageUrl?:          *string | fn.#Fn
						JsonBody?:          *string | fn.#Fn
						MediaUrl?:          *string | fn.#Fn
						RawContent?:        *string | fn.#Fn
						SilentPush?:        *bool | fn.#Fn
						TimeToLive?:        *int | fn.#Fn
						Title?:             *string | fn.#Fn
						Url?:               *string | fn.#Fn
					} | fn.#If
					InAppMessage?: *{
						Content?: *[...{
							BackgroundColor?: *string | fn.#Fn
							BodyConfig?:      *{
								Alignment?: *string | fn.#Fn
								Body?:      *string | fn.#Fn
								TextColor?: *string | fn.#Fn
							} | fn.#If
							HeaderConfig?: *{
								Alignment?: *string | fn.#Fn
								Header?:    *string | fn.#Fn
								TextColor?: *string | fn.#Fn
							} | fn.#If
							ImageUrl?:   *string | fn.#Fn
							PrimaryBtn?: *{
								Android?: *{
									ButtonAction?: *string | fn.#Fn
									Link?:         *string | fn.#Fn
								} | fn.#If
								DefaultConfig?: *{
									BackgroundColor?: *string | fn.#Fn
									BorderRadius?:    *int | fn.#Fn
									ButtonAction?:    *string | fn.#Fn
									Link?:            *string | fn.#Fn
									Text?:            *string | fn.#Fn
									TextColor?:       *string | fn.#Fn
								} | fn.#If
								IOS?: *{
									ButtonAction?: *string | fn.#Fn
									Link?:         *string | fn.#Fn
								} | fn.#If
								Web?: *{
									ButtonAction?: *string | fn.#Fn
									Link?:         *string | fn.#Fn
								} | fn.#If
							} | fn.#If
							SecondaryBtn?: *{
								Android?: *{
									ButtonAction?: *string | fn.#Fn
									Link?:         *string | fn.#Fn
								} | fn.#If
								DefaultConfig?: *{
									BackgroundColor?: *string | fn.#Fn
									BorderRadius?:    *int | fn.#Fn
									ButtonAction?:    *string | fn.#Fn
									Link?:            *string | fn.#Fn
									Text?:            *string | fn.#Fn
									TextColor?:       *string | fn.#Fn
								} | fn.#If
								IOS?: *{
									ButtonAction?: *string | fn.#Fn
									Link?:         *string | fn.#Fn
								} | fn.#If
								Web?: *{
									ButtonAction?: *string | fn.#Fn
									Link?:         *string | fn.#Fn
								} | fn.#If
							} | fn.#If
						}] | fn.#If
						CustomConfig?: *{
							[string]: _
						} | fn.#Fn
						Layout?: *string | fn.#Fn
					} | fn.#If
					SMSMessage?: *{
						Body?:              *string | fn.#Fn
						EntityId?:          *string | fn.#Fn
						MessageType?:       *string | fn.#Fn
						OriginationNumber?: *string | fn.#Fn
						SenderId?:          *string | fn.#Fn
						TemplateId?:        *string | fn.#Fn
					} | fn.#If
				} | fn.#If
				Schedule?: *{
					EndTime?:     *string | fn.#Fn
					EventFilter?: *{
						Dimensions?: *{
							Attributes?: *{
								[string]: _
							} | fn.#Fn
							EventType?: *{
								DimensionType?: *string | fn.#Fn
								Values?:        [...(*string | fn.#Fn)] | (*string | fn.#Fn)
							} | fn.#If
							Metrics?: *{
								[string]: _
							} | fn.#Fn
						} | fn.#If
						FilterType?: *string | fn.#Fn
					} | fn.#If
					Frequency?:   *string | fn.#Fn
					IsLocalTime?: *bool | fn.#Fn
					QuietTime?:   *{
						End:   *string | fn.#Fn
						Start: *string | fn.#Fn
					} | fn.#If
					StartTime?: *string | fn.#Fn
					TimeZone?:  *string | fn.#Fn
				} | fn.#If
				SizePercent?:          *int | fn.#Fn
				TreatmentDescription?: *string | fn.#Fn
				TreatmentName?:        *string | fn.#Fn
			}] | fn.#If
			ApplicationId: *string | fn.#Fn
			CampaignHook?: *{
				LambdaFunctionName?: *string | fn.#Fn
				Mode?:               *string | fn.#Fn
				WebUrl?:             *string | fn.#Fn
			} | fn.#If
			Description?:    *string | fn.#Fn
			HoldoutPercent?: *int | fn.#Fn
			IsPaused?:       *bool | fn.#Fn
			Limits?:         *{
				Daily?:             *int | fn.#Fn
				MaximumDuration?:   *int | fn.#Fn
				MessagesPerSecond?: *int | fn.#Fn
				Session?:           *int | fn.#Fn
				Total?:             *int | fn.#Fn
			} | fn.#If
			MessageConfiguration: *{
				ADMMessage?: *{
					Action?:            *string | fn.#Fn
					Body?:              *string | fn.#Fn
					ImageIconUrl?:      *string | fn.#Fn
					ImageSmallIconUrl?: *string | fn.#Fn
					ImageUrl?:          *string | fn.#Fn
					JsonBody?:          *string | fn.#Fn
					MediaUrl?:          *string | fn.#Fn
					RawContent?:        *string | fn.#Fn
					SilentPush?:        *bool | fn.#Fn
					TimeToLive?:        *int | fn.#Fn
					Title?:             *string | fn.#Fn
					Url?:               *string | fn.#Fn
				} | fn.#If
				APNSMessage?: *{
					Action?:            *string | fn.#Fn
					Body?:              *string | fn.#Fn
					ImageIconUrl?:      *string | fn.#Fn
					ImageSmallIconUrl?: *string | fn.#Fn
					ImageUrl?:          *string | fn.#Fn
					JsonBody?:          *string | fn.#Fn
					MediaUrl?:          *string | fn.#Fn
					RawContent?:        *string | fn.#Fn
					SilentPush?:        *bool | fn.#Fn
					TimeToLive?:        *int | fn.#Fn
					Title?:             *string | fn.#Fn
					Url?:               *string | fn.#Fn
				} | fn.#If
				BaiduMessage?: *{
					Action?:            *string | fn.#Fn
					Body?:              *string | fn.#Fn
					ImageIconUrl?:      *string | fn.#Fn
					ImageSmallIconUrl?: *string | fn.#Fn
					ImageUrl?:          *string | fn.#Fn
					JsonBody?:          *string | fn.#Fn
					MediaUrl?:          *string | fn.#Fn
					RawContent?:        *string | fn.#Fn
					SilentPush?:        *bool | fn.#Fn
					TimeToLive?:        *int | fn.#Fn
					Title?:             *string | fn.#Fn
					Url?:               *string | fn.#Fn
				} | fn.#If
				DefaultMessage?: *{
					Action?:            *string | fn.#Fn
					Body?:              *string | fn.#Fn
					ImageIconUrl?:      *string | fn.#Fn
					ImageSmallIconUrl?: *string | fn.#Fn
					ImageUrl?:          *string | fn.#Fn
					JsonBody?:          *string | fn.#Fn
					MediaUrl?:          *string | fn.#Fn
					RawContent?:        *string | fn.#Fn
					SilentPush?:        *bool | fn.#Fn
					TimeToLive?:        *int | fn.#Fn
					Title?:             *string | fn.#Fn
					Url?:               *string | fn.#Fn
				} | fn.#If
				EmailMessage?: *{
					Body?:        *string | fn.#Fn
					FromAddress?: *string | fn.#Fn
					HtmlBody?:    *string | fn.#Fn
					Title?:       *string | fn.#Fn
				} | fn.#If
				GCMMessage?: *{
					Action?:            *string | fn.#Fn
					Body?:              *string | fn.#Fn
					ImageIconUrl?:      *string | fn.#Fn
					ImageSmallIconUrl?: *string | fn.#Fn
					ImageUrl?:          *string | fn.#Fn
					JsonBody?:          *string | fn.#Fn
					MediaUrl?:          *string | fn.#Fn
					RawContent?:        *string | fn.#Fn
					SilentPush?:        *bool | fn.#Fn
					TimeToLive?:        *int | fn.#Fn
					Title?:             *string | fn.#Fn
					Url?:               *string | fn.#Fn
				} | fn.#If
				InAppMessage?: *{
					Content?: *[...{
						BackgroundColor?: *string | fn.#Fn
						BodyConfig?:      *{
							Alignment?: *string | fn.#Fn
							Body?:      *string | fn.#Fn
							TextColor?: *string | fn.#Fn
						} | fn.#If
						HeaderConfig?: *{
							Alignment?: *string | fn.#Fn
							Header?:    *string | fn.#Fn
							TextColor?: *string | fn.#Fn
						} | fn.#If
						ImageUrl?:   *string | fn.#Fn
						PrimaryBtn?: *{
							Android?: *{
								ButtonAction?: *string | fn.#Fn
								Link?:         *string | fn.#Fn
							} | fn.#If
							DefaultConfig?: *{
								BackgroundColor?: *string | fn.#Fn
								BorderRadius?:    *int | fn.#Fn
								ButtonAction?:    *string | fn.#Fn
								Link?:            *string | fn.#Fn
								Text?:            *string | fn.#Fn
								TextColor?:       *string | fn.#Fn
							} | fn.#If
							IOS?: *{
								ButtonAction?: *string | fn.#Fn
								Link?:         *string | fn.#Fn
							} | fn.#If
							Web?: *{
								ButtonAction?: *string | fn.#Fn
								Link?:         *string | fn.#Fn
							} | fn.#If
						} | fn.#If
						SecondaryBtn?: *{
							Android?: *{
								ButtonAction?: *string | fn.#Fn
								Link?:         *string | fn.#Fn
							} | fn.#If
							DefaultConfig?: *{
								BackgroundColor?: *string | fn.#Fn
								BorderRadius?:    *int | fn.#Fn
								ButtonAction?:    *string | fn.#Fn
								Link?:            *string | fn.#Fn
								Text?:            *string | fn.#Fn
								TextColor?:       *string | fn.#Fn
							} | fn.#If
							IOS?: *{
								ButtonAction?: *string | fn.#Fn
								Link?:         *string | fn.#Fn
							} | fn.#If
							Web?: *{
								ButtonAction?: *string | fn.#Fn
								Link?:         *string | fn.#Fn
							} | fn.#If
						} | fn.#If
					}] | fn.#If
					CustomConfig?: *{
						[string]: _
					} | fn.#Fn
					Layout?: *string | fn.#Fn
				} | fn.#If
				SMSMessage?: *{
					Body?:              *string | fn.#Fn
					EntityId?:          *string | fn.#Fn
					MessageType?:       *string | fn.#Fn
					OriginationNumber?: *string | fn.#Fn
					SenderId?:          *string | fn.#Fn
					TemplateId?:        *string | fn.#Fn
				} | fn.#If
			} | fn.#If
			Name:      *string | fn.#Fn
			Priority?: *int | fn.#Fn
			Schedule:  *{
				EndTime?:     *string | fn.#Fn
				EventFilter?: *{
					Dimensions?: *{
						Attributes?: *{
							[string]: _
						} | fn.#Fn
						EventType?: *{
							DimensionType?: *string | fn.#Fn
							Values?:        [...(*string | fn.#Fn)] | (*string | fn.#Fn)
						} | fn.#If
						Metrics?: *{
							[string]: _
						} | fn.#Fn
					} | fn.#If
					FilterType?: *string | fn.#Fn
				} | fn.#If
				Frequency?:   *string | fn.#Fn
				IsLocalTime?: *bool | fn.#Fn
				QuietTime?:   *{
					End:   *string | fn.#Fn
					Start: *string | fn.#Fn
				} | fn.#If
				StartTime?: *string | fn.#Fn
				TimeZone?:  *string | fn.#Fn
			} | fn.#If
			SegmentId:       *string | fn.#Fn
			SegmentVersion?: *int | fn.#Fn
			Tags?:           *{
				[string]: _
			} | fn.#Fn
			TreatmentDescription?: *string | fn.#Fn
			TreatmentName?:        *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#EmailChannel: {
		Type: "AWS::Pinpoint::EmailChannel"
		Properties: {
			ApplicationId:     *string | fn.#Fn
			ConfigurationSet?: *string | fn.#Fn
			Enabled?:          *bool | fn.#Fn
			FromAddress:       *string | fn.#Fn
			Identity:          *string | fn.#Fn
			RoleArn?:          *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#EmailTemplate: {
		Type: "AWS::Pinpoint::EmailTemplate"
		Properties: {
			DefaultSubstitutions?: *string | fn.#Fn
			HtmlPart?:             *string | fn.#Fn
			Subject:               *string | fn.#Fn
			Tags?:                 *{
				[string]: _
			} | fn.#Fn
			TemplateDescription?: *string | fn.#Fn
			TemplateName:         *string | fn.#Fn
			TextPart?:            *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#EventStream: {
		Type: "AWS::Pinpoint::EventStream"
		Properties: {
			ApplicationId:        *string | fn.#Fn
			DestinationStreamArn: *string | fn.#Fn
			RoleArn:              *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#GCMChannel: {
		Type: "AWS::Pinpoint::GCMChannel"
		Properties: {
			ApiKey:        *string | fn.#Fn
			ApplicationId: *string | fn.#Fn
			Enabled?:      *bool | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#InAppTemplate: {
		Type: "AWS::Pinpoint::InAppTemplate"
		Properties: {
			Content?: *[...{
				BackgroundColor?: *string | fn.#Fn
				BodyConfig?:      *{
					Alignment?: *("LEFT" | "CENTER" | "RIGHT") | fn.#Fn
					Body?:      *string | fn.#Fn
					TextColor?: *string | fn.#Fn
				} | fn.#If
				HeaderConfig?: *{
					Alignment?: *("LEFT" | "CENTER" | "RIGHT") | fn.#Fn
					Header?:    *string | fn.#Fn
					TextColor?: *string | fn.#Fn
				} | fn.#If
				ImageUrl?:   *string | fn.#Fn
				PrimaryBtn?: *{
					Android?: *{
						ButtonAction?: *("LINK" | "DEEP_LINK" | "CLOSE") | fn.#Fn
						Link?:         *string | fn.#Fn
					} | fn.#If
					DefaultConfig?: *{
						BackgroundColor?: *string | fn.#Fn
						BorderRadius?:    *int | fn.#Fn
						ButtonAction?:    *("LINK" | "DEEP_LINK" | "CLOSE") | fn.#Fn
						Link?:            *string | fn.#Fn
						Text?:            *string | fn.#Fn
						TextColor?:       *string | fn.#Fn
					} | fn.#If
					IOS?: *{
						ButtonAction?: *("LINK" | "DEEP_LINK" | "CLOSE") | fn.#Fn
						Link?:         *string | fn.#Fn
					} | fn.#If
					Web?: *{
						ButtonAction?: *("LINK" | "DEEP_LINK" | "CLOSE") | fn.#Fn
						Link?:         *string | fn.#Fn
					} | fn.#If
				} | fn.#If
				SecondaryBtn?: *{
					Android?: *{
						ButtonAction?: *("LINK" | "DEEP_LINK" | "CLOSE") | fn.#Fn
						Link?:         *string | fn.#Fn
					} | fn.#If
					DefaultConfig?: *{
						BackgroundColor?: *string | fn.#Fn
						BorderRadius?:    *int | fn.#Fn
						ButtonAction?:    *("LINK" | "DEEP_LINK" | "CLOSE") | fn.#Fn
						Link?:            *string | fn.#Fn
						Text?:            *string | fn.#Fn
						TextColor?:       *string | fn.#Fn
					} | fn.#If
					IOS?: *{
						ButtonAction?: *("LINK" | "DEEP_LINK" | "CLOSE") | fn.#Fn
						Link?:         *string | fn.#Fn
					} | fn.#If
					Web?: *{
						ButtonAction?: *("LINK" | "DEEP_LINK" | "CLOSE") | fn.#Fn
						Link?:         *string | fn.#Fn
					} | fn.#If
				} | fn.#If
			}] | fn.#If
			CustomConfig?: *{
				[string]: _
			} | fn.#Fn
			Layout?: *("BOTTOM_BANNER" | "TOP_BANNER" | "OVERLAYS" | "MOBILE_FEED" | "MIDDLE_BANNER" | "CAROUSEL") | fn.#Fn
			Tags?:   *{
				[string]: _
			} | fn.#Fn
			TemplateDescription?: *string | fn.#Fn
			TemplateName:         *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#PushTemplate: {
		Type: "AWS::Pinpoint::PushTemplate"
		Properties: {
			ADM?: *{
				Action?:            *string | fn.#Fn
				Body?:              *string | fn.#Fn
				ImageIconUrl?:      *string | fn.#Fn
				ImageUrl?:          *string | fn.#Fn
				SmallImageIconUrl?: *string | fn.#Fn
				Sound?:             *string | fn.#Fn
				Title?:             *string | fn.#Fn
				Url?:               *string | fn.#Fn
			} | fn.#If
			APNS?: *{
				Action?:   *string | fn.#Fn
				Body?:     *string | fn.#Fn
				MediaUrl?: *string | fn.#Fn
				Sound?:    *string | fn.#Fn
				Title?:    *string | fn.#Fn
				Url?:      *string | fn.#Fn
			} | fn.#If
			Baidu?: *{
				Action?:            *string | fn.#Fn
				Body?:              *string | fn.#Fn
				ImageIconUrl?:      *string | fn.#Fn
				ImageUrl?:          *string | fn.#Fn
				SmallImageIconUrl?: *string | fn.#Fn
				Sound?:             *string | fn.#Fn
				Title?:             *string | fn.#Fn
				Url?:               *string | fn.#Fn
			} | fn.#If
			Default?: *{
				Action?: *string | fn.#Fn
				Body?:   *string | fn.#Fn
				Sound?:  *string | fn.#Fn
				Title?:  *string | fn.#Fn
				Url?:    *string | fn.#Fn
			} | fn.#If
			DefaultSubstitutions?: *string | fn.#Fn
			GCM?:                  *{
				Action?:            *string | fn.#Fn
				Body?:              *string | fn.#Fn
				ImageIconUrl?:      *string | fn.#Fn
				ImageUrl?:          *string | fn.#Fn
				SmallImageIconUrl?: *string | fn.#Fn
				Sound?:             *string | fn.#Fn
				Title?:             *string | fn.#Fn
				Url?:               *string | fn.#Fn
			} | fn.#If
			Tags?: *{
				[string]: _
			} | fn.#Fn
			TemplateDescription?: *string | fn.#Fn
			TemplateName:         *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#SMSChannel: {
		Type: "AWS::Pinpoint::SMSChannel"
		Properties: {
			ApplicationId: *string | fn.#Fn
			Enabled?:      *bool | fn.#Fn
			SenderId?:     *string | fn.#Fn
			ShortCode?:    *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Segment: {
		Type: "AWS::Pinpoint::Segment"
		Properties: {
			ApplicationId: *string | fn.#Fn
			Dimensions?:   *{
				Attributes?: *{
					[string]: _
				} | fn.#Fn
				Behavior?: *{
					Recency?: *{
						Duration:    *string | fn.#Fn
						RecencyType: *string | fn.#Fn
					} | fn.#If
				} | fn.#If
				Demographic?: *{
					AppVersion?: *{
						DimensionType?: *string | fn.#Fn
						Values?:        [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					} | fn.#If
					Channel?: *{
						DimensionType?: *string | fn.#Fn
						Values?:        [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					} | fn.#If
					DeviceType?: *{
						DimensionType?: *string | fn.#Fn
						Values?:        [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					} | fn.#If
					Make?: *{
						DimensionType?: *string | fn.#Fn
						Values?:        [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					} | fn.#If
					Model?: *{
						DimensionType?: *string | fn.#Fn
						Values?:        [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					} | fn.#If
					Platform?: *{
						DimensionType?: *string | fn.#Fn
						Values?:        [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					} | fn.#If
				} | fn.#If
				Location?: *{
					Country?: *{
						DimensionType?: *string | fn.#Fn
						Values?:        [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					} | fn.#If
					GPSPoint?: *{
						Coordinates: *{
							Latitude:  *number | fn.#Fn
							Longitude: *number | fn.#Fn
						} | fn.#If
						RangeInKilometers: *number | fn.#Fn
					} | fn.#If
				} | fn.#If
				Metrics?: *{
					[string]: _
				} | fn.#Fn
				UserAttributes?: *{
					[string]: _
				} | fn.#Fn
			} | fn.#If
			Name:           *string | fn.#Fn
			SegmentGroups?: *{
				Groups?: *[...{
					Dimensions?: *[...{
						Attributes?: *{
							[string]: _
						} | fn.#Fn
						Behavior?: *{
							Recency?: *{
								Duration:    *string | fn.#Fn
								RecencyType: *string | fn.#Fn
							} | fn.#If
						} | fn.#If
						Demographic?: *{
							AppVersion?: *{
								DimensionType?: *string | fn.#Fn
								Values?:        [...(*string | fn.#Fn)] | (*string | fn.#Fn)
							} | fn.#If
							Channel?: *{
								DimensionType?: *string | fn.#Fn
								Values?:        [...(*string | fn.#Fn)] | (*string | fn.#Fn)
							} | fn.#If
							DeviceType?: *{
								DimensionType?: *string | fn.#Fn
								Values?:        [...(*string | fn.#Fn)] | (*string | fn.#Fn)
							} | fn.#If
							Make?: *{
								DimensionType?: *string | fn.#Fn
								Values?:        [...(*string | fn.#Fn)] | (*string | fn.#Fn)
							} | fn.#If
							Model?: *{
								DimensionType?: *string | fn.#Fn
								Values?:        [...(*string | fn.#Fn)] | (*string | fn.#Fn)
							} | fn.#If
							Platform?: *{
								DimensionType?: *string | fn.#Fn
								Values?:        [...(*string | fn.#Fn)] | (*string | fn.#Fn)
							} | fn.#If
						} | fn.#If
						Location?: *{
							Country?: *{
								DimensionType?: *string | fn.#Fn
								Values?:        [...(*string | fn.#Fn)] | (*string | fn.#Fn)
							} | fn.#If
							GPSPoint?: *{
								Coordinates: *{
									Latitude:  *number | fn.#Fn
									Longitude: *number | fn.#Fn
								} | fn.#If
								RangeInKilometers: *number | fn.#Fn
							} | fn.#If
						} | fn.#If
						Metrics?: *{
							[string]: _
						} | fn.#Fn
						UserAttributes?: *{
							[string]: _
						} | fn.#Fn
					}] | fn.#If
					SourceSegments?: *[...{
						Id:       *string | fn.#Fn
						Version?: *int | fn.#Fn
					}] | fn.#If
					SourceType?: *string | fn.#Fn
					Type?:       *string | fn.#Fn
				}] | fn.#If
				Include?: *string | fn.#Fn
			} | fn.#If
			Tags?: *{
				[string]: _
			} | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#SmsTemplate: {
		Type: "AWS::Pinpoint::SmsTemplate"
		Properties: {
			Body:                  *string | fn.#Fn
			DefaultSubstitutions?: *string | fn.#Fn
			Tags?:                 *{
				[string]: _
			} | fn.#Fn
			TemplateDescription?: *string | fn.#Fn
			TemplateName:         *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#VoiceChannel: {
		Type: "AWS::Pinpoint::VoiceChannel"
		Properties: {
			ApplicationId: *string | fn.#Fn
			Enabled?:      *bool | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
