"""This Stack is used to set up shared resources
used by CICD, Github Actions, and secrets management"""

from aws_cdk import CfnOutput, Stack
from aws_cdk import aws_iam as iam
from constructs import Construct

from deploy.constants import (
    DEVOPS_GITHUB_ORG,
    DEVOPS_GITHUB_REPO,
    DEVOPS_GITHUB_ROLE_ID,
    DEVOPS_OIDC_PROVIDER_ID,
    DEVOPS_STACK_ID,
)


class HarborDevOpsStack(Stack):
    def __init__(
        self,
        scope: Construct,
        **kwargs,
    ) -> None:

        # Run the constructor of the Stack superclass.
        super().__init__(
            scope,
            DEVOPS_STACK_ID,
            description="Contains resources common to CICD operations including those needed by Github Actions",
            **kwargs,
        )

        # Allows github actions to authenticate
        self.oidc_provider = iam.OpenIdConnectProvider(
            self,
            id=DEVOPS_OIDC_PROVIDER_ID,
            thumbprints=["6938fd4d98bab03faadb97b34396831e3780aea1"],
            client_ids=["sts.amazonaws.com"],
            url="https://token.actions.githubusercontent.com",
        )

        # A role for Github Actions to assume
        self.github_role = iam.Role(
            self,
            id=DEVOPS_GITHUB_ROLE_ID,
            assumed_by=iam.WebIdentityPrincipal(
                identity_provider=self.oidc_provider.open_id_connect_provider_arn,
                conditions={
                    "StringLike": {
                        "token.actions.githubusercontent.com:sub": f"repo:{DEVOPS_GITHUB_ORG}/{DEVOPS_GITHUB_REPO}:*"
                    },
                    "ForAllValues:StringEquals": {
                        "token.actions.githubusercontent.com:iss": "https://token.actions.githubusercontent.com",
                        "token.actions.githubusercontent.com:aud": "sts.amazonaws.com",
                    },
                },
            ),
            inline_policies={
                "CdkPermissions": iam.PolicyDocument(
                    statements=[
                        iam.PolicyStatement(
                            actions=["sts:AssumeRole"],
                            resources=[
                                "arn:aws:iam::476877758413:role/cdk-hnb659fds-deploy-role-476877758413-us-east-2",
                                "arn:aws:iam::476877758413:role/cdk-hnb659fds-lookup-role-476877758413-us-east-2",
                                "arn:aws:iam::476877758413:role/cdk-hnb659fds-image-publishing-role-476877758413-us-east-2",
                                "arn:aws:iam::476877758413:role/cdk-hnb659fds-file-publishing-role-476877758413-us-east-2",
                            ],
                        ),
                        iam.PolicyStatement(
                            actions=["cloudformation:DescribeStacks"],
                            resources=[
                                "arn:aws:cloudformation:us-east-2:476877758413:stack/*"
                            ],
                        ),
                        iam.PolicyStatement(
                            actions=["s3:ListAllMyBuckets"],
                            resources=["arn:aws:s3:::*"],
                        ),
                        iam.PolicyStatement(
                            actions=[
                                "s3:ListBucket",
                                "s3:ListBucketMultipartUploads",
                                "s3:GetBucketLocation",
                                "s3:AbortMultipartUpload",
                                "s3:GetObjectAcl",
                                "s3:GetObjectVersion",
                                "s3:DeleteObject",
                                "s3:DeleteObjectVersion",
                                "s3:GetObject",
                                "s3:PutObjectAcl",
                                "s3:PutObject",
                                "s3:GetObjectVersionAcl",
                            ],
                            resources=[
                                "arn:aws:s3:::dev-harbor-web-assets-476877758413-use2",
                                "arn:aws:s3:::dev-harbor-web-assets-476877758413-use2/*",
                            ],
                        ),
                    ]
                )
            },
        )

        CfnOutput(self, "GithubRole", value=self.github_role.role_arn)
