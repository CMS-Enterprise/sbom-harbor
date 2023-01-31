package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#Kendra: {
	#DataSource: {
		Type: "AWS::Kendra::DataSource"
		Properties: {
			DataSourceConfiguration?: *{
				ConfluenceConfiguration?: *{
					AttachmentConfiguration?: *{
						AttachmentFieldMappings?: *[...{
							DataSourceFieldName: *("AUTHOR" | "CONTENT_TYPE" | "CREATED_DATE" | "DISPLAY_URL" | "FILE_SIZE" | "ITEM_TYPE" | "PARENT_ID" | "SPACE_KEY" | "SPACE_NAME" | "URL" | "VERSION") | fn.#Fn
							DateFieldFormat?:    *(strings.MinRunes(4) & strings.MaxRunes(40)) | fn.#Fn
							IndexFieldName:      *(strings.MinRunes(1) & strings.MaxRunes(30)) | fn.#Fn
						}] | fn.#If
						CrawlAttachments?: *bool | fn.#Fn
					} | fn.#If
					BlogConfiguration?: *{
						BlogFieldMappings?: *[...{
							DataSourceFieldName: *("AUTHOR" | "DISPLAY_URL" | "ITEM_TYPE" | "LABELS" | "PUBLISH_DATE" | "SPACE_KEY" | "SPACE_NAME" | "URL" | "VERSION") | fn.#Fn
							DateFieldFormat?:    *(strings.MinRunes(4) & strings.MaxRunes(40)) | fn.#Fn
							IndexFieldName:      *(strings.MinRunes(1) & strings.MaxRunes(30)) | fn.#Fn
						}] | fn.#If
					} | fn.#If
					ExclusionPatterns?: [...(*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)
					InclusionPatterns?: [...(*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)
					PageConfiguration?: *{
						PageFieldMappings?: *[...{
							DataSourceFieldName: *("AUTHOR" | "CONTENT_STATUS" | "CREATED_DATE" | "DISPLAY_URL" | "ITEM_TYPE" | "LABELS" | "MODIFIED_DATE" | "PARENT_ID" | "SPACE_KEY" | "SPACE_NAME" | "URL" | "VERSION") | fn.#Fn
							DateFieldFormat?:    *(strings.MinRunes(4) & strings.MaxRunes(40)) | fn.#Fn
							IndexFieldName:      *(strings.MinRunes(1) & strings.MaxRunes(30)) | fn.#Fn
						}] | fn.#If
					} | fn.#If
					SecretArn:           *(strings.MinRunes(1) & strings.MaxRunes(1284) & (=~#"arn:[a-z0-9-\.]{1,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[^/].{0,1023}"#)) | fn.#Fn
					ServerUrl:           *(strings.MinRunes(1) & strings.MaxRunes(2048) & (=~#"^(https?|ftp|file)://([^\s]*)"#)) | fn.#Fn
					SpaceConfiguration?: *{
						CrawlArchivedSpaces?: *bool | fn.#Fn
						CrawlPersonalSpaces?: *bool | fn.#Fn
						ExcludeSpaces?:       [...(*(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn)
						IncludeSpaces?:       [...(*(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn)
						SpaceFieldMappings?:  *[...{
							DataSourceFieldName: *("DISPLAY_URL" | "ITEM_TYPE" | "SPACE_KEY" | "URL") | fn.#Fn
							DateFieldFormat?:    *(strings.MinRunes(4) & strings.MaxRunes(40)) | fn.#Fn
							IndexFieldName:      *(strings.MinRunes(1) & strings.MaxRunes(30)) | fn.#Fn
						}] | fn.#If
					} | fn.#If
					Version:           *("CLOUD" | "SERVER") | fn.#Fn
					VpcConfiguration?: *{
						SecurityGroupIds: [...(*(strings.MinRunes(1) & strings.MaxRunes(200) & (=~#"[\-0-9a-zA-Z]+"#)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(200) & (=~#"[\-0-9a-zA-Z]+"#)) | fn.#Fn)
						SubnetIds:        [...(*(strings.MinRunes(1) & strings.MaxRunes(200) & (=~#"[\-0-9a-zA-Z]+"#)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(200) & (=~#"[\-0-9a-zA-Z]+"#)) | fn.#Fn)
					} | fn.#If
				} | fn.#If
				DatabaseConfiguration?: *{
					AclConfiguration?: *{
						AllowedGroupsColumnName: *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
					} | fn.#If
					ColumnConfiguration: *{
						ChangeDetectingColumns:   [...(*(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn)
						DocumentDataColumnName:   *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
						DocumentIdColumnName:     *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
						DocumentTitleColumnName?: *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
						FieldMappings?:           *[...{
							DataSourceFieldName: *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
							DateFieldFormat?:    *(strings.MinRunes(4) & strings.MaxRunes(40)) | fn.#Fn
							IndexFieldName:      *(strings.MinRunes(1) & strings.MaxRunes(30)) | fn.#Fn
						}] | fn.#If
					} | fn.#If
					ConnectionConfiguration: *{
						DatabaseHost: *(strings.MinRunes(1) & strings.MaxRunes(253)) | fn.#Fn
						DatabaseName: *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
						DatabasePort: *(>=1 & <=65535) | fn.#Fn
						SecretArn:    *(strings.MinRunes(1) & strings.MaxRunes(1284) & (=~#"arn:[a-z0-9-\.]{1,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[^/].{0,1023}"#)) | fn.#Fn
						TableName:    *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
					} | fn.#If
					DatabaseEngineType: *("RDS_AURORA_MYSQL" | "RDS_AURORA_POSTGRESQL" | "RDS_MYSQL" | "RDS_POSTGRESQL") | fn.#Fn
					SqlConfiguration?:  *{
						QueryIdentifiersEnclosingOption?: *("DOUBLE_QUOTES" | "NONE") | fn.#Fn
					} | fn.#If
					VpcConfiguration?: *{
						SecurityGroupIds: [...(*(strings.MinRunes(1) & strings.MaxRunes(200) & (=~#"[\-0-9a-zA-Z]+"#)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(200) & (=~#"[\-0-9a-zA-Z]+"#)) | fn.#Fn)
						SubnetIds:        [...(*(strings.MinRunes(1) & strings.MaxRunes(200) & (=~#"[\-0-9a-zA-Z]+"#)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(200) & (=~#"[\-0-9a-zA-Z]+"#)) | fn.#Fn)
					} | fn.#If
				} | fn.#If
				GoogleDriveConfiguration?: *{
					ExcludeMimeTypes?:    [...(*(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn)
					ExcludeSharedDrives?: [...(*(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn)
					ExcludeUserAccounts?: [...(*(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn)
					ExclusionPatterns?:   [...(*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)
					FieldMappings?:       *[...{
						DataSourceFieldName: *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
						DateFieldFormat?:    *(strings.MinRunes(4) & strings.MaxRunes(40)) | fn.#Fn
						IndexFieldName:      *(strings.MinRunes(1) & strings.MaxRunes(30)) | fn.#Fn
					}] | fn.#If
					InclusionPatterns?: [...(*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)
					SecretArn:          *(strings.MinRunes(1) & strings.MaxRunes(1284) & (=~#"arn:[a-z0-9-\.]{1,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[^/].{0,1023}"#)) | fn.#Fn
				} | fn.#If
				OneDriveConfiguration?: *{
					DisableLocalGroups?: *bool | fn.#Fn
					ExclusionPatterns?:  [...(*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)
					FieldMappings?:      *[...{
						DataSourceFieldName: *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
						DateFieldFormat?:    *(strings.MinRunes(4) & strings.MaxRunes(40)) | fn.#Fn
						IndexFieldName:      *(strings.MinRunes(1) & strings.MaxRunes(30)) | fn.#Fn
					}] | fn.#If
					InclusionPatterns?: [...(*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)
					OneDriveUsers:      *{
						OneDriveUserList?:   [...(*(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"^(?!\s).+@([a-zA-Z0-9_\-\.]+)\.([a-zA-Z]{2,5})$"#)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"^(?!\s).+@([a-zA-Z0-9_\-\.]+)\.([a-zA-Z]{2,5})$"#)) | fn.#Fn)
						OneDriveUserS3Path?: *{
							Bucket: *(strings.MinRunes(3) & strings.MaxRunes(63) & (=~#"[a-z0-9][\.\-a-z0-9]{1,61}[a-z0-9]"#)) | fn.#Fn
							Key:    *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
						} | fn.#If
					} | fn.#If
					SecretArn:    *(strings.MinRunes(1) & strings.MaxRunes(1284) & (=~#"arn:[a-z0-9-\.]{1,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[^/].{0,1023}"#)) | fn.#Fn
					TenantDomain: *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"^([a-zA-Z0-9]+(-[a-zA-Z0-9]+)*\.)+[a-z]{2,}$"#)) | fn.#Fn
				} | fn.#If
				S3Configuration?: *{
					AccessControlListConfiguration?: *{
						KeyPath?: *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
					} | fn.#If
					BucketName:                      *(strings.MinRunes(3) & strings.MaxRunes(63) & (=~#"[a-z0-9][\.\-a-z0-9]{1,61}[a-z0-9]"#)) | fn.#Fn
					DocumentsMetadataConfiguration?: *{
						S3Prefix?: *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
					} | fn.#If
					ExclusionPatterns?: [...(*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)
					InclusionPatterns?: [...(*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)
					InclusionPrefixes?: [...(*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)
				} | fn.#If
				SalesforceConfiguration?: *{
					ChatterFeedConfiguration?: *{
						DocumentDataFieldName:   *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
						DocumentTitleFieldName?: *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
						FieldMappings?:          *[...{
							DataSourceFieldName: *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
							DateFieldFormat?:    *(strings.MinRunes(4) & strings.MaxRunes(40)) | fn.#Fn
							IndexFieldName:      *(strings.MinRunes(1) & strings.MaxRunes(30)) | fn.#Fn
						}] | fn.#If
						IncludeFilterTypes?: [...(*("ACTIVE_USER" | "STANDARD_USER") | fn.#Fn)] | (*("ACTIVE_USER" | "STANDARD_USER") | fn.#Fn)
					} | fn.#If
					CrawlAttachments?:              *bool | fn.#Fn
					ExcludeAttachmentFilePatterns?: [...(*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)
					IncludeAttachmentFilePatterns?: [...(*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)
					KnowledgeArticleConfiguration?: *{
						CustomKnowledgeArticleTypeConfigurations?: *[...{
							DocumentDataFieldName:   *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
							DocumentTitleFieldName?: *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
							FieldMappings?:          *[...{
								DataSourceFieldName: *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
								DateFieldFormat?:    *(strings.MinRunes(4) & strings.MaxRunes(40)) | fn.#Fn
								IndexFieldName:      *(strings.MinRunes(1) & strings.MaxRunes(30)) | fn.#Fn
							}] | fn.#If
							Name: *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
						}] | fn.#If
						IncludedStates:                             [...(*("DRAFT" | "PUBLISHED" | "ARCHIVED") | fn.#Fn)] | (*("DRAFT" | "PUBLISHED" | "ARCHIVED") | fn.#Fn)
						StandardKnowledgeArticleTypeConfiguration?: *{
							DocumentDataFieldName:   *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
							DocumentTitleFieldName?: *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
							FieldMappings?:          *[...{
								DataSourceFieldName: *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
								DateFieldFormat?:    *(strings.MinRunes(4) & strings.MaxRunes(40)) | fn.#Fn
								IndexFieldName:      *(strings.MinRunes(1) & strings.MaxRunes(30)) | fn.#Fn
							}] | fn.#If
						} | fn.#If
					} | fn.#If
					SecretArn:                              *(strings.MinRunes(1) & strings.MaxRunes(1284) & (=~#"arn:[a-z0-9-\.]{1,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[^/].{0,1023}"#)) | fn.#Fn
					ServerUrl:                              *(strings.MinRunes(1) & strings.MaxRunes(2048) & (=~#"^(https?|ftp|file)://([^\s]*)"#)) | fn.#Fn
					StandardObjectAttachmentConfiguration?: *{
						DocumentTitleFieldName?: *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
						FieldMappings?:          *[...{
							DataSourceFieldName: *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
							DateFieldFormat?:    *(strings.MinRunes(4) & strings.MaxRunes(40)) | fn.#Fn
							IndexFieldName:      *(strings.MinRunes(1) & strings.MaxRunes(30)) | fn.#Fn
						}] | fn.#If
					} | fn.#If
					StandardObjectConfigurations?: *[...{
						DocumentDataFieldName:   *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
						DocumentTitleFieldName?: *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
						FieldMappings?:          *[...{
							DataSourceFieldName: *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
							DateFieldFormat?:    *(strings.MinRunes(4) & strings.MaxRunes(40)) | fn.#Fn
							IndexFieldName:      *(strings.MinRunes(1) & strings.MaxRunes(30)) | fn.#Fn
						}] | fn.#If
						Name: *("ACCOUNT" | "CAMPAIGN" | "CASE" | "CONTACT" | "CONTRACT" | "DOCUMENT" | "GROUP" | "IDEA" | "LEAD" | "OPPORTUNITY" | "PARTNER" | "PRICEBOOK" | "PRODUCT" | "PROFILE" | "SOLUTION" | "TASK" | "USER") | fn.#Fn
					}] | fn.#If
				} | fn.#If
				ServiceNowConfiguration?: *{
					AuthenticationType?:            *("HTTP_BASIC" | "OAUTH2") | fn.#Fn
					HostUrl:                        *(strings.MinRunes(1) & strings.MaxRunes(2048) & (=~#"^(?!(^(https?|ftp|file):\/\/))[a-z0-9-]+(\.service-now\.com)$"#)) | fn.#Fn
					KnowledgeArticleConfiguration?: *{
						CrawlAttachments?:              *bool | fn.#Fn
						DocumentDataFieldName:          *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
						DocumentTitleFieldName?:        *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
						ExcludeAttachmentFilePatterns?: [...(*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)
						FieldMappings?:                 *[...{
							DataSourceFieldName: *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
							DateFieldFormat?:    *(strings.MinRunes(4) & strings.MaxRunes(40)) | fn.#Fn
							IndexFieldName:      *(strings.MinRunes(1) & strings.MaxRunes(30)) | fn.#Fn
						}] | fn.#If
						FilterQuery?:                   *(strings.MinRunes(1) & strings.MaxRunes(2048)) | fn.#Fn
						IncludeAttachmentFilePatterns?: [...(*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)
					} | fn.#If
					SecretArn:                    *(strings.MinRunes(1) & strings.MaxRunes(1284) & (=~#"arn:[a-z0-9-\.]{1,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[^/].{0,1023}"#)) | fn.#Fn
					ServiceCatalogConfiguration?: *{
						CrawlAttachments?:              *bool | fn.#Fn
						DocumentDataFieldName:          *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
						DocumentTitleFieldName?:        *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
						ExcludeAttachmentFilePatterns?: [...(*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)
						FieldMappings?:                 *[...{
							DataSourceFieldName: *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
							DateFieldFormat?:    *(strings.MinRunes(4) & strings.MaxRunes(40)) | fn.#Fn
							IndexFieldName:      *(strings.MinRunes(1) & strings.MaxRunes(30)) | fn.#Fn
						}] | fn.#If
						IncludeAttachmentFilePatterns?: [...(*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)
					} | fn.#If
					ServiceNowBuildVersion: *("LONDON" | "OTHERS") | fn.#Fn
				} | fn.#If
				SharePointConfiguration?: *{
					CrawlAttachments?:       *bool | fn.#Fn
					DisableLocalGroups?:     *bool | fn.#Fn
					DocumentTitleFieldName?: *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
					ExclusionPatterns?:      [...(*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)
					FieldMappings?:          *[...{
						DataSourceFieldName: *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
						DateFieldFormat?:    *(strings.MinRunes(4) & strings.MaxRunes(40)) | fn.#Fn
						IndexFieldName:      *(strings.MinRunes(1) & strings.MaxRunes(30)) | fn.#Fn
					}] | fn.#If
					InclusionPatterns?:    [...(*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)
					SecretArn:             *(strings.MinRunes(1) & strings.MaxRunes(1284) & (=~#"arn:[a-z0-9-\.]{1,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[^/].{0,1023}"#)) | fn.#Fn
					SharePointVersion:     *("SHAREPOINT_ONLINE" | "SHAREPOINT_2013" | "SHAREPOINT_2016") | fn.#Fn
					SslCertificateS3Path?: *{
						Bucket: *(strings.MinRunes(3) & strings.MaxRunes(63) & (=~#"[a-z0-9][\.\-a-z0-9]{1,61}[a-z0-9]"#)) | fn.#Fn
						Key:    *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
					} | fn.#If
					Urls:              [...(*(strings.MinRunes(1) & strings.MaxRunes(2048) & (=~#"^(https?|ftp|file)://([^\s]*)"#)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(2048) & (=~#"^(https?|ftp|file)://([^\s]*)"#)) | fn.#Fn)
					UseChangeLog?:     *bool | fn.#Fn
					VpcConfiguration?: *{
						SecurityGroupIds: [...(*(strings.MinRunes(1) & strings.MaxRunes(200) & (=~#"[\-0-9a-zA-Z]+"#)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(200) & (=~#"[\-0-9a-zA-Z]+"#)) | fn.#Fn)
						SubnetIds:        [...(*(strings.MinRunes(1) & strings.MaxRunes(200) & (=~#"[\-0-9a-zA-Z]+"#)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(200) & (=~#"[\-0-9a-zA-Z]+"#)) | fn.#Fn)
					} | fn.#If
				} | fn.#If
				WebCrawlerConfiguration?: *{
					AuthenticationConfiguration?: *{
						BasicAuthentication?: *[...{
							Credentials: *(strings.MinRunes(1) & strings.MaxRunes(1284) & (=~#"arn:[a-z0-9-\.]{1,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[^/].{0,1023}"#)) | fn.#Fn
							Host:        *(strings.MinRunes(1) & strings.MaxRunes(253) & (=~#"([^\s]*)"#)) | fn.#Fn
							Port:        *(>=1 & <=65535) | fn.#Fn
						}] | fn.#If
					} | fn.#If
					CrawlDepth?:                       *(>=1 & <=10) | fn.#Fn
					MaxContentSizePerPageInMegaBytes?: *number | fn.#Fn
					MaxLinksPerPage?:                  *(>=1 & <=1000) | fn.#Fn
					MaxUrlsPerMinuteCrawlRate?:        *(>=1 & <=300) | fn.#Fn
					ProxyConfiguration?:               *{
						Credentials?: *(strings.MinRunes(1) & strings.MaxRunes(1284) & (=~#"arn:[a-z0-9-\.]{1,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[^/].{0,1023}"#)) | fn.#Fn
						Host:         *(strings.MinRunes(1) & strings.MaxRunes(253) & (=~#"([^\s]*)"#)) | fn.#Fn
						Port:         *(>=1 & <=65535) | fn.#Fn
					} | fn.#If
					UrlExclusionPatterns?: [...(*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)
					UrlInclusionPatterns?: [...(*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)
					Urls:                  *{
						SeedUrlConfiguration?: *{
							SeedUrls:        [...(*(strings.MinRunes(1) & strings.MaxRunes(2048) & (=~#"^(https?)://([^\s]*)"#)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(2048) & (=~#"^(https?)://([^\s]*)"#)) | fn.#Fn)
							WebCrawlerMode?: *("HOST_ONLY" | "SUBDOMAINS" | "EVERYTHING") | fn.#Fn
						} | fn.#If
						SiteMapsConfiguration?: *{
							SiteMaps: [...(*(strings.MinRunes(1) & strings.MaxRunes(2048) & (=~#"^(https?):\/\/([^\s]*)"#)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(2048) & (=~#"^(https?):\/\/([^\s]*)"#)) | fn.#Fn)
						} | fn.#If
					} | fn.#If
				} | fn.#If
				WorkDocsConfiguration?: *{
					CrawlComments?:     *bool | fn.#Fn
					ExclusionPatterns?: [...(*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)
					FieldMappings?:     *[...{
						DataSourceFieldName: *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
						DateFieldFormat?:    *(strings.MinRunes(4) & strings.MaxRunes(40)) | fn.#Fn
						IndexFieldName:      *(strings.MinRunes(1) & strings.MaxRunes(30)) | fn.#Fn
					}] | fn.#If
					InclusionPatterns?: [...(*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)
					OrganizationId:     *(strings.MinRunes(12) & strings.MaxRunes(12) & (=~#"d-[0-9a-fA-F]{10}"#)) | fn.#Fn
					UseChangeLog?:      *bool | fn.#Fn
				} | fn.#If
			} | fn.#If
			Description?: *(strings.MinRunes(1) & strings.MaxRunes(1000)) | fn.#Fn
			IndexId:      *(strings.MinRunes(36) & strings.MaxRunes(36)) | fn.#Fn
			Name:         *(strings.MinRunes(1) & strings.MaxRunes(1000)) | fn.#Fn
			RoleArn?:     *(strings.MinRunes(1) & strings.MaxRunes(1284) & (=~#"arn:[a-z0-9-\.]{1,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[^/].{0,1023}"#)) | fn.#Fn
			Schedule?:    *string | fn.#Fn
			Tags?:        *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			Type: *("S3" | "SHAREPOINT" | "SALESFORCE" | "ONEDRIVE" | "SERVICENOW" | "DATABASE" | "CUSTOM" | "CONFLUENCE" | "GOOGLEDRIVE" | "WEBCRAWLER" | "WORKDOCS") | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Faq: {
		Type: "AWS::Kendra::Faq"
		Properties: {
			Description?: *(strings.MinRunes(1) & strings.MaxRunes(1000)) | fn.#Fn
			FileFormat?:  *("CSV" | "CSV_WITH_HEADER" | "JSON") | fn.#Fn
			IndexId:      *(strings.MinRunes(36) & strings.MaxRunes(36)) | fn.#Fn
			Name:         *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
			RoleArn:      *(strings.MinRunes(1) & strings.MaxRunes(1284) & (=~#"arn:[a-z0-9-\.]{1,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[^/].{0,1023}"#)) | fn.#Fn
			S3Path:       *{
				Bucket: *(strings.MinRunes(3) & strings.MaxRunes(63) & (=~#"[a-z0-9][\.\-a-z0-9]{1,61}[a-z0-9]"#)) | fn.#Fn
				Key:    *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
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
	#Index: {
		Type: "AWS::Kendra::Index"
		Properties: {
			CapacityUnits?: *{
				QueryCapacityUnits:   *int | fn.#Fn
				StorageCapacityUnits: *int | fn.#Fn
			} | fn.#If
			Description?:                    *string | fn.#Fn
			DocumentMetadataConfigurations?: *[...{
				Name:       *(strings.MinRunes(1) & strings.MaxRunes(30)) | fn.#Fn
				Relevance?: *{
					Duration?:             *(strings.MinRunes(1) & strings.MaxRunes(10) & (=~#"[0-9]+[s]"#)) | fn.#Fn
					Freshness?:            *bool | fn.#Fn
					Importance?:           *(>=1 & <=10) | fn.#Fn
					RankOrder?:            *("ASCENDING" | "DESCENDING") | fn.#Fn
					ValueImportanceItems?: *[...{
						Key?:   *(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn
						Value?: *(>=1 & <=10) | fn.#Fn
					}] | fn.#If
				} | fn.#If
				Search?: *{
					Displayable?: *bool | fn.#Fn
					Facetable?:   *bool | fn.#Fn
					Searchable?:  *bool | fn.#Fn
					Sortable?:    *bool | fn.#Fn
				} | fn.#If
				Type: *("STRING_VALUE" | "STRING_LIST_VALUE" | "LONG_VALUE" | "DATE_VALUE") | fn.#Fn
			}] | fn.#If
			Edition:                            *("DEVELOPER_EDITION" | "ENTERPRISE_EDITION") | fn.#Fn
			Name:                               *(strings.MinRunes(1) & strings.MaxRunes(1000)) | fn.#Fn
			RoleArn:                            *(strings.MinRunes(1) & strings.MaxRunes(1284) & (=~#"arn:[a-z0-9-\.]{1,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[^/].{0,1023}"#)) | fn.#Fn
			ServerSideEncryptionConfiguration?: *{
				KmsKeyId?: *(strings.MinRunes(1) & strings.MaxRunes(2048)) | fn.#Fn
			} | fn.#If
			Tags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			UserContextPolicy?:       *("ATTRIBUTE_FILTER" | "USER_TOKEN") | fn.#Fn
			UserTokenConfigurations?: *[...{
				JsonTokenTypeConfiguration?: *{
					GroupAttributeField:    *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
					UserNameAttributeField: *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
				} | fn.#If
				JwtTokenTypeConfiguration?: *{
					ClaimRegex?:             *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
					GroupAttributeField?:    *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
					Issuer?:                 *(strings.MinRunes(1) & strings.MaxRunes(65)) | fn.#Fn
					KeyLocation:             *("URL" | "SECRET_MANAGER") | fn.#Fn
					SecretManagerArn?:       *(strings.MinRunes(1) & strings.MaxRunes(1284) & (=~#"arn:[a-z0-9-\.]{1,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[a-z0-9-\.]{0,63}:[^/].{0,1023}"#)) | fn.#Fn
					URL?:                    *(strings.MinRunes(1) & strings.MaxRunes(2048) & (=~#"^(https?|ftp|file):\/\/([^\s]*)"#)) | fn.#Fn
					UserNameAttributeField?: *(strings.MinRunes(1) & strings.MaxRunes(100)) | fn.#Fn
				} | fn.#If
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
