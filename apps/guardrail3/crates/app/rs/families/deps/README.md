# RS-DEPS

Rust dependency-policy and tool-presence family.

This family owns:

- required tool presence on PATH for Rust dependency guardrails
- `Cargo.lock` presence and `.gitignore` masking policy
- fail-closed reporting when dependency-policy inputs cannot be trusted

The extracted package [g3rs-deps-config-checks](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/g3rs-deps-config-checks) owns the pure config checks:

- crate-local `allowed_deps` enforcement from workspace-root `guardrail3-rs.toml`
- library allowlist coverage policy
- direct dependency cap enforcement across top-level and target-specific dependency tables
- required deps tool presence on PATH

The extracted package [g3rs-deps-filetree-checks](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/deps/g3rs-deps-filetree-checks) owns the workspace-root filetree checks:

- `Cargo.lock` presence policy
- root `.gitignore` masking of `Cargo.lock`

This family is workspace-local:

- universal dependency baseline rules are enforced per legal workspace
- allowlist rules remain local to the owning workspace instead of collapsing
  into one repo-global weakest-common-denominator policy
- direct dependency cap stays local to the owning workspace surface
- lockfile rules bind to legal workspace roots

The package model validates one pointed workspace root. Package lanes must not
rediscover Cargo-root scope outside that pointed workspace.

## Boundary notes

- `RS-DEPS-CONFIG-01..09` run in `g3rs-deps-config-checks`.
- deps ingestion owns fail-closed handling for unreadable, malformed, or untrustworthy deps inputs.
- `RS-DEPS-CONFIG-01..03` own both top-level and `target.*` dependency tables for their respective sections.
- `RS-DEPS-CONFIG-05` owns the direct-dependency cap across both top-level and `target.*` dependency tables.
- `RS-DEPS-CONFIG-06..09` own workspace-scoped tool presence discovered from process PATH.
- `RS-DEPS-FILETREE-09/10` bind to the pointed workspace root rather than repo-global placement.
- local path dependencies that resolve to a real Cargo package under the pointed workspace root must either be declared workspace packages or fail closed in deps ingestion.
- malformed `target.*` dependency tables must fail closed in deps ingestion, but they must not suppress top-level allowlist findings from `RS-DEPS-CONFIG-01..05`.

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
