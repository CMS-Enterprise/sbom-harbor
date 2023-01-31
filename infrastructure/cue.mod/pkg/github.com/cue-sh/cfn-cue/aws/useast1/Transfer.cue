package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#Transfer: {
	#Server: {
		Type: "AWS::Transfer::Server"
		Properties: {
			Certificate?:     *string | fn.#Fn
			Domain?:          *string | fn.#Fn
			EndpointDetails?: *{
				AddressAllocationIds?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				SecurityGroupIds?:     [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				SubnetIds?:            [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				VpcEndpointId?:        *string | fn.#Fn
				VpcId?:                *string | fn.#Fn
			} | fn.#If
			EndpointType?:            *string | fn.#Fn
			IdentityProviderDetails?: *{
				DirectoryId?:    *string | fn.#Fn
				Function?:       *string | fn.#Fn
				InvocationRole?: *string | fn.#Fn
				Url?:            *string | fn.#Fn
			} | fn.#If
			IdentityProviderType?: *string | fn.#Fn
			LoggingRole?:          *string | fn.#Fn
			ProtocolDetails?:      *{
				PassiveIp?: *string | fn.#Fn
			} | fn.#If
			Protocols?:          *[...{}] | fn.#If
			SecurityPolicyName?: *string | fn.#Fn
			Tags?:               *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			WorkflowDetails?: *{
				OnUpload: *[...{
					ExecutionRole: *string | fn.#Fn
					WorkflowId:    *string | fn.#Fn
				}] | fn.#If
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#User: {
		Type: "AWS::Transfer::User"
		Properties: {
			HomeDirectory?:         *string | fn.#Fn
			HomeDirectoryMappings?: *[...{
				Entry:  *string | fn.#Fn
				Target: *string | fn.#Fn
			}] | fn.#If
			HomeDirectoryType?: *string | fn.#Fn
			Policy?:            *string | fn.#Fn
			PosixProfile?:      *{
				Gid:            *number | fn.#Fn
				SecondaryGids?: [...(*number | fn.#Fn)] | (*number | fn.#Fn)
				Uid:            *number | fn.#Fn
			} | fn.#If
			Role:           *string | fn.#Fn
			ServerId:       *string | fn.#Fn
			SshPublicKeys?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			Tags?:          *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			UserName: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Workflow: {
		Type: "AWS::Transfer::Workflow"
		Properties: {
			Description?:      *(=~#"^[\w\- ]*$"#) | fn.#Fn
			OnExceptionSteps?: *[...{
				CopyStepDetails?: *{
					[string]: _
				} | fn.#Fn
				CustomStepDetails?: *{
					[string]: _
				} | fn.#Fn
				DeleteStepDetails?: *{
					[string]: _
				} | fn.#Fn
				TagStepDetails?: *{
					[string]: _
				} | fn.#Fn
				Type?: *("COPY" | "CUSTOM" | "DELETE" | "TAG") | fn.#Fn
			}] | fn.#If
			Steps: *[...{
				CopyStepDetails?: *{
					[string]: _
				} | fn.#Fn
				CustomStepDetails?: *{
					[string]: _
				} | fn.#Fn
				DeleteStepDetails?: *{
					[string]: _
				} | fn.#Fn
				TagStepDetails?: *{
					[string]: _
				} | fn.#Fn
				Type?: *("COPY" | "CUSTOM" | "DELETE" | "TAG") | fn.#Fn
			}] | fn.#If
			Tags?: *[...{
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
}
