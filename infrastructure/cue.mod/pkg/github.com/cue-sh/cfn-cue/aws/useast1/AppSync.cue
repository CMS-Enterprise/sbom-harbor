package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#AppSync: {
	#ApiCache: {
		Type: "AWS::AppSync::ApiCache"
		Properties: {
			ApiCachingBehavior:        *string | fn.#Fn
			ApiId:                     *string | fn.#Fn
			AtRestEncryptionEnabled?:  *bool | fn.#Fn
			TransitEncryptionEnabled?: *bool | fn.#Fn
			Ttl:                       *number | fn.#Fn
			Type:                      *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ApiKey: {
		Type: "AWS::AppSync::ApiKey"
		Properties: {
			ApiId:        *string | fn.#Fn
			ApiKeyId?:    *string | fn.#Fn
			Description?: *string | fn.#Fn
			Expires?:     *number | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#DataSource: {
		Type: "AWS::AppSync::DataSource"
		Properties: {
			ApiId:           *string | fn.#Fn
			Description?:    *string | fn.#Fn
			DynamoDBConfig?: *{
				AwsRegion:        *string | fn.#Fn
				DeltaSyncConfig?: *{
					BaseTableTTL:       *string | fn.#Fn
					DeltaSyncTableName: *string | fn.#Fn
					DeltaSyncTableTTL:  *string | fn.#Fn
				} | fn.#If
				TableName:             *string | fn.#Fn
				UseCallerCredentials?: *bool | fn.#Fn
				Versioned?:            *bool | fn.#Fn
			} | fn.#If
			ElasticsearchConfig?: *{
				AwsRegion: *string | fn.#Fn
				Endpoint:  *string | fn.#Fn
			} | fn.#If
			HttpConfig?: *{
				AuthorizationConfig?: *{
					AuthorizationType: *string | fn.#Fn
					AwsIamConfig?:     *{
						SigningRegion?:      *string | fn.#Fn
						SigningServiceName?: *string | fn.#Fn
					} | fn.#If
				} | fn.#If
				Endpoint: *string | fn.#Fn
			} | fn.#If
			LambdaConfig?: *{
				LambdaFunctionArn: *string | fn.#Fn
			} | fn.#If
			Name:                     *string | fn.#Fn
			OpenSearchServiceConfig?: *{
				AwsRegion: *string | fn.#Fn
				Endpoint:  *string | fn.#Fn
			} | fn.#If
			RelationalDatabaseConfig?: *{
				RdsHttpEndpointConfig?: *{
					AwsRegion:           *string | fn.#Fn
					AwsSecretStoreArn:   *string | fn.#Fn
					DatabaseName?:       *string | fn.#Fn
					DbClusterIdentifier: *string | fn.#Fn
					Schema?:             *string | fn.#Fn
				} | fn.#If
				RelationalDatabaseSourceType: *string | fn.#Fn
			} | fn.#If
			ServiceRoleArn?: *string | fn.#Fn
			Type:            *("AMAZON_DYNAMODB" | "AMAZON_ELASTICSEARCH" | "AMAZON_OPENSEARCH_SERVICE" | "AWS_LAMBDA" | "HTTP" | "NONE" | "RELATIONAL_DATABASE") | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#FunctionConfiguration: {
		Type: "AWS::AppSync::FunctionConfiguration"
		Properties: {
			ApiId:                              *string | fn.#Fn
			DataSourceName:                     *string | fn.#Fn
			Description?:                       *string | fn.#Fn
			FunctionVersion:                    *string | fn.#Fn
			Name:                               *string | fn.#Fn
			RequestMappingTemplate?:            *string | fn.#Fn
			RequestMappingTemplateS3Location?:  *string | fn.#Fn
			ResponseMappingTemplate?:           *string | fn.#Fn
			ResponseMappingTemplateS3Location?: *string | fn.#Fn
			SyncConfig?:                        *{
				ConflictDetection:            *string | fn.#Fn
				ConflictHandler?:             *string | fn.#Fn
				LambdaConflictHandlerConfig?: *{
					LambdaConflictHandlerArn?: *string | fn.#Fn
				} | fn.#If
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#GraphQLApi: {
		Type: "AWS::AppSync::GraphQLApi"
		Properties: {
			AdditionalAuthenticationProviders?: *[...{
				AuthenticationType:      *string | fn.#Fn
				LambdaAuthorizerConfig?: *{
					AuthorizerResultTtlInSeconds?: *number | fn.#Fn
					AuthorizerUri?:                *string | fn.#Fn
					IdentityValidationExpression?: *string | fn.#Fn
				} | fn.#If
				OpenIDConnectConfig?: *{
					AuthTTL?:  *number | fn.#Fn
					ClientId?: *string | fn.#Fn
					IatTTL?:   *number | fn.#Fn
					Issuer?:   *string | fn.#Fn
				} | fn.#If
				UserPoolConfig?: *{
					AppIdClientRegex?: *string | fn.#Fn
					AwsRegion?:        *string | fn.#Fn
					UserPoolId?:       *string | fn.#Fn
				} | fn.#If
			}] | fn.#If
			AuthenticationType:      *("AMAZON_COGNITO_USER_POOLS" | "API_KEY" | "AWS_IAM" | "AWS_LAMBDA" | "OPENID_CONNECT") | fn.#Fn
			LambdaAuthorizerConfig?: *{
				AuthorizerResultTtlInSeconds?: *number | fn.#Fn
				AuthorizerUri?:                *string | fn.#Fn
				IdentityValidationExpression?: *string | fn.#Fn
			} | fn.#If
			LogConfig?: *{
				CloudWatchLogsRoleArn?: *string | fn.#Fn
				ExcludeVerboseContent?: *bool | fn.#Fn
				FieldLogLevel?:         *string | fn.#Fn
			} | fn.#If
			Name:                 *string | fn.#Fn
			OpenIDConnectConfig?: *{
				AuthTTL?:  *number | fn.#Fn
				ClientId?: *string | fn.#Fn
				IatTTL?:   *number | fn.#Fn
				Issuer?:   *string | fn.#Fn
			} | fn.#If
			Tags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			UserPoolConfig?: *{
				AppIdClientRegex?: *string | fn.#Fn
				AwsRegion?:        *string | fn.#Fn
				DefaultAction?:    *string | fn.#Fn
				UserPoolId?:       *string | fn.#Fn
			} | fn.#If
			XrayEnabled?: *bool | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#GraphQLSchema: {
		Type: "AWS::AppSync::GraphQLSchema"
		Properties: {
			ApiId:                 *string | fn.#Fn
			Definition?:           *string | fn.#Fn
			DefinitionS3Location?: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Resolver: {
		Type: "AWS::AppSync::Resolver"
		Properties: {
			ApiId:          *string | fn.#Fn
			CachingConfig?: *{
				CachingKeys?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				Ttl?:         *number | fn.#Fn
			} | fn.#If
			DataSourceName?: *string | fn.#Fn
			FieldName:       *string | fn.#Fn
			Kind?:           *("PIPELINE" | "UNIT") | fn.#Fn
			PipelineConfig?: *{
				Functions?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			} | fn.#If
			RequestMappingTemplate?:            *string | fn.#Fn
			RequestMappingTemplateS3Location?:  *string | fn.#Fn
			ResponseMappingTemplate?:           *string | fn.#Fn
			ResponseMappingTemplateS3Location?: *string | fn.#Fn
			SyncConfig?:                        *{
				ConflictDetection:            *string | fn.#Fn
				ConflictHandler?:             *string | fn.#Fn
				LambdaConflictHandlerConfig?: *{
					LambdaConflictHandlerArn?: *string | fn.#Fn
				} | fn.#If
			} | fn.#If
			TypeName: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
