## Use Cases

At this time, Harbor is targeting two primary use cases:

- Ingesting and managing SBOMs provided by SaaS/COTS vendors
- SBOMs related to custom software development initiatives

### Current State

After some early experimentation and discovery relative to the CMS operating environment, we
determined that the fastest way to create a pipeline that facilitated building out multiple facets
of the system was to focus on Snyk as both an SBOM and enrichment provider. Snyk has been widely 
adopted across development teams at CMS. Once a team has been onboarded to Snyk, it can operate as 
both an SBOM provider and a source of vulnerability data.

### Future State

This isn't the end of the story, however. Not all development teams at CMS use Snyk. Additionally, 
since Harbor is being built specifically with the goal of being usable by any agency or organization, 
there is no guarantee that the Snyk integration is an option. Therefore, other SBOM providers 
(e.g. GitHub) and enrichment sources are being developed. Contributors are encouraged to submit PRs
for any custom providers they write and wish to contribute to the community.
