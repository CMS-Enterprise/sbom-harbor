package iam

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#AccessKey: {
	Type: "AWS::IAM::AccessKey"
	Properties: {
		Serial?:  *int | fn.#Fn
		Status?:  *("Active" | "Inactive") | fn.#Fn
		UserName: *string | fn.#Fn
	}
	DependsOn?:           string | [...string]
	DeletionPolicy?:      "Delete" | "Retain"
	UpdateReplacePolicy?: "Delete" | "Retain"
	Metadata?: [string]: _
	Condition?: string
}
#Group: {
	Type: "AWS::IAM::Group"
	Properties: {
		GroupName?:         *string | fn.#Fn
		ManagedPolicyArns?: [...(*(=~#"arn:(aws[a-zA-Z-]*)?:iam::(\d{12}|aws):policy/[a-zA-Z_0-9+=,.@\-_/]+"#) | fn.#Fn)] | (*(=~#"arn:(aws[a-zA-Z-]*)?:iam::(\d{12}|aws):policy/[a-zA-Z_0-9+=,.@\-_/]+"#) | fn.#Fn)
		Path?:              *(=~#"^/(.+/)*$"#) | fn.#Fn
		Policies?:          *[...{
			PolicyDocument: *{
				{
					[string]: _
				}
				Version: string | *"2012-10-17"
			} | fn.#Fn
			PolicyName: *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[a-zA-Z0-9+=,.@\-_]+$"#)) | fn.#Fn
		}] | fn.#If
	}
	DependsOn?:           string | [...string]
	DeletionPolicy?:      "Delete" | "Retain"
	UpdateReplacePolicy?: "Delete" | "Retain"
	Metadata?: [string]: _
	Condition?: string
}
#InstanceProfile: {
	Type: "AWS::IAM::InstanceProfile"
	Properties: {
		InstanceProfileName?: *string | fn.#Fn
		Path?:                *(=~#"^/(.+/)*$"#) | fn.#Fn
		Roles:                [...(*string | fn.#Fn)] | (*string | fn.#Fn)
	}
	DependsOn?:           string | [...string]
	DeletionPolicy?:      "Delete" | "Retain"
	UpdateReplacePolicy?: "Delete" | "Retain"
	Metadata?: [string]: _
	Condition?: string
}
#ManagedPolicy: {
	Type: "AWS::IAM::ManagedPolicy"
	Properties: {
		Description?:       *string | fn.#Fn
		Groups?:            [...(*string | fn.#Fn)] | (*string | fn.#Fn)
		ManagedPolicyName?: *string | fn.#Fn
		Path?:              *(=~#"^/(.+/)*$"#) | fn.#Fn
		PolicyDocument:     *{
			{
				[string]: _
			}
			Version: string | *"2012-10-17"
		} | fn.#Fn
		Roles?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
		Users?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
	}
	DependsOn?:           string | [...string]
	DeletionPolicy?:      "Delete" | "Retain"
	UpdateReplacePolicy?: "Delete" | "Retain"
	Metadata?: [string]: _
	Condition?: string
}
#OIDCProvider: {
	Type: "AWS::IAM::OIDCProvider"
	Properties: {
		ClientIdList?: [...(*(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn)
		Tags?:         *[...{
			Key:   *string | fn.#Fn
			Value: *string | fn.#Fn
		}] | fn.#If
		ThumbprintList: [...(*(strings.MinRunes(40) & strings.MaxRunes(40) & (=~#"[0-9A-Fa-f]{40}"#)) | fn.#Fn)] | (*(strings.MinRunes(40) & strings.MaxRunes(40) & (=~#"[0-9A-Fa-f]{40}"#)) | fn.#Fn)
		Url?:           *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
	}
	DependsOn?:           string | [...string]
	DeletionPolicy?:      "Delete" | "Retain"
	UpdateReplacePolicy?: "Delete" | "Retain"
	Metadata?: [string]: _
	Condition?: string
}
#Policy: {
	Type: "AWS::IAM::Policy"
	Properties: {
		Groups?:        [...(*string | fn.#Fn)] | (*string | fn.#Fn)
		PolicyDocument: *{
			{
				[string]: _
			}
			Version: string | *"2012-10-17"
		} | fn.#Fn
		PolicyName: *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[a-zA-Z0-9+=,.@\-_]+$"#)) | fn.#Fn
		Roles?:     [...(*string | fn.#Fn)] | (*string | fn.#Fn)
		Users?:     [...(*string | fn.#Fn)] | (*string | fn.#Fn)
	}
	DependsOn?:           string | [...string]
	DeletionPolicy?:      "Delete" | "Retain"
	UpdateReplacePolicy?: "Delete" | "Retain"
	Metadata?: [string]: _
	Condition?: string
}
#Role: {
	Type: "AWS::IAM::Role"
	Properties: {
		AssumeRolePolicyDocument: *{
			{
				[string]: _
			}
			Version: string | *"2012-10-17"
		} | fn.#Fn
		Description?:         *string | fn.#Fn
		ManagedPolicyArns?:   [...(*(=~#"arn:(aws[a-zA-Z-]*)?:iam::(\d{12}|aws):policy/[a-zA-Z_0-9+=,.@\-_/]+"#) | fn.#Fn)] | (*(=~#"arn:(aws[a-zA-Z-]*)?:iam::(\d{12}|aws):policy/[a-zA-Z_0-9+=,.@\-_/]+"#) | fn.#Fn)
		MaxSessionDuration?:  *(>=3600 & <=43200) | fn.#Fn
		Path?:                *(=~#"^/(.+/)*$"#) | fn.#Fn
		PermissionsBoundary?: *string | fn.#Fn
		Policies?:            *[...{
			PolicyDocument: *{
				{
					[string]: _
				}
				Version: string | *"2012-10-17"
			} | fn.#Fn
			PolicyName: *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
		}] | fn.#If
		RoleName?: *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
		Tags?:     *[...{
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
#SAMLProvider: {
	Type: "AWS::IAM::SAMLProvider"
	Properties: {
		Name?:                *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"[\w._-]+"#)) | fn.#Fn
		SamlMetadataDocument: *(strings.MinRunes(1000) & strings.MaxRunes(10000000)) | fn.#Fn
		Tags?:                *[...{
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
#ServerCertificate: {
	Type: "AWS::IAM::ServerCertificate"
	Properties: {
		CertificateBody?:       *(strings.MinRunes(1) & strings.MaxRunes(16384) & (=~#"[\u0009\u000A\u000D\u0020-\u00FF]+"#)) | fn.#Fn
		CertificateChain?:      *(strings.MinRunes(1) & strings.MaxRunes(2097152) & (=~#"[\u0009\u000A\u000D\u0020-\u00FF]+"#)) | fn.#Fn
		Path?:                  *(strings.MinRunes(1) & strings.MaxRunes(512) & (=~#"(\u002F)|(\u002F[\u0021-\u007F]+\u002F)"#)) | fn.#Fn
		PrivateKey?:            *(strings.MinRunes(1) & strings.MaxRunes(16384) & (=~#"[\u0009\u000A\u000D\u0020-\u00FF]+"#)) | fn.#Fn
		ServerCertificateName?: *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"[\w+=,.@-]+"#)) | fn.#Fn
		Tags?:                  *[...{
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
#ServiceLinkedRole: {
	Type: "AWS::IAM::ServiceLinkedRole"
	Properties: {
		AWSServiceName: *string | fn.#Fn
		CustomSuffix?:  *string | fn.#Fn
		Description?:   *string | fn.#Fn
	}
	DependsOn?:           string | [...string]
	DeletionPolicy?:      "Delete" | "Retain"
	UpdateReplacePolicy?: "Delete" | "Retain"
	Metadata?: [string]: _
	Condition?: string
}
#User: {
	Type: "AWS::IAM::User"
	Properties: {
		Groups?:       [...(*string | fn.#Fn)] | (*string | fn.#Fn)
		LoginProfile?: *{
			Password:               *string | fn.#Fn
			PasswordResetRequired?: *bool | fn.#Fn
		} | fn.#If
		ManagedPolicyArns?:   [...(*(=~#"arn:(aws[a-zA-Z-]*)?:iam::(\d{12}|aws):policy/[a-zA-Z_0-9+=,.@\-_/]+"#) | fn.#Fn)] | (*(=~#"arn:(aws[a-zA-Z-]*)?:iam::(\d{12}|aws):policy/[a-zA-Z_0-9+=,.@\-_/]+"#) | fn.#Fn)
		Path?:                *(=~#"^/(.+/)*$"#) | fn.#Fn
		PermissionsBoundary?: *string | fn.#Fn
		Policies?:            *[...{
			PolicyDocument: *{
				{
					[string]: _
				}
				Version: string | *"2012-10-17"
			} | fn.#Fn
			PolicyName: *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[a-zA-Z0-9+=,.@\-_]+$"#)) | fn.#Fn
		}] | fn.#If
		Tags?: *[...{
			Key:   *string | fn.#Fn
			Value: *string | fn.#Fn
		}] | fn.#If
		UserName?: *string | fn.#Fn
	}
	DependsOn?:           string | [...string]
	DeletionPolicy?:      "Delete" | "Retain"
	UpdateReplacePolicy?: "Delete" | "Retain"
	Metadata?: [string]: _
	Condition?: string
}
#UserToGroupAddition: {
	Type: "AWS::IAM::UserToGroupAddition"
	Properties: {
		GroupName: *string | fn.#Fn
		Users:     [...(*string | fn.#Fn)] | (*string | fn.#Fn)
	}
	DependsOn?:           string | [...string]
	DeletionPolicy?:      "Delete" | "Retain"
	UpdateReplacePolicy?: "Delete" | "Retain"
	Metadata?: [string]: _
	Condition?: string
}
#VirtualMFADevice: {
	Type: "AWS::IAM::VirtualMFADevice"
	Properties: {
		Path?: *(strings.MinRunes(1) & strings.MaxRunes(512) & (=~#"(\u002F)|(\u002F[\u0021-\u007F]+\u002F)"#)) | fn.#Fn
		Tags?: *[...{
			Key:   *string | fn.#Fn
			Value: *string | fn.#Fn
		}] | fn.#If
		Users:                 [...(*string | fn.#Fn)] | (*string | fn.#Fn)
		VirtualMfaDeviceName?: *(strings.MinRunes(1) & strings.MaxRunes(226) & (=~#"[\w+=,.@-]+"#)) | fn.#Fn
	}
	DependsOn?:           string | [...string]
	DeletionPolicy?:      "Delete" | "Retain"
	UpdateReplacePolicy?: "Delete" | "Retain"
	Metadata?: [string]: _
	Condition?: string
}
