# g3rs-toolchain-filetree-checks

Extracted toolchain filetree checks for guardrail3.

Current scope:

- `g3rs-toolchain/exists`: `rust-toolchain.toml` exists
- `g3rs-toolchain/legacy-file`: legacy `rust-toolchain` file is warned or rejected

This package:

- validates root file presence and file conflict only
- does not parse toolchain contents
- does not discover files outside the pointed workspace root
