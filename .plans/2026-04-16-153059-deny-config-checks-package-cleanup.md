Goal

Make `packages/rs/deny/g3rs-deny-config-checks` validate cleanly under the current rules.

Approach

- Normalize the workspace root:
  - add `rust-toolchain.toml`
  - add `rustfmt.toml`
  - add `clippy.toml`
  - add `deny.toml`
  - add `guardrail3-rs.toml`
  - make publish intent explicit at the root and in child crates
- Remove the local `crates/types` wrapper if it only forwards `g3rs-deny-types`.
  - switch the facade and runtime to `g3rs-deny-types` directly
  - delete the fake child crate if that is all it does
- Normalize release metadata:
  - explicit `publish = true` or `publish = false` in each crate
  - if the package stays publishable, add workspace-root release files
  - if it is meant to stay internal, mark it unpublished consistently
- Reshape tests to the chosen sidecar contract:
  - `rule.rs` should use `#[path = "rule_tests/mod.rs"] mod rule_tests;` with a same-line reason
  - replace old `mod tests;` declarations
  - move final proof into owned shared assertions modules
  - remove direct sidecar assertions on `CheckResult`
  - remove sibling `test_support` reach-through from sidecars
- Split oversized runtime helpers:
  - `support.rs`
  - `sources/rs_deny_config_21_unknown_keys/rule.rs`
  - move parsing/normalization into smaller modules without changing rule behavior
- Fix weak test messages:
  - replace weak `expect(...)` strings like `serialize deny`
  - remove `panic!` use if it is only test scaffolding

Key decisions

- Fix the package, not the rules, unless a real contradiction appears while reshaping the deny sidecars or runtime support.
- Use the already-clean package pattern:
  - root policy files
  - no fake local types wrapper
  - owned `x_tests` sidecars
  - per-rule shared assertions modules

Files to modify

- `packages/rs/deny/g3rs-deny-config-checks/Cargo.toml`
- `packages/rs/deny/g3rs-deny-config-checks/guardrail3-rs.toml`
- `packages/rs/deny/g3rs-deny-config-checks/src/lib.rs`
- `packages/rs/deny/g3rs-deny-config-checks/crates/assertions/**`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/**`
- `packages/rs/deny/g3rs-deny-config-checks/crates/types/**` if it is only a wrapper
