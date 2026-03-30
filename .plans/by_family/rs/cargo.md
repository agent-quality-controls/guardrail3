# RS-CARGO

Status: current, implemented, self-hosted.

Implementation root:

- `apps/guardrail3/crates/app/rs/families/cargo/`

Current source of truth:

- this file for family planning/status
- `apps/guardrail3/crates/app/rs/families/cargo/README.md` for family-local behavior

Current state:

- multi-root Cargo lint-policy family
- validates workspace roots and standalone package policy roots
- self-hosted with `crates/runtime`, `crates/assertions`, `crates/assertions_common`, and `test_support`
- owns Cargo/workspace lint baseline, including Clippy lint enforcement that should not be reimplemented as source scanning

Historical/supplemental references:

- `.plans/todo/checks/rs/cargo.md`
- `.plans/by_file/rs/cargo-toml.md` for upstream/file-behavior research only

Next planning focus:

- keep the README and this file aligned if lint ownership moves between cargo/clippy/code
- avoid letting old `checks/rs/cargo/**` path references drift back into active docs
