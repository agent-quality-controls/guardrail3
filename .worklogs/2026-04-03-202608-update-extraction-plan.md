# Update extraction plan with audit findings

**Date:** 2026-04-03 20:26

## Summary
Updated family extraction plan with deep audit findings. Key changes:
cross-domain reads resolved by pre-extraction (fmt gets channel as string,
garde gets clippy bans as vec, not raw files). Workspace discovery and
profile resolution move to app (eliminates 4x duplication). parse_rust_file()
deduplicated into check-types. No rule overlap found between families.
