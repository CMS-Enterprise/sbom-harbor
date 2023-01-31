package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#LakeFormation: {
	#DataLakeSettings: {
		Type: "AWS::LakeFormation::DataLakeSettings"
		Properties: {
			Admins?: *[...{
				DataLakePrincipalIdentifier?: *string | fn.#Fn
			}] | fn.#If
			TrustedResourceOwners?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Permissions: {
		Type: "AWS::LakeFormation::Permissions"
		Properties: {
			DataLakePrincipal: *{
				DataLakePrincipalIdentifier?: *string | fn.#Fn
			} | fn.#If
			Permissions?:                [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			PermissionsWithGrantOption?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			Resource:                    *{
				DataLocationResource?: *{
					CatalogId?:  *string | fn.#Fn
					S3Resource?: *string | fn.#Fn
				} | fn.#If
				DatabaseResource?: *{
					CatalogId?: *string | fn.#Fn
					Name?:      *string | fn.#Fn
				} | fn.#If
				TableResource?: *{
					CatalogId?:     *string | fn.#Fn
					DatabaseName?:  *string | fn.#Fn
					Name?:          *string | fn.#Fn
					TableWildcard?: *{} | fn.#If
				} | fn.#If
				TableWithColumnsResource?: *{
					CatalogId?:      *string | fn.#Fn
					ColumnNames?:    [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					ColumnWildcard?: *{
						ExcludedColumnNames?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					} | fn.#If
					DatabaseName?: *string | fn.#Fn
					Name?:         *string | fn.#Fn
				} | fn.#If
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Resource: {
		Type: "AWS::LakeFormation::Resource"
		Properties: {
			ResourceArn:          *string | fn.#Fn
			RoleArn?:             *string | fn.#Fn
			UseServiceLinkedRole: *bool | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
