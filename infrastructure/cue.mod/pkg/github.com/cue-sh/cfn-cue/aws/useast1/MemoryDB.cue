package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#MemoryDB: {
	#ACL: {
		Type: "AWS::MemoryDB::ACL"
		Properties: {
			ACLName: *(=~#"[a-z][a-z0-9\\-]*"#) | fn.#Fn
			Tags?:   *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			UserNames?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Cluster: {
		Type: "AWS::MemoryDB::Cluster"
		Properties: {
			ACLName:                  *(=~#"[a-zA-Z][a-zA-Z0-9\-]*"#) | fn.#Fn
			AutoMinorVersionUpgrade?: *bool | fn.#Fn
			ClusterName:              *(=~#"[a-z][a-z0-9\-]*"#) | fn.#Fn
			Description?:             *string | fn.#Fn
			EngineVersion?:           *string | fn.#Fn
			FinalSnapshotName?:       *string | fn.#Fn
			KmsKeyId?:                *string | fn.#Fn
			MaintenanceWindow?:       *string | fn.#Fn
			NodeType:                 *string | fn.#Fn
			NumReplicasPerShard?:     *int | fn.#Fn
			NumShards?:               *int | fn.#Fn
			ParameterGroupName?:      *string | fn.#Fn
			Port?:                    *int | fn.#Fn
			SecurityGroupIds?:        [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			SnapshotArns?:            [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			SnapshotName?:            *string | fn.#Fn
			SnapshotRetentionLimit?:  *int | fn.#Fn
			SnapshotWindow?:          *string | fn.#Fn
			SnsTopicArn?:             *string | fn.#Fn
			SnsTopicStatus?:          *string | fn.#Fn
			SubnetGroupName?:         *string | fn.#Fn
			TLSEnabled?:              *bool | fn.#Fn
			Tags?:                    *[...{
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
	#ParameterGroup: {
		Type: "AWS::MemoryDB::ParameterGroup"
		Properties: {
			Description?:       *string | fn.#Fn
			Family:             *string | fn.#Fn
			ParameterGroupName: *string | fn.#Fn
			Parameters?:        *{
				[string]: _
			} | fn.#Fn
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
	#SubnetGroup: {
		Type: "AWS::MemoryDB::SubnetGroup"
		Properties: {
			Description?:    *string | fn.#Fn
			SubnetGroupName: *(=~#"[a-z][a-z0-9\-]*"#) | fn.#Fn
			SubnetIds:       [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			Tags?:           *[...{
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
	#User: {
		Type: "AWS::MemoryDB::User"
		Properties: {
			AccessString:       *string | fn.#Fn
			AuthenticationMode: *{
				[string]: _
			} | fn.#Fn
			Tags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			UserName: *(=~#"[a-z][a-z0-9\\-]*"#) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
