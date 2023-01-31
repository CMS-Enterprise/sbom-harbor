package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#IoTEvents: {
	#DetectorModel: {
		Type: "AWS::IoTEvents::DetectorModel"
		Properties: {
			DetectorModelDefinition: *{
				InitialStateName: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
				States:           *[...{
					OnEnter?: *{
						Events?: *[...{
							Actions?: *[...{
								ClearTimer?: *{
									TimerName: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
								} | fn.#If
								DynamoDB?: *{
									HashKeyField: *string | fn.#Fn
									HashKeyType?: *string | fn.#Fn
									HashKeyValue: *string | fn.#Fn
									Operation?:   *string | fn.#Fn
									Payload?:     *{
										ContentExpression: *string | fn.#Fn
										Type:              *string | fn.#Fn
									} | fn.#If
									PayloadField?:  *string | fn.#Fn
									RangeKeyField?: *string | fn.#Fn
									RangeKeyType?:  *string | fn.#Fn
									RangeKeyValue?: *string | fn.#Fn
									TableName:      *string | fn.#Fn
								} | fn.#If
								DynamoDBv2?: *{
									Payload?: *{
										ContentExpression: *string | fn.#Fn
										Type:              *string | fn.#Fn
									} | fn.#If
									TableName: *string | fn.#Fn
								} | fn.#If
								Firehose?: *{
									DeliveryStreamName: *string | fn.#Fn
									Payload?:           *{
										ContentExpression: *string | fn.#Fn
										Type:              *string | fn.#Fn
									} | fn.#If
									Separator?: *(=~#"([\n\t])|(\r\n)|(,)"#) | fn.#Fn
								} | fn.#If
								IotEvents?: *{
									InputName: *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[a-zA-Z][a-zA-Z0-9_]*$"#)) | fn.#Fn
									Payload?:  *{
										ContentExpression: *string | fn.#Fn
										Type:              *string | fn.#Fn
									} | fn.#If
								} | fn.#If
								IotSiteWise?: *{
									AssetId?:       *string | fn.#Fn
									EntryId?:       *string | fn.#Fn
									PropertyAlias?: *string | fn.#Fn
									PropertyId?:    *string | fn.#Fn
									PropertyValue:  *{
										Quality?:   *string | fn.#Fn
										Timestamp?: *{
											OffsetInNanos?: *string | fn.#Fn
											TimeInSeconds:  *string | fn.#Fn
										} | fn.#If
										Value: *{
											BooleanValue?: *string | fn.#Fn
											DoubleValue?:  *string | fn.#Fn
											IntegerValue?: *string | fn.#Fn
											StringValue?:  *string | fn.#Fn
										} | fn.#If
									} | fn.#If
								} | fn.#If
								IotTopicPublish?: *{
									MqttTopic: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
									Payload?:  *{
										ContentExpression: *string | fn.#Fn
										Type:              *string | fn.#Fn
									} | fn.#If
								} | fn.#If
								Lambda?: *{
									FunctionArn: *(strings.MinRunes(1) & strings.MaxRunes(2048)) | fn.#Fn
									Payload?:    *{
										ContentExpression: *string | fn.#Fn
										Type:              *string | fn.#Fn
									} | fn.#If
								} | fn.#If
								ResetTimer?: *{
									TimerName: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
								} | fn.#If
								SetTimer?: *{
									DurationExpression?: *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
									Seconds?:            *(>=60 & <=31622400) | fn.#Fn
									TimerName:           *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
								} | fn.#If
								SetVariable?: *{
									Value:        *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
									VariableName: *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[a-zA-Z][a-zA-Z0-9_]*$"#)) | fn.#Fn
								} | fn.#If
								Sns?: *{
									Payload?: *{
										ContentExpression: *string | fn.#Fn
										Type:              *string | fn.#Fn
									} | fn.#If
									TargetArn: *(strings.MinRunes(1) & strings.MaxRunes(2048)) | fn.#Fn
								} | fn.#If
								Sqs?: *{
									Payload?: *{
										ContentExpression: *string | fn.#Fn
										Type:              *string | fn.#Fn
									} | fn.#If
									QueueUrl:   *string | fn.#Fn
									UseBase64?: *bool | fn.#Fn
								} | fn.#If
							}] | fn.#If
							Condition?: *string | fn.#Fn
							EventName:  *string | fn.#Fn
						}] | fn.#If
					} | fn.#If
					OnExit?: *{
						Events?: *[...{
							Actions?: *[...{
								ClearTimer?: *{
									TimerName: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
								} | fn.#If
								DynamoDB?: *{
									HashKeyField: *string | fn.#Fn
									HashKeyType?: *string | fn.#Fn
									HashKeyValue: *string | fn.#Fn
									Operation?:   *string | fn.#Fn
									Payload?:     *{
										ContentExpression: *string | fn.#Fn
										Type:              *string | fn.#Fn
									} | fn.#If
									PayloadField?:  *string | fn.#Fn
									RangeKeyField?: *string | fn.#Fn
									RangeKeyType?:  *string | fn.#Fn
									RangeKeyValue?: *string | fn.#Fn
									TableName:      *string | fn.#Fn
								} | fn.#If
								DynamoDBv2?: *{
									Payload?: *{
										ContentExpression: *string | fn.#Fn
										Type:              *string | fn.#Fn
									} | fn.#If
									TableName: *string | fn.#Fn
								} | fn.#If
								Firehose?: *{
									DeliveryStreamName: *string | fn.#Fn
									Payload?:           *{
										ContentExpression: *string | fn.#Fn
										Type:              *string | fn.#Fn
									} | fn.#If
									Separator?: *(=~#"([\n\t])|(\r\n)|(,)"#) | fn.#Fn
								} | fn.#If
								IotEvents?: *{
									InputName: *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[a-zA-Z][a-zA-Z0-9_]*$"#)) | fn.#Fn
									Payload?:  *{
										ContentExpression: *string | fn.#Fn
										Type:              *string | fn.#Fn
									} | fn.#If
								} | fn.#If
								IotSiteWise?: *{
									AssetId?:       *string | fn.#Fn
									EntryId?:       *string | fn.#Fn
									PropertyAlias?: *string | fn.#Fn
									PropertyId?:    *string | fn.#Fn
									PropertyValue:  *{
										Quality?:   *string | fn.#Fn
										Timestamp?: *{
											OffsetInNanos?: *string | fn.#Fn
											TimeInSeconds:  *string | fn.#Fn
										} | fn.#If
										Value: *{
											BooleanValue?: *string | fn.#Fn
											DoubleValue?:  *string | fn.#Fn
											IntegerValue?: *string | fn.#Fn
											StringValue?:  *string | fn.#Fn
										} | fn.#If
									} | fn.#If
								} | fn.#If
								IotTopicPublish?: *{
									MqttTopic: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
									Payload?:  *{
										ContentExpression: *string | fn.#Fn
										Type:              *string | fn.#Fn
									} | fn.#If
								} | fn.#If
								Lambda?: *{
									FunctionArn: *(strings.MinRunes(1) & strings.MaxRunes(2048)) | fn.#Fn
									Payload?:    *{
										ContentExpression: *string | fn.#Fn
										Type:              *string | fn.#Fn
									} | fn.#If
								} | fn.#If
								ResetTimer?: *{
									TimerName: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
								} | fn.#If
								SetTimer?: *{
									DurationExpression?: *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
									Seconds?:            *(>=60 & <=31622400) | fn.#Fn
									TimerName:           *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
								} | fn.#If
								SetVariable?: *{
									Value:        *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
									VariableName: *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[a-zA-Z][a-zA-Z0-9_]*$"#)) | fn.#Fn
								} | fn.#If
								Sns?: *{
									Payload?: *{
										ContentExpression: *string | fn.#Fn
										Type:              *string | fn.#Fn
									} | fn.#If
									TargetArn: *(strings.MinRunes(1) & strings.MaxRunes(2048)) | fn.#Fn
								} | fn.#If
								Sqs?: *{
									Payload?: *{
										ContentExpression: *string | fn.#Fn
										Type:              *string | fn.#Fn
									} | fn.#If
									QueueUrl:   *string | fn.#Fn
									UseBase64?: *bool | fn.#Fn
								} | fn.#If
							}] | fn.#If
							Condition?: *string | fn.#Fn
							EventName:  *string | fn.#Fn
						}] | fn.#If
					} | fn.#If
					OnInput?: *{
						Events?: *[...{
							Actions?: *[...{
								ClearTimer?: *{
									TimerName: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
								} | fn.#If
								DynamoDB?: *{
									HashKeyField: *string | fn.#Fn
									HashKeyType?: *string | fn.#Fn
									HashKeyValue: *string | fn.#Fn
									Operation?:   *string | fn.#Fn
									Payload?:     *{
										ContentExpression: *string | fn.#Fn
										Type:              *string | fn.#Fn
									} | fn.#If
									PayloadField?:  *string | fn.#Fn
									RangeKeyField?: *string | fn.#Fn
									RangeKeyType?:  *string | fn.#Fn
									RangeKeyValue?: *string | fn.#Fn
									TableName:      *string | fn.#Fn
								} | fn.#If
								DynamoDBv2?: *{
									Payload?: *{
										ContentExpression: *string | fn.#Fn
										Type:              *string | fn.#Fn
									} | fn.#If
									TableName: *string | fn.#Fn
								} | fn.#If
								Firehose?: *{
									DeliveryStreamName: *string | fn.#Fn
									Payload?:           *{
										ContentExpression: *string | fn.#Fn
										Type:              *string | fn.#Fn
									} | fn.#If
									Separator?: *(=~#"([\n\t])|(\r\n)|(,)"#) | fn.#Fn
								} | fn.#If
								IotEvents?: *{
									InputName: *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[a-zA-Z][a-zA-Z0-9_]*$"#)) | fn.#Fn
									Payload?:  *{
										ContentExpression: *string | fn.#Fn
										Type:              *string | fn.#Fn
									} | fn.#If
								} | fn.#If
								IotSiteWise?: *{
									AssetId?:       *string | fn.#Fn
									EntryId?:       *string | fn.#Fn
									PropertyAlias?: *string | fn.#Fn
									PropertyId?:    *string | fn.#Fn
									PropertyValue:  *{
										Quality?:   *string | fn.#Fn
										Timestamp?: *{
											OffsetInNanos?: *string | fn.#Fn
											TimeInSeconds:  *string | fn.#Fn
										} | fn.#If
										Value: *{
											BooleanValue?: *string | fn.#Fn
											DoubleValue?:  *string | fn.#Fn
											IntegerValue?: *string | fn.#Fn
											StringValue?:  *string | fn.#Fn
										} | fn.#If
									} | fn.#If
								} | fn.#If
								IotTopicPublish?: *{
									MqttTopic: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
									Payload?:  *{
										ContentExpression: *string | fn.#Fn
										Type:              *string | fn.#Fn
									} | fn.#If
								} | fn.#If
								Lambda?: *{
									FunctionArn: *(strings.MinRunes(1) & strings.MaxRunes(2048)) | fn.#Fn
									Payload?:    *{
										ContentExpression: *string | fn.#Fn
										Type:              *string | fn.#Fn
									} | fn.#If
								} | fn.#If
								ResetTimer?: *{
									TimerName: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
								} | fn.#If
								SetTimer?: *{
									DurationExpression?: *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
									Seconds?:            *(>=60 & <=31622400) | fn.#Fn
									TimerName:           *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
								} | fn.#If
								SetVariable?: *{
									Value:        *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
									VariableName: *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[a-zA-Z][a-zA-Z0-9_]*$"#)) | fn.#Fn
								} | fn.#If
								Sns?: *{
									Payload?: *{
										ContentExpression: *string | fn.#Fn
										Type:              *string | fn.#Fn
									} | fn.#If
									TargetArn: *(strings.MinRunes(1) & strings.MaxRunes(2048)) | fn.#Fn
								} | fn.#If
								Sqs?: *{
									Payload?: *{
										ContentExpression: *string | fn.#Fn
										Type:              *string | fn.#Fn
									} | fn.#If
									QueueUrl:   *string | fn.#Fn
									UseBase64?: *bool | fn.#Fn
								} | fn.#If
							}] | fn.#If
							Condition?: *string | fn.#Fn
							EventName:  *string | fn.#Fn
						}] | fn.#If
						TransitionEvents?: *[...{
							Actions?: *[...{
								ClearTimer?: *{
									TimerName: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
								} | fn.#If
								DynamoDB?: *{
									HashKeyField: *string | fn.#Fn
									HashKeyType?: *string | fn.#Fn
									HashKeyValue: *string | fn.#Fn
									Operation?:   *string | fn.#Fn
									Payload?:     *{
										ContentExpression: *string | fn.#Fn
										Type:              *string | fn.#Fn
									} | fn.#If
									PayloadField?:  *string | fn.#Fn
									RangeKeyField?: *string | fn.#Fn
									RangeKeyType?:  *string | fn.#Fn
									RangeKeyValue?: *string | fn.#Fn
									TableName:      *string | fn.#Fn
								} | fn.#If
								DynamoDBv2?: *{
									Payload?: *{
										ContentExpression: *string | fn.#Fn
										Type:              *string | fn.#Fn
									} | fn.#If
									TableName: *string | fn.#Fn
								} | fn.#If
								Firehose?: *{
									DeliveryStreamName: *string | fn.#Fn
									Payload?:           *{
										ContentExpression: *string | fn.#Fn
										Type:              *string | fn.#Fn
									} | fn.#If
									Separator?: *(=~#"([\n\t])|(\r\n)|(,)"#) | fn.#Fn
								} | fn.#If
								IotEvents?: *{
									InputName: *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[a-zA-Z][a-zA-Z0-9_]*$"#)) | fn.#Fn
									Payload?:  *{
										ContentExpression: *string | fn.#Fn
										Type:              *string | fn.#Fn
									} | fn.#If
								} | fn.#If
								IotSiteWise?: *{
									AssetId?:       *string | fn.#Fn
									EntryId?:       *string | fn.#Fn
									PropertyAlias?: *string | fn.#Fn
									PropertyId?:    *string | fn.#Fn
									PropertyValue:  *{
										Quality?:   *string | fn.#Fn
										Timestamp?: *{
											OffsetInNanos?: *string | fn.#Fn
											TimeInSeconds:  *string | fn.#Fn
										} | fn.#If
										Value: *{
											BooleanValue?: *string | fn.#Fn
											DoubleValue?:  *string | fn.#Fn
											IntegerValue?: *string | fn.#Fn
											StringValue?:  *string | fn.#Fn
										} | fn.#If
									} | fn.#If
								} | fn.#If
								IotTopicPublish?: *{
									MqttTopic: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
									Payload?:  *{
										ContentExpression: *string | fn.#Fn
										Type:              *string | fn.#Fn
									} | fn.#If
								} | fn.#If
								Lambda?: *{
									FunctionArn: *(strings.MinRunes(1) & strings.MaxRunes(2048)) | fn.#Fn
									Payload?:    *{
										ContentExpression: *string | fn.#Fn
										Type:              *string | fn.#Fn
									} | fn.#If
								} | fn.#If
								ResetTimer?: *{
									TimerName: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
								} | fn.#If
								SetTimer?: *{
									DurationExpression?: *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
									Seconds?:            *(>=60 & <=31622400) | fn.#Fn
									TimerName:           *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
								} | fn.#If
								SetVariable?: *{
									Value:        *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
									VariableName: *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[a-zA-Z][a-zA-Z0-9_]*$"#)) | fn.#Fn
								} | fn.#If
								Sns?: *{
									Payload?: *{
										ContentExpression: *string | fn.#Fn
										Type:              *string | fn.#Fn
									} | fn.#If
									TargetArn: *(strings.MinRunes(1) & strings.MaxRunes(2048)) | fn.#Fn
								} | fn.#If
								Sqs?: *{
									Payload?: *{
										ContentExpression: *string | fn.#Fn
										Type:              *string | fn.#Fn
									} | fn.#If
									QueueUrl:   *string | fn.#Fn
									UseBase64?: *bool | fn.#Fn
								} | fn.#If
							}] | fn.#If
							Condition: *string | fn.#Fn
							EventName: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
							NextState: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
						}] | fn.#If
					} | fn.#If
					StateName: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
				}] | fn.#If
			} | fn.#If
			DetectorModelDescription?: *string | fn.#Fn
			DetectorModelName?:        *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[a-zA-Z0-9_-]+$"#)) | fn.#Fn
			EvaluationMethod?:         *("BATCH" | "SERIAL") | fn.#Fn
			Key?:                      *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^((`[\w\- ]+`)|([\w\-]+))(\.((`[\w\- ]+`)|([\w\-]+)))*$"#)) | fn.#Fn
			RoleArn:                   *(strings.MinRunes(1) & strings.MaxRunes(2048)) | fn.#Fn
			Tags?:                     *[...{
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
	#Input: {
		Type: "AWS::IoTEvents::Input"
		Properties: {
			InputDefinition: *{
				Attributes: *[...{
					JsonPath: *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^((`[a-zA-Z0-9_\- ]+`)|([a-zA-Z0-9_\-]+))(\.((`[a-zA-Z0-9_\- ]+`)|([a-zA-Z0-9_\-]+)))*$"#)) | fn.#Fn
				}] | fn.#If
			} | fn.#If
			InputDescription?: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
			InputName?:        *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[a-zA-Z][a-zA-Z0-9_]*$"#)) | fn.#Fn
			Tags?:             *[...{
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
