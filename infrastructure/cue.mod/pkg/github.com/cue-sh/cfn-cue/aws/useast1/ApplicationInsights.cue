package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#ApplicationInsights: {
	#Application: {
		Type: "AWS::ApplicationInsights::Application"
		Properties: {
			AutoConfigurationEnabled?:    *bool | fn.#Fn
			CWEMonitorEnabled?:           *bool | fn.#Fn
			ComponentMonitoringSettings?: *[...{
				ComponentARN?:                 *(strings.MinRunes(20) & strings.MaxRunes(300) & (=~#"^arn:aws(-[\w]+)*:[\w\d-]+:([\w\d-]*)?:[\w\d_-]*([:/].+)*$"#)) | fn.#Fn
				ComponentConfigurationMode:    *("DEFAULT" | "DEFAULT_WITH_OVERWRITE" | "CUSTOM") | fn.#Fn
				ComponentName?:                *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[\d\w\-_.+]*$"#)) | fn.#Fn
				CustomComponentConfiguration?: *{
					ConfigurationDetails?: *{
						AlarmMetrics?: *[...{
							AlarmMetricName: *string | fn.#Fn
						}] | fn.#If
						Alarms?: *[...{
							AlarmName: *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
							Severity?: *("HIGH" | "MEDIUM" | "LOW") | fn.#Fn
						}] | fn.#If
						JMXPrometheusExporter?: *{
							HostPort?:       *string | fn.#Fn
							JMXURL?:         *string | fn.#Fn
							PrometheusPort?: *string | fn.#Fn
						} | fn.#If
						Logs?: *[...{
							Encoding?:     *("utf-8" | "utf-16" | "ascii") | fn.#Fn
							LogGroupName?: *(strings.MinRunes(1) & strings.MaxRunes(512) & (=~#"[\.\-_/#A-Za-z0-9]+"#)) | fn.#Fn
							LogPath?:      *(strings.MinRunes(1) & strings.MaxRunes(260) & (=~#"^([a-zA-Z]:\\[\\\S|*\S]?.*|/[^"']*)$"#)) | fn.#Fn
							LogType:       *(=~#"^[A-Z][[A-Z]_]*$"#) | fn.#Fn
							PatternSet?:   *(strings.MinRunes(1) & strings.MaxRunes(30) & (=~#"[a-zA-Z0-9.-_]*"#)) | fn.#Fn
						}] | fn.#If
						WindowsEvents?: *[...{
							EventLevels:  [...(*("INFORMATION" | "WARNING" | "ERROR" | "CRITICAL" | "VERBOSE") | fn.#Fn)] | (*("INFORMATION" | "WARNING" | "ERROR" | "CRITICAL" | "VERBOSE") | fn.#Fn)
							EventName:    *(strings.MinRunes(1) & strings.MaxRunes(260) & (=~#"^[a-zA-Z0-9_ \\/-]$"#)) | fn.#Fn
							LogGroupName: *(strings.MinRunes(1) & strings.MaxRunes(512) & (=~#"[\.\-_/#A-Za-z0-9]+"#)) | fn.#Fn
							PatternSet?:  *(strings.MinRunes(1) & strings.MaxRunes(30) & (=~#"[a-zA-Z0-9.-_]*"#)) | fn.#Fn
						}] | fn.#If
					} | fn.#If
					SubComponentTypeConfigurations?: *[...{
						SubComponentConfigurationDetails: *{
							AlarmMetrics?: *[...{
								AlarmMetricName: *string | fn.#Fn
							}] | fn.#If
							Logs?: *[...{
								Encoding?:     *("utf-8" | "utf-16" | "ascii") | fn.#Fn
								LogGroupName?: *(strings.MinRunes(1) & strings.MaxRunes(512) & (=~#"[\.\-_/#A-Za-z0-9]+"#)) | fn.#Fn
								LogPath?:      *(strings.MinRunes(1) & strings.MaxRunes(260) & (=~#"^([a-zA-Z]:\\[\\\S|*\S]?.*|/[^"']*)$"#)) | fn.#Fn
								LogType:       *(=~#"^[A-Z][[A-Z]_]*$"#) | fn.#Fn
								PatternSet?:   *(strings.MinRunes(1) & strings.MaxRunes(30) & (=~#"[a-zA-Z0-9.-_]*"#)) | fn.#Fn
							}] | fn.#If
							WindowsEvents?: *[...{
								EventLevels:  [...(*("INFORMATION" | "WARNING" | "ERROR" | "CRITICAL" | "VERBOSE") | fn.#Fn)] | (*("INFORMATION" | "WARNING" | "ERROR" | "CRITICAL" | "VERBOSE") | fn.#Fn)
								EventName:    *(strings.MinRunes(1) & strings.MaxRunes(260) & (=~#"^[a-zA-Z0-9_ \\/-]$"#)) | fn.#Fn
								LogGroupName: *(strings.MinRunes(1) & strings.MaxRunes(512) & (=~#"[\.\-_/#A-Za-z0-9]+"#)) | fn.#Fn
								PatternSet?:  *(strings.MinRunes(1) & strings.MaxRunes(30) & (=~#"[a-zA-Z0-9.-_]*"#)) | fn.#Fn
							}] | fn.#If
						} | fn.#If
						SubComponentType: *("AWS::EC2::Instance" | "AWS::EC2::Volume") | fn.#Fn
					}] | fn.#If
				} | fn.#If
				DefaultOverwriteComponentConfiguration?: *{
					ConfigurationDetails?: *{
						AlarmMetrics?: *[...{
							AlarmMetricName: *string | fn.#Fn
						}] | fn.#If
						Alarms?: *[...{
							AlarmName: *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
							Severity?: *("HIGH" | "MEDIUM" | "LOW") | fn.#Fn
						}] | fn.#If
						JMXPrometheusExporter?: *{
							HostPort?:       *string | fn.#Fn
							JMXURL?:         *string | fn.#Fn
							PrometheusPort?: *string | fn.#Fn
						} | fn.#If
						Logs?: *[...{
							Encoding?:     *("utf-8" | "utf-16" | "ascii") | fn.#Fn
							LogGroupName?: *(strings.MinRunes(1) & strings.MaxRunes(512) & (=~#"[\.\-_/#A-Za-z0-9]+"#)) | fn.#Fn
							LogPath?:      *(strings.MinRunes(1) & strings.MaxRunes(260) & (=~#"^([a-zA-Z]:\\[\\\S|*\S]?.*|/[^"']*)$"#)) | fn.#Fn
							LogType:       *(=~#"^[A-Z][[A-Z]_]*$"#) | fn.#Fn
							PatternSet?:   *(strings.MinRunes(1) & strings.MaxRunes(30) & (=~#"[a-zA-Z0-9.-_]*"#)) | fn.#Fn
						}] | fn.#If
						WindowsEvents?: *[...{
							EventLevels:  [...(*("INFORMATION" | "WARNING" | "ERROR" | "CRITICAL" | "VERBOSE") | fn.#Fn)] | (*("INFORMATION" | "WARNING" | "ERROR" | "CRITICAL" | "VERBOSE") | fn.#Fn)
							EventName:    *(strings.MinRunes(1) & strings.MaxRunes(260) & (=~#"^[a-zA-Z0-9_ \\/-]$"#)) | fn.#Fn
							LogGroupName: *(strings.MinRunes(1) & strings.MaxRunes(512) & (=~#"[\.\-_/#A-Za-z0-9]+"#)) | fn.#Fn
							PatternSet?:  *(strings.MinRunes(1) & strings.MaxRunes(30) & (=~#"[a-zA-Z0-9.-_]*"#)) | fn.#Fn
						}] | fn.#If
					} | fn.#If
					SubComponentTypeConfigurations?: *[...{
						SubComponentConfigurationDetails: *{
							AlarmMetrics?: *[...{
								AlarmMetricName: *string | fn.#Fn
							}] | fn.#If
							Logs?: *[...{
								Encoding?:     *("utf-8" | "utf-16" | "ascii") | fn.#Fn
								LogGroupName?: *(strings.MinRunes(1) & strings.MaxRunes(512) & (=~#"[\.\-_/#A-Za-z0-9]+"#)) | fn.#Fn
								LogPath?:      *(strings.MinRunes(1) & strings.MaxRunes(260) & (=~#"^([a-zA-Z]:\\[\\\S|*\S]?.*|/[^"']*)$"#)) | fn.#Fn
								LogType:       *(=~#"^[A-Z][[A-Z]_]*$"#) | fn.#Fn
								PatternSet?:   *(strings.MinRunes(1) & strings.MaxRunes(30) & (=~#"[a-zA-Z0-9.-_]*"#)) | fn.#Fn
							}] | fn.#If
							WindowsEvents?: *[...{
								EventLevels:  [...(*("INFORMATION" | "WARNING" | "ERROR" | "CRITICAL" | "VERBOSE") | fn.#Fn)] | (*("INFORMATION" | "WARNING" | "ERROR" | "CRITICAL" | "VERBOSE") | fn.#Fn)
								EventName:    *(strings.MinRunes(1) & strings.MaxRunes(260) & (=~#"^[a-zA-Z0-9_ \\/-]$"#)) | fn.#Fn
								LogGroupName: *(strings.MinRunes(1) & strings.MaxRunes(512) & (=~#"[\.\-_/#A-Za-z0-9]+"#)) | fn.#Fn
								PatternSet?:  *(strings.MinRunes(1) & strings.MaxRunes(30) & (=~#"[a-zA-Z0-9.-_]*"#)) | fn.#Fn
							}] | fn.#If
						} | fn.#If
						SubComponentType: *("AWS::EC2::Instance" | "AWS::EC2::Volume") | fn.#Fn
					}] | fn.#If
				} | fn.#If
				Tier: *(=~#"^[A-Z][[A-Z]_]*$"#) | fn.#Fn
			}] | fn.#If
			CustomComponents?: *[...{
				ComponentName: *(strings.MinRunes(1) & strings.MaxRunes(128) & (=~#"^[\d\w\-_.+]*$"#)) | fn.#Fn
				ResourceList:  [...(*(strings.MinRunes(20) & strings.MaxRunes(300) & (=~#"^arn:aws(-[\w]+)*:[\w\d-]+:([\w\d-]*)?:[\w\d_-]*([:/].+)*$"#)) | fn.#Fn)] | (*(strings.MinRunes(20) & strings.MaxRunes(300) & (=~#"^arn:aws(-[\w]+)*:[\w\d-]+:([\w\d-]*)?:[\w\d_-]*([:/].+)*$"#)) | fn.#Fn)
			}] | fn.#If
			LogPatternSets?: *[...{
				LogPatterns: *[...{
					Pattern:     *(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn
					PatternName: *(strings.MinRunes(1) & strings.MaxRunes(50) & (=~#"[a-zA-Z0-9.-_]*"#)) | fn.#Fn
					Rank:        *int | fn.#Fn
				}] | fn.#If
				PatternSetName: *(strings.MinRunes(1) & strings.MaxRunes(30) & (=~#"[a-zA-Z0-9.-_]*"#)) | fn.#Fn
			}] | fn.#If
			OpsCenterEnabled?:   *bool | fn.#Fn
			OpsItemSNSTopicArn?: *(strings.MinRunes(20) & strings.MaxRunes(300) & (=~#"^arn:aws(-[\w]+)*:[\w\d-]+:([\w\d-]*)?:[\w\d_-]*([:/].+)*$"#)) | fn.#Fn
			ResourceGroupName:   *(strings.MinRunes(1) & strings.MaxRunes(256) & (=~#"[a-zA-Z0-9.-_]*"#)) | fn.#Fn
			Tags?:               *[...{
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
