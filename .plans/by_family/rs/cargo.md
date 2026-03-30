# RS-CARGO

Status: current, implemented, self-hosted.

Implementation root:

- `apps/guardrail3/crates/app/rs/families/cargo/`

Current source of truth:

- this file for family planning/status
- `apps/guardrail3/crates/app/rs/families/cargo/README.md` for family-local behavior

Current state:

- multi-root Cargo lint-policy family
- validates legal workspace roots and their member/package surfaces
- self-hosted with `crates/runtime`, `crates/assertions`, `crates/assertions_common`, and `test_support`
- owns Cargo/workspace lint baseline, including Clippy lint enforcement that should not be reimplemented as source scanning

Scope model:

- workspace-local family
- it should receive all legal workspaces plus Cargo-family files relevant to
  those workspaces, not standalone package policy roots

Agent handoff focus:

- audit production path first:
  - `apps/guardrail3/crates/app/rs/runtime/src/runners.rs`
  - `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/lib.rs`
  - `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/discover.rs`
  - `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs`
- prove legal workspace roots and member coverage all come from shared routing
- add subtree tests that show sibling workspaces do not bleed in and that
  misplaced Cargo-family surfaces stay visible

Known current risk:

- no confirmed production routing bug, but subtree behavior is under-tested
- easy false-green shape: family still looks right on full-repo runs while
  overreaching on nested-path runs

Done means:

- nested-path runtime tests prove only the owning routed policy roots are active
- malformed routed manifests still fail closed
- no family-local root discovery escapes the route

Historical/supplemental references:

- `.plans/todo/checks/rs/cargo.md`
- `.plans/by_file/rs/cargo-toml.md` for upstream/file-behavior research only

Next planning focus:

- keep the README and this file aligned if lint ownership moves between cargo/clippy/code
- avoid letting old `checks/rs/cargo/**` path references drift back into active docs
