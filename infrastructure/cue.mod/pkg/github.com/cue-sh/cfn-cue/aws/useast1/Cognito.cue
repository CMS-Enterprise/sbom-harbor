package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#Cognito: {
	#IdentityPool: {
		Type: "AWS::Cognito::IdentityPool"
		Properties: {
			AllowClassicFlow?:              *bool | fn.#Fn
			AllowUnauthenticatedIdentities: *bool | fn.#Fn
			CognitoEvents?:                 *{
				[string]: _
			} | fn.#Fn
			CognitoIdentityProviders?: *[...{
				ClientId?:             *string | fn.#Fn
				ProviderName?:         *string | fn.#Fn
				ServerSideTokenCheck?: *bool | fn.#Fn
			}] | fn.#If
			CognitoStreams?: *{
				RoleArn?:         *string | fn.#Fn
				StreamName?:      *string | fn.#Fn
				StreamingStatus?: *("DISABLED" | "ENABLED") | fn.#Fn
			} | fn.#If
			DeveloperProviderName?:     *string | fn.#Fn
			IdentityPoolName?:          *string | fn.#Fn
			OpenIdConnectProviderARNs?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			PushSync?:                  *{
				ApplicationArns?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				RoleArn?:         *string | fn.#Fn
			} | fn.#If
			SamlProviderARNs?:        [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			SupportedLoginProviders?: *{
				[string]: _
			} | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#IdentityPoolRoleAttachment: {
		Type: "AWS::Cognito::IdentityPoolRoleAttachment"
		Properties: {
			IdentityPoolId: *string | fn.#Fn
			RoleMappings?:  *{
				[string]: _
			} | fn.#Fn
			Roles?: *{
				[string]: _
			} | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#UserPool: {
		Type: "AWS::Cognito::UserPool"
		Properties: {
			AccountRecoverySetting?: *{
				RecoveryMechanisms?: *[...{
					Name?:     *string | fn.#Fn
					Priority?: *int | fn.#Fn
				}] | fn.#If
			} | fn.#If
			AdminCreateUserConfig?: *{
				AllowAdminCreateUserOnly?: *bool | fn.#Fn
				InviteMessageTemplate?:    *{
					EmailMessage?: *string | fn.#Fn
					EmailSubject?: *string | fn.#Fn
					SMSMessage?:   *string | fn.#Fn
				} | fn.#If
				UnusedAccountValidityDays?: *int | fn.#Fn
			} | fn.#If
			AliasAttributes?:        [...(*("email" | "phone_number" | "preferred_username") | fn.#Fn)] | (*("email" | "phone_number" | "preferred_username") | fn.#Fn)
			AutoVerifiedAttributes?: [...(*("email" | "phone_number") | fn.#Fn)] | (*("email" | "phone_number") | fn.#Fn)
			DeviceConfiguration?:    *{
				ChallengeRequiredOnNewDevice?:     *bool | fn.#Fn
				DeviceOnlyRememberedOnUserPrompt?: *bool | fn.#Fn
			} | fn.#If
			EmailConfiguration?: *{
				ConfigurationSet?:    *string | fn.#Fn
				EmailSendingAccount?: *string | fn.#Fn
				From?:                *string | fn.#Fn
				ReplyToEmailAddress?: *string | fn.#Fn
				SourceArn?:           *string | fn.#Fn
			} | fn.#If
			EmailVerificationMessage?: *string | fn.#Fn
			EmailVerificationSubject?: *string | fn.#Fn
			EnabledMfas?:              [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			LambdaConfig?:             *{
				CreateAuthChallenge?: *string | fn.#Fn
				CustomEmailSender?:   *{
					LambdaArn?:     *string | fn.#Fn
					LambdaVersion?: *string | fn.#Fn
				} | fn.#If
				CustomMessage?:   *string | fn.#Fn
				CustomSMSSender?: *{
					LambdaArn?:     *string | fn.#Fn
					LambdaVersion?: *string | fn.#Fn
				} | fn.#If
				DefineAuthChallenge?:         *string | fn.#Fn
				KMSKeyID?:                    *string | fn.#Fn
				PostAuthentication?:          *string | fn.#Fn
				PostConfirmation?:            *string | fn.#Fn
				PreAuthentication?:           *string | fn.#Fn
				PreSignUp?:                   *string | fn.#Fn
				PreTokenGeneration?:          *string | fn.#Fn
				UserMigration?:               *string | fn.#Fn
				VerifyAuthChallengeResponse?: *string | fn.#Fn
			} | fn.#If
			MfaConfiguration?: *("OFF" | "ON" | "OPTIONAL") | fn.#Fn
			Policies?:         *{
				PasswordPolicy?: *{
					MinimumLength?:                 *int | fn.#Fn
					RequireLowercase?:              *bool | fn.#Fn
					RequireNumbers?:                *bool | fn.#Fn
					RequireSymbols?:                *bool | fn.#Fn
					RequireUppercase?:              *bool | fn.#Fn
					TemporaryPasswordValidityDays?: *int | fn.#Fn
				} | fn.#If
			} | fn.#If
			Schema?: *[...{
				AttributeDataType?:          *("Boolean" | "DateTime" | "Number" | "String") | fn.#Fn
				DeveloperOnlyAttribute?:     *bool | fn.#Fn
				Mutable?:                    *bool | fn.#Fn
				Name?:                       *string | fn.#Fn
				NumberAttributeConstraints?: *{
					MaxValue?: *string | fn.#Fn
					MinValue?: *string | fn.#Fn
				} | fn.#If
				Required?:                   *bool | fn.#Fn
				StringAttributeConstraints?: *{
					MaxLength?: *string | fn.#Fn
					MinLength?: *string | fn.#Fn
				} | fn.#If
			}] | fn.#If
			SmsAuthenticationMessage?: *string | fn.#Fn
			SmsConfiguration?:         *{
				ExternalId:    *string | fn.#Fn
				SnsCallerArn?: *string | fn.#Fn
			} | fn.#If
			SmsVerificationMessage?: *string | fn.#Fn
			UserPoolAddOns?:         *{
				AdvancedSecurityMode?: *string | fn.#Fn
			} | fn.#If
			UserPoolName?: *string | fn.#Fn
			UserPoolTags?: *{
				[string]: _
			} | fn.#Fn
			UsernameAttributes?:    [...(*("email" | "phone_number") | fn.#Fn)] | (*("email" | "phone_number") | fn.#Fn)
			UsernameConfiguration?: *{
				CaseSensitive?: *bool | fn.#Fn
			} | fn.#If
			VerificationMessageTemplate?: *{
				DefaultEmailOption?: *string | fn.#Fn
				EmailMessage?:       *string | fn.#Fn
				EmailMessageByLink?: *string | fn.#Fn
				EmailSubject?:       *string | fn.#Fn
				EmailSubjectByLink?: *string | fn.#Fn
				SmsMessage?:         *string | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#UserPoolClient: {
		Type: "AWS::Cognito::UserPoolClient"
		Properties: {
			AccessTokenValidity?:             *int | fn.#Fn
			AllowedOAuthFlows?:               [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			AllowedOAuthFlowsUserPoolClient?: *bool | fn.#Fn
			AllowedOAuthScopes?:              [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			AnalyticsConfiguration?:          *{
				ApplicationArn?: *string | fn.#Fn
				ApplicationId?:  *string | fn.#Fn
				ExternalId?:     *string | fn.#Fn
				RoleArn?:        *string | fn.#Fn
				UserDataShared?: *bool | fn.#Fn
			} | fn.#If
			CallbackURLs?:               [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			ClientName?:                 *string | fn.#Fn
			DefaultRedirectURI?:         *string | fn.#Fn
			EnableTokenRevocation?:      *bool | fn.#Fn
			ExplicitAuthFlows?:          [...(*("ADMIN_NO_SRP_AUTH" | "ALLOW_ADMIN_USER_PASSWORD_AUTH" | "ALLOW_CUSTOM_AUTH" | "ALLOW_REFRESH_TOKEN_AUTH" | "ALLOW_USER_PASSWORD_AUTH" | "ALLOW_USER_SRP_AUTH" | "CUSTOM_AUTH_FLOW_ONLY" | "USER_PASSWORD_AUTH") | fn.#Fn)] | (*("ADMIN_NO_SRP_AUTH" | "ALLOW_ADMIN_USER_PASSWORD_AUTH" | "ALLOW_CUSTOM_AUTH" | "ALLOW_REFRESH_TOKEN_AUTH" | "ALLOW_USER_PASSWORD_AUTH" | "ALLOW_USER_SRP_AUTH" | "CUSTOM_AUTH_FLOW_ONLY" | "USER_PASSWORD_AUTH") | fn.#Fn)
			GenerateSecret?:             *bool | fn.#Fn
			IdTokenValidity?:            *int | fn.#Fn
			LogoutURLs?:                 [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			PreventUserExistenceErrors?: *string | fn.#Fn
			ReadAttributes?:             [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			RefreshTokenValidity?:       *(>=0 & <=3650) | fn.#Fn
			SupportedIdentityProviders?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			TokenValidityUnits?:         *{
				AccessToken?:  *string | fn.#Fn
				IdToken?:      *string | fn.#Fn
				RefreshToken?: *string | fn.#Fn
			} | fn.#If
			UserPoolId:       *string | fn.#Fn
			WriteAttributes?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#UserPoolDomain: {
		Type: "AWS::Cognito::UserPoolDomain"
		Properties: {
			CustomDomainConfig?: *{
				CertificateArn?: *string | fn.#Fn
			} | fn.#If
			Domain:     *string | fn.#Fn
			UserPoolId: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#UserPoolGroup: {
		Type: "AWS::Cognito::UserPoolGroup"
		Properties: {
			Description?: *string | fn.#Fn
			GroupName?:   *string | fn.#Fn
			Precedence?:  *number | fn.#Fn
			RoleArn?:     *string | fn.#Fn
			UserPoolId:   *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#UserPoolIdentityProvider: {
		Type: "AWS::Cognito::UserPoolIdentityProvider"
		Properties: {
			AttributeMapping?: *{
				[string]: _
			} | fn.#Fn
			IdpIdentifiers?:  [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			ProviderDetails?: *{
				[string]: _
			} | fn.#Fn
			ProviderName: *string | fn.#Fn
			ProviderType: *string | fn.#Fn
			UserPoolId:   *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#UserPoolResourceServer: {
		Type: "AWS::Cognito::UserPoolResourceServer"
		Properties: {
			Identifier: *string | fn.#Fn
			Name:       *string | fn.#Fn
			Scopes?:    *[...{
				ScopeDescription: *string | fn.#Fn
				ScopeName:        *string | fn.#Fn
			}] | fn.#If
			UserPoolId: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#UserPoolRiskConfigurationAttachment: {
		Type: "AWS::Cognito::UserPoolRiskConfigurationAttachment"
		Properties: {
			AccountTakeoverRiskConfiguration?: *{
				Actions: *{
					HighAction?: *{
						EventAction: *string | fn.#Fn
						Notify:      *bool | fn.#Fn
					} | fn.#If
					LowAction?: *{
						EventAction: *string | fn.#Fn
						Notify:      *bool | fn.#Fn
					} | fn.#If
					MediumAction?: *{
						EventAction: *string | fn.#Fn
						Notify:      *bool | fn.#Fn
					} | fn.#If
				} | fn.#If
				NotifyConfiguration?: *{
					BlockEmail?: *{
						HtmlBody?: *string | fn.#Fn
						Subject:   *string | fn.#Fn
						TextBody?: *string | fn.#Fn
					} | fn.#If
					From?:     *string | fn.#Fn
					MfaEmail?: *{
						HtmlBody?: *string | fn.#Fn
						Subject:   *string | fn.#Fn
						TextBody?: *string | fn.#Fn
					} | fn.#If
					NoActionEmail?: *{
						HtmlBody?: *string | fn.#Fn
						Subject:   *string | fn.#Fn
						TextBody?: *string | fn.#Fn
					} | fn.#If
					ReplyTo?:  *string | fn.#Fn
					SourceArn: *string | fn.#Fn
				} | fn.#If
			} | fn.#If
			ClientId:                                 *string | fn.#Fn
			CompromisedCredentialsRiskConfiguration?: *{
				Actions: *{
					EventAction: *string | fn.#Fn
				} | fn.#If
				EventFilter?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			} | fn.#If
			RiskExceptionConfiguration?: *{
				BlockedIPRangeList?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				SkippedIPRangeList?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			} | fn.#If
			UserPoolId: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#UserPoolUICustomizationAttachment: {
		Type: "AWS::Cognito::UserPoolUICustomizationAttachment"
		Properties: {
			CSS?:       *string | fn.#Fn
			ClientId:   *string | fn.#Fn
			UserPoolId: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#UserPoolUser: {
		Type: "AWS::Cognito::UserPoolUser"
		Properties: {
			ClientMetadata?: *{
				[string]: _
			} | fn.#Fn
			DesiredDeliveryMediums?: [...(*("EMAIL" | "SMS") | fn.#Fn)] | (*("EMAIL" | "SMS") | fn.#Fn)
			ForceAliasCreation?:     *bool | fn.#Fn
			MessageAction?:          *("RESEND" | "SUPPRESS") | fn.#Fn
			UserAttributes?:         *[...{
				Name?:  *string | fn.#Fn
				Value?: *string | fn.#Fn
			}] | fn.#If
			UserPoolId:      *string | fn.#Fn
			Username?:       *string | fn.#Fn
			ValidationData?: *[...{
				Name?:  *string | fn.#Fn
				Value?: *string | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#UserPoolUserToGroupAttachment: {
		Type: "AWS::Cognito::UserPoolUserToGroupAttachment"
		Properties: {
			GroupName:  *string | fn.#Fn
			UserPoolId: *string | fn.#Fn
			Username:   *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
