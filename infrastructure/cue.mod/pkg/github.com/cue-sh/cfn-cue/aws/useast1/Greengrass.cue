package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#Greengrass: {
	#ConnectorDefinition: {
		Type: "AWS::Greengrass::ConnectorDefinition"
		Properties: {
			InitialVersion?: *{
				Connectors: *[...{
					ConnectorArn: *string | fn.#Fn
					Id:           *string | fn.#Fn
					Parameters?:  *{
						[string]: _
					} | fn.#Fn
				}] | fn.#If
			} | fn.#If
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
	#ConnectorDefinitionVersion: {
		Type: "AWS::Greengrass::ConnectorDefinitionVersion"
		Properties: {
			ConnectorDefinitionId: *string | fn.#Fn
			Connectors:            *[...{
				ConnectorArn: *string | fn.#Fn
				Id:           *string | fn.#Fn
				Parameters?:  *{
					[string]: _
				} | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#CoreDefinition: {
		Type: "AWS::Greengrass::CoreDefinition"
		Properties: {
			InitialVersion?: *{
				Cores: *[...{
					CertificateArn: *string | fn.#Fn
					Id:             *string | fn.#Fn
					SyncShadow?:    *bool | fn.#Fn
					ThingArn:       *string | fn.#Fn
				}] | fn.#If
			} | fn.#If
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
	#CoreDefinitionVersion: {
		Type: "AWS::Greengrass::CoreDefinitionVersion"
		Properties: {
			CoreDefinitionId: *string | fn.#Fn
			Cores:            *[...{
				CertificateArn: *string | fn.#Fn
				Id:             *string | fn.#Fn
				SyncShadow?:    *bool | fn.#Fn
				ThingArn:       *string | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#DeviceDefinition: {
		Type: "AWS::Greengrass::DeviceDefinition"
		Properties: {
			InitialVersion?: *{
				Devices: *[...{
					CertificateArn: *string | fn.#Fn
					Id:             *string | fn.#Fn
					SyncShadow?:    *bool | fn.#Fn
					ThingArn:       *string | fn.#Fn
				}] | fn.#If
			} | fn.#If
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
	#DeviceDefinitionVersion: {
		Type: "AWS::Greengrass::DeviceDefinitionVersion"
		Properties: {
			DeviceDefinitionId: *string | fn.#Fn
			Devices:            *[...{
				CertificateArn: *string | fn.#Fn
				Id:             *string | fn.#Fn
				SyncShadow?:    *bool | fn.#Fn
				ThingArn:       *string | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#FunctionDefinition: {
		Type: "AWS::Greengrass::FunctionDefinition"
		Properties: {
			InitialVersion?: *{
				DefaultConfig?: *{
					Execution: *{
						IsolationMode?: *string | fn.#Fn
						RunAs?:         *{
							Gid?: *int | fn.#Fn
							Uid?: *int | fn.#Fn
						} | fn.#If
					} | fn.#If
				} | fn.#If
				Functions: *[...{
					FunctionArn:           *string | fn.#Fn
					FunctionConfiguration: *{
						EncodingType?: *string | fn.#Fn
						Environment?:  *{
							AccessSysfs?: *bool | fn.#Fn
							Execution?:   *{
								IsolationMode?: *string | fn.#Fn
								RunAs?:         *{
									Gid?: *int | fn.#Fn
									Uid?: *int | fn.#Fn
								} | fn.#If
							} | fn.#If
							ResourceAccessPolicies?: *[...{
								Permission?: *string | fn.#Fn
								ResourceId:  *string | fn.#Fn
							}] | fn.#If
							Variables?: *{
								[string]: _
							} | fn.#Fn
						} | fn.#If
						ExecArgs?:   *string | fn.#Fn
						Executable?: *string | fn.#Fn
						MemorySize?: *int | fn.#Fn
						Pinned?:     *bool | fn.#Fn
						Timeout?:    *int | fn.#Fn
					} | fn.#If
					Id: *string | fn.#Fn
				}] | fn.#If
			} | fn.#If
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
	#FunctionDefinitionVersion: {
		Type: "AWS::Greengrass::FunctionDefinitionVersion"
		Properties: {
			DefaultConfig?: *{
				Execution: *{
					IsolationMode?: *string | fn.#Fn
					RunAs?:         *{
						Gid?: *int | fn.#Fn
						Uid?: *int | fn.#Fn
					} | fn.#If
				} | fn.#If
			} | fn.#If
			FunctionDefinitionId: *string | fn.#Fn
			Functions:            *[...{
				FunctionArn:           *string | fn.#Fn
				FunctionConfiguration: *{
					EncodingType?: *string | fn.#Fn
					Environment?:  *{
						AccessSysfs?: *bool | fn.#Fn
						Execution?:   *{
							IsolationMode?: *string | fn.#Fn
							RunAs?:         *{
								Gid?: *int | fn.#Fn
								Uid?: *int | fn.#Fn
							} | fn.#If
						} | fn.#If
						ResourceAccessPolicies?: *[...{
							Permission?: *string | fn.#Fn
							ResourceId:  *string | fn.#Fn
						}] | fn.#If
						Variables?: *{
							[string]: _
						} | fn.#Fn
					} | fn.#If
					ExecArgs?:   *string | fn.#Fn
					Executable?: *string | fn.#Fn
					MemorySize?: *int | fn.#Fn
					Pinned?:     *bool | fn.#Fn
					Timeout?:    *int | fn.#Fn
				} | fn.#If
				Id: *string | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Group: {
		Type: "AWS::Greengrass::Group"
		Properties: {
			InitialVersion?: *{
				ConnectorDefinitionVersionArn?:    *string | fn.#Fn
				CoreDefinitionVersionArn?:         *string | fn.#Fn
				DeviceDefinitionVersionArn?:       *string | fn.#Fn
				FunctionDefinitionVersionArn?:     *string | fn.#Fn
				LoggerDefinitionVersionArn?:       *string | fn.#Fn
				ResourceDefinitionVersionArn?:     *string | fn.#Fn
				SubscriptionDefinitionVersionArn?: *string | fn.#Fn
			} | fn.#If
			Name:     *string | fn.#Fn
			RoleArn?: *string | fn.#Fn
			Tags?:    *{
				[string]: _
			} | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#GroupVersion: {
		Type: "AWS::Greengrass::GroupVersion"
		Properties: {
			ConnectorDefinitionVersionArn?:    *string | fn.#Fn
			CoreDefinitionVersionArn?:         *string | fn.#Fn
			DeviceDefinitionVersionArn?:       *string | fn.#Fn
			FunctionDefinitionVersionArn?:     *string | fn.#Fn
			GroupId:                           *string | fn.#Fn
			LoggerDefinitionVersionArn?:       *string | fn.#Fn
			ResourceDefinitionVersionArn?:     *string | fn.#Fn
			SubscriptionDefinitionVersionArn?: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#LoggerDefinition: {
		Type: "AWS::Greengrass::LoggerDefinition"
		Properties: {
			InitialVersion?: *{
				Loggers: *[...{
					Component: *string | fn.#Fn
					Id:        *string | fn.#Fn
					Level:     *string | fn.#Fn
					Space?:    *int | fn.#Fn
					Type:      *string | fn.#Fn
				}] | fn.#If
			} | fn.#If
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
	#LoggerDefinitionVersion: {
		Type: "AWS::Greengrass::LoggerDefinitionVersion"
		Properties: {
			LoggerDefinitionId: *string | fn.#Fn
			Loggers:            *[...{
				Component: *string | fn.#Fn
				Id:        *string | fn.#Fn
				Level:     *string | fn.#Fn
				Space?:    *int | fn.#Fn
				Type:      *string | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ResourceDefinition: {
		Type: "AWS::Greengrass::ResourceDefinition"
		Properties: {
			InitialVersion?: *{
				Resources: *[...{
					Id:                    *string | fn.#Fn
					Name:                  *string | fn.#Fn
					ResourceDataContainer: *{
						LocalDeviceResourceData?: *{
							GroupOwnerSetting?: *{
								AutoAddGroupOwner: *bool | fn.#Fn
								GroupOwner?:       *string | fn.#Fn
							} | fn.#If
							SourcePath: *string | fn.#Fn
						} | fn.#If
						LocalVolumeResourceData?: *{
							DestinationPath:    *string | fn.#Fn
							GroupOwnerSetting?: *{
								AutoAddGroupOwner: *bool | fn.#Fn
								GroupOwner?:       *string | fn.#Fn
							} | fn.#If
							SourcePath: *string | fn.#Fn
						} | fn.#If
						S3MachineLearningModelResourceData?: *{
							DestinationPath: *string | fn.#Fn
							OwnerSetting?:   *{
								GroupOwner:      *string | fn.#Fn
								GroupPermission: *string | fn.#Fn
							} | fn.#If
							S3Uri: *string | fn.#Fn
						} | fn.#If
						SageMakerMachineLearningModelResourceData?: *{
							DestinationPath: *string | fn.#Fn
							OwnerSetting?:   *{
								GroupOwner:      *string | fn.#Fn
								GroupPermission: *string | fn.#Fn
							} | fn.#If
							SageMakerJobArn: *string | fn.#Fn
						} | fn.#If
						SecretsManagerSecretResourceData?: *{
							ARN:                                *string | fn.#Fn
							AdditionalStagingLabelsToDownload?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
						} | fn.#If
					} | fn.#If
				}] | fn.#If
			} | fn.#If
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
	#ResourceDefinitionVersion: {
		Type: "AWS::Greengrass::ResourceDefinitionVersion"
		Properties: {
			ResourceDefinitionId: *string | fn.#Fn
			Resources:            *[...{
				Id:                    *string | fn.#Fn
				Name:                  *string | fn.#Fn
				ResourceDataContainer: *{
					LocalDeviceResourceData?: *{
						GroupOwnerSetting?: *{
							AutoAddGroupOwner: *bool | fn.#Fn
							GroupOwner?:       *string | fn.#Fn
						} | fn.#If
						SourcePath: *string | fn.#Fn
					} | fn.#If
					LocalVolumeResourceData?: *{
						DestinationPath:    *string | fn.#Fn
						GroupOwnerSetting?: *{
							AutoAddGroupOwner: *bool | fn.#Fn
							GroupOwner?:       *string | fn.#Fn
						} | fn.#If
						SourcePath: *string | fn.#Fn
					} | fn.#If
					S3MachineLearningModelResourceData?: *{
						DestinationPath: *string | fn.#Fn
						OwnerSetting?:   *{
							GroupOwner:      *string | fn.#Fn
							GroupPermission: *string | fn.#Fn
						} | fn.#If
						S3Uri: *string | fn.#Fn
					} | fn.#If
					SageMakerMachineLearningModelResourceData?: *{
						DestinationPath: *string | fn.#Fn
						OwnerSetting?:   *{
							GroupOwner:      *string | fn.#Fn
							GroupPermission: *string | fn.#Fn
						} | fn.#If
						SageMakerJobArn: *string | fn.#Fn
					} | fn.#If
					SecretsManagerSecretResourceData?: *{
						ARN:                                *string | fn.#Fn
						AdditionalStagingLabelsToDownload?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					} | fn.#If
				} | fn.#If
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#SubscriptionDefinition: {
		Type: "AWS::Greengrass::SubscriptionDefinition"
		Properties: {
			InitialVersion?: *{
				Subscriptions: *[...{
					Id:      *string | fn.#Fn
					Source:  *string | fn.#Fn
					Subject: *string | fn.#Fn
					Target:  *string | fn.#Fn
				}] | fn.#If
			} | fn.#If
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
	#SubscriptionDefinitionVersion: {
		Type: "AWS::Greengrass::SubscriptionDefinitionVersion"
		Properties: {
			SubscriptionDefinitionId: *string | fn.#Fn
			Subscriptions:            *[...{
				Id:      *string | fn.#Fn
				Source:  *string | fn.#Fn
				Subject: *string | fn.#Fn
				Target:  *string | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
