# Plans: new parsers + family checks packages

**Date:** 2026-04-04 14:29

## Summary
Two new plans written:
1. cargo-config-toml + guardrail3-toml parser crates
2. Workspace-local family checks packages — split by concern not file,
   each family checks its own concerns across multiple parsed file types.
   App handles topology/coverage/discovery/parsing. Package handles
   content validation with full policy knowledge.
