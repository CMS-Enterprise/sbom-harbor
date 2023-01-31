package sre

import (
	aws "github.com/cue-sh/cfn-cue/aws/useast1"
	"strings"
)

Stack: {
	DependsOn?: [...string]
	Environment: EnvironmentSchema
	Name:     string
	Overrides?:{
		[string]: {
			SopsProfile?: string
			Map?: {...}
		}
	}
	Params?: {}
	Profile:     string
	Region:      RegionSchema
	RegionCode:  string
	Template: aws.Template
	Template: AWSTemplateFormatVersion: "2010-09-09"
	Tags: {
		Organization?: string
		Environment: string
		CuePath:     "${STX::CuePath}"
	}
	TagsEnabled: *true | false
}

#Environments: [
	"sand",
	"dev",
	"test",
	"stage",
	"prod",
	"all",
	"v1",
]

#EnvironmentPattern: strings.Join(Environments, "|")

#EnvironmentSchema: or(Environments)

#RegionCodes: {
  usw2: "us-west-2"
  usw1: "us-west-1"
  use2: "us-east-2"
  use1: "us-east-1"
}

#RegionCodePattern: strings.Join([ regionCode for regionCode, region in RegionCodes ], "|")

#RegionSchema: or([ region for regionCode, region in RegionCodes ])
