## What is an SBOM?

SBOM is an acronym that stands for _Software Bill of Materials_. One way to think of an SBOM is 
as a parts list for your car. A car isn't naturally a car. It's a combination of components such as 
the wheels, chassis, steering-wheel, airbags, etc.

Similarly, in this day and age, most if not all software uses external, often open source, 
components written by other, usually unrelated people, teams or companies. So, similar to how a 
car is made up of components, a piece of software, or a `Package` in Harbor terminology, is made 
up of other components, and just like a car, a component, might have subcomponents.

When one software component depends on another we typically refer that relationship as a `Dependency`.
Both components are ultimately a `Package`, but Harbor uses the terms `Primary` and `Dependency` 
to provides context about the relationship between two `Packages`.

Taking all that into context, we can update our definition of an SBOM to the following:

> An SBOM is a machine-readable document that:
> - Describes a software package
> - Provides a unique identifier for the package, similar to a part number in manufacturing
> - Provides information about the supplier
> - Lists the dependencies that make up the package

## Specifications

That expanded definition includes the requirement that an SBOM be defined in a machine-readable 
format. At this time, there are two dominant SBOM specifications for the format of an SBOM which 
are:

- [CycloneDX](https://cyclonedx.org/) backed by the [OWASP Foundation](https://owasp.org/).
- [SPDX](https://spdx.dev/) backed by the [Linux Foundation](https://www.linuxfoundation.org/).

At this time, Harbor is focused on processing files in the CycloneDX format. This is not an 
endorsement of one specification over the other, as much as a pragmatic decision based on 
availability of data. We do have SPDX support in our backlog, and ultimately intend to support 
both formats. Community PRs welcome!

### Example

Here is an example of a very minimal SBOM in CycloneDX JSON format.

```json
{
    "$schema": "http://cyclonedx.org/schema/bom-1.4.schema.json",
    "bomFormat": "CycloneDX",
    "specVersion": "1.4",
    "version": 1,
    "metadata": {
        "timestamp": "2023-06-06T11:29:16Z",
        "component": {
            "bom-ref": "example-app@1.2.3",
            "type": "application",
            "name": "project",
            "purl": "pkg:nuget/exampl-app@1.2.3"
        }
    },
    "components": [
        {
            "bom-ref": "example-library@2.3.4",
            "type": "library",
            "name": "stateless-4.0",
            "version": "2.3.1.1",
            "purl": "pkg:nuget/example-library@2.3.4"
        }
    ],
    "dependencies": [
        {
            "ref": "example-app@1.2.3",
            "dependsOn": [
                "example-library@2.3.4"
            ]
        },
        {
            "ref": "example-library@2.3.4",
            "dependsOn": []
        }
    ]
}
```

