package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#WAFv2: {
	#IPSet: {
		Type: "AWS::WAFv2::IPSet"
		Properties: {
			Addresses:        [...(*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(50)) | fn.#Fn)
			Description?:     *(=~#"^[a-zA-Z0-9=:#@/\-,.][a-zA-Z0-9+=:#@/\-,.\s]+[a-zA-Z0-9+=:#@/\-,.]{1,256}$"#) | fn.#Fn
			IPAddressVersion: *("IPV4" | "IPV6") | fn.#Fn
			Name?:            *(=~#"^[0-9A-Za-z_-]{1,128}$"#) | fn.#Fn
			Scope:            *("CLOUDFRONT" | "REGIONAL") | fn.#Fn
			Tags?:            *[...{
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
	#LoggingConfiguration: {
		Type: "AWS::WAFv2::LoggingConfiguration"
		Properties: {
			LogDestinationConfigs: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			LoggingFilter?:        *{
				[string]: _
			} | fn.#Fn
			RedactedFields?: *[...{
				JsonBody?: *{
					[string]: _
				} | fn.#Fn
				Method?: *{
					[string]: _
				} | fn.#Fn
				QueryString?: *{
					[string]: _
				} | fn.#Fn
				SingleHeader?: *{
					[string]: _
				} | fn.#Fn
				UriPath?: *{
					[string]: _
				} | fn.#Fn
			}] | fn.#If
			ResourceArn: *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#RegexPatternSet: {
		Type: "AWS::WAFv2::RegexPatternSet"
		Properties: {
			Description?:          *(=~#"^[a-zA-Z0-9=:#@/\-,.][a-zA-Z0-9+=:#@/\-,.\s]+[a-zA-Z0-9+=:#@/\-,.]{1,256}$"#) | fn.#Fn
			Name?:                 *(=~#"^[0-9A-Za-z_-]{1,128}$"#) | fn.#Fn
			RegularExpressionList: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			Scope:                 *("CLOUDFRONT" | "REGIONAL") | fn.#Fn
			Tags?:                 *[...{
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
	#RuleGroup: {
		Type: "AWS::WAFv2::RuleGroup"
		Properties: {
			Capacity:              *int | fn.#Fn
			CustomResponseBodies?: *{
				[string]: {
					Content:     *string | fn.#Fn
					ContentType: *string | fn.#Fn
				}
			} | fn.#If
			Description?: *(=~#"^[a-zA-Z0-9=:#@/\-,.][a-zA-Z0-9+=:#@/\-,.\s]+[a-zA-Z0-9+=:#@/\-,.]{1,256}$"#) | fn.#Fn
			Name?:        *(=~#"^[0-9A-Za-z_-]{1,128}$"#) | fn.#Fn
			Rules?:       *[...{
				Action?: *{
					Allow?: *{
						[string]: _
					} | fn.#Fn
					Block?: *{
						[string]: _
					} | fn.#Fn
					Count?: *{
						[string]: _
					} | fn.#Fn
				} | fn.#If
				Name:        *(=~#"^[0-9A-Za-z_-]{1,128}$"#) | fn.#Fn
				Priority:    *int | fn.#Fn
				RuleLabels?: *[...{
					Name: *(=~#"^[0-9A-Za-z_:-]{1,1024}$"#) | fn.#Fn
				}] | fn.#If
				Statement:        *_#Statement | fn.#If
				VisibilityConfig: *{
					CloudWatchMetricsEnabled: *bool | fn.#Fn
					MetricName:               *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
					SampledRequestsEnabled:   *bool | fn.#Fn
				} | fn.#If
			}] | fn.#If
			Scope: *("CLOUDFRONT" | "REGIONAL") | fn.#Fn
			Tags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			VisibilityConfig: *{
				CloudWatchMetricsEnabled: *bool | fn.#Fn
				MetricName:               *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
				SampledRequestsEnabled:   *bool | fn.#Fn
			} | fn.#If
			_#Statement: {
				AndStatement?: *{
					Statements: *[..._#Statement] | fn.#If
				} | fn.#If
				ByteMatchStatement?: *{
					FieldToMatch: *{
						AllQueryArguments?: *{
							[string]: _
						} | fn.#Fn
						Body?: *{
							[string]: _
						} | fn.#Fn
						JsonBody?: *{
							InvalidFallbackBehavior?: *("MATCH" | "NO_MATCH" | "EVALUATE_AS_STRING") | fn.#Fn
							MatchPattern:             *{
								All?: *{
									[string]: _
								} | fn.#Fn
								IncludedPaths?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
							} | fn.#If
							MatchScope: *("ALL" | "KEY" | "VALUE") | fn.#Fn
						} | fn.#If
						Method?: *{
							[string]: _
						} | fn.#Fn
						QueryString?: *{
							[string]: _
						} | fn.#Fn
						SingleHeader?: *{
							[string]: _
						} | fn.#Fn
						SingleQueryArgument?: *{
							[string]: _
						} | fn.#Fn
						UriPath?: *{
							[string]: _
						} | fn.#Fn
					} | fn.#If
					PositionalConstraint: *("EXACTLY" | "STARTS_WITH" | "ENDS_WITH" | "CONTAINS" | "CONTAINS_WORD") | fn.#Fn
					SearchString?:        *string | fn.#Fn
					SearchStringBase64?:  *string | fn.#Fn
					TextTransformations:  *[...{
						Priority: *int | fn.#Fn
						Type:     *("NONE" | "COMPRESS_WHITE_SPACE" | "HTML_ENTITY_DECODE" | "LOWERCASE" | "CMD_LINE" | "URL_DECODE" | "BASE64_DECODE" | "HEX_DECODE" | "MD5" | "REPLACE_COMMENTS" | "ESCAPE_SEQ_DECODE" | "SQL_HEX_DECODE" | "CSS_DECODE" | "JS_DECODE" | "NORMALIZE_PATH" | "NORMALIZE_PATH_WIN" | "REMOVE_NULLS" | "REPLACE_NULLS" | "BASE64_DECODE_EXT" | "URL_DECODE_UNI" | "UTF8_TO_UNICODE") | fn.#Fn
					}] | fn.#If
				} | fn.#If
				GeoMatchStatement?: *{
					CountryCodes?:      [...(*(strings.MinRunes(1) & strings.MaxRunes(2)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(2)) | fn.#Fn)
					ForwardedIPConfig?: *{
						FallbackBehavior: *("MATCH" | "NO_MATCH") | fn.#Fn
						HeaderName:       *string | fn.#Fn
					} | fn.#If
				} | fn.#If
				IPSetReferenceStatement?: *{
					Arn:                     *(strings.MinRunes(20) & strings.MaxRunes(2048)) | fn.#Fn
					IPSetForwardedIPConfig?: *{
						FallbackBehavior: *("MATCH" | "NO_MATCH") | fn.#Fn
						HeaderName:       *string | fn.#Fn
						Position:         *("FIRST" | "LAST" | "ANY") | fn.#Fn
					} | fn.#If
				} | fn.#If
				LabelMatchStatement?: *{
					Key:   *(=~#"^[0-9A-Za-z_:-]{1,1024}$"#) | fn.#Fn
					Scope: *("LABEL" | "NAMESPACE") | fn.#Fn
				} | fn.#If
				NotStatement?: *{
					Statement: *_#Statement | fn.#If
				} | fn.#If
				OrStatement?: *{
					Statements: *[..._#Statement] | fn.#If
				} | fn.#If
				RateBasedStatement?: *{
					AggregateKeyType:   *("IP" | "FORWARDED_IP") | fn.#Fn
					ForwardedIPConfig?: *{
						FallbackBehavior: *("MATCH" | "NO_MATCH") | fn.#Fn
						HeaderName:       *string | fn.#Fn
					} | fn.#If
					Limit:               *(>=100 & <=2000000000) | fn.#Fn
					ScopeDownStatement?: *_#Statement | fn.#If
				} | fn.#If
				RegexPatternSetReferenceStatement?: *{
					Arn:          *(strings.MinRunes(20) & strings.MaxRunes(2048)) | fn.#Fn
					FieldToMatch: *{
						AllQueryArguments?: *{
							[string]: _
						} | fn.#Fn
						Body?: *{
							[string]: _
						} | fn.#Fn
						JsonBody?: *{
							InvalidFallbackBehavior?: *("MATCH" | "NO_MATCH" | "EVALUATE_AS_STRING") | fn.#Fn
							MatchPattern:             *{
								All?: *{
									[string]: _
								} | fn.#Fn
								IncludedPaths?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
							} | fn.#If
							MatchScope: *("ALL" | "KEY" | "VALUE") | fn.#Fn
						} | fn.#If
						Method?: *{
							[string]: _
						} | fn.#Fn
						QueryString?: *{
							[string]: _
						} | fn.#Fn
						SingleHeader?: *{
							[string]: _
						} | fn.#Fn
						SingleQueryArgument?: *{
							[string]: _
						} | fn.#Fn
						UriPath?: *{
							[string]: _
						} | fn.#Fn
					} | fn.#If
					TextTransformations: *[...{
						Priority: *int | fn.#Fn
						Type:     *("NONE" | "COMPRESS_WHITE_SPACE" | "HTML_ENTITY_DECODE" | "LOWERCASE" | "CMD_LINE" | "URL_DECODE" | "BASE64_DECODE" | "HEX_DECODE" | "MD5" | "REPLACE_COMMENTS" | "ESCAPE_SEQ_DECODE" | "SQL_HEX_DECODE" | "CSS_DECODE" | "JS_DECODE" | "NORMALIZE_PATH" | "NORMALIZE_PATH_WIN" | "REMOVE_NULLS" | "REPLACE_NULLS" | "BASE64_DECODE_EXT" | "URL_DECODE_UNI" | "UTF8_TO_UNICODE") | fn.#Fn
					}] | fn.#If
				} | fn.#If
				SizeConstraintStatement?: *{
					ComparisonOperator: *("EQ" | "NE" | "LE" | "LT" | "GE" | "GT") | fn.#Fn
					FieldToMatch:       *{
						AllQueryArguments?: *{
							[string]: _
						} | fn.#Fn
						Body?: *{
							[string]: _
						} | fn.#Fn
						JsonBody?: *{
							InvalidFallbackBehavior?: *("MATCH" | "NO_MATCH" | "EVALUATE_AS_STRING") | fn.#Fn
							MatchPattern:             *{
								All?: *{
									[string]: _
								} | fn.#Fn
								IncludedPaths?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
							} | fn.#If
							MatchScope: *("ALL" | "KEY" | "VALUE") | fn.#Fn
						} | fn.#If
						Method?: *{
							[string]: _
						} | fn.#Fn
						QueryString?: *{
							[string]: _
						} | fn.#Fn
						SingleHeader?: *{
							[string]: _
						} | fn.#Fn
						SingleQueryArgument?: *{
							[string]: _
						} | fn.#Fn
						UriPath?: *{
							[string]: _
						} | fn.#Fn
					} | fn.#If
					Size:                *number | fn.#Fn
					TextTransformations: *[...{
						Priority: *int | fn.#Fn
						Type:     *("NONE" | "COMPRESS_WHITE_SPACE" | "HTML_ENTITY_DECODE" | "LOWERCASE" | "CMD_LINE" | "URL_DECODE" | "BASE64_DECODE" | "HEX_DECODE" | "MD5" | "REPLACE_COMMENTS" | "ESCAPE_SEQ_DECODE" | "SQL_HEX_DECODE" | "CSS_DECODE" | "JS_DECODE" | "NORMALIZE_PATH" | "NORMALIZE_PATH_WIN" | "REMOVE_NULLS" | "REPLACE_NULLS" | "BASE64_DECODE_EXT" | "URL_DECODE_UNI" | "UTF8_TO_UNICODE") | fn.#Fn
					}] | fn.#If
				} | fn.#If
				SqliMatchStatement?: *{
					FieldToMatch: *{
						AllQueryArguments?: *{
							[string]: _
						} | fn.#Fn
						Body?: *{
							[string]: _
						} | fn.#Fn
						JsonBody?: *{
							InvalidFallbackBehavior?: *("MATCH" | "NO_MATCH" | "EVALUATE_AS_STRING") | fn.#Fn
							MatchPattern:             *{
								All?: *{
									[string]: _
								} | fn.#Fn
								IncludedPaths?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
							} | fn.#If
							MatchScope: *("ALL" | "KEY" | "VALUE") | fn.#Fn
						} | fn.#If
						Method?: *{
							[string]: _
						} | fn.#Fn
						QueryString?: *{
							[string]: _
						} | fn.#Fn
						SingleHeader?: *{
							[string]: _
						} | fn.#Fn
						SingleQueryArgument?: *{
							[string]: _
						} | fn.#Fn
						UriPath?: *{
							[string]: _
						} | fn.#Fn
					} | fn.#If
					TextTransformations: *[...{
						Priority: *int | fn.#Fn
						Type:     *("NONE" | "COMPRESS_WHITE_SPACE" | "HTML_ENTITY_DECODE" | "LOWERCASE" | "CMD_LINE" | "URL_DECODE" | "BASE64_DECODE" | "HEX_DECODE" | "MD5" | "REPLACE_COMMENTS" | "ESCAPE_SEQ_DECODE" | "SQL_HEX_DECODE" | "CSS_DECODE" | "JS_DECODE" | "NORMALIZE_PATH" | "NORMALIZE_PATH_WIN" | "REMOVE_NULLS" | "REPLACE_NULLS" | "BASE64_DECODE_EXT" | "URL_DECODE_UNI" | "UTF8_TO_UNICODE") | fn.#Fn
					}] | fn.#If
				} | fn.#If
				XssMatchStatement?: *{
					FieldToMatch: *{
						AllQueryArguments?: *{
							[string]: _
						} | fn.#Fn
						Body?: *{
							[string]: _
						} | fn.#Fn
						JsonBody?: *{
							InvalidFallbackBehavior?: *("MATCH" | "NO_MATCH" | "EVALUATE_AS_STRING") | fn.#Fn
							MatchPattern:             *{
								All?: *{
									[string]: _
								} | fn.#Fn
								IncludedPaths?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
							} | fn.#If
							MatchScope: *("ALL" | "KEY" | "VALUE") | fn.#Fn
						} | fn.#If
						Method?: *{
							[string]: _
						} | fn.#Fn
						QueryString?: *{
							[string]: _
						} | fn.#Fn
						SingleHeader?: *{
							[string]: _
						} | fn.#Fn
						SingleQueryArgument?: *{
							[string]: _
						} | fn.#Fn
						UriPath?: *{
							[string]: _
						} | fn.#Fn
					} | fn.#If
					TextTransformations: *[...{
						Priority: *int | fn.#Fn
						Type:     *("NONE" | "COMPRESS_WHITE_SPACE" | "HTML_ENTITY_DECODE" | "LOWERCASE" | "CMD_LINE" | "URL_DECODE" | "BASE64_DECODE" | "HEX_DECODE" | "MD5" | "REPLACE_COMMENTS" | "ESCAPE_SEQ_DECODE" | "SQL_HEX_DECODE" | "CSS_DECODE" | "JS_DECODE" | "NORMALIZE_PATH" | "NORMALIZE_PATH_WIN" | "REMOVE_NULLS" | "REPLACE_NULLS" | "BASE64_DECODE_EXT" | "URL_DECODE_UNI" | "UTF8_TO_UNICODE") | fn.#Fn
					}] | fn.#If
				} | fn.#If
			}
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#WebACL: {
		Type: "AWS::WAFv2::WebACL"
		Properties: {
			CustomResponseBodies?: *{
				[string]: {
					Content:     *string | fn.#Fn
					ContentType: *string | fn.#Fn
				}
			} | fn.#If
			DefaultAction: *{
				Allow?: *{
					CustomRequestHandling?: *{
						InsertHeaders: *[...{
							Name:  *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
							Value: *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
						}] | fn.#If
					} | fn.#If
				} | fn.#If
				Block?: *{
					CustomResponse?: *{
						CustomResponseBodyKey?: *(=~#"^[\w\-]+$"#) | fn.#Fn
						ResponseCode:           *(>=200 & <=600) | fn.#Fn
						ResponseHeaders?:       *[...{
							Name:  *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
							Value: *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
						}] | fn.#If
					} | fn.#If
				} | fn.#If
			} | fn.#If
			Description?: *(=~#"^[a-zA-Z0-9=:#@/\-,.][a-zA-Z0-9+=:#@/\-,.\s]+[a-zA-Z0-9+=:#@/\-,.]{1,256}$"#) | fn.#Fn
			Name?:        *(=~#"^[0-9A-Za-z_-]{1,128}$"#) | fn.#Fn
			Rules?:       *[...{
				Action?: *{
					Allow?: *{
						CustomRequestHandling?: *{
							InsertHeaders: *[...{
								Name:  *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
								Value: *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
							}] | fn.#If
						} | fn.#If
					} | fn.#If
					Block?: *{
						CustomResponse?: *{
							CustomResponseBodyKey?: *(=~#"^[\w\-]+$"#) | fn.#Fn
							ResponseCode:           *(>=200 & <=600) | fn.#Fn
							ResponseHeaders?:       *[...{
								Name:  *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
								Value: *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
							}] | fn.#If
						} | fn.#If
					} | fn.#If
					Count?: *{
						CustomRequestHandling?: *{
							InsertHeaders: *[...{
								Name:  *(strings.MinRunes(1) & strings.MaxRunes(64)) | fn.#Fn
								Value: *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
							}] | fn.#If
						} | fn.#If
					} | fn.#If
				} | fn.#If
				Name:            *(=~#"^[0-9A-Za-z_-]{1,128}$"#) | fn.#Fn
				OverrideAction?: *{
					Count?: *{
						[string]: _
					} | fn.#Fn
					None?: *{
						[string]: _
					} | fn.#Fn
				} | fn.#If
				Priority:    *int | fn.#Fn
				RuleLabels?: *[...{
					Name: *(=~#"^[0-9A-Za-z_:-]{1,1024}$"#) | fn.#Fn
				}] | fn.#If
				Statement:        *_#Statement | fn.#If
				VisibilityConfig: *{
					CloudWatchMetricsEnabled: *bool | fn.#Fn
					MetricName:               *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
					SampledRequestsEnabled:   *bool | fn.#Fn
				} | fn.#If
			}] | fn.#If
			Scope: *("CLOUDFRONT" | "REGIONAL") | fn.#Fn
			Tags?: *[...{
				Key:   *string | fn.#Fn
				Value: *string | fn.#Fn
			}] | fn.#If
			VisibilityConfig: *{
				CloudWatchMetricsEnabled: *bool | fn.#Fn
				MetricName:               *(strings.MinRunes(1) & strings.MaxRunes(128)) | fn.#Fn
				SampledRequestsEnabled:   *bool | fn.#Fn
			} | fn.#If
			_#Statement: {
				AndStatement?: *{
					Statements: *[..._#Statement] | fn.#If
				} | fn.#If
				ByteMatchStatement?: *{
					FieldToMatch: *{
						AllQueryArguments?: *{
							[string]: _
						} | fn.#Fn
						Body?: *{
							[string]: _
						} | fn.#Fn
						JsonBody?: *{
							InvalidFallbackBehavior?: *("MATCH" | "NO_MATCH" | "EVALUATE_AS_STRING") | fn.#Fn
							MatchPattern:             *{
								All?: *{
									[string]: _
								} | fn.#Fn
								IncludedPaths?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
							} | fn.#If
							MatchScope: *("ALL" | "KEY" | "VALUE") | fn.#Fn
						} | fn.#If
						Method?: *{
							[string]: _
						} | fn.#Fn
						QueryString?: *{
							[string]: _
						} | fn.#Fn
						SingleHeader?: *{
							[string]: _
						} | fn.#Fn
						SingleQueryArgument?: *{
							[string]: _
						} | fn.#Fn
						UriPath?: *{
							[string]: _
						} | fn.#Fn
					} | fn.#If
					PositionalConstraint: *("EXACTLY" | "STARTS_WITH" | "ENDS_WITH" | "CONTAINS" | "CONTAINS_WORD") | fn.#Fn
					SearchString?:        *string | fn.#Fn
					SearchStringBase64?:  *string | fn.#Fn
					TextTransformations:  *[...{
						Priority: *int | fn.#Fn
						Type:     *("NONE" | "COMPRESS_WHITE_SPACE" | "HTML_ENTITY_DECODE" | "LOWERCASE" | "CMD_LINE" | "URL_DECODE" | "BASE64_DECODE" | "HEX_DECODE" | "MD5" | "REPLACE_COMMENTS" | "ESCAPE_SEQ_DECODE" | "SQL_HEX_DECODE" | "CSS_DECODE" | "JS_DECODE" | "NORMALIZE_PATH" | "NORMALIZE_PATH_WIN" | "REMOVE_NULLS" | "REPLACE_NULLS" | "BASE64_DECODE_EXT" | "URL_DECODE_UNI" | "UTF8_TO_UNICODE") | fn.#Fn
					}] | fn.#If
				} | fn.#If
				GeoMatchStatement?: *{
					CountryCodes?:      [...(*(strings.MinRunes(1) & strings.MaxRunes(2)) | fn.#Fn)] | (*(strings.MinRunes(1) & strings.MaxRunes(2)) | fn.#Fn)
					ForwardedIPConfig?: *{
						FallbackBehavior: *("MATCH" | "NO_MATCH") | fn.#Fn
						HeaderName:       *string | fn.#Fn
					} | fn.#If
				} | fn.#If
				IPSetReferenceStatement?: *{
					Arn:                     *(strings.MinRunes(20) & strings.MaxRunes(2048)) | fn.#Fn
					IPSetForwardedIPConfig?: *{
						FallbackBehavior: *("MATCH" | "NO_MATCH") | fn.#Fn
						HeaderName:       *string | fn.#Fn
						Position:         *("FIRST" | "LAST" | "ANY") | fn.#Fn
					} | fn.#If
				} | fn.#If
				LabelMatchStatement?: *{
					Key:   *(=~#"^[0-9A-Za-z_:-]{1,1024}$"#) | fn.#Fn
					Scope: *("LABEL" | "NAMESPACE") | fn.#Fn
				} | fn.#If
				ManagedRuleGroupStatement?: *{
					ExcludedRules?: *[...{
						Name: *(=~#"^[0-9A-Za-z_-]{1,128}$"#) | fn.#Fn
					}] | fn.#If
					Name:                *(=~#"^[0-9A-Za-z_-]{1,128}$"#) | fn.#Fn
					ScopeDownStatement?: *_#Statement | fn.#If
					VendorName:          *string | fn.#Fn
					Version?:            *(strings.MinRunes(1) & strings.MaxRunes(64) & (=~#"^[\w#:\.\-/]+$"#)) | fn.#Fn
				} | fn.#If
				NotStatement?: *{
					Statement: *_#Statement | fn.#If
				} | fn.#If
				OrStatement?: *{
					Statements: *[..._#Statement] | fn.#If
				} | fn.#If
				RateBasedStatement?: *{
					AggregateKeyType:   *("IP" | "FORWARDED_IP") | fn.#Fn
					ForwardedIPConfig?: *{
						FallbackBehavior: *("MATCH" | "NO_MATCH") | fn.#Fn
						HeaderName:       *string | fn.#Fn
					} | fn.#If
					Limit:               *(>=100 & <=2000000000) | fn.#Fn
					ScopeDownStatement?: *_#Statement | fn.#If
				} | fn.#If
				RegexPatternSetReferenceStatement?: *{
					Arn:          *(strings.MinRunes(20) & strings.MaxRunes(2048)) | fn.#Fn
					FieldToMatch: *{
						AllQueryArguments?: *{
							[string]: _
						} | fn.#Fn
						Body?: *{
							[string]: _
						} | fn.#Fn
						JsonBody?: *{
							InvalidFallbackBehavior?: *("MATCH" | "NO_MATCH" | "EVALUATE_AS_STRING") | fn.#Fn
							MatchPattern:             *{
								All?: *{
									[string]: _
								} | fn.#Fn
								IncludedPaths?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
							} | fn.#If
							MatchScope: *("ALL" | "KEY" | "VALUE") | fn.#Fn
						} | fn.#If
						Method?: *{
							[string]: _
						} | fn.#Fn
						QueryString?: *{
							[string]: _
						} | fn.#Fn
						SingleHeader?: *{
							[string]: _
						} | fn.#Fn
						SingleQueryArgument?: *{
							[string]: _
						} | fn.#Fn
						UriPath?: *{
							[string]: _
						} | fn.#Fn
					} | fn.#If
					TextTransformations: *[...{
						Priority: *int | fn.#Fn
						Type:     *("NONE" | "COMPRESS_WHITE_SPACE" | "HTML_ENTITY_DECODE" | "LOWERCASE" | "CMD_LINE" | "URL_DECODE" | "BASE64_DECODE" | "HEX_DECODE" | "MD5" | "REPLACE_COMMENTS" | "ESCAPE_SEQ_DECODE" | "SQL_HEX_DECODE" | "CSS_DECODE" | "JS_DECODE" | "NORMALIZE_PATH" | "NORMALIZE_PATH_WIN" | "REMOVE_NULLS" | "REPLACE_NULLS" | "BASE64_DECODE_EXT" | "URL_DECODE_UNI" | "UTF8_TO_UNICODE") | fn.#Fn
					}] | fn.#If
				} | fn.#If
				RuleGroupReferenceStatement?: *{
					Arn:            *(strings.MinRunes(20) & strings.MaxRunes(2048)) | fn.#Fn
					ExcludedRules?: *[...{
						Name: *(=~#"^[0-9A-Za-z_-]{1,128}$"#) | fn.#Fn
					}] | fn.#If
				} | fn.#If
				SizeConstraintStatement?: *{
					ComparisonOperator: *("EQ" | "NE" | "LE" | "LT" | "GE" | "GT") | fn.#Fn
					FieldToMatch:       *{
						AllQueryArguments?: *{
							[string]: _
						} | fn.#Fn
						Body?: *{
							[string]: _
						} | fn.#Fn
						JsonBody?: *{
							InvalidFallbackBehavior?: *("MATCH" | "NO_MATCH" | "EVALUATE_AS_STRING") | fn.#Fn
							MatchPattern:             *{
								All?: *{
									[string]: _
								} | fn.#Fn
								IncludedPaths?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
							} | fn.#If
							MatchScope: *("ALL" | "KEY" | "VALUE") | fn.#Fn
						} | fn.#If
						Method?: *{
							[string]: _
						} | fn.#Fn
						QueryString?: *{
							[string]: _
						} | fn.#Fn
						SingleHeader?: *{
							[string]: _
						} | fn.#Fn
						SingleQueryArgument?: *{
							[string]: _
						} | fn.#Fn
						UriPath?: *{
							[string]: _
						} | fn.#Fn
					} | fn.#If
					Size:                *number | fn.#Fn
					TextTransformations: *[...{
						Priority: *int | fn.#Fn
						Type:     *("NONE" | "COMPRESS_WHITE_SPACE" | "HTML_ENTITY_DECODE" | "LOWERCASE" | "CMD_LINE" | "URL_DECODE" | "BASE64_DECODE" | "HEX_DECODE" | "MD5" | "REPLACE_COMMENTS" | "ESCAPE_SEQ_DECODE" | "SQL_HEX_DECODE" | "CSS_DECODE" | "JS_DECODE" | "NORMALIZE_PATH" | "NORMALIZE_PATH_WIN" | "REMOVE_NULLS" | "REPLACE_NULLS" | "BASE64_DECODE_EXT" | "URL_DECODE_UNI" | "UTF8_TO_UNICODE") | fn.#Fn
					}] | fn.#If
				} | fn.#If
				SqliMatchStatement?: *{
					FieldToMatch: *{
						AllQueryArguments?: *{
							[string]: _
						} | fn.#Fn
						Body?: *{
							[string]: _
						} | fn.#Fn
						JsonBody?: *{
							InvalidFallbackBehavior?: *("MATCH" | "NO_MATCH" | "EVALUATE_AS_STRING") | fn.#Fn
							MatchPattern:             *{
								All?: *{
									[string]: _
								} | fn.#Fn
								IncludedPaths?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
							} | fn.#If
							MatchScope: *("ALL" | "KEY" | "VALUE") | fn.#Fn
						} | fn.#If
						Method?: *{
							[string]: _
						} | fn.#Fn
						QueryString?: *{
							[string]: _
						} | fn.#Fn
						SingleHeader?: *{
							[string]: _
						} | fn.#Fn
						SingleQueryArgument?: *{
							[string]: _
						} | fn.#Fn
						UriPath?: *{
							[string]: _
						} | fn.#Fn
					} | fn.#If
					TextTransformations: *[...{
						Priority: *int | fn.#Fn
						Type:     *("NONE" | "COMPRESS_WHITE_SPACE" | "HTML_ENTITY_DECODE" | "LOWERCASE" | "CMD_LINE" | "URL_DECODE" | "BASE64_DECODE" | "HEX_DECODE" | "MD5" | "REPLACE_COMMENTS" | "ESCAPE_SEQ_DECODE" | "SQL_HEX_DECODE" | "CSS_DECODE" | "JS_DECODE" | "NORMALIZE_PATH" | "NORMALIZE_PATH_WIN" | "REMOVE_NULLS" | "REPLACE_NULLS" | "BASE64_DECODE_EXT" | "URL_DECODE_UNI" | "UTF8_TO_UNICODE") | fn.#Fn
					}] | fn.#If
				} | fn.#If
				XssMatchStatement?: *{
					FieldToMatch: *{
						AllQueryArguments?: *{
							[string]: _
						} | fn.#Fn
						Body?: *{
							[string]: _
						} | fn.#Fn
						JsonBody?: *{
							InvalidFallbackBehavior?: *("MATCH" | "NO_MATCH" | "EVALUATE_AS_STRING") | fn.#Fn
							MatchPattern:             *{
								All?: *{
									[string]: _
								} | fn.#Fn
								IncludedPaths?: [...(*string | fn.#Fn)] | (*string | fn.#Fn)
							} | fn.#If
							MatchScope: *("ALL" | "KEY" | "VALUE") | fn.#Fn
						} | fn.#If
						Method?: *{
							[string]: _
						} | fn.#Fn
						QueryString?: *{
							[string]: _
						} | fn.#Fn
						SingleHeader?: *{
							[string]: _
						} | fn.#Fn
						SingleQueryArgument?: *{
							[string]: _
						} | fn.#Fn
						UriPath?: *{
							[string]: _
						} | fn.#Fn
					} | fn.#If
					TextTransformations: *[...{
						Priority: *int | fn.#Fn
						Type:     *("NONE" | "COMPRESS_WHITE_SPACE" | "HTML_ENTITY_DECODE" | "LOWERCASE" | "CMD_LINE" | "URL_DECODE" | "BASE64_DECODE" | "HEX_DECODE" | "MD5" | "REPLACE_COMMENTS" | "ESCAPE_SEQ_DECODE" | "SQL_HEX_DECODE" | "CSS_DECODE" | "JS_DECODE" | "NORMALIZE_PATH" | "NORMALIZE_PATH_WIN" | "REMOVE_NULLS" | "REPLACE_NULLS" | "BASE64_DECODE_EXT" | "URL_DECODE_UNI" | "UTF8_TO_UNICODE") | fn.#Fn
					}] | fn.#If
				} | fn.#If
			}
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#WebACLAssociation: {
		Type: "AWS::WAFv2::WebACLAssociation"
		Properties: {
			ResourceArn: *(strings.MinRunes(20) & strings.MaxRunes(2048)) | fn.#Fn
			WebACLArn:   *(strings.MinRunes(20) & strings.MaxRunes(2048)) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
