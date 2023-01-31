package Maps

import (
	"strings"
)

RegionCodes: {
  usw2: "us-west-2"
  usw1: "us-west-1"
  use2: "us-east-2"
  use1: "us-east-1"
}

RegionCodePattern: strings.Join([ regionCode for regionCode, region in RegionCodes ], "|")

RegionSchema: or([ region for regionCode, region in RegionCodes ])
