# g3rs-garde-source-checks

Extracted garde source checks.

This package owns the garde rules that operate on governed Rust source files:

- `RS-GARDE-SOURCE-01`
- `RS-GARDE-SOURCE-02`
- `RS-GARDE-SOURCE-03`
- `RS-GARDE-SOURCE-04`
- `RS-GARDE-SOURCE-05`
- `RS-GARDE-SOURCE-06`
- `RS-GARDE-SOURCE-07`
- `RS-GARDE-SOURCE-08`

The app family still owns:

- root discovery and garde applicability gating
- root-policy config checks in `g3rs-garde-config-checks`
- malformed-input ownership in `RS-GARDE-SOURCE-10`
- deciding which source files and `guardrail3.toml` govern one garde root

The package reads and analyzes the source files itself from the explicit file
list in its input contract.
