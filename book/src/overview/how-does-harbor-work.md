## How does Harbor work?

Since the primary feature of an SBOM is the list of components that a software package depends 
on, Harbor works by either collecting or accepting an SBOM and then analyzing it. 

Keep in mind that an SBOM is just a file that is guaranteed to conform to a specification that is 
designed for machine-readability. As long as the file conforms to the specification, Harbor can 
parse it once it is received, and identify its dependencies. 

Once, the dependencies are identified they can be cross-referenced with the different enrichment 
sources to identify known vulnerabilities and ideally infer some sort of risk score for the issue.
Many times the enrichment sources can even provide remediation advice.

### Lifecycle

Specifically, the lifecycle of an SBOM within Harbor consists of four phases:

- Ingestion
- Enrichment
- Continuous Monitoring
- Continuous Enrichment
