package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#ElasticLoadBalancingV2: {
	#Listener: {
		Type: "AWS::ElasticLoadBalancingV2::Listener"
		Properties: {
			AlpnPolicy?:   [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			Certificates?: *[...{
				CertificateArn?: *string | fn.#Fn
			}] | fn.#If
			DefaultActions: *[...{
				AuthenticateCognitoConfig?: *{
					AuthenticationRequestExtraParams?: *{
						[string]: *string | fn.#Fn
					} | fn.#If
					OnUnauthenticatedRequest?: *string | fn.#Fn
					Scope?:                    *string | fn.#Fn
					SessionCookieName?:        *string | fn.#Fn
					SessionTimeout?:           *string | fn.#Fn
					UserPoolArn:               *string | fn.#Fn
					UserPoolClientId:          *string | fn.#Fn
					UserPoolDomain:            *string | fn.#Fn
				} | fn.#If
				AuthenticateOidcConfig?: *{
					AuthenticationRequestExtraParams?: *{
						[string]: *string | fn.#Fn
					} | fn.#If
					AuthorizationEndpoint:     *string | fn.#Fn
					ClientId:                  *string | fn.#Fn
					ClientSecret:              *string | fn.#Fn
					Issuer:                    *string | fn.#Fn
					OnUnauthenticatedRequest?: *string | fn.#Fn
					Scope?:                    *string | fn.#Fn
					SessionCookieName?:        *string | fn.#Fn
					SessionTimeout?:           *string | fn.#Fn
					TokenEndpoint:             *string | fn.#Fn
					UserInfoEndpoint:          *string | fn.#Fn
				} | fn.#If
				FixedResponseConfig?: *{
					ContentType?: *string | fn.#Fn
					MessageBody?: *string | fn.#Fn
					StatusCode:   *string | fn.#Fn
				} | fn.#If
				ForwardConfig?: *{
					TargetGroupStickinessConfig?: *{
						DurationSeconds?: *int | fn.#Fn
						Enabled?:         *bool | fn.#Fn
					} | fn.#If
					TargetGroups?: *[...{
						TargetGroupArn?: *string | fn.#Fn
						Weight?:         *int | fn.#Fn
					}] | fn.#If
				} | fn.#If
				Order?:          *int | fn.#Fn
				RedirectConfig?: *{
					Host?:      *string | fn.#Fn
					Path?:      *string | fn.#Fn
					Port?:      *string | fn.#Fn
					Protocol?:  *string | fn.#Fn
					Query?:     *string | fn.#Fn
					StatusCode: *string | fn.#Fn
				} | fn.#If
				TargetGroupArn?: *string | fn.#Fn
				Type:            *string | fn.#Fn
			}] | fn.#If
			LoadBalancerArn: *string | fn.#Fn
			Port?:           *(>=1 & <=65535) | fn.#Fn
			Protocol?:       *string | fn.#Fn
			SslPolicy?:      *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ListenerCertificate: {
		Type: "AWS::ElasticLoadBalancingV2::ListenerCertificate"
		Properties: {
			Certificates: *[...{
				CertificateArn?: *string | fn.#Fn
			}] | fn.#If
			ListenerArn: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ListenerRule: {
		Type: "AWS::ElasticLoadBalancingV2::ListenerRule"
		Properties: {
			Actions: *[...{
				AuthenticateCognitoConfig?: *{
					AuthenticationRequestExtraParams?: *{
						[string]: *string | fn.#Fn
					} | fn.#If
					OnUnauthenticatedRequest?: *string | fn.#Fn
					Scope?:                    *string | fn.#Fn
					SessionCookieName?:        *string | fn.#Fn
					SessionTimeout?:           *int | fn.#Fn
					UserPoolArn:               *string | fn.#Fn
					UserPoolClientId:          *string | fn.#Fn
					UserPoolDomain:            *string | fn.#Fn
				} | fn.#If
				AuthenticateOidcConfig?: *{
					AuthenticationRequestExtraParams?: *{
						[string]: *string | fn.#Fn
					} | fn.#If
					AuthorizationEndpoint:     *string | fn.#Fn
					ClientId:                  *string | fn.#Fn
					ClientSecret:              *string | fn.#Fn
					Issuer:                    *string | fn.#Fn
					OnUnauthenticatedRequest?: *string | fn.#Fn
					Scope?:                    *string | fn.#Fn
					SessionCookieName?:        *string | fn.#Fn
					SessionTimeout?:           *int | fn.#Fn
					TokenEndpoint:             *string | fn.#Fn
					UseExistingClientSecret?:  *bool | fn.#Fn
					UserInfoEndpoint:          *string | fn.#Fn
				} | fn.#If
				FixedResponseConfig?: *{
					ContentType?: *string | fn.#Fn
					MessageBody?: *string | fn.#Fn
					StatusCode:   *string | fn.#Fn
				} | fn.#If
				ForwardConfig?: *{
					TargetGroupStickinessConfig?: *{
						DurationSeconds?: *int | fn.#Fn
						Enabled?:         *bool | fn.#Fn
					} | fn.#If
					TargetGroups?: *[...{
						TargetGroupArn?: *string | fn.#Fn
						Weight?:         *int | fn.#Fn
					}] | fn.#If
				} | fn.#If
				Order?:          *int | fn.#Fn
				RedirectConfig?: *{
					Host?:      *string | fn.#Fn
					Path?:      *string | fn.#Fn
					Port?:      *string | fn.#Fn
					Protocol?:  *string | fn.#Fn
					Query?:     *string | fn.#Fn
					StatusCode: *string | fn.#Fn
				} | fn.#If
				TargetGroupArn?: *string | fn.#Fn
				Type:            *string | fn.#Fn
			}] | fn.#If
			Conditions: *[...{
				Field?:            *string | fn.#Fn
				HostHeaderConfig?: *{
					Values?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				} | fn.#If
				HttpHeaderConfig?: *{
					HttpHeaderName?: *string | fn.#Fn
					Values?:         [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				} | fn.#If
				HttpRequestMethodConfig?: *{
					Values?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				} | fn.#If
				PathPatternConfig?: *{
					Values?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				} | fn.#If
				QueryStringConfig?: *{
					Values?: *[...{
						Key?:   *string | fn.#Fn
						Value?: *string | fn.#Fn
					}] | fn.#If
				} | fn.#If
				SourceIpConfig?: *{
					Values?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				} | fn.#If
				Values?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			}] | fn.#If
			ListenerArn: *string | fn.#Fn
			Priority:    *(>=1 & <=50000) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#LoadBalancer: {
		Type: "AWS::ElasticLoadBalancingV2::LoadBalancer"
		Properties: {
			IpAddressType?:          *string | fn.#Fn
			LoadBalancerAttributes?: *[...{
				Key?:   *string | fn.#Fn
				Value?: *string | fn.#Fn
			}] | fn.#If
			Name?:           *string | fn.#Fn
			Scheme?:         *string | fn.#Fn
			SecurityGroups?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			SubnetMappings?: *[...{
				AllocationId?:       *string | fn.#Fn
				IPv6Address?:        *string | fn.#Fn
				PrivateIPv4Address?: *string | fn.#Fn
				SubnetId:            *string | fn.#Fn
			}] | fn.#If
			Subnets?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			Tags?:    *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			Type?: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#TargetGroup: {
		Type: "AWS::ElasticLoadBalancingV2::TargetGroup"
		Properties: {
			HealthCheckEnabled?:         *bool | fn.#Fn
			HealthCheckIntervalSeconds?: *(>=5 & <=300) | fn.#Fn
			HealthCheckPath?:            *string | fn.#Fn
			HealthCheckPort?:            *string | fn.#Fn
			HealthCheckProtocol?:        *string | fn.#Fn
			HealthCheckTimeoutSeconds?:  *(>=2 & <=120) | fn.#Fn
			HealthyThresholdCount?:      *(>=2 & <=10) | fn.#Fn
			IpAddressType?:              *string | fn.#Fn
			Matcher?:                    *{
				GrpcCode?: *string | fn.#Fn
				HttpCode?: *string | fn.#Fn
			} | fn.#If
			Name?:            *string | fn.#Fn
			Port?:            *int | fn.#Fn
			Protocol?:        *string | fn.#Fn
			ProtocolVersion?: *string | fn.#Fn
			Tags?:            *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			TargetGroupAttributes?: *[...{
				Key?:   *string | fn.#Fn
				Value?: *string | fn.#Fn
			}] | fn.#If
			TargetType?: *string | fn.#Fn
			Targets?:    *[...{
				AvailabilityZone?: *(=~#"[a-z0-9-]+"#) | fn.#Fn
				Id:                *string | fn.#Fn
				Port?:             *int | fn.#Fn
			}] | fn.#If
			UnhealthyThresholdCount?: *(>=2 & <=10) | fn.#Fn
			VpcId?:                   *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
