package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#ApiGatewayV2: {
	#Api: {
		Type: "AWS::ApiGatewayV2::Api"
		Properties: {
			ApiKeySelectionExpression?: *string | fn.#Fn
			BasePath?:                  *string | fn.#Fn
			Body?:                      *{
				[string]: _
			} | fn.#Fn
			BodyS3Location?: *{
				Bucket?:  *string | fn.#Fn
				Etag?:    *string | fn.#Fn
				Key?:     *string | fn.#Fn
				Version?: *string | fn.#Fn
			} | fn.#If
			CorsConfiguration?: *{
				AllowCredentials?: *bool | fn.#Fn
				AllowHeaders?:     [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				AllowMethods?:     [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				AllowOrigins?:     [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				ExposeHeaders?:    [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				MaxAge?:           *int | fn.#Fn
			} | fn.#If
			CredentialsArn?:            *string | fn.#Fn
			Description?:               *string | fn.#Fn
			DisableExecuteApiEndpoint?: *bool | fn.#Fn
			DisableSchemaValidation?:   *bool | fn.#Fn
			FailOnWarnings?:            *bool | fn.#Fn
			Name?:                      *string | fn.#Fn
			ProtocolType?:              *string | fn.#Fn
			RouteKey?:                  *string | fn.#Fn
			RouteSelectionExpression?:  *string | fn.#Fn
			Tags?:                      *{
				[string]: _
			} | fn.#Fn
			Target?:  *string | fn.#Fn
			Version?: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ApiGatewayManagedOverrides: {
		Type: "AWS::ApiGatewayV2::ApiGatewayManagedOverrides"
		Properties: {
			ApiId:        *string | fn.#Fn
			Integration?: *{
				Description?:          *string | fn.#Fn
				IntegrationMethod?:    *string | fn.#Fn
				PayloadFormatVersion?: *string | fn.#Fn
				TimeoutInMillis?:      *int | fn.#Fn
			} | fn.#If
			Route?: *{
				AuthorizationScopes?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				AuthorizationType?:   *string | fn.#Fn
				AuthorizerId?:        *string | fn.#Fn
				OperationName?:       *string | fn.#Fn
				Target?:              *string | fn.#Fn
			} | fn.#If
			Stage?: *{
				AccessLogSettings?: *{
					DestinationArn?: *string | fn.#Fn
					Format?:         *string | fn.#Fn
				} | fn.#If
				AutoDeploy?:           *bool | fn.#Fn
				DefaultRouteSettings?: *{
					DataTraceEnabled?:       *bool | fn.#Fn
					DetailedMetricsEnabled?: *bool | fn.#Fn
					LoggingLevel?:           *string | fn.#Fn
					ThrottlingBurstLimit?:   *int | fn.#Fn
					ThrottlingRateLimit?:    *number | fn.#Fn
				} | fn.#If
				Description?:   *string | fn.#Fn
				RouteSettings?: *{
					[string]: _
				} | fn.#Fn
				StageVariables?: *{
					[string]: _
				} | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ApiMapping: {
		Type: "AWS::ApiGatewayV2::ApiMapping"
		Properties: {
			ApiId:          *string | fn.#Fn
			ApiMappingKey?: *string | fn.#Fn
			DomainName:     *string | fn.#Fn
			Stage:          *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Authorizer: {
		Type: "AWS::ApiGatewayV2::Authorizer"
		Properties: {
			ApiId:                           *string | fn.#Fn
			AuthorizerCredentialsArn?:       *string | fn.#Fn
			AuthorizerPayloadFormatVersion?: *string | fn.#Fn
			AuthorizerResultTtlInSeconds?:   *int | fn.#Fn
			AuthorizerType:                  *string | fn.#Fn
			AuthorizerUri?:                  *string | fn.#Fn
			EnableSimpleResponses?:          *bool | fn.#Fn
			IdentitySource?:                 [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			IdentityValidationExpression?:   *string | fn.#Fn
			JwtConfiguration?:               *{
				Audience?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				Issuer?:   *string | fn.#Fn
			} | fn.#If
			Name: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Deployment: {
		Type: "AWS::ApiGatewayV2::Deployment"
		Properties: {
			ApiId:        *string | fn.#Fn
			Description?: *string | fn.#Fn
			StageName?:   *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#DomainName: {
		Type: "AWS::ApiGatewayV2::DomainName"
		Properties: {
			DomainName:                *string | fn.#Fn
			DomainNameConfigurations?: *[...{
				CertificateArn?:                      *string | fn.#Fn
				CertificateName?:                     *string | fn.#Fn
				EndpointType?:                        *string | fn.#Fn
				OwnershipVerificationCertificateArn?: *string | fn.#Fn
				SecurityPolicy?:                      *string | fn.#Fn
			}] | fn.#If
			MutualTlsAuthentication?: *{
				TruststoreUri?:     *string | fn.#Fn
				TruststoreVersion?: *string | fn.#Fn
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
	#Integration: {
		Type: "AWS::ApiGatewayV2::Integration"
		Properties: {
			ApiId:                    *string | fn.#Fn
			ConnectionId?:            *string | fn.#Fn
			ConnectionType?:          *string | fn.#Fn
			ContentHandlingStrategy?: *string | fn.#Fn
			CredentialsArn?:          *string | fn.#Fn
			Description?:             *string | fn.#Fn
			IntegrationMethod?:       *string | fn.#Fn
			IntegrationSubtype?:      *string | fn.#Fn
			IntegrationType:          *string | fn.#Fn
			IntegrationUri?:          *string | fn.#Fn
			PassthroughBehavior?:     *string | fn.#Fn
			PayloadFormatVersion?:    *string | fn.#Fn
			RequestParameters?:       *{
				[string]: _
			} | fn.#Fn
			RequestTemplates?: *{
				[string]: _
			} | fn.#Fn
			ResponseParameters?: *{
				[string]: _
			} | fn.#Fn
			TemplateSelectionExpression?: *string | fn.#Fn
			TimeoutInMillis?:             *int | fn.#Fn
			TlsConfig?:                   *{
				ServerNameToVerify?: *string | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#IntegrationResponse: {
		Type: "AWS::ApiGatewayV2::IntegrationResponse"
		Properties: {
			ApiId:                    *string | fn.#Fn
			ContentHandlingStrategy?: *string | fn.#Fn
			IntegrationId:            *string | fn.#Fn
			IntegrationResponseKey:   *string | fn.#Fn
			ResponseParameters?:      *{
				[string]: _
			} | fn.#Fn
			ResponseTemplates?: *{
				[string]: _
			} | fn.#Fn
			TemplateSelectionExpression?: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Model: {
		Type: "AWS::ApiGatewayV2::Model"
		Properties: {
			ApiId:        *string | fn.#Fn
			ContentType?: *string | fn.#Fn
			Description?: *string | fn.#Fn
			Name:         *string | fn.#Fn
			Schema:       *{
				[string]: _
			} | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Route: {
		Type: "AWS::ApiGatewayV2::Route"
		Properties: {
			ApiId:                     *string | fn.#Fn
			ApiKeyRequired?:           *bool | fn.#Fn
			AuthorizationScopes?:      [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			AuthorizationType?:        *string | fn.#Fn
			AuthorizerId?:             *string | fn.#Fn
			ModelSelectionExpression?: *string | fn.#Fn
			OperationName?:            *string | fn.#Fn
			RequestModels?:            *{
				[string]: _
			} | fn.#Fn
			RequestParameters?: *{
				[string]: _
			} | fn.#Fn
			RouteKey:                          *string | fn.#Fn
			RouteResponseSelectionExpression?: *string | fn.#Fn
			Target?:                           *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#RouteResponse: {
		Type: "AWS::ApiGatewayV2::RouteResponse"
		Properties: {
			ApiId:                     *string | fn.#Fn
			ModelSelectionExpression?: *string | fn.#Fn
			ResponseModels?:           *{
				[string]: _
			} | fn.#Fn
			ResponseParameters?: *{
				[string]: _
			} | fn.#Fn
			RouteId:          *string | fn.#Fn
			RouteResponseKey: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Stage: {
		Type: "AWS::ApiGatewayV2::Stage"
		Properties: {
			AccessLogSettings?: *{
				DestinationArn?: *string | fn.#Fn
				Format?:         *string | fn.#Fn
			} | fn.#If
			AccessPolicyId?:       *string | fn.#Fn
			ApiId:                 *string | fn.#Fn
			AutoDeploy?:           *bool | fn.#Fn
			ClientCertificateId?:  *string | fn.#Fn
			DefaultRouteSettings?: *{
				DataTraceEnabled?:       *bool | fn.#Fn
				DetailedMetricsEnabled?: *bool | fn.#Fn
				LoggingLevel?:           *string | fn.#Fn
				ThrottlingBurstLimit?:   *int | fn.#Fn
				ThrottlingRateLimit?:    *number | fn.#Fn
			} | fn.#If
			DeploymentId?:  *string | fn.#Fn
			Description?:   *string | fn.#Fn
			RouteSettings?: *{
				[string]: _
			} | fn.#Fn
			StageName:       *string | fn.#Fn
			StageVariables?: *{
				[string]: _
			} | fn.#Fn
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
	#VpcLink: {
		Type: "AWS::ApiGatewayV2::VpcLink"
		Properties: {
			Name:              *string | fn.#Fn
			SecurityGroupIds?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			SubnetIds:         [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			Tags?:             *{
				[string]: _
			} | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
