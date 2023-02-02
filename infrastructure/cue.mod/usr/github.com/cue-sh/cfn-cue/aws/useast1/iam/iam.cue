package iam

#Role: Properties: {
	PermissionsBoundary: "Fn::Sub": "arn:aws:iam::${AWS::AccountId}:policy/cms-cloud-admin/ct-ado-poweruser-permissions-boundary-policy"
	Path: string | *"/delegatedadmin/developer/" | "/delegatedadmin/adodeveloper/service-role/"
}
