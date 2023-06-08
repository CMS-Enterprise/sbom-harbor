## Continuous Enrichment

Vulnerabilities emerge non-deterministically over time. A dependency that did not have any known
vulnerabilities yesterday may have one or more today. Similarly, EPSS and CVSS scores may change
over time.

To ensure that the latest enrichment data is available, the CLI `enrich` command can be run
idempotently at scheduled intervals. We call this batch scan process **continuous enrichment**.
Follow the code path in `cli/commands/enrich` for examples of how to implement a continuous
enrichment provider.
