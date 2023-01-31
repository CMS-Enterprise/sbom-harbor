package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#ElasticLoadBalancing: {
	#LoadBalancer: {
		Type: "AWS::ElasticLoadBalancing::LoadBalancer"
		Properties: {
			AccessLoggingPolicy?: *{
				EmitInterval?:   *int | fn.#Fn
				Enabled:         *bool | fn.#Fn
				S3BucketName:    *string | fn.#Fn
				S3BucketPrefix?: *string | fn.#Fn
			} | fn.#If
			AppCookieStickinessPolicy?: *[...{
				CookieName: *string | fn.#Fn
				PolicyName: *string | fn.#Fn
			}] | fn.#If
			AvailabilityZones?:        [...(*(=~#"[a-z0-9-]+"#) | fn.#Fn)] | (*(=~#"[a-z0-9-]+"#) | fn.#Fn)
			ConnectionDrainingPolicy?: *{
				Enabled:  *bool | fn.#Fn
				Timeout?: *int | fn.#Fn
			} | fn.#If
			ConnectionSettings?: *{
				IdleTimeout: *int | fn.#Fn
			} | fn.#If
			CrossZone?:   *bool | fn.#Fn
			HealthCheck?: *{
				HealthyThreshold:   *string | fn.#Fn
				Interval:           *string | fn.#Fn
				Target:             *string | fn.#Fn
				Timeout:            *string | fn.#Fn
				UnhealthyThreshold: *string | fn.#Fn
			} | fn.#If
			Instances?:                [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			LBCookieStickinessPolicy?: *[...{
				CookieExpirationPeriod?: *string | fn.#Fn
				PolicyName?:             *string | fn.#Fn
			}] | fn.#If
			Listeners: *[...{
				InstancePort:      *(>=1 & <=65535) | fn.#Fn
				InstanceProtocol?: *string | fn.#Fn
				LoadBalancerPort:  *(>=1 & <=65535) | fn.#Fn
				PolicyNames?:      [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				Protocol:          *string | fn.#Fn
				SSLCertificateId?: *string | fn.#Fn
			}] | fn.#If
			LoadBalancerName?: *string | fn.#Fn
			Policies?:         *[...{
				Attributes: [...(*{
					[string]: _
				} | fn.#Fn)] | (*{
					[string]: _
				} | fn.#Fn)
				InstancePorts?:     [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				LoadBalancerPorts?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				PolicyName:         *string | fn.#Fn
				PolicyType:         *string | fn.#Fn
			}] | fn.#If
			Scheme?:         *string | fn.#Fn
			SecurityGroups?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			Subnets?:        [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			Tags?:           *[...{
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
