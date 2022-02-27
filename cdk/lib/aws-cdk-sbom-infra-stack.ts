import * as cdk from '@aws-cdk/core';
import * as lambda from '@aws-cdk/aws-lambda';
import * as apigateway from '@aws-cdk/aws-apigateway';
import * as s3 from '@aws-cdk/aws-s3';
import { exec } from 'child_process';

export class AwsCdkSbomInfraStack extends cdk.Stack {
  constructor(scope: cdk.Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    //exec('git clone git@github.com:aquia-inc/aquia-sbom-lambda.git tmp/')
    //exec('mvn clean install -f tmp/pom.xml')


    const sbomBucket = new s3.Bucket(this, 'sbomBucket', {
      //enforceSSL: true,
      //encryption: s3.BucketEncryption.S3_MANAGED
    })

    const sbomIngestFunction = new lambda.Function(this, 'sbomIngestFunction', {
      runtime: lambda.Runtime.PYTHON_3_9,
      code: new lambda.AssetCode('../dist/lambda.zip'),
      handler: "cyclonedx.api.lambda_handler",
      environment: {
        SBOM_BUCKET_NAME: sbomBucket.bucketName
      },
      timeout: cdk.Duration.minutes(2),
      memorySize: 512
    })

    sbomBucket.grantPut(sbomIngestFunction)

    const sbomApi = new apigateway.LambdaRestApi(this, 'sbomApi', {
      handler: sbomIngestFunction,
      proxy: true
    })
  }
}
