package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#DataBrew: {
	#Dataset: {
		Type: "AWS::DataBrew::Dataset"
		Properties: {
			Format?:        *("CSV" | "JSON" | "PARQUET" | "EXCEL") | fn.#Fn
			FormatOptions?: *{
				Csv?: *{
					Delimiter?: *(strings.MinRunes(1) & strings.MaxRunes(1)) | fn.#Fn
					HeaderRow?: *bool | fn.#Fn
				} | fn.#If
				Excel?: *{
					HeaderRow?:    *bool | fn.#Fn
					SheetIndexes?: [...(*int | fn.#Fn)] | (*int | fn.#Fn)
					SheetNames?:   [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				} | fn.#If
				Json?: *{
					MultiLine?: *bool | fn.#Fn
				} | fn.#If
			} | fn.#If
			Input: *{
				DataCatalogInputDefinition?: *{
					CatalogId?:     *string | fn.#Fn
					DatabaseName?:  *string | fn.#Fn
					TableName?:     *string | fn.#Fn
					TempDirectory?: *{
						Bucket: *string | fn.#Fn
						Key?:   *string | fn.#Fn
					} | fn.#If
				} | fn.#If
				DatabaseInputDefinition?: *{
					DatabaseTableName?: *string | fn.#Fn
					GlueConnectionName: *string | fn.#Fn
					QueryString?:       *string | fn.#Fn
					TempDirectory?:     *{
						Bucket: *string | fn.#Fn
						Key?:   *string | fn.#Fn
					} | fn.#If
				} | fn.#If
				Metadata?: *{
					SourceArn?: *string | fn.#Fn
				} | fn.#If
				S3InputDefinition?: *{
					Bucket: *string | fn.#Fn
					Key?:   *string | fn.#Fn
				} | fn.#If
			} | fn.#If
			Name:         *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
			PathOptions?: *{
				FilesLimit?: *{
					MaxFiles:   *int | fn.#Fn
					Order?:     *("ASCENDING" | "DESCENDING") | fn.#Fn
					OrderedBy?: *("LAST_MODIFIED_DATE") | fn.#Fn
				} | fn.#If
				LastModifiedDateCondition?: *{
					Expression: *(strings.MinRunes(4) & strings.MaxRunes(1024) & (=~#"^[><0-9A-Za-z_.,:)(!= ]+$"#)) | fn.#Fn
					ValuesMap:  *[...{
						Value:          *string | fn.#Fn
						ValueReference: *(strings.MinRunes(2) & strings.MaxRunes(128) & (=~#"^:[A-Za-z0-9_]+$"#)) | fn.#Fn
					}] | fn.#If
				} | fn.#If
				Parameters?: *[...{
					DatasetParameter: *{
						CreateColumn?:    *bool | fn.#Fn
						DatetimeOptions?: *{
							Format:          *(strings.MinRunes(2) & strings.MaxRunes(100)) | fn.#Fn
							LocaleCode?:     *(strings.MinRunes(2) & strings.MaxRunes(100) & (=~#"^[A-Za-z0-9_\.#@\-]+$"#)) | fn.#Fn
							TimezoneOffset?: *(strings.MinRunes(1) & strings.MaxRunes(6) & (=~#"^(Z|[-+](\d|\d{2}|\d{2}:?\d{2}))$"#)) | fn.#Fn
						} | fn.#If
						Filter?: *{
							Expression: *(strings.MinRunes(4) & strings.MaxRunes(1024) & (=~#"^[><0-9A-Za-z_.,:)(!= ]+$"#)) | fn.#Fn
							ValuesMap:  *[...{
								Value:          *string | fn.#Fn
								ValueReference: *(strings.MinRunes(2) & strings.MaxRunes(128) & (=~#"^:[A-Za-z0-9_]+$"#)) | fn.#Fn
							}] | fn.#If
						} | fn.#If
						Name: *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
						Type: *("String" | "Number" | "Datetime") | fn.#Fn
					} | fn.#If
					PathParameterName: *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
				}] | fn.#If
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
	#Job: {
		Type: "AWS::DataBrew::Job"
		Properties: {
			DataCatalogOutputs?: *[...{
				CatalogId?:       *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
				DatabaseName:     *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
				DatabaseOptions?: *{
					TableName:      *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
					TempDirectory?: *{
						Bucket: *string | fn.#Fn
						Key?:   *string | fn.#Fn
					} | fn.#If
				} | fn.#If
				Overwrite?: *bool | fn.#Fn
				S3Options?: *{
					Location: *{
						Bucket: *string | fn.#Fn
						Key?:   *string | fn.#Fn
					} | fn.#If
				} | fn.#If
				TableName: *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
			}] | fn.#If
			DatabaseOutputs?: *[...{
				DatabaseOptions: *{
					TableName:      *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
					TempDirectory?: *{
						Bucket: *string | fn.#Fn
						Key?:   *string | fn.#Fn
					} | fn.#If
				} | fn.#If
				DatabaseOutputMode?: *("NEW_TABLE") | fn.#Fn
				GlueConnectionName:  *string | fn.#Fn
			}] | fn.#If
			DatasetName?:      *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
			EncryptionKeyArn?: *(strings.MinRunes(20) & strings.MaxRunes(2048)) | fn.#Fn
			EncryptionMode?:   *("SSE-KMS" | "SSE-S3") | fn.#Fn
			JobSample?:        *{
				Mode?: *("FULL_DATASET" | "CUSTOM_ROWS") | fn.#Fn
				Size?: *int | fn.#Fn
			} | fn.#If
			LogSubscription?: *("ENABLE" | "DISABLE") | fn.#Fn
			MaxCapacity?:     *int | fn.#Fn
			MaxRetries?:      *int | fn.#Fn
			Name:             *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
			OutputLocation?:  *{
				Bucket: *string | fn.#Fn
				Key?:   *string | fn.#Fn
			} | fn.#If
			Outputs?: *[...{
				CompressionFormat?: *("GZIP" | "LZ4" | "SNAPPY" | "BZIP2" | "DEFLATE" | "LZO" | "BROTLI" | "ZSTD" | "ZLIB") | fn.#Fn
				Format?:            *("CSV" | "JSON" | "PARQUET" | "GLUEPARQUET" | "AVRO" | "ORC" | "XML" | "TABLEAUHYPER") | fn.#Fn
				FormatOptions?:     *{
					Csv?: *{
						Delimiter?: *(strings.MinRunes(1) & strings.MaxRunes(1)) | fn.#Fn
					} | fn.#If
				} | fn.#If
				Location: *{
					Bucket: *string | fn.#Fn
					Key?:   *string | fn.#Fn
				} | fn.#If
				Overwrite?:        *bool | fn.#Fn
				PartitionColumns?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			}] | fn.#If
			ProfileConfiguration?: *{
				ColumnStatisticsConfigurations?: *[...{
					Selectors?: *[...{
						Name?:  *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
						Regex?: *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
					}] | fn.#If
					Statistics: *{
						IncludedStatistics?: [...(*(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[A-Z\_]+$"#)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[A-Z\_]+$"#)) | fn.#Fn)
						Overrides?:          *[...{
							Parameters: *{} | fn.#If
							Statistic:  *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[A-Z\_]+$"#)) | fn.#Fn
						}] | fn.#If
					} | fn.#If
				}] | fn.#If
				DatasetStatisticsConfiguration?: *{
					IncludedStatistics?: [...(*(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[A-Z\_]+$"#)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[A-Z\_]+$"#)) | fn.#Fn)
					Overrides?:          *[...{
						Parameters: *{} | fn.#If
						Statistic:  *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[A-Z\_]+$"#)) | fn.#Fn
					}] | fn.#If
				} | fn.#If
				EntityDetectorConfiguration?: *{
					AllowedStatistics?: *{
						Statistics: [...(*(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[A-Z\_]+$"#)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[A-Z\_]+$"#)) | fn.#Fn)
					} | fn.#If
					EntityTypes: [...(*(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[A-Z_][A-Z\\d_]*$"#)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[A-Z_][A-Z\\d_]*$"#)) | fn.#Fn)
				} | fn.#If
				ProfileColumns?: *[...{
					Name?:  *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
					Regex?: *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
				}] | fn.#If
			} | fn.#If
			ProjectName?: *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
			Recipe?:      *{
				Name:     *string | fn.#Fn
				Version?: *string | fn.#Fn
			} | fn.#If
			RoleArn: *string | fn.#Fn
			Tags?:   *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			Timeout?:                  *int | fn.#Fn
			Type:                      *("PROFILE" | "RECIPE") | fn.#Fn
			ValidationConfigurations?: *[...{
				RulesetArn:      *(strings.MinRunes(20) & strings.MaxRunes(2048)) | fn.#Fn
				ValidationMode?: *("CHECK_ALL") | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Project: {
		Type: "AWS::DataBrew::Project"
		Properties: {
			DatasetName: *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
			Name:        *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
			RecipeName:  *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
			RoleArn:     *string | fn.#Fn
			Sample?:     *{
				Size?: *int | fn.#Fn
				Type:  *("FIRST_N" | "LAST_N" | "RANDOM") | fn.#Fn
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
	#Recipe: {
		Type: "AWS::DataBrew::Recipe"
		Properties: {
			Description?: *string | fn.#Fn
			Name:         *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
			Steps:        *[...{
				Action: *{
					Operation:   *string | fn.#Fn
					Parameters?: *{
						[string]: *string | fn.#Fn
					} | fn.#If
				} | fn.#If
				ConditionExpressions?: *[...{
					Condition:    *string | fn.#Fn
					TargetColumn: *string | fn.#Fn
					Value?:       *string | fn.#Fn
				}] | fn.#If
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
	#Ruleset: {
		Type: "AWS::DataBrew::Ruleset"
		Properties: {
			Description?: *string | fn.#Fn
			Name:         *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
			Rules:        *[...{
				CheckExpression:  *(strings.MinRunes(4) & strings.MaxRunes(1024) & (=~#"^[><0-9A-Za-z_.,:)(!= ]+$"#)) | fn.#Fn
				ColumnSelectors?: *[...{
					Name?:  *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
					Regex?: *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
				}] | fn.#If
				Disabled?:        *bool | fn.#Fn
				Name:             *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
				SubstitutionMap?: *[...{
					Value:          *string | fn.#Fn
					ValueReference: *(strings.MinRunes(2) & strings.MaxRunes(128) & (=~#"^:[A-Za-z0-9_]+$"#)) | fn.#Fn
				}] | fn.#If
				Threshold?: *{
					Type?: *("GREATER_THAN_OR_EQUAL" | "LESS_THAN_OR_EQUAL" | "GREATER_THAN" | "LESS_THAN") | fn.#Fn
					Unit?: *("COUNT" | "PERCENTAGE") | fn.#Fn
					Value: *number | fn.#Fn
				} | fn.#If
			}] | fn.#If
			Tags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			TargetArn: *(strings.MinRunes(20) & strings.MaxRunes(2048)) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Schedule: {
		Type: "AWS::DataBrew::Schedule"
		Properties: {
			CronExpression: *(strings.MinRunes(1) & strings.MaxRunes(512)) | fn.#Fn
			JobNames?:      [...(*(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn)
			Name:           *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
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
}
