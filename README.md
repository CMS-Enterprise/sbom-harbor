
#### Python version
3.10.2

#### Build and Deploy
##### poetry run build
Uses Poetry to build the python project into a single artifact

##### poetry run package
Re-Packages the project and all dependencies into a zip file compatible with AWS Lamda

##### poetry run deploy 
Deploys the zip file to AWS Lambda using AWS CDK.