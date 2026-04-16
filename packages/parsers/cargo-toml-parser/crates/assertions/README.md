# cargo-toml-parser-runtime-assertions

Shared proof helpers for `cargo-toml-parser-runtime` tests.

This crate is intentionally internal and unpublished. Runtime sidecar tests use
these helpers so the final result checks live in one shared proof surface
instead of being duplicated across test files.
