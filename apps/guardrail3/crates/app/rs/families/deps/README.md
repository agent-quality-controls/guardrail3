# RS-DEPS

Rust dependency-policy and tool-presence family.

This family owns:

- required tool presence on PATH for Rust dependency guardrails
- crate-local `allowed_deps` enforcement from validation-root `guardrail3.toml`
- library allowlist coverage policy
- direct dependency cap enforcement across top-level and target-specific dependency tables
- `Cargo.lock` presence and `.gitignore` masking policy
- fail-closed reporting when dependency-policy inputs cannot be trusted

This family is mixed-scope:

- tool rules are validation-root scoped
- allowlist rules are crate-local
- direct dependency cap is crate-local but spans multiple dependency-table surfaces
- lockfile rules are Rust-root scoped

The family consumes routed Rust roots from `FamilyMapper`. It must not rediscover its own Cargo-root universe outside the routed `RsDepsRoute`.

## Workspace shape

```text
families/deps/
  Cargo.toml
  README.md
  crates/
    runtime/
      Cargo.toml
      src/
        lib.rs
        facts.rs
        inputs.rs
        rs_deps_*.rs
        rs_deps_*_tests/
    assertions/
      Cargo.toml
      src/
        lib.rs
        common.rs
        rs_deps_*.rs
  test_support/
    Cargo.toml
    src/
      lib.rs
```

## Test boundaries

- `runtime` owns rule execution, family-specific fact/input harness helpers, and rule sidecars.
- `assertions` owns reusable result-selection and result-assertion helpers per rule.
- `test_support` owns only generic tree/tool test helpers and must stay free of `FamilyMapper` or rule-semantic result inspection.
