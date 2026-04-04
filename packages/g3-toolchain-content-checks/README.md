# g3-toolchain-content-checks

Extracted toolchain content checks for guardrail3.

This package is intentionally narrower than the in-app toolchain family:

- it validates toolchain file content only
- it does not discover files
- it does not decide workspace ownership
- it does not report placement or coverage problems

Current scope:

- `RS-TOOLCHAIN-02`: channel and components policy
- `RS-TOOLCHAIN-03`: pinned stable toolchain vs `Cargo.toml` `rust-version`

The app remains responsible for:

- file discovery
- workspace routing
- placement / shadowing / coverage rules
- upstream parse failures for files it chooses not to pass into this package
