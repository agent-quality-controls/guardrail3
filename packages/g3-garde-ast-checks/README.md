# g3-garde-ast-checks

Extracted garde AST/source checks.

This package owns the garde rules that operate on governed Rust source files:

- `RS-GARDE-05`
- `RS-GARDE-07`
- `RS-GARDE-08`
- `RS-GARDE-09`
- `RS-GARDE-11`
- `RS-GARDE-12`
- `RS-GARDE-13`
- `RS-GARDE-14`

The app family still owns:

- root discovery and garde applicability gating
- root-policy config checks in `g3-garde-content-checks`
- malformed-input ownership in `RS-GARDE-10`
- deciding which source files and `guardrail3.toml` govern one garde root

The package reads and analyzes the source files itself from the explicit file
list in its input contract.
