package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#CodeBuild: {
	#Project: {
		Type: "AWS::CodeBuild::Project"
		Properties: {
			Artifacts: *{
				ArtifactIdentifier?:   *string | fn.#Fn
				EncryptionDisabled?:   *bool | fn.#Fn
				Location?:             *string | fn.#Fn
				Name?:                 *string | fn.#Fn
				NamespaceType?:        *string | fn.#Fn
				OverrideArtifactName?: *bool | fn.#Fn
				Packaging?:            *("NONE" | "ZIP") | fn.#Fn
				Path?:                 *string | fn.#Fn
				Type:                  *("CODEPIPELINE" | "NO_ARTIFACTS" | "S3") | fn.#Fn
			} | fn.#If
			BadgeEnabled?:     *bool | fn.#Fn
			BuildBatchConfig?: *{
				BatchReportMode?:  *string | fn.#Fn
				CombineArtifacts?: *bool | fn.#Fn
				Restrictions?:     *{
					ComputeTypesAllowed?:  [...(*string | fn.#Fn)] | (*string | fn.#Fn)
					MaximumBuildsAllowed?: *int | fn.#Fn
				} | fn.#If
				ServiceRole?:   *string | fn.#Fn
				TimeoutInMins?: *int | fn.#Fn
			} | fn.#If
			Cache?: *{
				Location?: *string | fn.#Fn
				Modes?:    [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				Type:      *("LOCAL" | "NO_CACHE" | "S3") | fn.#Fn
			} | fn.#If
			ConcurrentBuildLimit?: *int | fn.#Fn
			Description?:          *string | fn.#Fn
			EncryptionKey?:        *string | fn.#Fn
			Environment:           *{
				Certificate?:          *string | fn.#Fn
				ComputeType:           *("BUILD_GENERAL1_2XLARGE" | "BUILD_GENERAL1_LARGE" | "BUILD_GENERAL1_MEDIUM" | "BUILD_GENERAL1_SMALL") | fn.#Fn
				EnvironmentVariables?: *[...{
					Name:  *string | fn.#Fn
					Type?: *string | fn.#Fn
					Value: *string | fn.#Fn
				}] | fn.#If
				Image:                     *string | fn.#Fn
				ImagePullCredentialsType?: *("CODEBUILD" | "SERVICE_ROLE") | fn.#Fn
				PrivilegedMode?:           *bool | fn.#Fn
				RegistryCredential?:       *{
					Credential:         *string | fn.#Fn
					CredentialProvider: *string | fn.#Fn
				} | fn.#If
				Type: *("ARM_CONTAINER" | "LINUX_CONTAINER" | "LINUX_GPU_CONTAINER" | "WINDOWS_CONTAINER" | "WINDOWS_SERVER_2019_CONTAINER") | fn.#Fn
			} | fn.#If
			FileSystemLocations?: *[...{
				Identifier:    *string | fn.#Fn
				Location:      *string | fn.#Fn
				MountOptions?: *string | fn.#Fn
				MountPoint:    *string | fn.#Fn
				Type:          *string | fn.#Fn
			}] | fn.#If
			LogsConfig?: *{
				CloudWatchLogs?: *{
					GroupName?:  *string | fn.#Fn
					Status:      *("DISABLED" | "ENABLED") | fn.#Fn
					StreamName?: *string | fn.#Fn
				} | fn.#If
				S3Logs?: *{
					EncryptionDisabled?: *bool | fn.#Fn
					Location?:           *string | fn.#Fn
					Status:              *("DISABLED" | "ENABLED") | fn.#Fn
				} | fn.#If
			} | fn.#If
			Name?:                   *string | fn.#Fn
			QueuedTimeoutInMinutes?: *(>=5 & <=480) | fn.#Fn
			ResourceAccessRole?:     *string | fn.#Fn
			SecondaryArtifacts?:     *[...{
				ArtifactIdentifier?:   *string | fn.#Fn
				EncryptionDisabled?:   *bool | fn.#Fn
				Location?:             *string | fn.#Fn
				Name?:                 *string | fn.#Fn
				NamespaceType?:        *string | fn.#Fn
				OverrideArtifactName?: *bool | fn.#Fn
				Packaging?:            *("NONE" | "ZIP") | fn.#Fn
				Path?:                 *string | fn.#Fn
				Type:                  *("CODEPIPELINE" | "NO_ARTIFACTS" | "S3") | fn.#Fn
			}] | fn.#If
			SecondarySourceVersions?: *[...{
				SourceIdentifier: *string | fn.#Fn
				SourceVersion?:   *string | fn.#Fn
			}] | fn.#If
			SecondarySources?: *[...{
				Auth?: *{
					Resource?: *string | fn.#Fn
					Type:      *string | fn.#Fn
				} | fn.#If
				BuildSpec?:         *string | fn.#Fn
				BuildStatusConfig?: *{
					Context?:   *string | fn.#Fn
					TargetUrl?: *string | fn.#Fn
				} | fn.#If
				GitCloneDepth?:       *int | fn.#Fn
				GitSubmodulesConfig?: *{
					FetchSubmodules: *bool | fn.#Fn
				} | fn.#If
				InsecureSsl?:       *bool | fn.#Fn
				Location?:          *string | fn.#Fn
				ReportBuildStatus?: *bool | fn.#Fn
				SourceIdentifier?:  *string | fn.#Fn
				Type:               *("BITBUCKET" | "CODECOMMIT" | "CODEPIPELINE" | "GITHUB" | "GITHUB_ENTERPRISE" | "NO_SOURCE" | "S3") | fn.#Fn
			}] | fn.#If
			ServiceRole: *string | fn.#Fn
			Source:      *{
				Auth?: *{
					Resource?: *string | fn.#Fn
					Type:      *string | fn.#Fn
				} | fn.#If
				BuildSpec?:         *string | fn.#Fn
				BuildStatusConfig?: *{
					Context?:   *string | fn.#Fn
					TargetUrl?: *string | fn.#Fn
				} | fn.#If
				GitCloneDepth?:       *int | fn.#Fn
				GitSubmodulesConfig?: *{
					FetchSubmodules: *bool | fn.#Fn
				} | fn.#If
				InsecureSsl?:       *bool | fn.#Fn
				Location?:          *string | fn.#Fn
				ReportBuildStatus?: *bool | fn.#Fn
				SourceIdentifier?:  *string | fn.#Fn
				Type:               *("BITBUCKET" | "CODECOMMIT" | "CODEPIPELINE" | "GITHUB" | "GITHUB_ENTERPRISE" | "NO_SOURCE" | "S3") | fn.#Fn
			} | fn.#If
			SourceVersion?: *string | fn.#Fn
			Tags?:          *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			TimeoutInMinutes?: *(>=5 & <=480) | fn.#Fn
			Triggers?:         *{
				BuildType?:    *string | fn.#Fn
				FilterGroups?: *[...[...{
					ExcludeMatchedPattern?: *bool | fn.#Fn
					Pattern:                *string | fn.#Fn
					Type:                   *string | fn.#Fn
				}]] | fn.#If
				Webhook?: *bool | fn.#Fn
			} | fn.#If
			Visibility?: *string | fn.#Fn
			VpcConfig?:  *{
				SecurityGroupIds?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				Subnets?:          [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				VpcId?:            *string | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#ReportGroup: {
		Type: "AWS::CodeBuild::ReportGroup"
		Properties: {
			DeleteReports?: *bool | fn.#Fn
			ExportConfig:   *{
				ExportConfigType: *string | fn.#Fn
				S3Destination?:   *{
					Bucket:              *string | fn.#Fn
					BucketOwner?:        *string | fn.#Fn
					EncryptionDisabled?: *bool | fn.#Fn
					EncryptionKey?:      *string | fn.#Fn
					Packaging?:          *string | fn.#Fn
					Path?:               *string | fn.#Fn
				} | fn.#If
			} | fn.#If
			Name?: *string | fn.#Fn
			Tags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			Type: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#SourceCredential: {
		Type: "AWS::CodeBuild::SourceCredential"
		Properties: {
			AuthType:   *string | fn.#Fn
			ServerType: *string | fn.#Fn
			Token:      *string | fn.#Fn
			Username?:  *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
