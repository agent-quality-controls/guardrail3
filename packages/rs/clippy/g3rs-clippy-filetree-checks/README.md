# g3rs-clippy-filetree-checks

Extracted clippy filetree checks for guardrail3.

Current scope:

- `g3rs-clippy/coverage-exists`: workspace root covered by `clippy.toml` or `.clippy.toml`
- `g3rs-clippy/same-root-conflict`: same-root `clippy.toml` / `.clippy.toml` conflict

This package:

- validates root coverage and same-root shadowing only
- does not parse clippy contents
- does not discover files outside the pointed workspace root
