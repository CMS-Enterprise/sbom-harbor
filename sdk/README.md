## Overview

This directory contains resources for organizations and contributors that want to contribute to 
or extend Harbor, or who want to run a Harbor instance for themselves. It contains a number of 
tools which are explained in the [documentation](). 

The goal of these tools is to make contributing to or extending Harbor simple and intuitive. 
To that end, the tooling is designed to help contributors conform to Harbor's core conventions. If 
you are writing something you hope to contribute upstream to Harbor, we ask that you conform to the 
conventions as explained in the documentation. 

If you are writing a custom extension that isn't designed for general adoption, you are of course 
free to adopt any conventions you like. However, these tools may still be helpful as a way of 
understanding Harbor's internals and default architecture.

### Core vs. Custom Extension Tasks

When implementing a `TaskProvider` it is crucial to keep in mind whether you are building something 
for general use by the community or something that is bespoke to your Harbor operating environment. 
The following matrix should help in determining whether a new `TaskProvider` is a custom 
extension or a candidate for inclusion in Harbor itself.

| Factor                                                                         	 | Custom Extension 	| Candidate for General Use 	| Examples                                                                                	           |
|----------------------------------------------------------------------------------|------------------	|---------------------------	|-----------------------------------------------------------------------------------------------------|
| Relies on a data source or API specific to my environment                      	 |        X         	|                           	| * CSV export from your own database <br>* API calls to services you wrote                   	       |
| Relies on custom configuration of a third-party service or project             	 |        X         	|                           	| * CMS FISMA ID extension (relies on Snyk tag configuration specific to CMS)                  	      |
| Uses a public data source or the standard API of a third-party service/project 	 |                  	|             X             	| * EPSS Provider (uses a public API) <br/>* Snyk sbom/enrich providers (use the standard Snyk API) 	 |


