package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#CUR: {
	#ReportDefinition: {
		Type: "AWS::CUR::ReportDefinition"
		Properties: {
			AdditionalArtifacts?:      [...(*("REDSHIFT" | "QUICKSIGHT" | "ATHENA") | fn.#Fn)] | (*("REDSHIFT" | "QUICKSIGHT" | "ATHENA") | fn.#Fn)
			AdditionalSchemaElements?: [...(*("RESOURCES") | fn.#Fn)] | (*("RESOURCES") | fn.#Fn)
			BillingViewArn?:           *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"(arn:aws(-cn)?:billing::[0-9]{12}:billingview/)?[a-zA-Z0-9_\+=\.\-@].{1,30}"#)) | fn.#Fn
			Compression:               *("ZIP" | "GZIP" | "Parquet") | fn.#Fn
			Format:                    *("textORcsv" | "Parquet") | fn.#Fn
			RefreshClosedReports:      *bool | fn.#Fn
			ReportName:                *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"[0-9A-Za-z!\-_.*\'()]+"#)) | fn.#Fn
			ReportVersioning:          *("CREATE_NEW_REPORT" | "OVERWRITE_REPORT") | fn.#Fn
			S3Bucket:                  *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"[A-Za-z0-9_\.\-]+"#)) | fn.#Fn
			S3Prefix:                  *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"[0-9A-Za-z!\-_.*\'()/]*"#)) | fn.#Fn
			S3Region:                  *string | fn.#Fn
			TimeUnit:                  *("HOURLY" | "DAILY" | "MONTHLY") | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
