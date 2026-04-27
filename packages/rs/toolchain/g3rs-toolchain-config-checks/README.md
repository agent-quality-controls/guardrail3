# g3rs-toolchain-config-checks

Extracted toolchain config checks for guardrail3.

This package is intentionally narrower than the in-app toolchain family:

- it validates toolchain file content only
- it does not discover files
- it does not decide workspace ownership
- it does not report placement or coverage problems

Current scope:

- `g3rs-toolchain/channel-and-components`: channel and components policy
- `g3rs-toolchain/msrv-consistency`: pinned stable toolchain vs `Cargo.toml` `rust-version`

The app remains responsible for:

- file discovery
- workspace routing
- filetree rules
- upstream parse failures for files it chooses not to pass into this package
