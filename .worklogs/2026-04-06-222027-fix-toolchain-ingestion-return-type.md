# Fix toolchain ingestion to return checks input directly

**Date:** 2026-04-06 22:20
**Scope:** g3rs-toolchain-config-ingestion

## Summary
Toolchain ingestion was returning a custom `G3RsToolchainConfigIngestionResult` instead of `G3RsToolchainConfigChecksInput` directly. Every other ingestion package returns the checks input type. Fixed by deleting the result type, updating the runtime to depend on the checks facade, and returning the checks input directly. Also fixed cross-boundary dependency on internal crawl types crate.
