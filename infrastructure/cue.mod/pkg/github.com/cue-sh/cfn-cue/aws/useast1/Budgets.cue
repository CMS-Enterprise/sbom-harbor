package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#Budgets: {
	#Budget: {
		Type: "AWS::Budgets::Budget"
		Properties: {
			Budget: *{
				BudgetLimit?: *{
					Amount: *number | fn.#Fn
					Unit:   *string | fn.#Fn
				} | fn.#If
				BudgetName?:  *string | fn.#Fn
				BudgetType:   *("COST" | "RI_COVERAGE" | "RI_UTILIZATION" | "SAVINGS_PLANS_COVERAGE" | "SAVINGS_PLANS_UTILIZATION" | "USAGE") | fn.#Fn
				CostFilters?: *{
					[string]: _
				} | fn.#Fn
				CostTypes?: *{
					IncludeCredit?:            *bool | fn.#Fn
					IncludeDiscount?:          *bool | fn.#Fn
					IncludeOtherSubscription?: *bool | fn.#Fn
					IncludeRecurring?:         *bool | fn.#Fn
					IncludeRefund?:            *bool | fn.#Fn
					IncludeSubscription?:      *bool | fn.#Fn
					IncludeSupport?:           *bool | fn.#Fn
					IncludeTax?:               *bool | fn.#Fn
					IncludeUpfront?:           *bool | fn.#Fn
					UseAmortized?:             *bool | fn.#Fn
					UseBlended?:               *bool | fn.#Fn
				} | fn.#If
				PlannedBudgetLimits?: *{
					[string]: _
				} | fn.#Fn
				TimePeriod?: *{
					End?:   *string | fn.#Fn
					Start?: *string | fn.#Fn
				} | fn.#If
				TimeUnit: *("ANNUALLY" | "DAILY" | "MONTHLY" | "QUARTERLY") | fn.#Fn
			} | fn.#If
			NotificationsWithSubscribers?: *[...{
				Notification: *{
					ComparisonOperator: *("EQUAL_TO" | "GREATER_THAN" | "LESS_THAN") | fn.#Fn
					NotificationType:   *("ACTUAL" | "FORECASTED") | fn.#Fn
					Threshold:          *(>=0.1 & <=1000000000) | fn.#Fn
					ThresholdType?:     *("ABSOLUTE_VALUE" | "PERCENTAGE") | fn.#Fn
				} | fn.#If
				Subscribers: *[...{
					Address:          *string | fn.#Fn
					SubscriptionType: *("EMAIL" | "SNS") | fn.#Fn
				}] | fn.#If
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#BudgetsAction: {
		Type: "AWS::Budgets::BudgetsAction"
		Properties: {
			ActionThreshold: *{
				Type:  *("PERCENTAGE" | "ABSOLUTE_VALUE") | fn.#Fn
				Value: *number | fn.#Fn
			} | fn.#If
			ActionType:     *("APPLY_IAM_POLICY" | "APPLY_SCP_POLICY" | "RUN_SSM_DOCUMENTS") | fn.#Fn
			ApprovalModel?: *("AUTOMATIC" | "MANUAL") | fn.#Fn
			BudgetName:     *string | fn.#Fn
			Definition:     *{
				IamActionDefinition?: *{
					Groups?:   [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					PolicyArn: *string | fn.#Fn
					Roles?:    [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					Users?:    [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				} | fn.#If
				ScpActionDefinition?: *{
					PolicyId:  *string | fn.#Fn
					TargetIds: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				} | fn.#If
				SsmActionDefinition?: *{
					InstanceIds: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					Region:      *string | fn.#Fn
					Subtype:     *("STOP_EC2_INSTANCES" | "STOP_RDS_INSTANCES") | fn.#Fn
				} | fn.#If
			} | fn.#If
			ExecutionRoleArn: *string | fn.#Fn
			NotificationType: *("ACTUAL" | "FORECASTED") | fn.#Fn
			Subscribers:      *[...{
				Address: *string | fn.#Fn
				Type:    *("SNS" | "EMAIL") | fn.#Fn
			}] | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
