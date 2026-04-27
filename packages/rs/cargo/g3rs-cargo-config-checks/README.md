# g3rs-cargo-config-checks

Extracted Cargo config checks for guardrail3.

Current package boundary:

- one pointed crawl root
- one parsed root `Cargo.toml` that may be:
  - a workspace root
  - a standalone package root
  - another Cargo manifest shape
- zero or more parsed workspace member `Cargo.toml` files when that root is a workspace root
  - members come from normalized `[workspace].members` expansion after `[workspace].exclude`
- optional root-local `guardrail3-rs.toml` Rust policy state

Package-owned config rules:

- `g3rs-cargo/workspace-lints`
- `g3rs-cargo/lint-levels`
- `g3rs-cargo/workspace-metadata`
- `g3rs-cargo/priority-order`
- `g3rs-cargo/resolver`
- `g3rs-cargo/disallowed-macros-deny`
- `g3rs-cargo/approved-allow-inventory`
- `g3rs-cargo/workspace-lints-inherited`
- `g3rs-cargo/no-weakened-overrides`
- `g3rs-cargo/member-edition-drift`
- `g3rs-cargo/unapproved-allow-entries`
- `g3rs-cargo/member-local-allows-forbidden`
- `g3rs-cargo/rust-version-policy`

Still outside this package:

- filetree rules:
  - `g3rs-cargo/missing-member-cargo`
  - `g3rs-cargo/input-failures`
- source lane:
  - none
