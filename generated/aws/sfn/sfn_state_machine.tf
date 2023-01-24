resource "aws_sfn_state_machine" "tfer--sandbox_Enrichment_usw2" {
  definition = "{\"StartAt\":\"ENRICHMENT_JOBS\",\"States\":{\"ENRICHMENT_JOBS\":{\"Type\":\"Parallel\",\"Next\":\"ENRICHMENT_SUMMARIZER_TASK\",\"Branches\":[{\"StartAt\":\"ENRICHMENT_DT_TASK\",\"States\":{\"ENRICHMENT_DT_TASK\":{\"End\":true,\"Retry\":[{\"ErrorEquals\":[\"Lambda.ServiceException\",\"Lambda.AWSLambdaException\",\"Lambda.SdkClientException\"],\"IntervalSeconds\":2,\"MaxAttempts\":6,\"BackoffRate\":2}],\"Type\":\"Task\",\"InputPath\":\"$.detail\",\"OutputPath\":\"$.detail\",\"ResultPath\":\"$.detail.results\",\"Resource\":\"arn:aws:states:::lambda:invoke\",\"Parameters\":{\"FunctionName\":\"arn:aws:lambda:us-west-2:393419659647:function:sandbox_DependencyTrackInterface_usw2\",\"Payload.$\":\"$\"}}}},{\"StartAt\":\"ENRICHMENT_IC_TASK\",\"States\":{\"ENRICHMENT_IC_TASK\":{\"End\":true,\"Retry\":[{\"ErrorEquals\":[\"Lambda.ServiceException\",\"Lambda.AWSLambdaException\",\"Lambda.SdkClientException\"],\"IntervalSeconds\":2,\"MaxAttempts\":6,\"BackoffRate\":2}],\"Type\":\"Task\",\"InputPath\":\"$.detail\",\"OutputPath\":\"$.detail\",\"ResultPath\":\"$.detail.results\",\"Resource\":\"arn:aws:states:::lambda:invoke\",\"Parameters\":{\"FunctionName\":\"arn:aws:lambda:us-west-2:393419659647:function:sandbox_IonChannelInterface_usw2\",\"Payload.$\":\"$\"}}}}]},\"ENRICHMENT_SUMMARIZER_TASK\":{\"End\":true,\"Retry\":[{\"ErrorEquals\":[\"Lambda.ServiceException\",\"Lambda.AWSLambdaException\",\"Lambda.SdkClientException\"],\"IntervalSeconds\":2,\"MaxAttempts\":6,\"BackoffRate\":2}],\"Type\":\"Task\",\"Resource\":\"arn:aws:states:::lambda:invoke\",\"Parameters\":{\"FunctionName\":\"arn:aws:lambda:us-west-2:393419659647:function:sandbox_Summarizer_usw2\",\"Payload.$\":\"$\"}}}}"

  logging_configuration {
    include_execution_data = "false"
    level                  = "OFF"
  }

  name     = "sandbox_Enrichment_usw2"
  role_arn = "arn:aws:iam::393419659647:role/sandbox-harbor-enrichment-ENRICHMENTSTATEMACHINERo-1TKL415Y7O63T"

  tracing_configuration {
    enabled = "false"
  }

  type = "STANDARD"
}
