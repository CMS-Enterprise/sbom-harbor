## Overview

Harbor is a system for:

- Collecting, categorizing, and storing SBOMs. The process of collecting SBOMs is called 
  **_ingestion_**.
- Analyzing SBOMs to identify associated vulnerability and supply chain data. The process of 
  identifying and storing vulnerability and supply chain data associated with SBOMs is called 
  **_enrichment_**.

### Compliance

Harbor was born out of the [Cyber Executive Order](https://www.whitehouse.gov/briefing-room/presidential-actions/2021/05/12/executive-order-on-improving-the-nations-cybersecurity/)
which mandates that all federal agencies must procure and catalog SBOMs for all software they 
purchase or develop. Compliance with the Executive Order is achieved through the **_ingestion_** 
and storage of SBOMs.

### Operational Security

While **_ingestion_** and **storage** (I&S) are mandatory first steps, those activities alone do 
nothing to secure an organization's software supply chain. The Executive Order implies that in 
addition to collecting and storing SBOMs, they should be analyzed, and the resulting threat 
intelligence be leveraged by operators responsible for securing systems. Harbor provides 
operators built-in **_enrichment_** providers that generate actionable risk intelligence derived 
from both free and commercial sources. The community is welcome to use the built-in providers or 
contribute additional implementations.

### Inter-Organization Collaboration

Harbor is being developed by CMS as an open source project so that it can be used by any 
organization, public or private, to improve their software supply chain security posture and to
comply with the Executive Order.
