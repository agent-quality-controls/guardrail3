# g3rs-clippy-filetree-checks

Extracted clippy filetree checks for guardrail3.

Current scope:

- `RS-CLIPPY-FILETREE-01`: workspace root covered by `clippy.toml` or `.clippy.toml`
- `RS-CLIPPY-FILETREE-02`: same-root `clippy.toml` / `.clippy.toml` conflict

This package:

- validates root coverage and same-root shadowing only
- does not parse clippy contents
- does not discover files outside the pointed workspace root
