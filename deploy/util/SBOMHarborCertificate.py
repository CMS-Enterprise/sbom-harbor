""" SBOM HarborCertificate has classes and methods that
register a FQDN with a certificate with Route53 """
import logging
import os
from logging import config

import constructs
from aws_cdk import aws_certificatemanager as acm
from aws_cdk import aws_cloudfront as cf
from aws_cdk import aws_route53 as r53
from aws_cdk import aws_route53_targets as r53_targets
from aws_cdk.aws_cloudfront import ViewerCertificate

from deploy.constants import PYTHON_LOGGING_CONFIG

config.fileConfig(PYTHON_LOGGING_CONFIG)
logger = logging.getLogger(__name__)


def get_env_var(key: str) -> str:
    """
    returns environment variable
    """
    err = KeyError(f"Unable to find value for {key}")
    try:
        val = os.environ[key]
        if not val:
            raise err

        return val
    except KeyError as keyError:
        raise keyError


class Cert:

    """
    Class to manage adding a domain name to CloudFront and
    associating a Certificate.

    This class requires some loaded environment variables:
    * SBOM_HARBOR_CERT_ENABLED: If true, this class will add the
      FQDN to the CloudFront Distribution and associate the Certificate
    * SBOM_HARBOR_CERT_FQDN: The fully qualified domain name that should
      be added to the CloudFront Distribution
    * SBOM_HARBOR_CERT_HZONE_ID: The ID of the Hosted Zone in Route53. This class
      needs to dynamically add a Host(A) record to route the FQDN to the CloudFront
      generated FQDN.
    * SBOM_HARBOR_CERT_ARN: The ARN of the certificate in AWS Certificate Manager.
    """

    def __init__(self, scope: constructs.Construct):

        # Get the CDK Construct and set as scope
        self.scope = scope

        try:
            self.enabled = bool(get_env_var("SBOM_HARBOR_CERT_ENABLED"))
        except KeyError:
            logger.info("SBOM Harbor Certificate is DISABLED")
            self.enabled = False

        if self.enabled:
            logger.info("SBOM Harbor Certificate is ENABLED")

            try:
                self.domain_name = get_env_var("SBOM_HARBOR_CERT_FQDN")
                self.hosted_zone_id = get_env_var("SBOM_HARBOR_CERT_HZONE_ID")
                self.cert_arn = get_env_var("SBOM_HARBOR_CERT_ARN")

                logger.info(
                    "Data: %s, %s, %s",
                    self.domain_name,
                    self.hosted_zone_id,
                    self.cert_arn,
                )

                self.my_hosted_zone = r53.HostedZone.from_hosted_zone_attributes(
                    self.scope,
                    "SBOMHarborZone",
                    hosted_zone_id=self.hosted_zone_id,
                    zone_name=self.domain_name,
                )

                self.acm_cert = acm.Certificate.from_certificate_arn(
                    self.scope, "SBOMHarborCert", certificate_arn=self.cert_arn
                )
            except KeyError as ke:
                logger.info("KeyError acquiring cert values, %s is missing", ke)
                self.enabled = False

    def get_viewer_cert(self):
        """
        -> Returns viewer certificate
        """
        return ViewerCertificate.from_acm_certificate(
            self.acm_cert,
            aliases=[self.domain_name],
            security_policy=cf.SecurityPolicyProtocol.TLS_V1_2_2021,
            ssl_method=cf.SSLMethod.SNI,
        )

    def create_host_record(self, dist: cf.CloudFrontWebDistribution):
        """
        -> Creates a host record
        """
        if self.enabled:
            cf_target = r53_targets.CloudFrontTarget(dist)
            r53.ARecord(
                self.scope,
                "SBOMAliasRecord",
                zone=self.my_hosted_zone,
                target=r53.RecordTarget.from_alias(cf_target),
            )
