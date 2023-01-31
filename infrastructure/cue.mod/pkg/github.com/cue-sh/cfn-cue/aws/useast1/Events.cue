package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#Events: {
	#ApiDestination: {
		Type: "AWS::Events::ApiDestination"
		Properties: {
			ConnectionArn:                 *string | fn.#Fn
			Description?:                  *string | fn.#Fn
			HttpMethod:                    *("GET" | "HEAD" | "POST" | "OPTIONS" | "PUT" | "DELETE" | "PATCH") | fn.#Fn
			InvocationEndpoint:            *string | fn.#Fn
			InvocationRateLimitPerSecond?: *int | fn.#Fn
			Name?:                         *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Archive: {
		Type: "AWS::Events::Archive"
		Properties: {
			ArchiveName?:  *(strings.MinRunes(1) & strings.MaxRunes(48)) | fn.#Fn
			Description?:  *string | fn.#Fn
			EventPattern?: *{
				[string]: _
			} | fn.#Fn
			RetentionDays?: *int | fn.#Fn
			SourceArn:      *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Connection: {
		Type: "AWS::Events::Connection"
		Properties: {
			AuthParameters: *{
				[string]: _
			} | fn.#Fn
			AuthorizationType: *("API_KEY" | "BASIC" | "OAUTH_CLIENT_CREDENTIALS") | fn.#Fn
			Description?:      *string | fn.#Fn
			Name?:             *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#EventBus: {
		Type: "AWS::Events::EventBus"
		Properties: {
			EventSourceName?: *string | fn.#Fn
			Name:             *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#EventBusPolicy: {
		Type: "AWS::Events::EventBusPolicy"
		Properties: {
			Action?:    *("events:PutEvents") | fn.#Fn
			Condition?: *{
				Key?:   *("aws:PrincipalOrgID") | fn.#Fn
				Type?:  *("StringEquals") | fn.#Fn
				Value?: *string | fn.#Fn
			} | fn.#If
			EventBusName?: *string | fn.#Fn
			Principal?:    *string | fn.#Fn
			Statement?:    *{
				[string]: _
			} | fn.#Fn
			StatementId: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Rule: {
		Type: "AWS::Events::Rule"
		Properties: {
			Description?:  *string | fn.#Fn
			EventBusName?: *string | fn.#Fn
			EventPattern?: *{
				[string]: _
			} | fn.#Fn
			Name?:               *string | fn.#Fn
			RoleArn?:            *string | fn.#Fn
			ScheduleExpression?: *string | fn.#Fn
			State?:              *("DISABLED" | "ENABLED") | fn.#Fn
			Targets?:            *[...{
				Arn:              *string | fn.#Fn
				BatchParameters?: *{
					ArrayProperties?: *{
						Size?: *int | fn.#Fn
					} | fn.#If
					JobDefinition:  *string | fn.#Fn
					JobName:        *string | fn.#Fn
					RetryStrategy?: *{
						Attempts?: *int | fn.#Fn
					} | fn.#If
				} | fn.#If
				DeadLetterConfig?: *{
					Arn?: *string | fn.#Fn
				} | fn.#If
				EcsParameters?: *{
					CapacityProviderStrategy?: *[...{
						Base?:            *int | fn.#Fn
						CapacityProvider: *string | fn.#Fn
						Weight?:          *int | fn.#Fn
					}] | fn.#If
					EnableECSManagedTags?: *bool | fn.#Fn
					EnableExecuteCommand?: *bool | fn.#Fn
					Group?:                *string | fn.#Fn
					LaunchType?:           *string | fn.#Fn
					NetworkConfiguration?: *{
						AwsVpcConfiguration?: *{
							AssignPublicIp?: *string | fn.#Fn
							SecurityGroups?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
							Subnets:         [...(*string | fn.#Fn)] | (*string | fn.#Fn)
						} | fn.#If
					} | fn.#If
					PlacementConstraints?: *[...{
						Expression?: *string | fn.#Fn
						Type?:       *string | fn.#Fn
					}] | fn.#If
					PlacementStrategies?: *[...{
						Field?: *string | fn.#Fn
						Type?:  *string | fn.#Fn
					}] | fn.#If
					PlatformVersion?: *string | fn.#Fn
					PropagateTags?:   *string | fn.#Fn
					ReferenceId?:     *string | fn.#Fn
					TagList?:         *[...{
						Key:   *string | fn.#Fn
						Value: *string | fn.#Fn
					}] | fn.#If
					TaskCount?:        *int | fn.#Fn
					TaskDefinitionArn: *string | fn.#Fn
				} | fn.#If
				HttpParameters?: *{
					HeaderParameters?: *{
						[string]: *string | fn.#Fn
					} | fn.#If
					PathParameterValues?:   [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					QueryStringParameters?: *{
						[string]: *string | fn.#Fn
					} | fn.#If
				} | fn.#If
				Id:                *string | fn.#Fn
				Input?:            *string | fn.#Fn
				InputPath?:        *string | fn.#Fn
				InputTransformer?: *{
					InputPathsMap?: *{
						[string]: *string | fn.#Fn
					} | fn.#If
					InputTemplate: *string | fn.#Fn
				} | fn.#If
				KinesisParameters?: *{
					PartitionKeyPath: *string | fn.#Fn
				} | fn.#If
				RedshiftDataParameters?: *{
					Database:          *string | fn.#Fn
					DbUser?:           *string | fn.#Fn
					SecretManagerArn?: *string | fn.#Fn
					Sql:               *string | fn.#Fn
					StatementName?:    *string | fn.#Fn
					WithEvent?:        *bool | fn.#Fn
				} | fn.#If
				RetryPolicy?: *{
					MaximumEventAgeInSeconds?: *int | fn.#Fn
					MaximumRetryAttempts?:     *int | fn.#Fn
				} | fn.#If
				RoleArn?:              *string | fn.#Fn
				RunCommandParameters?: *{
					RunCommandTargets: *[...{
						Key:    *string | fn.#Fn
						Values: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					}] | fn.#If
				} | fn.#If
				SqsParameters?: *{
					MessageGroupId: *string | fn.#Fn
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
