package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#IoTWireless: {
	#Destination: {
		Type: "AWS::IoTWireless::Destination"
		Properties: {
			Description?:   *string | fn.#Fn
			Expression:     *string | fn.#Fn
			ExpressionType: *("RuleName" | "MqttTopic") | fn.#Fn
			Name:           *(=~#"[a-zA-Z0-9:_-]+"#) | fn.#Fn
			RoleArn:        *(strings.MinRunes(20) & strings.MaxRunes(2048)) | fn.#Fn
			Tags?:          *[...{
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
	#DeviceProfile: {
		Type: "AWS::IoTWireless::DeviceProfile"
		Properties: {
			LoRaWAN?: *{
				ClassBTimeout?:     *int | fn.#Fn
				ClassCTimeout?:     *int | fn.#Fn
				MacVersion?:        *string | fn.#Fn
				MaxDutyCycle?:      *int | fn.#Fn
				MaxEirp?:           *int | fn.#Fn
				PingSlotDr?:        *int | fn.#Fn
				PingSlotFreq?:      *(>=1000000 & <=16700000) | fn.#Fn
				PingSlotPeriod?:    *(>=128 & <=4096) | fn.#Fn
				RegParamsRevision?: *string | fn.#Fn
				RfRegion?:          *string | fn.#Fn
				Supports32BitFCnt?: *bool | fn.#Fn
				SupportsClassB?:    *bool | fn.#Fn
				SupportsClassC?:    *bool | fn.#Fn
				SupportsJoin?:      *bool | fn.#Fn
			} | fn.#If
			Name?: *string | fn.#Fn
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
	#FuotaTask: {
		Type: "AWS::IoTWireless::FuotaTask"
		Properties: {
			AssociateMulticastGroup?:    *string | fn.#Fn
			AssociateWirelessDevice?:    *string | fn.#Fn
			Description?:                *string | fn.#Fn
			DisassociateMulticastGroup?: *string | fn.#Fn
			DisassociateWirelessDevice?: *string | fn.#Fn
			FirmwareUpdateImage:         *(strings.MinRunes(1) & strings.MaxRunes(2048)) | fn.#Fn
			FirmwareUpdateRole:          *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
			LoRaWAN:                     *{
				RfRegion:   *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
				StartTime?: *string | fn.#Fn
			} | fn.#If
			Name?: *string | fn.#Fn
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
	#MulticastGroup: {
		Type: "AWS::IoTWireless::MulticastGroup"
		Properties: {
			AssociateWirelessDevice?:    *string | fn.#Fn
			Description?:                *string | fn.#Fn
			DisassociateWirelessDevice?: *string | fn.#Fn
			LoRaWAN:                     *{
				DlClass:                   *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
				NumberOfDevicesInGroup?:   *int | fn.#Fn
				NumberOfDevicesRequested?: *int | fn.#Fn
				RfRegion:                  *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
			} | fn.#If
			Name?: *string | fn.#Fn
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
	#PartnerAccount: {
		Type: "AWS::IoTWireless::PartnerAccount"
		Properties: {
			AccountLinked?:    *bool | fn.#Fn
			Fingerprint?:      *(=~#"[a-fA-F0-9]{64}"#) | fn.#Fn
			PartnerAccountId?: *string | fn.#Fn
			PartnerType?:      *("Sidewalk") | fn.#Fn
			Sidewalk?:         *{
				AppServerPrivateKey: *(strings.MinRunes(1) & strings.MaxRunes(4096) & (=~#"[a-fA-F0-9]{64}"#)) | fn.#Fn
			} | fn.#If
			SidewalkUpdate?: *{
				AppServerPrivateKey?: *(strings.MinRunes(1) & strings.MaxRunes(4096) & (=~#"[a-fA-F0-9]{64}"#)) | fn.#Fn
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
	#ServiceProfile: {
		Type: "AWS::IoTWireless::ServiceProfile"
		Properties: {
			LoRaWAN?: *{
				AddGwMetadata?:          *bool | fn.#Fn
				ChannelMask?:            *string | fn.#Fn
				DevStatusReqFreq?:       *int | fn.#Fn
				DlBucketSize?:           *int | fn.#Fn
				DlRate?:                 *int | fn.#Fn
				DlRatePolicy?:           *string | fn.#Fn
				DrMax?:                  *int | fn.#Fn
				DrMin?:                  *int | fn.#Fn
				HrAllowed?:              *bool | fn.#Fn
				MinGwDiversity?:         *int | fn.#Fn
				NwkGeoLoc?:              *bool | fn.#Fn
				PrAllowed?:              *bool | fn.#Fn
				RaAllowed?:              *bool | fn.#Fn
				ReportDevStatusBattery?: *bool | fn.#Fn
				ReportDevStatusMargin?:  *bool | fn.#Fn
				TargetPer?:              *int | fn.#Fn
				UlBucketSize?:           *int | fn.#Fn
				UlRate?:                 *int | fn.#Fn
				UlRatePolicy?:           *string | fn.#Fn
			} | fn.#If
			Name?: *string | fn.#Fn
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
	#TaskDefinition: {
		Type: "AWS::IoTWireless::TaskDefinition"
		Properties: {
			AutoCreateTasks:                *bool | fn.#Fn
			LoRaWANUpdateGatewayTaskEntry?: *{
				CurrentVersion?: *{
					Model?:          *(strings.MinRunes(1) & strings.MaxRunes(4096)) | fn.#Fn
					PackageVersion?: *(strings.MinRunes(1) & strings.MaxRunes(32)) | fn.#Fn
					Station?:        *(strings.MinRunes(1) & strings.MaxRunes(4096)) | fn.#Fn
				} | fn.#If
				UpdateVersion?: *{
					Model?:          *(strings.MinRunes(1) & strings.MaxRunes(4096)) | fn.#Fn
					PackageVersion?: *(strings.MinRunes(1) & strings.MaxRunes(32)) | fn.#Fn
					Station?:        *(strings.MinRunes(1) & strings.MaxRunes(4096)) | fn.#Fn
				} | fn.#If
			} | fn.#If
			Name?: *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
			Tags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			TaskDefinitionType?: *("UPDATE") | fn.#Fn
			Update?:             *{
				LoRaWAN?: *{
					CurrentVersion?: *{
						Model?:          *(strings.MinRunes(1) & strings.MaxRunes(4096)) | fn.#Fn
						PackageVersion?: *(strings.MinRunes(1) & strings.MaxRunes(32)) | fn.#Fn
						Station?:        *(strings.MinRunes(1) & strings.MaxRunes(4096)) | fn.#Fn
					} | fn.#If
					SigKeyCrc?:       *int | fn.#Fn
					UpdateSignature?: *(strings.MinRunes(1) & strings.MaxRunes(4096)) | fn.#Fn
					UpdateVersion?:   *{
						Model?:          *(strings.MinRunes(1) & strings.MaxRunes(4096)) | fn.#Fn
						PackageVersion?: *(strings.MinRunes(1) & strings.MaxRunes(32)) | fn.#Fn
						Station?:        *(strings.MinRunes(1) & strings.MaxRunes(4096)) | fn.#Fn
					} | fn.#If
				} | fn.#If
				UpdateDataRole?:   *(strings.MinRunes(1) & strings.MaxRunes(2048)) | fn.#Fn
				UpdateDataSource?: *(strings.MinRunes(1) & strings.MaxRunes(4096)) | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#WirelessDevice: {
		Type: "AWS::IoTWireless::WirelessDevice"
		Properties: {
			Description?:          *string | fn.#Fn
			DestinationName:       *string | fn.#Fn
			LastUplinkReceivedAt?: *(=~#"^^([\+-]?\d{4}(?!\d{2}\b))((-?)((0[1-9]|1[0-2])(\3([12]\d|0[1-9]|3[01]))?|W([0-4]\d|5[0-2])(-?[1-7])?|(00[1-9]|0[1-9]\d|[12]\d{2}|3([0-5]\d|6[1-6])))([T\s]((([01]\d|2[0-3])((:?)[0-5]\d)?|24\:?00)([\.,]\d+(?!:))?)?(\17[0-5]\d([\.,]\d+)?)?([zZ]|([\+-])([01]\d|2[0-3]):?([0-5]\d)?)?)?)?$"#) | fn.#Fn
			LoRaWAN?:              *{
				AbpV10x?: *{
					DevAddr:     *(=~#"[a-fA-F0-9]{8}"#) | fn.#Fn
					SessionKeys: *{
						AppSKey: *(=~#"[a-fA-F0-9]{32}"#) | fn.#Fn
						NwkSKey: *(=~#"[a-fA-F0-9]{32}"#) | fn.#Fn
					} | fn.#If
				} | fn.#If
				AbpV11?: *{
					DevAddr:     *(=~#"[a-fA-F0-9]{8}"#) | fn.#Fn
					SessionKeys: *{
						AppSKey:     *(=~#"[a-fA-F0-9]{32}"#) | fn.#Fn
						FNwkSIntKey: *(=~#"[a-fA-F0-9]{32}"#) | fn.#Fn
						NwkSEncKey:  *(=~#"[a-fA-F0-9]{32}"#) | fn.#Fn
						SNwkSIntKey: *(=~#"[a-fA-F0-9]{32}"#) | fn.#Fn
					} | fn.#If
				} | fn.#If
				DevEui?:          *(=~#"[a-f0-9]{16}"#) | fn.#Fn
				DeviceProfileId?: *string | fn.#Fn
				OtaaV10x?:        *{
					AppEui: *(=~#"[a-fA-F0-9]{16}"#) | fn.#Fn
					AppKey: *(=~#"[a-fA-F0-9]{32}"#) | fn.#Fn
				} | fn.#If
				OtaaV11?: *{
					AppKey:  *(=~#"[a-fA-F0-9]{32}"#) | fn.#Fn
					JoinEui: *(=~#"[a-fA-F0-9]{16}"#) | fn.#Fn
					NwkKey:  *(=~#"[a-fA-F0-9]{32}"#) | fn.#Fn
				} | fn.#If
				ServiceProfileId?: *string | fn.#Fn
			} | fn.#If
			Name?: *string | fn.#Fn
			Tags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			ThingArn?: *string | fn.#Fn
			Type:      *("Sidewalk" | "LoRaWAN") | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#WirelessGateway: {
		Type: "AWS::IoTWireless::WirelessGateway"
		Properties: {
			Description?:          *string | fn.#Fn
			LastUplinkReceivedAt?: *(=~#"^^([\+-]?\d{4}(?!\d{2}\b))((-?)((0[1-9]|1[0-2])(\3([12]\d|0[1-9]|3[01]))?|W([0-4]\d|5[0-2])(-?[1-7])?|(00[1-9]|0[1-9]\d|[12]\d{2}|3([0-5]\d|6[1-6])))([T\s]((([01]\d|2[0-3])((:?)[0-5]\d)?|24\:?00)([\.,]\d+(?!:))?)?(\17[0-5]\d([\.,]\d+)?)?([zZ]|([\+-])([01]\d|2[0-3]):?([0-5]\d)?)?)?)?$"#) | fn.#Fn
			LoRaWAN:               *{
				GatewayEui: *(=~#"^(([0-9A-Fa-f]{2}-){7}|([0-9A-Fa-f]{2}:){7}|([0-9A-Fa-f]{2}\s){7}|([0-9A-Fa-f]{2}){7})([0-9A-Fa-f]{2})$"#) | fn.#Fn
				RfRegion:   *string | fn.#Fn
			} | fn.#If
			Name?: *string | fn.#Fn
			Tags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			ThingArn?: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
