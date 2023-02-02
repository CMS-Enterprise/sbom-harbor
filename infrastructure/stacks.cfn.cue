package cfn

import (
	aws "github.com/cue-sh/cfn-cue/aws/useast1"
	opsCommonOutouts "local.io/ops/common:outputs"
	"strings"
	"regexp"
)

#Stack: {
	DependsOn?: [...string]
	Environment: #EnvironmentSchema
	Name:        string
	Overrides?: {
		[string]: {
			SopsProfile?: string
			Map?: {...}
		}
	}
	Params?: {}
	Profile:    string
	Region:     #RegionSchema
	RegionCode: string
	Role?:      string
	Template:   aws.#Template
	Template: AWSTemplateFormatVersion: "2010-09-09"
	Tags: {
		Organization?: string
		Environment:   string
		CuePath:       "${STX::CuePath}"
	}
	TagsEnabled: *true | false
}

#Environments: [
	"sand",
	"dev",
	"prod",
]

#EnvironmentPattern: strings.Join(#Environments, "|")

#EnvironmentSchema: or(#Environments)

#RegionCodes: {
	usw2: "us-west-2"
	usw1: "us-west-1"
	use2: "us-east-2"
	use1: "us-east-1"
}

#RegionCodePattern: strings.Join([ for regionCode, region in #RegionCodes {regionCode}], "|")

#RegionSchema: or([ for regionCode, region in #RegionCodes {region}])

// loosely contrained to the cloudformation stack name limit
// cannot start with number, max 128 characters, letters, numbers, and hyphens only
Stacks: close({
	[StackName= =~"^([a-zA-Z]{1})[a-zA-Z0-9-]{2,128}"]: #Stack
})

Stacks: {
	// newly created stacks should match the following pattern: <environment>-<semantic name>-<regionCode>
	[StackName= =~"^(\(#EnvironmentPattern))-([a-zA-Z0-9-]{3,})-(\(#RegionCodePattern))$"]: {
		Name:        StackName
		Environment: regexp.Find("^\(#EnvironmentPattern)", StackName)
		Profile:     string | *"default"
		RegionCode:  regexp.Find("\(#RegionCodePattern)$", StackName)
		Region:      #RegionCodes[RegionCode]
		Role:        string | *opsCommonOutouts["\(Environment)-\(_baseNames.opsCommon)-\(RegionCode)"].CloudFormationRoleArn
		Tags: "Environment": Environment
	}

	// environment specific settings
	[=~"^prod-"]: {
		Profile: "cms-prod"
	}
}

_baseNames: {
	vpcSubnets: "harbor-network-vpc-subnets"
	gateways:   "harbor-network-gateways"
	routes:     "harbor-network-routes"
	nacls:      "harbor-network-nacls"
	endpoints:  "harbor-network-endpoints"
	opsCommon:  "harbor-ops-common"
	ghaRunners: "harbor-ops-gha-runners"
}
