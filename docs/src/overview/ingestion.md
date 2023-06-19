## Ingestion

The _ingestion_ phase is the phase during which Harbor receives an SBOM for a software package
being monitored. SBOMs can be _ingested_ in one of three ways:

- Syncing with an external SBOM provider (see `cli/src/commands/sbom` and
  `sdk/core/src/services/sboms/snyk`).
- Manual upload through the UI.
- Automated upload via the API.

During the _ingestion_ phase, SBOMs are parsed, analyzed, and then mapped to the Harbor data model.
Once that process is complete the raw SBOM is then stored. In order to maintain the integrity of the
system relative to the tampering concern, SBOMs should be treated as **immutable** once stored. The
CMS Harbor instance stores SBOMs in Amazon S3 and leverages features of that platform to ensure
immutability. Organizations that wish to run a Harbor instance need to be aware of this concern and
develop a tamper-resistance storage strategy appropriate for their environment.

> By convention, the Harbor team implements tasks as CLI commands that can be run via an 
> orchestrator. In our case, we deploy Fargate tasks that can be parameterized and invoked.
> Organizations are free to leverage the business logic found in `sdk/core/src/services` using 
> whatever scheduling mechanism is appropriate for their operating environment.
