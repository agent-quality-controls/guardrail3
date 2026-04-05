# RS-DEPS

Rust dependency-policy and tool-presence family.

This family owns:

- required tool presence on PATH for Rust dependency guardrails
- `Cargo.lock` presence and `.gitignore` masking policy
- fail-closed reporting when dependency-policy inputs cannot be trusted

The extracted package [g3-deps-content-checks](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/g3-deps-content-checks) owns the pure content checks:

- crate-local `allowed_deps` enforcement from validation-root `guardrail3.toml`
- library allowlist coverage policy
- direct dependency cap enforcement across top-level and target-specific dependency tables

This family is workspace-local:

- universal dependency baseline rules are enforced per legal workspace
- allowlist rules remain local to the owning workspace instead of collapsing
  into one repo-global weakest-common-denominator policy
- direct dependency cap stays local to the owning workspace surface
- lockfile rules bind to legal workspace roots

The family consumes legal workspaces and deps-owned file surfaces from
`FamilyMapper`. It must not rediscover its own Cargo-root universe outside the
routed `RsDepsRoute`.

## Boundary notes

- `RS-DEPS-05`, `06`, `07`, `08`, and `12` run in `g3-deps-content-checks` on parsed files only.
- `RS-DEPS-11` only owns dependency-policy inputs and dependency-table shape needed by this family. Foreign `rust.*` or crate-policy keys outside deps-owned fields must not fail closed here.
- `RS-DEPS-05..07` own both top-level and `target.*` dependency tables for their respective sections.
- `RS-DEPS-12` owns the direct-dependency cap across both top-level and `target.*` dependency tables.
- `RS-DEPS-09/10` should bind to legal workspace roots rather than standalone
  package escape hatches.
- local path dependencies that resolve to a real Cargo package under a discovered workspace root must either be declared workspace packages or fail closed through `RS-DEPS-11`.
- malformed `target.*` dependency tables must surface through `RS-DEPS-11`, but they must not suppress top-level allowlist findings from `RS-DEPS-05..07`.
- routed subtree runs must keep dependency findings scoped to the owning
  workspace while preserving the family's local baseline rules.

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
        tooling/
        policy/
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
