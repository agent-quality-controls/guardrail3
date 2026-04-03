# Plan: extract families into independent packages

**Date:** 2026-04-03 20:18

## Summary
Designed architecture for extracting each rule family into an independent
package under packages/. Families receive typed input structs (no filesystem),
return Vec<CheckResult>. Shared types in guardrail3-check-types. Topology
becomes a legality report phase. Plan at .plans/2026-04-03-201700-family-extraction.md.
