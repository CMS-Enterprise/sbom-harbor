package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#ApiGateway: {
	#Account: {
		Type: "AWS::ApiGateway::Account"
		Properties: CloudWatchRoleArn?: *string | fn.#Fn
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ApiKey: {
		Type: "AWS::ApiGateway::ApiKey"
		Properties: {
			CustomerId?:         *string | fn.#Fn
			Description?:        *string | fn.#Fn
			Enabled?:            *bool | fn.#Fn
			GenerateDistinctId?: *bool | fn.#Fn
			Name?:               *string | fn.#Fn
			StageKeys?:          *[...{
				RestApiId?: *string | fn.#Fn
				StageName?: *string | fn.#Fn
			}] | fn.#If
			Tags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			Value?: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Authorizer: {
		Type: "AWS::ApiGateway::Authorizer"
		Properties: {
			AuthType?:                     *string | fn.#Fn
			AuthorizerCredentials?:        *string | fn.#Fn
			AuthorizerResultTtlInSeconds?: *(>=0 & <=3600) | fn.#Fn
			AuthorizerUri?:                *string | fn.#Fn
			IdentitySource?:               *string | fn.#Fn
			IdentityValidationExpression?: *string | fn.#Fn
			Name:                          *string | fn.#Fn
			ProviderARNs?:                 [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			RestApiId:                     *string | fn.#Fn
			Type:                          *("COGNITO_USER_POOLS" | "REQUEST" | "TOKEN") | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#BasePathMapping: {
		Type: "AWS::ApiGateway::BasePathMapping"
		Properties: {
			BasePath?:  *string | fn.#Fn
			DomainName: *string | fn.#Fn
			RestApiId?: *string | fn.#Fn
			Stage?:     *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ClientCertificate: {
		Type: "AWS::ApiGateway::ClientCertificate"
		Properties: {
			Description?: *string | fn.#Fn
			Tags?:        *[...{
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
	#Deployment: {
		Type: "AWS::ApiGateway::Deployment"
		Properties: {
			DeploymentCanarySettings?: *{
				PercentTraffic?:         *number | fn.#Fn
				StageVariableOverrides?: *{
					[string]: *string | fn.#Fn
				} | fn.#If
				UseStageCache?: *bool | fn.#Fn
			} | fn.#If
			Description?:      *string | fn.#Fn
			RestApiId:         *string | fn.#Fn
			StageDescription?: *{
				AccessLogSetting?: *{
					DestinationArn?: *string | fn.#Fn
					Format?:         *string | fn.#Fn
				} | fn.#If
				CacheClusterEnabled?: *bool | fn.#Fn
				CacheClusterSize?:    *string | fn.#Fn
				CacheDataEncrypted?:  *bool | fn.#Fn
				CacheTtlInSeconds?:   *int | fn.#Fn
				CachingEnabled?:      *bool | fn.#Fn
				CanarySetting?:       *{
					PercentTraffic?:         *number | fn.#Fn
					StageVariableOverrides?: *{
						[string]: *string | fn.#Fn
					} | fn.#If
					UseStageCache?: *bool | fn.#Fn
				} | fn.#If
				ClientCertificateId?:  *string | fn.#Fn
				DataTraceEnabled?:     *bool | fn.#Fn
				Description?:          *string | fn.#Fn
				DocumentationVersion?: *string | fn.#Fn
				LoggingLevel?:         *string | fn.#Fn
				MethodSettings?:       *[...{
					CacheDataEncrypted?:   *bool | fn.#Fn
					CacheTtlInSeconds?:    *int | fn.#Fn
					CachingEnabled?:       *bool | fn.#Fn
					DataTraceEnabled?:     *bool | fn.#Fn
					HttpMethod?:           *string | fn.#Fn
					LoggingLevel?:         *string | fn.#Fn
					MetricsEnabled?:       *bool | fn.#Fn
					ResourcePath?:         *string | fn.#Fn
					ThrottlingBurstLimit?: *int | fn.#Fn
					ThrottlingRateLimit?:  *number | fn.#Fn
				}] | fn.#If
				MetricsEnabled?: *bool | fn.#Fn
				Tags?:           *[...{
					Key:   *string | fn.#Fn
					Value: *string | fn.#Fn
				}] | fn.#If
				ThrottlingBurstLimit?: *int | fn.#Fn
				ThrottlingRateLimit?:  *number | fn.#Fn
				TracingEnabled?:       *bool | fn.#Fn
				Variables?:            *{
					[string]: *string | fn.#Fn
				} | fn.#If
			} | fn.#If
			StageName?: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#DocumentationPart: {
		Type: "AWS::ApiGateway::DocumentationPart"
		Properties: {
			Location: *{
				Method?:     *string | fn.#Fn
				Name?:       *string | fn.#Fn
				Path?:       *string | fn.#Fn
				StatusCode?: *string | fn.#Fn
				Type?:       *string | fn.#Fn
			} | fn.#If
			Properties: *string | fn.#Fn
			RestApiId:  *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#DocumentationVersion: {
		Type: "AWS::ApiGateway::DocumentationVersion"
		Properties: {
			Description?:         *string | fn.#Fn
			DocumentationVersion: *string | fn.#Fn
			RestApiId:            *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#DomainName: {
		Type: "AWS::ApiGateway::DomainName"
		Properties: {
			CertificateArn?:        *string | fn.#Fn
			DomainName?:            *string | fn.#Fn
			EndpointConfiguration?: *{
				Types?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			} | fn.#If
			MutualTlsAuthentication?: *{
				TruststoreUri?:     *string | fn.#Fn
				TruststoreVersion?: *string | fn.#Fn
			} | fn.#If
			OwnershipVerificationCertificateArn?: *string | fn.#Fn
			RegionalCertificateArn?:              *string | fn.#Fn
			SecurityPolicy?:                      *string | fn.#Fn
			Tags?:                                *[...{
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
	#GatewayResponse: {
		Type: "AWS::ApiGateway::GatewayResponse"
		Properties: {
			ResponseParameters?: *{
				[string]: *string | fn.#Fn
			} | fn.#If
			ResponseTemplates?: *{
				[string]: *string | fn.#Fn
			} | fn.#If
			ResponseType: *("ACCESS_DENIED" | "API_CONFIGURATION_ERROR" | "AUTHORIZER_CONFIGURATION_ERROR" | "AUTHORIZER_FAILURE" | "BAD_REQUEST_BODY" | "BAD_REQUEST_PARAMETERS" | "DEFAULT_4XX" | "DEFAULT_5XX" | "EXPIRED_TOKEN" | "INTEGRATION_FAILURE" | "INTEGRATION_TIMEOUT" | "INVALID_API_KEY" | "INVALID_SIGNATURE" | "MISSING_AUTHENTICATION_TOKEN" | "QUOTA_EXCEEDED" | "REQUEST_TOO_LARGE" | "RESOURCE_NOT_FOUND" | "THROTTLED" | "UNAUTHORIZED" | "UNSUPPORTED_MEDIA_TYPE" | "WAF_FILTERED") | fn.#Fn
			RestApiId:    *string | fn.#Fn
			StatusCode?:  *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Method: {
		Type: "AWS::ApiGateway::Method"
		Properties: {
			ApiKeyRequired?:      *bool | fn.#Fn
			AuthorizationScopes?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			AuthorizationType?:   *("NONE" | "AWS_IAM" | "CUSTOM" | "COGNITO_USER_POOLS") | fn.#Fn
			AuthorizerId?:        *string | fn.#Fn
			HttpMethod:           *string | fn.#Fn
			Integration?:         *{
				CacheKeyParameters?:    [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				CacheNamespace?:        *string | fn.#Fn
				ConnectionId?:          *string | fn.#Fn
				ConnectionType?:        *("INTERNET" | "VPC_LINK") | fn.#Fn
				ContentHandling?:       *("CONVERT_TO_BINARY" | "CONVERT_TO_TEXT") | fn.#Fn
				Credentials?:           *string | fn.#Fn
				IntegrationHttpMethod?: *string | fn.#Fn
				IntegrationResponses?:  *[...{
					ContentHandling?:    *("CONVERT_TO_BINARY" | "CONVERT_TO_TEXT") | fn.#Fn
					ResponseParameters?: *{
						[string]: *string | fn.#Fn
					} | fn.#If
					ResponseTemplates?: *{
						[string]: *string | fn.#Fn
					} | fn.#If
					SelectionPattern?: *string | fn.#Fn
					StatusCode:        *string | fn.#Fn
				}] | fn.#If
				PassthroughBehavior?: *("WHEN_NO_MATCH" | "WHEN_NO_TEMPLATES" | "NEVER") | fn.#Fn
				RequestParameters?:   *{
					[string]: *string | fn.#Fn
				} | fn.#If
				RequestTemplates?: *{
					[string]: *string | fn.#Fn
				} | fn.#If
				TimeoutInMillis?: *(>=50 & <=29000) | fn.#Fn
				Type?:            *("AWS" | "AWS_PROXY" | "HTTP" | "HTTP_PROXY" | "MOCK") | fn.#Fn
				Uri?:             *string | fn.#Fn
			} | fn.#If
			MethodResponses?: *[...{
				ResponseModels?: *{
					[string]: *string | fn.#Fn
				} | fn.#If
				ResponseParameters?: *{
					[string]: *bool | fn.#Fn
				} | fn.#If
				StatusCode: *string | fn.#Fn
			}] | fn.#If
			OperationName?: *string | fn.#Fn
			RequestModels?: *{
				[string]: *string | fn.#Fn
			} | fn.#If
			RequestParameters?: *{
				[string]: *bool | fn.#Fn
			} | fn.#If
			RequestValidatorId?: *string | fn.#Fn
			ResourceId:          *string | fn.#Fn
			RestApiId:           *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Model: {
		Type: "AWS::ApiGateway::Model"
		Properties: {
			ContentType?: *string | fn.#Fn
			Description?: *string | fn.#Fn
			Name?:        *string | fn.#Fn
			RestApiId:    *string | fn.#Fn
			Schema?:      *{
				[string]: _
			} | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#RequestValidator: {
		Type: "AWS::ApiGateway::RequestValidator"
		Properties: {
			Name?:                      *string | fn.#Fn
			RestApiId:                  *string | fn.#Fn
			ValidateRequestBody?:       *bool | fn.#Fn
			ValidateRequestParameters?: *bool | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Resource: {
		Type: "AWS::ApiGateway::Resource"
		Properties: {
			ParentId:  *string | fn.#Fn
			PathPart:  *string | fn.#Fn
			RestApiId: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#RestApi: {
		Type: "AWS::ApiGateway::RestApi"
		Properties: {
			ApiKeySourceType?: *("AUTHORIZER" | "HEADER") | fn.#Fn
			BinaryMediaTypes?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			Body?:             *{
				[string]: _
			} | fn.#Fn
			BodyS3Location?: *{
				Bucket?:  *string | fn.#Fn
				ETag?:    *string | fn.#Fn
				Key?:     *string | fn.#Fn
				Version?: *string | fn.#Fn
			} | fn.#If
			CloneFrom?:                 *string | fn.#Fn
			Description?:               *string | fn.#Fn
			DisableExecuteApiEndpoint?: *bool | fn.#Fn
			EndpointConfiguration?:     *{
				Types?:          [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				VpcEndpointIds?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			} | fn.#If
			FailOnWarnings?:         *bool | fn.#Fn
			MinimumCompressionSize?: *int | fn.#Fn
			Mode?:                   *string | fn.#Fn
			Name?:                   *string | fn.#Fn
			Parameters?:             *{
				[string]: *string | fn.#Fn
			} | fn.#If
			Policy?: *{
				[string]: _
			} | fn.#Fn
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
	#Stage: {
		Type: "AWS::ApiGateway::Stage"
		Properties: {
			AccessLogSetting?: *{
				DestinationArn?: *string | fn.#Fn
				Format?:         *string | fn.#Fn
			} | fn.#If
			CacheClusterEnabled?: *bool | fn.#Fn
			CacheClusterSize?:    *string | fn.#Fn
			CanarySetting?:       *{
				DeploymentId?:           *string | fn.#Fn
				PercentTraffic?:         *number | fn.#Fn
				StageVariableOverrides?: *{
					[string]: *string | fn.#Fn
				} | fn.#If
				UseStageCache?: *bool | fn.#Fn
			} | fn.#If
			ClientCertificateId?:  *string | fn.#Fn
			DeploymentId?:         *string | fn.#Fn
			Description?:          *string | fn.#Fn
			DocumentationVersion?: *string | fn.#Fn
			MethodSettings?:       *[...{
				CacheDataEncrypted?:   *bool | fn.#Fn
				CacheTtlInSeconds?:    *int | fn.#Fn
				CachingEnabled?:       *bool | fn.#Fn
				DataTraceEnabled?:     *bool | fn.#Fn
				HttpMethod?:           *string | fn.#Fn
				LoggingLevel?:         *string | fn.#Fn
				MetricsEnabled?:       *bool | fn.#Fn
				ResourcePath?:         *string | fn.#Fn
				ThrottlingBurstLimit?: *int | fn.#Fn
				ThrottlingRateLimit?:  *number | fn.#Fn
			}] | fn.#If
			RestApiId:  *string | fn.#Fn
			StageName?: *string | fn.#Fn
			Tags?:      *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			TracingEnabled?: *bool | fn.#Fn
			Variables?:      *{
				[string]: *string | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#UsagePlan: {
		Type: "AWS::ApiGateway::UsagePlan"
		Properties: {
			ApiStages?: *[...{
				ApiId?:    *string | fn.#Fn
				Stage?:    *string | fn.#Fn
				Throttle?: *{
					[string]: {
						BurstLimit?: *int | fn.#Fn
						RateLimit?:  *number | fn.#Fn
					}
				} | fn.#If
			}] | fn.#If
			Description?: *string | fn.#Fn
			Quota?:       *{
				Limit?:  *int | fn.#Fn
				Offset?: *int | fn.#Fn
				Period?: *string | fn.#Fn
			} | fn.#If
			Tags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			Throttle?: *{
				BurstLimit?: *int | fn.#Fn
				RateLimit?:  *number | fn.#Fn
			} | fn.#If
			UsagePlanName?: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#UsagePlanKey: {
		Type: "AWS::ApiGateway::UsagePlanKey"
		Properties: {
			KeyId:       *string | fn.#Fn
			KeyType:     *("API_KEY") | fn.#Fn
			UsagePlanId: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#VpcLink: {
		Type: "AWS::ApiGateway::VpcLink"
		Properties: {
			Description?: *string | fn.#Fn
			Name:         *string | fn.#Fn
			Tags?:        *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			TargetArns: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
