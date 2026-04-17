# g3rs-release-config-checks

Release-family config checks for Rust package roots.

This package validates:

- crate publish intent and publish metadata in `Cargo.toml`
- repo release baselines such as `release-plz.toml` and `cliff.toml`
- local release dependency/version consistency across publishable crates

The facade crate exposes the typed input contract and the runtime check entrypoint.
