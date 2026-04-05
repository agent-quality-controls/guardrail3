# g3-cargo-content-checks

Extracted Cargo content checks for guardrail3.

This package is intentionally narrower than the in-app `cargo` family:

- it validates parsed manifest content only
- it does not discover owned roots or members
- it does not report parse-failure/input-failure routing
- it does not depend on app-domain config types

Current package boundary:

- `RS-CARGO-01..09`
- `RS-CARGO-11..13`
- `RS-CARGO-15`

The app remains responsible for:

- root and member discovery
- workspace coverage and missing-member reporting
- malformed-input fail-closed behavior
- translating root-local cargo policy config into package-local profile/waiver inputs
- `RS-CARGO-10`
- `RS-CARGO-14`
