package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#QuickSight: {
	#Analysis: {
		Type: "AWS::QuickSight::Analysis"
		Properties: {
			AnalysisId:   *(strings.MinRunes(1) & strings.MaxRunes(2048) & (=~#"[\w\-]+"#)) | fn.#Fn
			AwsAccountId: *(strings.MinRunes(12) & strings.MaxRunes(12) & (=~#"^[0-9]{12}$"#)) | fn.#Fn
			Errors?:      *[...{
				Message?: *(=~#".*\S.*"#) | fn.#Fn
				Type?:    *("ACCESS_DENIED" | "SOURCE_NOT_FOUND" | "DATA_SET_NOT_FOUND" | "INTERNAL_FAILURE" | "PARAMETER_VALUE_INCOMPATIBLE" | "PARAMETER_TYPE_INVALID" | "PARAMETER_NOT_FOUND" | "COLUMN_TYPE_MISMATCH" | "COLUMN_GEOGRAPHIC_ROLE_MISMATCH" | "COLUMN_REPLACEMENT_MISSING") | fn.#Fn
			}] | fn.#If
			Name?:       *(strings.MinRunes(1) & strings.MaxRunes(2048) & (=~#"[\u0020-\u00FF]+"#)) | fn.#Fn
			Parameters?: *{
				DateTimeParameters?: *[...{
					Name:   *(=~#".*\S.*"#) | fn.#Fn
					Values: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				}] | fn.#If
				DecimalParameters?: *[...{
					Name:   *(=~#".*\S.*"#) | fn.#Fn
					Values: [...(*number | fn.#Fn)] | (*number | fn.#Fn)
				}] | fn.#If
				IntegerParameters?: *[...{
					Name:   *(=~#".*\S.*"#) | fn.#Fn
					Values: [...(*number | fn.#Fn)] | (*number | fn.#Fn)
				}] | fn.#If
				StringParameters?: *[...{
					Name:   *(=~#".*\S.*"#) | fn.#Fn
					Values: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				}] | fn.#If
			} | fn.#If
			Permissions?: *[...{
				Actions:   [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				Principal: *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
			}] | fn.#If
			SourceEntity: *{
				SourceTemplate?: *{
					Arn:               *string | fn.#Fn
					DataSetReferences: *[...{
						DataSetArn:         *string | fn.#Fn
						DataSetPlaceholder: *(=~#".*\S.*"#) | fn.#Fn
					}] | fn.#If
				} | fn.#If
			} | fn.#If
			Tags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			ThemeArn?: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Dashboard: {
		Type: "AWS::QuickSight::Dashboard"
		Properties: {
			AwsAccountId:             *(strings.MinRunes(12) & strings.MaxRunes(12) & (=~#"^[0-9]{12}$"#)) | fn.#Fn
			DashboardId:              *(strings.MinRunes(1) & strings.MaxRunes(2048) & (=~#"[\w\-]+"#)) | fn.#Fn
			DashboardPublishOptions?: *{
				AdHocFilteringOption?: *{
					AvailabilityStatus?: *("ENABLED" | "DISABLED") | fn.#Fn
				} | fn.#If
				ExportToCSVOption?: *{
					AvailabilityStatus?: *("ENABLED" | "DISABLED") | fn.#Fn
				} | fn.#If
				SheetControlsOption?: *{
					VisibilityState?: *("EXPANDED" | "COLLAPSED") | fn.#Fn
				} | fn.#If
			} | fn.#If
			Name?:       *(strings.MinRunes(1) & strings.MaxRunes(2048) & (=~#"[\u0020-\u00FF]+"#)) | fn.#Fn
			Parameters?: *{
				DateTimeParameters?: *[...{
					Name:   *(=~#".*\S.*"#) | fn.#Fn
					Values: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				}] | fn.#If
				DecimalParameters?: *[...{
					Name:   *(=~#".*\S.*"#) | fn.#Fn
					Values: [...(*number | fn.#Fn)] | (*number | fn.#Fn)
				}] | fn.#If
				IntegerParameters?: *[...{
					Name:   *(=~#".*\S.*"#) | fn.#Fn
					Values: [...(*number | fn.#Fn)] | (*number | fn.#Fn)
				}] | fn.#If
				StringParameters?: *[...{
					Name:   *(=~#".*\S.*"#) | fn.#Fn
					Values: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				}] | fn.#If
			} | fn.#If
			Permissions?: *[...{
				Actions:   [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				Principal: *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
			}] | fn.#If
			SourceEntity: *{
				SourceTemplate?: *{
					Arn:               *string | fn.#Fn
					DataSetReferences: *[...{
						DataSetArn:         *string | fn.#Fn
						DataSetPlaceholder: *(=~#".*\S.*"#) | fn.#Fn
					}] | fn.#If
				} | fn.#If
			} | fn.#If
			Tags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			ThemeArn?:           *string | fn.#Fn
			VersionDescription?: *(strings.MinRunes(1) & strings.MaxRunes(512)) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#DataSet: {
		Type: "AWS::QuickSight::DataSet"
		Properties: {
			AwsAccountId?: *(strings.MinRunes(12) & strings.MaxRunes(12) & (=~#"^[0-9]{12}$"#)) | fn.#Fn
			ColumnGroups?: *[...{
				GeoSpatialColumnGroup?: *{
					Columns:      [...(*(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn)
					CountryCode?: *("US") | fn.#Fn
					Name:         *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
				} | fn.#If
			}] | fn.#If
			ColumnLevelPermissionRules?: *[...{
				ColumnNames?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				Principals?:  [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			}] | fn.#If
			DataSetId?:    *string | fn.#Fn
			FieldFolders?: *{
				[string]: {
					Columns?:     [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					Description?: *string | fn.#Fn
				}
			} | fn.#If
			ImportMode?:          *("SPICE" | "DIRECT_QUERY") | fn.#Fn
			IngestionWaitPolicy?: *{
				IngestionWaitTimeInHours?: *(>=1 & <=36) | fn.#Fn
				WaitForSpiceIngestion?:    *bool | fn.#Fn
			} | fn.#If
			LogicalTableMap?: *{
				[string]: {
					Alias:           *string | fn.#Fn
					DataTransforms?: *[...{
						CastColumnTypeOperation?: *{
							ColumnName:    *string | fn.#Fn
							Format?:       *string | fn.#Fn
							NewColumnType: *string | fn.#Fn
						} | fn.#If
						CreateColumnsOperation?: *{
							Columns: *[...{
								ColumnId:   *string | fn.#Fn
								ColumnName: *string | fn.#Fn
								Expression: *string | fn.#Fn
							}] | fn.#If
						} | fn.#If
						FilterOperation?: *{
							ConditionExpression: *string | fn.#Fn
						} | fn.#If
						ProjectOperation?: *{
							ProjectedColumns: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
						} | fn.#If
						RenameColumnOperation?: *{
							ColumnName:    *string | fn.#Fn
							NewColumnName: *string | fn.#Fn
						} | fn.#If
						TagColumnOperation?: *{
							ColumnName: *string | fn.#Fn
							Tags:       *[...{
								ColumnDescription?: *{
									Text?: *string | fn.#Fn
								} | fn.#If
								ColumnGeographicRole?: *string | fn.#Fn
							}] | fn.#If
						} | fn.#If
					}] | fn.#If
					Source: *{
						JoinInstruction?: *{
							LeftJoinKeyProperties?: *{
								UniqueKey?: *bool | fn.#Fn
							} | fn.#If
							LeftOperand:             *string | fn.#Fn
							OnClause:                *string | fn.#Fn
							RightJoinKeyProperties?: *{
								UniqueKey?: *bool | fn.#Fn
							} | fn.#If
							RightOperand: *string | fn.#Fn
							Type:         *string | fn.#Fn
						} | fn.#If
						PhysicalTableId?: *string | fn.#Fn
					} | fn.#If
				}
			} | fn.#If
			Name?:        *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
			Permissions?: *[...{
				Actions:   [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				Principal: *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
			}] | fn.#If
			PhysicalTableMap?: *{
				[string]: {
					CustomSql?: *{
						Columns: *[...{
							Name: *string | fn.#Fn
							Type: *string | fn.#Fn
						}] | fn.#If
						DataSourceArn: *string | fn.#Fn
						Name:          *string | fn.#Fn
						SqlQuery:      *string | fn.#Fn
					} | fn.#If
					RelationalTable?: *{
						Catalog?:      *string | fn.#Fn
						DataSourceArn: *string | fn.#Fn
						InputColumns:  *[...{
							Name: *string | fn.#Fn
							Type: *string | fn.#Fn
						}] | fn.#If
						Name:    *string | fn.#Fn
						Schema?: *string | fn.#Fn
					} | fn.#If
					S3Source?: *{
						DataSourceArn: *string | fn.#Fn
						InputColumns:  *[...{
							Name: *string | fn.#Fn
							Type: *string | fn.#Fn
						}] | fn.#If
						UploadSettings?: *{
							ContainsHeader?: *bool | fn.#Fn
							Delimiter?:      *string | fn.#Fn
							Format?:         *string | fn.#Fn
							StartFromRow?:   *number | fn.#Fn
							TextQualifier?:  *string | fn.#Fn
						} | fn.#If
					} | fn.#If
				}
			} | fn.#If
			RowLevelPermissionDataSet?: *{
				Arn:              *string | fn.#Fn
				FormatVersion?:   *("VERSION_1" | "VERSION_2") | fn.#Fn
				Namespace?:       *(=~#"^[a-zA-Z0-9._-]*$"#) | fn.#Fn
				PermissionPolicy: *("GRANT_ACCESS" | "DENY_ACCESS") | fn.#Fn
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
	#DataSource: {
		Type: "AWS::QuickSight::DataSource"
		Properties: {
			AlternateDataSourceParameters?: *[...{
				AmazonElasticsearchParameters?: *{
					Domain: *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
				} | fn.#If
				AmazonOpenSearchParameters?: *{
					Domain: *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
				} | fn.#If
				AthenaParameters?: *{
					WorkGroup?: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
				} | fn.#If
				AuroraParameters?: *{
					Database: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
					Host:     *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
					Port:     *(>=1 & <=65535) | fn.#Fn
				} | fn.#If
				AuroraPostgreSqlParameters?: *{
					Database: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
					Host:     *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
					Port:     *(>=1 & <=65535) | fn.#Fn
				} | fn.#If
				MariaDbParameters?: *{
					Database: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
					Host:     *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
					Port:     *(>=1 & <=65535) | fn.#Fn
				} | fn.#If
				MySqlParameters?: *{
					Database: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
					Host:     *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
					Port:     *(>=1 & <=65535) | fn.#Fn
				} | fn.#If
				OracleParameters?: *{
					Database: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
					Host:     *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
					Port:     *(>=1 & <=65535) | fn.#Fn
				} | fn.#If
				PostgreSqlParameters?: *{
					Database: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
					Host:     *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
					Port:     *(>=1 & <=65535) | fn.#Fn
				} | fn.#If
				PrestoParameters?: *{
					Catalog: *string | fn.#Fn
					Host:    *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
					Port:    *(>=1 & <=65535) | fn.#Fn
				} | fn.#If
				RdsParameters?: *{
					Database:   *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
					InstanceId: *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
				} | fn.#If
				RedshiftParameters?: *{
					ClusterId?: *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
					Database:   *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
					Host?:      *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
					Port?:      *number | fn.#Fn
				} | fn.#If
				S3Parameters?: *{
					ManifestFileLocation: *{
						Bucket: *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
						Key:    *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
					} | fn.#If
				} | fn.#If
				SnowflakeParameters?: *{
					Database:  *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
					Host:      *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
					Warehouse: *string | fn.#Fn
				} | fn.#If
				SparkParameters?: *{
					Host: *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
					Port: *(>=1 & <=65535) | fn.#Fn
				} | fn.#If
				SqlServerParameters?: *{
					Database: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
					Host:     *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
					Port:     *(>=1 & <=65535) | fn.#Fn
				} | fn.#If
				TeradataParameters?: *{
					Database: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
					Host:     *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
					Port:     *(>=1 & <=65535) | fn.#Fn
				} | fn.#If
			}] | fn.#If
			AwsAccountId?: *(strings.MinRunes(12) & strings.MaxRunes(12) & (=~#"^[0-9]{12}$"#)) | fn.#Fn
			Credentials?:  *{
				CopySourceArn?:  *(=~#"^arn:[-a-z0-9]*:quicksight:[-a-z0-9]*:[0-9]{12}:datasource/.+"#) | fn.#Fn
				CredentialPair?: *{
					AlternateDataSourceParameters?: *[...{
						AmazonElasticsearchParameters?: *{
							Domain: *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
						} | fn.#If
						AmazonOpenSearchParameters?: *{
							Domain: *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
						} | fn.#If
						AthenaParameters?: *{
							WorkGroup?: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
						} | fn.#If
						AuroraParameters?: *{
							Database: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
							Host:     *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
							Port:     *(>=1 & <=65535) | fn.#Fn
						} | fn.#If
						AuroraPostgreSqlParameters?: *{
							Database: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
							Host:     *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
							Port:     *(>=1 & <=65535) | fn.#Fn
						} | fn.#If
						MariaDbParameters?: *{
							Database: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
							Host:     *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
							Port:     *(>=1 & <=65535) | fn.#Fn
						} | fn.#If
						MySqlParameters?: *{
							Database: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
							Host:     *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
							Port:     *(>=1 & <=65535) | fn.#Fn
						} | fn.#If
						OracleParameters?: *{
							Database: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
							Host:     *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
							Port:     *(>=1 & <=65535) | fn.#Fn
						} | fn.#If
						PostgreSqlParameters?: *{
							Database: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
							Host:     *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
							Port:     *(>=1 & <=65535) | fn.#Fn
						} | fn.#If
						PrestoParameters?: *{
							Catalog: *string | fn.#Fn
							Host:    *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
							Port:    *(>=1 & <=65535) | fn.#Fn
						} | fn.#If
						RdsParameters?: *{
							Database:   *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
							InstanceId: *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
						} | fn.#If
						RedshiftParameters?: *{
							ClusterId?: *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
							Database:   *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
							Host?:      *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
							Port?:      *number | fn.#Fn
						} | fn.#If
						S3Parameters?: *{
							ManifestFileLocation: *{
								Bucket: *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
								Key:    *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
							} | fn.#If
						} | fn.#If
						SnowflakeParameters?: *{
							Database:  *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
							Host:      *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
							Warehouse: *string | fn.#Fn
						} | fn.#If
						SparkParameters?: *{
							Host: *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
							Port: *(>=1 & <=65535) | fn.#Fn
						} | fn.#If
						SqlServerParameters?: *{
							Database: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
							Host:     *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
							Port:     *(>=1 & <=65535) | fn.#Fn
						} | fn.#If
						TeradataParameters?: *{
							Database: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
							Host:     *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
							Port:     *(>=1 & <=65535) | fn.#Fn
						} | fn.#If
					}] | fn.#If
					Password: *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
					Username: *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
				} | fn.#If
			} | fn.#If
			DataSourceId?:         *string | fn.#Fn
			DataSourceParameters?: *{
				AmazonElasticsearchParameters?: *{
					Domain: *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
				} | fn.#If
				AmazonOpenSearchParameters?: *{
					Domain: *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
				} | fn.#If
				AthenaParameters?: *{
					WorkGroup?: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
				} | fn.#If
				AuroraParameters?: *{
					Database: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
					Host:     *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
					Port:     *(>=1 & <=65535) | fn.#Fn
				} | fn.#If
				AuroraPostgreSqlParameters?: *{
					Database: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
					Host:     *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
					Port:     *(>=1 & <=65535) | fn.#Fn
				} | fn.#If
				MariaDbParameters?: *{
					Database: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
					Host:     *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
					Port:     *(>=1 & <=65535) | fn.#Fn
				} | fn.#If
				MySqlParameters?: *{
					Database: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
					Host:     *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
					Port:     *(>=1 & <=65535) | fn.#Fn
				} | fn.#If
				OracleParameters?: *{
					Database: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
					Host:     *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
					Port:     *(>=1 & <=65535) | fn.#Fn
				} | fn.#If
				PostgreSqlParameters?: *{
					Database: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
					Host:     *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
					Port:     *(>=1 & <=65535) | fn.#Fn
				} | fn.#If
				PrestoParameters?: *{
					Catalog: *string | fn.#Fn
					Host:    *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
					Port:    *(>=1 & <=65535) | fn.#Fn
				} | fn.#If
				RdsParameters?: *{
					Database:   *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
					InstanceId: *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
				} | fn.#If
				RedshiftParameters?: *{
					ClusterId?: *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
					Database:   *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
					Host?:      *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
					Port?:      *number | fn.#Fn
				} | fn.#If
				S3Parameters?: *{
					ManifestFileLocation: *{
						Bucket: *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
						Key:    *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
					} | fn.#If
				} | fn.#If
				SnowflakeParameters?: *{
					Database:  *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
					Host:      *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
					Warehouse: *string | fn.#Fn
				} | fn.#If
				SparkParameters?: *{
					Host: *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
					Port: *(>=1 & <=65535) | fn.#Fn
				} | fn.#If
				SqlServerParameters?: *{
					Database: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
					Host:     *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
					Port:     *(>=1 & <=65535) | fn.#Fn
				} | fn.#If
				TeradataParameters?: *{
					Database: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
					Host:     *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
					Port:     *(>=1 & <=65535) | fn.#Fn
				} | fn.#If
			} | fn.#If
			ErrorInfo?: *{
				Message?: *string | fn.#Fn
				Type?:    *("ACCESS_DENIED" | "COPY_SOURCE_NOT_FOUND" | "TIMEOUT" | "ENGINE_VERSION_NOT_SUPPORTED" | "UNKNOWN_HOST" | "GENERIC_SQL_FAILURE" | "CONFLICT" | "UNKNOWN") | fn.#Fn
			} | fn.#If
			Name?:        *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
			Permissions?: *[...{
				Actions:   [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				Principal: *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
			}] | fn.#If
			SslProperties?: *{
				DisableSsl?: *bool | fn.#Fn
			} | fn.#If
			Tags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			Type?:                    *("ADOBE_ANALYTICS" | "AMAZON_ELASTICSEARCH" | "AMAZON_OPENSEARCH" | "ATHENA" | "AURORA" | "AURORA_POSTGRESQL" | "AWS_IOT_ANALYTICS" | "GITHUB" | "JIRA" | "MARIADB" | "MYSQL" | "ORACLE" | "POSTGRESQL" | "PRESTO" | "REDSHIFT" | "S3" | "SALESFORCE" | "SERVICENOW" | "SNOWFLAKE" | "SPARK" | "SQLSERVER" | "TERADATA" | "TWITTER" | "TIMESTREAM") | fn.#Fn
			VpcConnectionProperties?: *{
				VpcConnectionArn: *string | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Template: {
		Type: "AWS::QuickSight::Template"
		Properties: {
			AwsAccountId: *(strings.MinRunes(12) & strings.MaxRunes(12) & (=~#"^[0-9]{12}$"#)) | fn.#Fn
			Name?:        *(strings.MinRunes(1) & strings.MaxRunes(2048) & (=~#"[\u0020-\u00FF]+"#)) | fn.#Fn
			Permissions?: *[...{
				Actions:   [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				Principal: *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
			}] | fn.#If
			SourceEntity: *{
				SourceAnalysis?: *{
					Arn:               *string | fn.#Fn
					DataSetReferences: *[...{
						DataSetArn:         *string | fn.#Fn
						DataSetPlaceholder: *(=~#".*\S.*"#) | fn.#Fn
					}] | fn.#If
				} | fn.#If
				SourceTemplate?: *{
					Arn: *string | fn.#Fn
				} | fn.#If
			} | fn.#If
			Tags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			TemplateId:          *(strings.MinRunes(1) & strings.MaxRunes(2048) & (=~#"[\w\-]+"#)) | fn.#Fn
			VersionDescription?: *(strings.MinRunes(1) & strings.MaxRunes(512)) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Theme: {
		Type: "AWS::QuickSight::Theme"
		Properties: {
			AwsAccountId:   *(strings.MinRunes(12) & strings.MaxRunes(12) & (=~#"^[0-9]{12}$"#)) | fn.#Fn
			BaseThemeId?:   *(strings.MinRunes(1) & strings.MaxRunes(2048) & (=~#"[\w\-]+"#)) | fn.#Fn
			Configuration?: *{
				DataColorPalette?: *{
					Colors?:         [...(*(=~#"^#[A-F0-9]{6}$"#) | fn.#Fn)] | (*(=~#"^#[A-F0-9]{6}$"#) | fn.#Fn)
					EmptyFillColor?: *(=~#"^#[A-F0-9]{6}$"#) | fn.#Fn
					MinMaxGradient?: [...(*(=~#"^#[A-F0-9]{6}$"#) | fn.#Fn)] | (*(=~#"^#[A-F0-9]{6}$"#) | fn.#Fn)
				} | fn.#If
				Sheet?: *{
					Tile?: *{
						Border?: *{
							Show?: *bool | fn.#Fn
						} | fn.#If
					} | fn.#If
					TileLayout?: *{
						Gutter?: *{
							Show?: *bool | fn.#Fn
						} | fn.#If
						Margin?: *{
							Show?: *bool | fn.#Fn
						} | fn.#If
					} | fn.#If
				} | fn.#If
				Typography?: *{
					FontFamilies?: *[...{
						FontFamily?: *string | fn.#Fn
					}] | fn.#If
				} | fn.#If
				UIColorPalette?: *{
					Accent?:              *(=~#"^#[A-F0-9]{6}$"#) | fn.#Fn
					AccentForeground?:    *(=~#"^#[A-F0-9]{6}$"#) | fn.#Fn
					Danger?:              *(=~#"^#[A-F0-9]{6}$"#) | fn.#Fn
					DangerForeground?:    *(=~#"^#[A-F0-9]{6}$"#) | fn.#Fn
					Dimension?:           *(=~#"^#[A-F0-9]{6}$"#) | fn.#Fn
					DimensionForeground?: *(=~#"^#[A-F0-9]{6}$"#) | fn.#Fn
					Measure?:             *(=~#"^#[A-F0-9]{6}$"#) | fn.#Fn
					MeasureForeground?:   *(=~#"^#[A-F0-9]{6}$"#) | fn.#Fn
					PrimaryBackground?:   *(=~#"^#[A-F0-9]{6}$"#) | fn.#Fn
					PrimaryForeground?:   *(=~#"^#[A-F0-9]{6}$"#) | fn.#Fn
					SecondaryBackground?: *(=~#"^#[A-F0-9]{6}$"#) | fn.#Fn
					SecondaryForeground?: *(=~#"^#[A-F0-9]{6}$"#) | fn.#Fn
					Success?:             *(=~#"^#[A-F0-9]{6}$"#) | fn.#Fn
					SuccessForeground?:   *(=~#"^#[A-F0-9]{6}$"#) | fn.#Fn
					Warning?:             *(=~#"^#[A-F0-9]{6}$"#) | fn.#Fn
					WarningForeground?:   *(=~#"^#[A-F0-9]{6}$"#) | fn.#Fn
				} | fn.#If
			} | fn.#If
			Name?:        *(strings.MinRunes(1) & strings.MaxRunes(2048)) | fn.#Fn
			Permissions?: *[...{
				Actions:   [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				Principal: *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
			}] | fn.#If
			Tags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			ThemeId:             *(strings.MinRunes(1) & strings.MaxRunes(2048) & (=~#"[\w\-]+"#)) | fn.#Fn
			VersionDescription?: *(strings.MinRunes(1) & strings.MaxRunes(512)) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
