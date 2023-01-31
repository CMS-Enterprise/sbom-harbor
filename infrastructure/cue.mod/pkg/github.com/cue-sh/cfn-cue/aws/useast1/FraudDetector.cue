package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#FraudDetector: {
	#Detector: {
		Type: "AWS::FraudDetector::Detector"
		Properties: {
			AssociatedModels?: *[...{
				Arn?: *string | fn.#Fn
			}] | fn.#If
			Description?:           *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
			DetectorId:             *(strings.MinRunes(1) & strings.MaxRunes(64) & (=~#"^[0-9a-z_-]+$"#)) | fn.#Fn
			DetectorVersionStatus?: *("DRAFT" | "ACTIVE") | fn.#Fn
			EventType:              *{
				Arn?:         *string | fn.#Fn
				CreatedTime?: *string | fn.#Fn
				Description?: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
				EntityTypes?: *[...{
					Arn?:             *string | fn.#Fn
					CreatedTime?:     *string | fn.#Fn
					Description?:     *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
					Inline?:          *bool | fn.#Fn
					LastUpdatedTime?: *string | fn.#Fn
					Name?:            *string | fn.#Fn
					Tags?:            *[...{
						Key:   *string | fn.#Fn
						Value: *string | fn.#Fn
					}] | fn.#If
				}] | fn.#If
				EventVariables?: *[...{
					Arn?:             *string | fn.#Fn
					CreatedTime?:     *string | fn.#Fn
					DataSource?:      *("EVENT") | fn.#Fn
					DataType?:        *("STRING" | "INTEGER" | "FLOAT" | "BOOLEAN") | fn.#Fn
					DefaultValue?:    *string | fn.#Fn
					Description?:     *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
					Inline?:          *bool | fn.#Fn
					LastUpdatedTime?: *string | fn.#Fn
					Name?:            *string | fn.#Fn
					Tags?:            *[...{
						Key:   *string | fn.#Fn
						Value: *string | fn.#Fn
					}] | fn.#If
					VariableType?: *("AUTH_CODE" | "AVS" | "BILLING_ADDRESS_L1" | "BILLING_ADDRESS_L2" | "BILLING_CITY" | "BILLING_COUNTRY" | "BILLING_NAME" | "BILLING_PHONE" | "BILLING_STATE" | "BILLING_ZIP" | "CARD_BIN" | "CATEGORICAL" | "CURRENCY_CODE" | "EMAIL_ADDRESS" | "FINGERPRINT" | "FRAUD_LABEL" | "FREE_FORM_TEXT" | "IP_ADDRESS" | "NUMERIC" | "ORDER_ID" | "PAYMENT_TYPE" | "PHONE_NUMBER" | "PRICE" | "PRODUCT_CATEGORY" | "SHIPPING_ADDRESS_L1" | "SHIPPING_ADDRESS_L2" | "SHIPPING_CITY" | "SHIPPING_COUNTRY" | "SHIPPING_NAME" | "SHIPPING_PHONE" | "SHIPPING_STATE" | "SHIPPING_ZIP" | "USERAGENT") | fn.#Fn
				}] | fn.#If
				Inline?: *bool | fn.#Fn
				Labels?: *[...{
					Arn?:             *string | fn.#Fn
					CreatedTime?:     *string | fn.#Fn
					Description?:     *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
					Inline?:          *bool | fn.#Fn
					LastUpdatedTime?: *string | fn.#Fn
					Name?:            *string | fn.#Fn
					Tags?:            *[...{
						Key:   *string | fn.#Fn
						Value: *string | fn.#Fn
					}] | fn.#If
				}] | fn.#If
				LastUpdatedTime?: *string | fn.#Fn
				Name?:            *(strings.MinRunes(1) & strings.MaxRunes(64) & (=~#"^[0-9a-z_-]+$"#)) | fn.#Fn
				Tags?:            *[...{
					Key:   *string | fn.#Fn
					Value: *string | fn.#Fn
				}] | fn.#If
			} | fn.#If
			RuleExecutionMode?: *("FIRST_MATCHED" | "ALL_MATCHED") | fn.#Fn
			Rules:              *[...{
				Arn?:             *string | fn.#Fn
				CreatedTime?:     *string | fn.#Fn
				Description?:     *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
				DetectorId?:      *string | fn.#Fn
				Expression?:      *string | fn.#Fn
				Language?:        *("DETECTORPL") | fn.#Fn
				LastUpdatedTime?: *string | fn.#Fn
				Outcomes?:        *[...{
					Arn?:             *string | fn.#Fn
					CreatedTime?:     *string | fn.#Fn
					Description?:     *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
					Inline?:          *bool | fn.#Fn
					LastUpdatedTime?: *string | fn.#Fn
					Name?:            *string | fn.#Fn
					Tags?:            *[...{
						Key:   *string | fn.#Fn
						Value: *string | fn.#Fn
					}] | fn.#If
				}] | fn.#If
				RuleId?:      *string | fn.#Fn
				RuleVersion?: *string | fn.#Fn
				Tags?:        *[...{
					Key:   *string | fn.#Fn
					Value: *string | fn.#Fn
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
	#EntityType: {
		Type: "AWS::FraudDetector::EntityType"
		Properties: {
			Description?: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
			Name:         *(strings.MinRunes(1) & strings.MaxRunes(64) & (=~#"^[0-9a-z_-]+$"#)) | fn.#Fn
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
	#EventType: {
		Type: "AWS::FraudDetector::EventType"
		Properties: {
			Description?: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
			EntityTypes:  *[...{
				Arn?:             *string | fn.#Fn
				CreatedTime?:     *string | fn.#Fn
				Description?:     *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
				Inline?:          *bool | fn.#Fn
				LastUpdatedTime?: *string | fn.#Fn
				Name?:            *string | fn.#Fn
				Tags?:            *[...{
					Key:   *string | fn.#Fn
					Value: *string | fn.#Fn
				}] | fn.#If
			}] | fn.#If
			EventVariables: *[...{
				Arn?:             *string | fn.#Fn
				CreatedTime?:     *string | fn.#Fn
				DataSource?:      *("EVENT") | fn.#Fn
				DataType?:        *("STRING" | "INTEGER" | "FLOAT" | "BOOLEAN") | fn.#Fn
				DefaultValue?:    *string | fn.#Fn
				Description?:     *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
				Inline?:          *bool | fn.#Fn
				LastUpdatedTime?: *string | fn.#Fn
				Name?:            *string | fn.#Fn
				Tags?:            *[...{
					Key:   *string | fn.#Fn
					Value: *string | fn.#Fn
				}] | fn.#If
				VariableType?: *("AUTH_CODE" | "AVS" | "BILLING_ADDRESS_L1" | "BILLING_ADDRESS_L2" | "BILLING_CITY" | "BILLING_COUNTRY" | "BILLING_NAME" | "BILLING_PHONE" | "BILLING_STATE" | "BILLING_ZIP" | "CARD_BIN" | "CATEGORICAL" | "CURRENCY_CODE" | "EMAIL_ADDRESS" | "FINGERPRINT" | "FRAUD_LABEL" | "FREE_FORM_TEXT" | "IP_ADDRESS" | "NUMERIC" | "ORDER_ID" | "PAYMENT_TYPE" | "PHONE_NUMBER" | "PRICE" | "PRODUCT_CATEGORY" | "SHIPPING_ADDRESS_L1" | "SHIPPING_ADDRESS_L2" | "SHIPPING_CITY" | "SHIPPING_COUNTRY" | "SHIPPING_NAME" | "SHIPPING_PHONE" | "SHIPPING_STATE" | "SHIPPING_ZIP" | "USERAGENT") | fn.#Fn
			}] | fn.#If
			Labels: *[...{
				Arn?:             *string | fn.#Fn
				CreatedTime?:     *string | fn.#Fn
				Description?:     *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
				Inline?:          *bool | fn.#Fn
				LastUpdatedTime?: *string | fn.#Fn
				Name?:            *string | fn.#Fn
				Tags?:            *[...{
					Key:   *string | fn.#Fn
					Value: *string | fn.#Fn
				}] | fn.#If
			}] | fn.#If
			Name:  *(strings.MinRunes(1) & strings.MaxRunes(64) & (=~#"^[0-9a-z_-]+$"#)) | fn.#Fn
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
	#Label: {
		Type: "AWS::FraudDetector::Label"
		Properties: {
			Description?: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
			Name:         *(strings.MinRunes(1) & strings.MaxRunes(64) & (=~#"^[0-9a-z_-]+$"#)) | fn.#Fn
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
	#Outcome: {
		Type: "AWS::FraudDetector::Outcome"
		Properties: {
			Description?: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
			Name:         *(strings.MinRunes(1) & strings.MaxRunes(64) & (=~#"^[0-9a-z_-]+$"#)) | fn.#Fn
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
	#Variable: {
		Type: "AWS::FraudDetector::Variable"
		Properties: {
			DataSource:   *("EVENT" | "EXTERNAL_MODEL_SCORE") | fn.#Fn
			DataType:     *("STRING" | "INTEGER" | "FLOAT" | "BOOLEAN") | fn.#Fn
			DefaultValue: *string | fn.#Fn
			Description?: *(strings.MinRunes(1) & strings.MaxRunes(256)) | fn.#Fn
			Name:         *(=~#"^[a-z_][a-z0-9_]{0,99}?$"#) | fn.#Fn
			Tags?:        *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			VariableType?: *("AUTH_CODE" | "AVS" | "BILLING_ADDRESS_L1" | "BILLING_ADDRESS_L2" | "BILLING_CITY" | "BILLING_COUNTRY" | "BILLING_NAME" | "BILLING_PHONE" | "BILLING_STATE" | "BILLING_ZIP" | "CARD_BIN" | "CATEGORICAL" | "CURRENCY_CODE" | "EMAIL_ADDRESS" | "FINGERPRINT" | "FRAUD_LABEL" | "FREE_FORM_TEXT" | "IP_ADDRESS" | "NUMERIC" | "ORDER_ID" | "PAYMENT_TYPE" | "PHONE_NUMBER" | "PRICE" | "PRODUCT_CATEGORY" | "SHIPPING_ADDRESS_L1" | "SHIPPING_ADDRESS_L2" | "SHIPPING_CITY" | "SHIPPING_COUNTRY" | "SHIPPING_NAME" | "SHIPPING_PHONE" | "SHIPPING_STATE" | "SHIPPING_ZIP" | "USERAGENT") | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
