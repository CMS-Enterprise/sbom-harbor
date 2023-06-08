
## Deployment

```mermaid
graph LR
    client([client])<-->API[API];
    subgraph public
        API
    end
    subgraph private
        direction BT
        subgraph db-cluster
            API<-->MongoDB
            MongoDB
        end
        subgraph task-orchestrator
            direction BT
            IngestTask
            EnrichmentTask
            IngestTask-->MongoDB[MongoDB]
            EnrichmentTask-->MongoDB[MongoDB]
        end
        subgraph storage
            direction TB
            FileStore
            API-->FileStore
            IngestTask-->FileStore
            EnrichmentTask-->FileStore
        end
    end
 classDef plain fill:#ddd,stroke:#fff,stroke-width:4px,color:#000
 classDef component fill:#326ce5,stroke:#fff,stroke-width:4px,color:#fff
 classDef cluster fill:#fff,stroke:#bbb,stroke-width:2px,color:#326ce5
 class Ingress,API,IngestTask,EnrichmentTask,MongoDB,FileStore component
 class client plain
 class public,private,db-cluster,task-orchestrator,storage cluster
```

## Overview

```mermaid
classDiagram
    Sbom --> Package
    Sbom *-- Author
    Sbom --> TaskRef
    Sbom *-- SpecKind
    Sbom --> Xref
    Package o-- Xref
    Package o-- PackageKind
    Package o-- PackageCdx
    Package --> Package
    Package --> Vulnerability
    Package --> Xref
    Package --> TaskRef
    Task o-- TaskKind
    Task o-- TaskRef
    Xref o-- XrefKind
    CvssSummary o-- CvssScore
    CvssSummary o-- Maturity
    CvssScore o-- CvssVersion
    Vulnerability o-- Remediation
    Vulnerability o-- CvssSummary
    Vulnerability o-- CvssScore
    Vulnerability o-- Cwe
    Vulnerability --> TaskRef
    SpecKind *-- CycloneDx
    SpecKind *-- Spdx
    CycloneDx *-- CycloneDxFormat
    Spdx *-- SpdxFormat

    class Package {
        +String id
        +String purl
        +PackageKind kind
        +String package_manager
        +String version
        +String cpe
        +PackageCdx cdx
        +List~Xref~ xrefs
        +List~TaskRef~ taskRefs
        +List~Package~ Dependencies
        +List~Vulnerability~ Vulnerabilities
    }

    class PackageCdx {
        +String purl
        +String name
        +String packageManager
        +String componentType
        +String bomRef
        +String version
        +List~String~ dependencies
    }

    class Sbom {
        +String id
        +Integer version
        +SpecKind kind
        +Author author
        +Integer timestamp
        +List~Package~ dependencies
        +List~Xref~ xrefs
        +List~TaskRef~ taskRefs
    }

    class Vulnerability {
        +String id
        +String purl
        +String severity
        +String cve
        +String description
        +Remediation remediation
        +List~CvssSummary~ cvss
        +List~Cwe~ cwes
        +List~TaskRef~ taskRefs

    }

    class CvssSummary {
        +Maturity Maturity
        +Decimal mean_score
        +Decimal mode_score
        +Decimal median_score
        +List~Score~ scores
    }

    class CvssScore {
        +Decimal scores
        +String source
        +String version
        +String vector
    }

    class Cwe {
        +String id
        +String name
        +String description
    }

    class Remediation {
        +String description
    }

    class Task {
        +String id
        +TaskKind kind
        +Integer timestamp
    }

    class TaskRef {
        +String task_id
        +String target_id
        +String err
    }

    class Xref {
        +XrefKind kind
        +List~String~ map
    }

    class Maturity
    <<enumeration>> Maturity

    class TaskKind
    <<enumeration>> TaskKind
    
    class PackageKind
    <<enumeration>> PackageKind

    class CycloneDx {

    }

    class Spdx {

    }

    class Author
    <<enumeration>> Author

    class SpecKind
    <<enumeration>> SpecKind

    class CycloneDxFormat
    <<enumeration>> CycloneDxFormat

    class SpdxFormat
    <<enumeration>> SpdxFormat

    class XrefKind
    <<enumeration>> XrefKind

    class CvssVersion
    <<enumeration>> CvssVersion
```


## SBOM

```mermaid
classDiagram
    Sbom --> Package
    Sbom *-- Author
    Sbom *-- SpecKind
    Package --> Package
    Package --> Vulnerability
    SpecKind *-- CycloneDx
    SpecKind *-- Spdx
    CycloneDx *-- CycloneDxFormat
    Spdx *-- SpdxFormat

    class Package {
        +String id
        +List~Package~ Dependencies
        +List~Vulnerability~ Vulnerabilities
    }

    class Sbom {
        +String id
        +Integer version
        +SpecKind kind
        +Author author
        +Integer timestamp
        +List~Package~ dependencies
        +List~Xref~ xrefs
        +List~TaskRef~ taskRefs
    }

    class Vulnerability {
        +String id
    }

    class CycloneDx {

    }

    class Spdx {

    }

    class Author
    <<enumeration>> Author

    class SpecKind
    <<enumeration>> SpecKind

    class CycloneDxFormat
    <<enumeration>> CycloneDxFormat

    class SpdxFormat
    <<enumeration>> SpdxFormat
```

## Organizations

```mermaid
classDiagram
    Organization *-- Repository
    Organization --> Product
    Vendor *-- Product

    class Organization {
        +String id
        +String name
        List~Repository~ repositories
        List~Product~ products
    }

    class Repository {
        +String id
        +String name
        *String organization_id
    }

    class Vendor {
        +String id
        +String name
        List~Product~ products
    }

    class Product {
        +String id
        +String name
        +String vendor_id
    }
```

## Packages

```mermaid
classDiagram
    Package o-- Sbom
    Package o-- PackageKind
    Package o-- PackageCdx
    Package o-- Package
    Package --> Vulnerability

    class Package {
        +String id
        +String purl
        +PackageKind kind
        +String package_manager
        +String version
        +String cpe
        +PackageCdx cdx
        +List~Xref~ xrefs
        +List~TaskRef~ taskRefs
        +List~Package~ Dependencies
        +List~Vulnerability~ Vulnerabilities
    }

    class PackageCdx {
        +String purl
        +String name
        +String packageManager
        +String componentType
        +String bomRef
        +String version
        +List~String~ dependencies
    }

    class Sbom {
        +String id
        +List~Package~ dependencies
    }
    
    class PackageKind
    <<enumeration>> PackageKind
```

## Vulnerabilities

```mermaid
classDiagram
    Sbom --> Package

    Package --> Vulnerability

    CvssSummary o-- CvssScore
    CvssSummary o-- Maturity
    CvssScore o-- CvssVersion
    Vulnerability o-- Remediation
    Vulnerability o-- CvssSummary
    Vulnerability o-- CvssScore
    Vulnerability o-- Cwe

    class Package {
        +String id
        +String cpe
        +List~Vulnerability~ Vulnerabilities
    }

    class Sbom {
        +String id
        +List~Package~ dependencies
    }

    class Vulnerability {
        +String id
        +String purl
        +String severity
        +String cve
        +String description
        +Remediation remediation
        +List~CvssSummary~ cvss
        +List~Cwe~ cwes
        +List~TaskRef~ taskRefs

    }

    class CvssSummary {
        +Maturity Maturity
        +Decimal mean_score
        +Decimal mode_score
        +Decimal median_score
        +List~Score~ scores
    }

    class CvssScore {
        +Decimal scores
        +String source
        +String version
        +String vector
    }

    class Cwe {
        +String id
        +String name
        +String description
    }

    class Remediation {
        +String description
    }

    class Maturity
    <<enumeration>> Maturity

    class CvssVersion
    <<enumeration>> CvssVersion
```

## Tasks

```mermaid
classDiagram
    Task o-- TaskKind
    Task o-- TaskRef
    Sbom --> TaskRef
    Package --> TaskRef
    Vulnerability --> TaskRef

    class Package {
        +String id
        +List~TaskRef~ taskRefs
        +List~Package~ Dependencies
        +List~Vulnerability~ Vulnerabilities
    }

    class Sbom {
        +String id
        +List~Package~ dependencies
        +List~TaskRef~ taskRefs
    }

    class Vulnerability {
        +String id
        +List~TaskRef~ taskRefs
    }

    class Task {
        +String id
        +TaskKind kind
        +Integer timestamp
    }

    class TaskRef {
        +String task_id
        +String target_id
        +String err
    }

    class TaskKind
    <<enumeration>> TaskKind
```

## Xrefs

```mermaid
classDiagram
    Sbom --> Xref
    Package --> Xref
    Xref o-- XrefKind
    Xref -- Organization
    Xref -- Repository
    Xref -- Vendor
    Xref -- Product
    Organization -- Product
    Organization o-- Repository
    Vendor o-- Product

    class Organization {
        +String id
        +String name
    }

    class Repository {
        +String id
        +String name
        +String organization_id
    }

    class Vendor {
        +String id
        +String name
    }

    class Product {
        +String id
        +String name
        +String vendor_id
    }

    class Package {
        +String id
        +String purl
        +PackageKind kind
        +String package_manager
        +String version
        +String cpe
        +PackageCdx cdx
        +List~Xref~ xrefs
        +List~TaskRef~ taskRefs
        +List~Package~ Dependencies
        +List~Vulnerability~ Vulnerabilities
    }

    class Sbom {
        +String id
        +Integer version
        +Spec spec
        +Source source
        +Integer timestamp
        +List~Package~ dependencies
        +List~Xref~ xrefs
        +List~TaskRef~ taskRefs
    }

    class Xref {
        +XrefKind kind
        +List~String~ map
    }

    class XrefKind
    <<enumeration>> XrefKind
```

## Enrichment Engine Sequence

```mermaid
sequenceDiagram
    participant cli as cli
    participant provider as Task Provider
    participant service as Service
    participant entity as Entity
    participant store as Data Store

    cli->>+provider: Executes provider
    provider->>store: Creates new task log entry
    provider->>service: Initializes service(s)
    service->>store: Requests targeted entities
    store->>service: Returns targeted entities
    loop
        service->>entity: Creates or mutates targeted entities
        service->>store: Updates data store
        service->>service: Tracks errors on task log entry
    end
    service->>store: Updates task log entry
    service->>provider: Returns result
    provider->>cli: Returns result
```

## API Sequence

```mermaid
sequenceDiagram
    participant client as client
    participant API as API
    participant controller as Controller
    participant service as Service
    participant store as Data Store

    client->>+API: Submit request
    alt POST/PUT
        API->>+API: Decodes payload
    end
    API->>+controller: Invokes controller for route
    controller->>service: Calls service method
    alt GET
        service->>store: Requests entities
        store->>service: Returns entities
        service->>controller: Returns result
    end
    alt POST/PUT
        service->>+store: Persists entities
    end
    alt DELETE
        service->>+store: Deletes entities
    end
    controller->>API: Returns status
    alt GET
        API->>API: Encodes payload
    end
    API->>client: Returns response
```

## MongoDB Overview

```mermaid
sequenceDiagram
    participant service as Service
    participant entity as Entity
    participant store as Store
    service->>store: Submit query
    store->>service: Return result
    service->>entity: Mutate entity
    service->>store: Persist entity
```

## Auth Overview

```mermaid
classDiagram
    Group "1" --> "1..*" User
    Group "1" --> "1..*" Role
    Role "1" --> "1..*" Policy
    Policy "1" --> "1" Resource
    Policy "1" --> "1" Action
    Policy "1" --> "1" Effect
    Resource "1" --> "1" ResourceKind

    class Authorizer {
        <<Service>>
        +Bool assert(User, Resource, Effect)
        -List~Group~ groups(User)
    }

    class User {
      +String id
      +String email
    }

    class Group {
      +List~User~
      +List~Role~
    }

    class Role {
      List~Policy~
    }

    class Policy {
        +Resource
        +Action
        +Effect
    }

    class Resource {
        +String id
        +ResourceKind
    }

    class ResourceKind
    <<enumeration>> ResourceKind

    class Action
    <<enumeration>> Action

    class Effect
    <<enumeration>> Effect
```
