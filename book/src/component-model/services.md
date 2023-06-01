## Services

A **service** in the Harbor component model can be thought of as a set of behaviors (i.e.
functions) that relate to a specific part of the problem domain or subsystem. Some services 
reference others and act as coordinators across multiple areas of the domain. Services typically 
contain some sort of shared state, whether it be a set of entities or configuration data.

Examples of grouping functionality by service:

- Provide data access for a single entity or a family of entities
- Provide access to an external system
- Translate between external models and internal entities
- Coordinate activities for a task
- Wrap an instance of a resource that is expensive to create so that it can be reused across 
  loop iterations or functional boundaries
