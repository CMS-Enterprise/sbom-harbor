## How does Harbor work?

An SBOM is just a file that is guaranteed to conform to a specification that is designed for
machine-readability. As long as the file conforms to the specification, Harbor can parse it once it
is received, and identify its dependencies. The primary feature of an SBOM is to provide the 
list of components that a software package depends on. Harbor either collects or accepts an SBOM and
then analyzes it.

Once, the dependencies are identified they can be cross-referenced with the different enrichment 
sources to identify known vulnerabilities and ideally infer some sort of risk score for the issue.
Many times the enrichment sources can even provide remediation advice.

### Lifecycle

Specifically, the lifecycle of an SBOM within Harbor consists of four phases:

- Ingestion
- Enrichment
- Continuous Monitoring
- Continuous Enrichment
