import aws_cdk.aws_ec2 as ec2
import aws_cdk.aws_elasticloadbalancingv2 as elbv2
import aws_cdk.aws_lambda as lambda_
import aws_cdk.aws_sqs as sqs
import aws_cdk.aws_ssm as ssm
from aws_cdk import Duration
from aws_cdk.aws_lambda import AssetCode
from aws_cdk.aws_lambda_event_sources import SqsEventSource
from aws_cdk.aws_s3 import Bucket
from constructs import Construct

from cyclonedx.constants import (
    DT_API_BASE,
    DT_API_KEY,
    DT_ROOT_PWD,
    EMPTY_VALUE,
    FINDINGS_QUEUE_URL_EV,
)

from scripts.constants import (
    DT_INTERFACE_LN,
    PRIVATE,
    SBOM_API_PYTHON_RUNTIME,
)


class DependencyTrackInterfaceLambda(Construct):

    def __init__(self, scope: Construct, *, vpc: ec2.Vpc,
                 code: AssetCode, s3_bucket: Bucket,
                 dt_ingress_queue: sqs.Queue,
                 findings_queue: sqs.Queue,
                 load_balancer: elbv2.ApplicationLoadBalancer):

        super().__init__(scope, DT_INTERFACE_LN)

        """Create the Lambda Function responsible for
        extracting results from DT given an SBOM."""

        dt_func_sg = ec2.SecurityGroup(self, "LaunchTemplateSG", vpc=vpc)
        load_balancer.load_balancer_security_groups.append(dt_func_sg)

        fq_dn = load_balancer.load_balancer_dns_name

        dt_interface_function = lambda_.Function(
            self,
            DT_INTERFACE_LN,
            runtime=SBOM_API_PYTHON_RUNTIME,
            vpc=vpc,
            vpc_subnets=ec2.SubnetSelection(subnet_type=PRIVATE),
            handler="cyclonedx.api.dt_interface_handler",
            code=code,
            environment={
                FINDINGS_QUEUE_URL_EV: findings_queue.queue_url,
                DT_API_BASE: fq_dn,
            },
            timeout=Duration.minutes(1),
            security_groups=[dt_func_sg],
            memory_size=512,
        )

        # Grant rights to send messages to the Queue
        findings_queue.grant_send_messages(dt_interface_function)

        s3_bucket.grant_put(dt_interface_function)
        s3_bucket.grant_read_write(dt_interface_function)

        root_pwd_param = ssm.StringParameter(
            self,
            DT_ROOT_PWD,
            string_value=EMPTY_VALUE,
            parameter_name=DT_ROOT_PWD,
        )

        root_pwd_param.grant_read(dt_interface_function)
        root_pwd_param.grant_write(dt_interface_function)

        api_key_param = ssm.StringParameter(
            self,
            DT_API_KEY,
            string_value=EMPTY_VALUE,
            parameter_name=DT_API_KEY
        )

        api_key_param.grant_read(dt_interface_function)
        api_key_param.grant_write(dt_interface_function)

        event_source = SqsEventSource(dt_ingress_queue)
        dt_interface_function.add_event_source(event_source)