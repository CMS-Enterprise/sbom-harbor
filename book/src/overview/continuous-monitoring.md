## Continuous Monitoring

SBOMs can change over time. In a perfect world, an SBOM for a given tag/version of a component
will never change. In reality, teams don't always conform and treat a release tag as immutable.
Sometimes, a problematic release is deleted and replaced with an update that reuses the version
number. This could happen with code in a repository being directly monitored or in a dependency.

Additionally, sometimes what you want to monitor is one or more branches of a code repository
rather than a release tag. This is useful in CI/CD situation where you want to get an early signal
that a vulnerable dependency is being added, or when a new vulnerability emerges for an existing
dependency.

To support these use cases, the CLI `sbom` command can be run on a schedule to get an up to date
SBOM for each code repository being monitored. We call this batch sync process **continuous monitoring**.
Follow the code path starting in `cli/commands/sbom` for an example of how to implement a 
continuous monitoring provider.

> The CMS Harbor team uses [Fargate Tasks](https://docs.aws.amazon.com/AmazonECS/latest/userguide/fargate-task-defs.html)
> to schedule and run cli commands at predictable intervals based on organizational policies.
> Organizations that wish to run a Harbor instance will need to determine the appropriate
> synchronization strategy and tooling for their environment.

> From a file storage perspective, operators should be aware that SBOM file names are not 
> guaranteed to be unique at this time. As previously mentioned, the CMS Harbor team leverages S3 for 
> file storage and versioning. Harbor itself ships with a file system storage provider that we use 
> primarily for local development. It should be possible to use this provider to leverage attached 
> storage in a production system, though we haven't fully explored this use case. You may need to 
> modify the source and submit a PR if the current implementation doesn't fully support this approach.

