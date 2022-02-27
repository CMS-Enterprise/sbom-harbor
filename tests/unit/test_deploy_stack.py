import aws_cdk as core
import aws_cdk.assertions as assertions

from deploy.aquia_sbom_api_deploy import DeployStack

# example tests. To run these tests, uncomment this file along with the example
# resource in deploy/aquia_sbom_api_deploy.py
def test_sqs_queue_created():
    app = core.App()
    stack = DeployStack(app, "deploy")
    template = assertions.Template.from_stack(stack)

#     template.has_resource_properties("AWS::SQS::Queue", {
#         "VisibilityTimeout": 300
#     })
