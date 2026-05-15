# Summary

Added the Serde-first fixture output migration plan. The plan replaces adapter/exporter thinking with direct serialization of owned Rust structs through `serde::Serialize` and `serde_json`.

# Decisions Made

- Ingestion fixture output must serialize the actual owned Rust ingestion structs.
- Custom conversion is allowed only as a documented exception after `serde::Serialize` cannot be derived.
- The fixture contract remains fixture input, verifier command, received output, approved output, and diff.
- The fixture contract verifier now checks this new plan and its disposition counts.

# Key Files

- `.plans/2026-05-15-151150-serde-first-fixture-output-migration.md`
- `.plans/2026-05-15-151150-serde-first-fixture-output-migration.md.manifest.toml`
- `scripts/behavior/verify-fixture-contract-language.py`

# Verification

- `python3 scripts/behavior/verify-fixture-contract-language.py`
- `scripts/behavior/verify-all.sh`

# Next Steps

- Start with `needs_serialized_ingestion_output` rows.
- Add `serde::Serialize` derives to the owned ingestion output types before writing any fixture command output.
