package Maps

import (
	"strings"
)

Environments: [
	"sand",
	"dev",
	"test",
	"stage",
	"prod",
	"all",
	"v1",
]

EnvironmentPattern: strings.Join(Environments, "|")
EnvironmentSchema: or(Environments)
