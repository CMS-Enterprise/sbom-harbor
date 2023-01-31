package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#RUM: {
	#AppMonitor: {
		Type: "AWS::RUM::AppMonitor"
		Properties: {
			AppMonitorConfiguration?: *{
				AllowCookies?:      *bool | fn.#Fn
				EnableXRay?:        *bool | fn.#Fn
				ExcludedPages?:     [...(*(strings.MinRunes(1) & strings.MaxRunes(1260) & (=~#"https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)"#)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(1260) & (=~#"https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)"#)) | fn.#Fn)
				FavoritePages?:     [...(*string | fn.#Fn)] | (*string | fn.#Fn)
				GuestRoleArn?:      *(=~#"arn:[^:]*:[^:]*:[^:]*:[^:]*:.*"#) | fn.#Fn
				IdentityPoolId?:    *(strings.MinRunes(1) & strings.MaxRunes(55) & (=~#"[\w-]+:[0-9a-f-]+"#)) | fn.#Fn
				IncludedPages?:     [...(*(strings.MinRunes(1) & strings.MaxRunes(1260) & (=~#"https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)"#)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(1260) & (=~#"https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)"#)) | fn.#Fn)
				SessionSampleRate?: *number | fn.#Fn
				Telemetries?:       [...(*("errors" | "performance" | "http") | fn.#Fn)] | (*("errors" | "performance" | "http") | fn.#Fn)
			} | fn.#If
			CwLogEnabled?: *bool | fn.#Fn
			Domain?:       *(strings.MinRunes(1) & strings.MaxRunes(253) & (=~#"^(localhost)|^((25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$|^(?![-.])([A-Za-z0-9-\.\-]{0,63})((?![-])([a-zA-Z0-9]{1}|^[a-zA-Z0-9]{0,1}))\.(?![-])[A-Za-z-0-9]{1,63}((?![-])([a-zA-Z0-9]{1}|^[a-zA-Z0-9]{0,1}))|^(\*\.)(?![-.])([A-Za-z0-9-\.\-]{0,63})((?![-])([a-zA-Z0-9]{1}|^[a-zA-Z0-9]{0,1}))\.(?![-])[A-Za-z-0-9]{1,63}((?![-])([a-zA-Z0-9]{1}|^[a-zA-Z0-9]{0,1}))"#)) | fn.#Fn
			Name?:         *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"[\.\-_/#A-Za-z0-9]+"#)) | fn.#Fn
			Tags?:         *[...{
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
