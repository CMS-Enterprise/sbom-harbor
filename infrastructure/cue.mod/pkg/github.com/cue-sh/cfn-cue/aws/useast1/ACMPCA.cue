package useast1

import "github.com/cue-sh/cfn-cue/aws/fn"

#ACMPCA: {
	#Certificate: {
		Type: "AWS::ACMPCA::Certificate"
		Properties: {
			ApiPassthrough?: *{
				Extensions?: *{
					CertificatePolicies?: *[...{
						CertPolicyId:      *string | fn.#Fn
						PolicyQualifiers?: *[...{
							PolicyQualifierId: *string | fn.#Fn
							Qualifier:         *{
								CpsUri: *string | fn.#Fn
							} | fn.#If
						}] | fn.#If
					}] | fn.#If
					ExtendedKeyUsage?: *[...{
						ExtendedKeyUsageObjectIdentifier?: *string | fn.#Fn
						ExtendedKeyUsageType?:             *string | fn.#Fn
					}] | fn.#If
					KeyUsage?: *{
						CRLSign?:          *bool | fn.#Fn
						DataEncipherment?: *bool | fn.#Fn
						DecipherOnly?:     *bool | fn.#Fn
						DigitalSignature?: *bool | fn.#Fn
						EncipherOnly?:     *bool | fn.#Fn
						KeyAgreement?:     *bool | fn.#Fn
						KeyCertSign?:      *bool | fn.#Fn
						KeyEncipherment?:  *bool | fn.#Fn
						NonRepudiation?:   *bool | fn.#Fn
					} | fn.#If
					SubjectAlternativeNames?: *[...{
						DirectoryName?: *{
							CommonName?:                 *string | fn.#Fn
							Country?:                    *string | fn.#Fn
							DistinguishedNameQualifier?: *string | fn.#Fn
							GenerationQualifier?:        *string | fn.#Fn
							GivenName?:                  *string | fn.#Fn
							Initials?:                   *string | fn.#Fn
							Locality?:                   *string | fn.#Fn
							Organization?:               *string | fn.#Fn
							OrganizationalUnit?:         *string | fn.#Fn
							Pseudonym?:                  *string | fn.#Fn
							SerialNumber?:               *string | fn.#Fn
							State?:                      *string | fn.#Fn
							Surname?:                    *string | fn.#Fn
							Title?:                      *string | fn.#Fn
						} | fn.#If
						DnsName?:      *string | fn.#Fn
						EdiPartyName?: *{
							NameAssigner: *string | fn.#Fn
							PartyName:    *string | fn.#Fn
						} | fn.#If
						IpAddress?: *string | fn.#Fn
						OtherName?: *{
							TypeId: *string | fn.#Fn
							Value:  *string | fn.#Fn
						} | fn.#If
						RegisteredId?:              *string | fn.#Fn
						Rfc822Name?:                *string | fn.#Fn
						UniformResourceIdentifier?: *string | fn.#Fn
					}] | fn.#If
				} | fn.#If
				Subject?: *{
					CommonName?:                 *string | fn.#Fn
					Country?:                    *string | fn.#Fn
					DistinguishedNameQualifier?: *string | fn.#Fn
					GenerationQualifier?:        *string | fn.#Fn
					GivenName?:                  *string | fn.#Fn
					Initials?:                   *string | fn.#Fn
					Locality?:                   *string | fn.#Fn
					Organization?:               *string | fn.#Fn
					OrganizationalUnit?:         *string | fn.#Fn
					Pseudonym?:                  *string | fn.#Fn
					SerialNumber?:               *string | fn.#Fn
					State?:                      *string | fn.#Fn
					Surname?:                    *string | fn.#Fn
					Title?:                      *string | fn.#Fn
				} | fn.#If
			} | fn.#If
			CertificateAuthorityArn:   *string | fn.#Fn
			CertificateSigningRequest: *string | fn.#Fn
			SigningAlgorithm:          *string | fn.#Fn
			TemplateArn?:              *string | fn.#Fn
			Validity:                  *{
				Type:  *string | fn.#Fn
				Value: *number | fn.#Fn
			} | fn.#If
			ValidityNotBefore?: *{
				Type:  *string | fn.#Fn
				Value: *number | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#CertificateAuthority: {
		Type: "AWS::ACMPCA::CertificateAuthority"
		Properties: {
			CsrExtensions?: *{
				KeyUsage?: *{
					CRLSign?:          *bool | fn.#Fn
					DataEncipherment?: *bool | fn.#Fn
					DecipherOnly?:     *bool | fn.#Fn
					DigitalSignature?: *bool | fn.#Fn
					EncipherOnly?:     *bool | fn.#Fn
					KeyAgreement?:     *bool | fn.#Fn
					KeyCertSign?:      *bool | fn.#Fn
					KeyEncipherment?:  *bool | fn.#Fn
					NonRepudiation?:   *bool | fn.#Fn
				} | fn.#If
				SubjectInformationAccess?: *[...{
					AccessLocation: *{
						DirectoryName?: *{
							CommonName?:                 *string | fn.#Fn
							Country?:                    *string | fn.#Fn
							DistinguishedNameQualifier?: *string | fn.#Fn
							GenerationQualifier?:        *string | fn.#Fn
							GivenName?:                  *string | fn.#Fn
							Initials?:                   *string | fn.#Fn
							Locality?:                   *string | fn.#Fn
							Organization?:               *string | fn.#Fn
							OrganizationalUnit?:         *string | fn.#Fn
							Pseudonym?:                  *string | fn.#Fn
							SerialNumber?:               *string | fn.#Fn
							State?:                      *string | fn.#Fn
							Surname?:                    *string | fn.#Fn
							Title?:                      *string | fn.#Fn
						} | fn.#If
						DnsName?:      *string | fn.#Fn
						EdiPartyName?: *{
							NameAssigner: *string | fn.#Fn
							PartyName:    *string | fn.#Fn
						} | fn.#If
						IpAddress?: *string | fn.#Fn
						OtherName?: *{
							TypeId: *string | fn.#Fn
							Value:  *string | fn.#Fn
						} | fn.#If
						RegisteredId?:              *string | fn.#Fn
						Rfc822Name?:                *string | fn.#Fn
						UniformResourceIdentifier?: *string | fn.#Fn
					} | fn.#If
					AccessMethod: *{
						AccessMethodType?:       *string | fn.#Fn
						CustomObjectIdentifier?: *string | fn.#Fn
					} | fn.#If
				}] | fn.#If
			} | fn.#If
			KeyAlgorithm:                *string | fn.#Fn
			KeyStorageSecurityStandard?: *string | fn.#Fn
			RevocationConfiguration?:    *{
				CrlConfiguration?: *{
					CustomCname?:      *string | fn.#Fn
					Enabled?:          *bool | fn.#Fn
					ExpirationInDays?: *int | fn.#Fn
					S3BucketName?:     *string | fn.#Fn
					S3ObjectAcl?:      *string | fn.#Fn
				} | fn.#If
				OcspConfiguration?: *{
					Enabled?:         *bool | fn.#Fn
					OcspCustomCname?: *string | fn.#Fn
				} | fn.#If
			} | fn.#If
			SigningAlgorithm: *string | fn.#Fn
			Subject:          *{
				CommonName?:                 *string | fn.#Fn
				Country?:                    *string | fn.#Fn
				DistinguishedNameQualifier?: *string | fn.#Fn
				GenerationQualifier?:        *string | fn.#Fn
				GivenName?:                  *string | fn.#Fn
				Initials?:                   *string | fn.#Fn
				Locality?:                   *string | fn.#Fn
				Organization?:               *string | fn.#Fn
				OrganizationalUnit?:         *string | fn.#Fn
				Pseudonym?:                  *string | fn.#Fn
				SerialNumber?:               *string | fn.#Fn
				State?:                      *string | fn.#Fn
				Surname?:                    *string | fn.#Fn
				Title?:                      *string | fn.#Fn
			} | fn.#If
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
	#CertificateAuthorityActivation: {
		Type: "AWS::ACMPCA::CertificateAuthorityActivation"
		Properties: {
			Certificate:             *string | fn.#Fn
			CertificateAuthorityArn: *string | fn.#Fn
			CertificateChain?:       *string | fn.#Fn
			Status?:                 *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Permission: {
		Type: "AWS::ACMPCA::Permission"
		Properties: {
			Actions:                 [...(*string | fn.#Fn)] | (*string | fn.#Fn)
			CertificateAuthorityArn: *string | fn.#Fn
			Principal:               *string | fn.#Fn
			SourceAccount?:          *string | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
