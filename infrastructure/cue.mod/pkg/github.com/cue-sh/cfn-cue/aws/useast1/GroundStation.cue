package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#GroundStation: {
	#Config: {
		Type: "AWS::GroundStation::Config"
		Properties: {
			ConfigData: *{
				AntennaDownlinkConfig?: *{
					SpectrumConfig?: *{
						Bandwidth?: *{
							Units?: *("GHz" | "MHz" | "kHz") | fn.#Fn
							Value?: *number | fn.#Fn
						} | fn.#If
						CenterFrequency?: *{
							Units?: *("GHz" | "MHz" | "kHz") | fn.#Fn
							Value?: *number | fn.#Fn
						} | fn.#If
						Polarization?: *("LEFT_HAND" | "RIGHT_HAND" | "NONE") | fn.#Fn
					} | fn.#If
				} | fn.#If
				AntennaDownlinkDemodDecodeConfig?: *{
					DecodeConfig?: *{
						UnvalidatedJSON?: *(=~#"^[{}\[\]:.,"0-9A-z\-_\s]{1,8192}$"#) | fn.#Fn
					} | fn.#If
					DemodulationConfig?: *{
						UnvalidatedJSON?: *(=~#"^[{}\[\]:.,"0-9A-z\-_\s]{1,8192}$"#) | fn.#Fn
					} | fn.#If
					SpectrumConfig?: *{
						Bandwidth?: *{
							Units?: *("GHz" | "MHz" | "kHz") | fn.#Fn
							Value?: *number | fn.#Fn
						} | fn.#If
						CenterFrequency?: *{
							Units?: *("GHz" | "MHz" | "kHz") | fn.#Fn
							Value?: *number | fn.#Fn
						} | fn.#If
						Polarization?: *("LEFT_HAND" | "RIGHT_HAND" | "NONE") | fn.#Fn
					} | fn.#If
				} | fn.#If
				AntennaUplinkConfig?: *{
					SpectrumConfig?: *{
						CenterFrequency?: *{
							Units?: *("GHz" | "MHz" | "kHz") | fn.#Fn
							Value?: *number | fn.#Fn
						} | fn.#If
						Polarization?: *("LEFT_HAND" | "RIGHT_HAND" | "NONE") | fn.#Fn
					} | fn.#If
					TargetEirp?: *{
						Units?: *("dBW") | fn.#Fn
						Value?: *number | fn.#Fn
					} | fn.#If
					TransmitDisabled?: *bool | fn.#Fn
				} | fn.#If
				DataflowEndpointConfig?: *{
					DataflowEndpointName?:   *string | fn.#Fn
					DataflowEndpointRegion?: *string | fn.#Fn
				} | fn.#If
				S3RecordingConfig?: *{
					BucketArn?: *string | fn.#Fn
					Prefix?:    *(=~#"^([a-zA-Z0-9_\-=/]|\{satellite_id\}|\{config\-name}|\{s3\-config-id}|\{year\}|\{month\}|\{day\}){1,900}$"#) | fn.#Fn
					RoleArn?:   *string | fn.#Fn
				} | fn.#If
				TrackingConfig?: *{
					Autotrack?: *("REQUIRED" | "PREFERRED" | "REMOVED") | fn.#Fn
				} | fn.#If
				UplinkEchoConfig?: *{
					AntennaUplinkConfigArn?: *string | fn.#Fn
					Enabled?:                *bool | fn.#Fn
				} | fn.#If
			} | fn.#If
			Name:  *(=~#"^[ a-zA-Z0-9_:-]{1,256}$"#) | fn.#Fn
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
	#DataflowEndpointGroup: {
		Type: "AWS::GroundStation::DataflowEndpointGroup"
		Properties: {
			EndpointDetails: *[...{
				Endpoint?: *{
					Address?: *{
						Name?: *string | fn.#Fn
						Port?: *int | fn.#Fn
					} | fn.#If
					Mtu?:  *int | fn.#Fn
					Name?: *(=~#"^[ a-zA-Z0-9_:-]{1,256}$"#) | fn.#Fn
				} | fn.#If
				SecurityDetails?: *{
					RoleArn?:          *string | fn.#Fn
					SecurityGroupIds?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					SubnetIds?:        [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				} | fn.#If
			}] | fn.#If
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
	#MissionProfile: {
		Type: "AWS::GroundStation::MissionProfile"
		Properties: {
			ContactPostPassDurationSeconds?: *int | fn.#Fn
			ContactPrePassDurationSeconds?:  *int | fn.#Fn
			DataflowEdges:                   *[...{
				Destination?: *string | fn.#Fn
				Source?:      *string | fn.#Fn
			}] | fn.#If
			MinimumViableContactDurationSeconds: *int | fn.#Fn
			Name:                                *(=~#"^[ a-zA-Z0-9_:-]{1,256}$"#) | fn.#Fn
			Tags?:                               *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			TrackingConfigArn: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
