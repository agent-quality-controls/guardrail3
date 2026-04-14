# g3rs-cargo-config-checks

Extracted Cargo config checks for guardrail3.

Current package boundary:

- one pointed workspace root
- one parsed root `Cargo.toml`
- zero or more parsed workspace member `Cargo.toml` files
- optional root-local `guardrail3-rs.toml` Rust policy state

Package-owned config rules:

- `RS-CARGO-CONFIG-01`
- `RS-CARGO-CONFIG-02`
- `RS-CARGO-CONFIG-03`
- `RS-CARGO-CONFIG-04`
- `RS-CARGO-CONFIG-05`
- `RS-CARGO-CONFIG-06`
- `RS-CARGO-CONFIG-07`
- `RS-CARGO-CONFIG-08`
- `RS-CARGO-CONFIG-09`
- `RS-CARGO-CONFIG-10`
- `RS-CARGO-CONFIG-11`
- `RS-CARGO-CONFIG-12`
- `RS-CARGO-CONFIG-13`

Still outside this package:

- filetree rules:
  - `RS-CARGO-FILETREE-10`
  - `RS-CARGO-FILETREE-14`
- source lane:
  - none
