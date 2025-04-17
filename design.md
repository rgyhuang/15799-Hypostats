# Component Name

## Overview

Hypostats is a PostgreSQL extension that allows for the injection of statistics between different instances of PostgreSQL. The end goal
is to mimic [HypoPG](https://github.com/HypoPG/hypopg), allowing for the testing of query optimizers on hypothetical statistics without raw data.

## Scope

> Which parts of the system will this feature rely on or modify? Write down specifics so people involved can review the design doc
> The system

## Architectural Design

> Explain the input and output of the component, describe interactions and breakdown the smaller components if any. Include diagrams if appropriate.
> Here is the current workflow:

1. Users dump pg_statistics into json format (pg_statistic_dump(starelid))
2. Create empty table corresponding to statistics
3. Users inject pg_statistics into separate PostgreSQL instance (pg_statistic_load(json)) with modified starelid
4. Query EXPLAIN

## Design Rationale

## Testing Plan

TBD

## Trade-offs and Potential Problems

Explain has the wrong number of rows

## Future Work

Integrating GUI + ML based forecasting
