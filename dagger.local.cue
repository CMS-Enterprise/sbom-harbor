package main

_local: {
    awsCredsProcess: "get-cms-creds.sh 557147098836 sbom-application-admin"
    dbName: "derekstrickland"
	secretNames: {
		DbConnectionJson: "dev-harbor-documentdb-use1"
		SnykToken:        "dev-harbor-snyk-token-use1"
		IonChannelToken:  "dev-harbor-ion-channel-token-use1"
		GitHubPAT:        "dev-harbor-github-pat-use1"
	}
}
