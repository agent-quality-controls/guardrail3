# g3rs-cargo-config-checks

Extracted Cargo config checks for guardrail3.

This package is intentionally narrower than the in-app `cargo` family:

- it validates one parsed `Cargo.toml` file at a time
- it infers workspace/package applicability from the parsed file itself
- it does not discover roots, members, or cross-file relationships
- it does not report parse-failure/input-failure routing
- it does not depend on app-domain config types or normalized policy subsets

Current package boundary:

- one public input: `G3RsCargoConfigChecksInput { cargo_rel_path, cargo }`
- one call validates only what can be known from that single parsed file

The app remains responsible for:

- root and member discovery
- workspace coverage and missing-member reporting
- malformed-input fail-closed behavior
- any cross-file comparisons between workspace and members
- any policy that depends on parsed `guardrail3-rs.toml`
- `RS-CARGO-10`
- `RS-CARGO-14`

Initial single-file rules that fit this boundary:

- workspace/package local config rules such as `RS-CARGO-CONFIG-01`, `02`, `05`, `07`, `08`, `11`

Rules that do not fit this package boundary stay in the app until they are
redesigned around full parsed-file inputs instead of derived subsets:

- workspace-member relationship rules: `RS-CARGO-04`
- cross-file comparison rules: `RS-CARGO-06`, `RS-CARGO-09`
- structural rules: `RS-CARGO-10`, `RS-CARGO-14`
- policy-file-dependent rules: `RS-CARGO-03`, `RS-CARGO-12`, `RS-CARGO-13`, `RS-CARGO-15`
