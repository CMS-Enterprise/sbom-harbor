package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#IoTSiteWise: {
	#AccessPolicy: {
		Type: "AWS::IoTSiteWise::AccessPolicy"
		Properties: {
			AccessPolicyIdentity: *{
				IamRole?: *{
					arn?: *string | fn.#Fn
				} | fn.#If
				IamUser?: *{
					arn?: *string | fn.#Fn
				} | fn.#If
				User?: *{
					id?: *string | fn.#Fn
				} | fn.#If
			} | fn.#If
			AccessPolicyPermission: *string | fn.#Fn
			AccessPolicyResource:   *{
				Portal?: *{
					id?: *string | fn.#Fn
				} | fn.#If
				Project?: *{
					id?: *string | fn.#Fn
				} | fn.#If
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Asset: {
		Type: "AWS::IoTSiteWise::Asset"
		Properties: {
			AssetHierarchies?: *[...{
				ChildAssetId: *string | fn.#Fn
				LogicalId:    *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"[^\u0000-\u001F\u007F]+"#)) | fn.#Fn
			}] | fn.#If
			AssetModelId:     *string | fn.#Fn
			AssetName:        *string | fn.#Fn
			AssetProperties?: *[...{
				Alias?:             *string | fn.#Fn
				LogicalId:          *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"[^\u0000-\u001F\u007F]+"#)) | fn.#Fn
				NotificationState?: *("ENABLED" | "DISABLED") | fn.#Fn
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
	#AssetModel: {
		Type: "AWS::IoTSiteWise::AssetModel"
		Properties: {
			AssetModelCompositeModels?: *[...{
				CompositeModelProperties?: *[...{
					DataType:      *("STRING" | "INTEGER" | "DOUBLE" | "BOOLEAN" | "STRUCT") | fn.#Fn
					DataTypeSpec?: *("AWS/ALARM_STATE") | fn.#Fn
					LogicalId:     *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"[^\u0000-\u001F\u007F]+"#)) | fn.#Fn
					Name:          *string | fn.#Fn
					Type:          *{
						Attribute?: *{
							DefaultValue?: *string | fn.#Fn
						} | fn.#If
						Metric?: *{
							Expression: *string | fn.#Fn
							Variables:  *[...{
								Name:  *string | fn.#Fn
								Value: *{
									HierarchyLogicalId?: *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"[^\u0000-\u001F\u007F]+"#)) | fn.#Fn
									PropertyLogicalId:   *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"[^\u0000-\u001F\u007F]+"#)) | fn.#Fn
								} | fn.#If
							}] | fn.#If
							Window: *{
								Tumbling?: *{
									Interval: *string | fn.#Fn
									Offset?:  *string | fn.#Fn
								} | fn.#If
							} | fn.#If
						} | fn.#If
						Transform?: *{
							Expression: *string | fn.#Fn
							Variables:  *[...{
								Name:  *string | fn.#Fn
								Value: *{
									HierarchyLogicalId?: *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"[^\u0000-\u001F\u007F]+"#)) | fn.#Fn
									PropertyLogicalId:   *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"[^\u0000-\u001F\u007F]+"#)) | fn.#Fn
								} | fn.#If
							}] | fn.#If
						} | fn.#If
						TypeName: *("Measurement" | "Attribute" | "Transform" | "Metric") | fn.#Fn
					} | fn.#If
					Unit?: *string | fn.#Fn
				}] | fn.#If
				Description?: *string | fn.#Fn
				Name:         *string | fn.#Fn
				Type:         *string | fn.#Fn
			}] | fn.#If
			AssetModelDescription?: *string | fn.#Fn
			AssetModelHierarchies?: *[...{
				ChildAssetModelId: *string | fn.#Fn
				LogicalId:         *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"[^\u0000-\u001F\u007F]+"#)) | fn.#Fn
				Name:              *string | fn.#Fn
			}] | fn.#If
			AssetModelName:        *string | fn.#Fn
			AssetModelProperties?: *[...{
				DataType:      *("STRING" | "INTEGER" | "DOUBLE" | "BOOLEAN" | "STRUCT") | fn.#Fn
				DataTypeSpec?: *("AWS/ALARM_STATE") | fn.#Fn
				LogicalId:     *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"[^\u0000-\u001F\u007F]+"#)) | fn.#Fn
				Name:          *string | fn.#Fn
				Type:          *{
					Attribute?: *{
						DefaultValue?: *string | fn.#Fn
					} | fn.#If
					Metric?: *{
						Expression: *string | fn.#Fn
						Variables:  *[...{
							Name:  *string | fn.#Fn
							Value: *{
								HierarchyLogicalId?: *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"[^\u0000-\u001F\u007F]+"#)) | fn.#Fn
								PropertyLogicalId:   *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"[^\u0000-\u001F\u007F]+"#)) | fn.#Fn
							} | fn.#If
						}] | fn.#If
						Window: *{
							Tumbling?: *{
								Interval: *string | fn.#Fn
								Offset?:  *string | fn.#Fn
							} | fn.#If
						} | fn.#If
					} | fn.#If
					Transform?: *{
						Expression: *string | fn.#Fn
						Variables:  *[...{
							Name:  *string | fn.#Fn
							Value: *{
								HierarchyLogicalId?: *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"[^\u0000-\u001F\u007F]+"#)) | fn.#Fn
								PropertyLogicalId:   *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"[^\u0000-\u001F\u007F]+"#)) | fn.#Fn
							} | fn.#If
						}] | fn.#If
					} | fn.#If
					TypeName: *("Measurement" | "Attribute" | "Transform" | "Metric") | fn.#Fn
				} | fn.#If
				Unit?: *string | fn.#Fn
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
	#Dashboard: {
		Type: "AWS::IoTSiteWise::Dashboard"
		Properties: {
			DashboardDefinition:  *string | fn.#Fn
			DashboardDescription: *string | fn.#Fn
			DashboardName:        *string | fn.#Fn
			ProjectId?:           *string | fn.#Fn
			Tags?:                *[...{
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
	#Gateway: {
		Type: "AWS::IoTSiteWise::Gateway"
		Properties: {
			GatewayCapabilitySummaries?: *[...{
				CapabilityConfiguration?: *string | fn.#Fn
				CapabilityNamespace:      *string | fn.#Fn
			}] | fn.#If
			GatewayName:     *string | fn.#Fn
			GatewayPlatform: *{
				Greengrass: *{
					GroupArn: *string | fn.#Fn
				} | fn.#If
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
	#Portal: {
		Type: "AWS::IoTSiteWise::Portal"
		Properties: {
			Alarms?: *{
				[string]: _
			} | fn.#Fn
			NotificationSenderEmail?: *string | fn.#Fn
			PortalAuthMode?:          *string | fn.#Fn
			PortalContactEmail:       *string | fn.#Fn
			PortalDescription?:       *string | fn.#Fn
			PortalName:               *string | fn.#Fn
			RoleArn:                  *string | fn.#Fn
			Tags?:                    *[...{
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
	#Project: {
		Type: "AWS::IoTSiteWise::Project"
		Properties: {
			AssetIds?:           [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			PortalId:            *string | fn.#Fn
			ProjectDescription?: *string | fn.#Fn
			ProjectName:         *string | fn.#Fn
			Tags?:               *[...{
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
