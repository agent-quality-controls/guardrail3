# Goal

Make every Rust workspace root in this repository pass:

```bash
g3rs validate --path <workspace>
```

The repo root must remain invalid for G3RS unless a root `Cargo.toml` is added. Validation is workspace-scoped.

# Source Run

Command used to discover failures:

```bash
g3rs validate --path <workspace>
```

Workspace roots checked: 156.

Current result: 125 pass, 31 fail.

Failure logs live under:

```text
.tmp/g3rs-all-workspaces/
```

# Global Fix Rules

- Do not weaken G3RS rules to make packages pass.
- Fix each package at the failing package boundary.
- If `lib.rs` owns `#[path = "lib_tests/mod.rs"]`, move those tests to the implementation file they exercise.
- `lib.rs` must remain a facade: module declarations and feature-gated re-exports only.
- If a package has config drift, copy the managed config shape from a passing sibling package of the same kind.
- If a rule reports too many effective code lines, split the assertion/helper file by semantic responsibility, not by arbitrary line count.
- After fixing each workspace root, run `g3rs validate --path <workspace>`.
- After a workspace passes, send an adversarial reviewer to compare the package against this plan and the actual G3RS output.

# Failed Workspace Plans

## 1. `packages/parsers/cargo-toml-parser`

Failure:

```text
g3rs-code/too-many-effective-code-lines crates/assertions/src/parser.rs
```

Plan:

- Read `crates/assertions/src/parser.rs`.
- Split assertion helpers into semantic modules under `crates/assertions/src/parser/` or sibling modules with facade exports from `parser.rs`.
- Keep public assertion API stable unless callers can be updated mechanically.
- Verify `g3rs validate --path packages/parsers/cargo-toml-parser`.

## 2. `packages/parsers/rustfmt-toml-parser`

Failure:

```text
g3rs-clippy/max-struct-bools clippy.toml max-struct-bools wrong value
```

Plan:

- Compare `clippy.toml` to a passing parser package, for example `packages/parsers/clippy-toml-parser/clippy.toml`.
- Set `max-struct-bools` to the managed value required by G3RS.
- Verify `g3rs validate --path packages/parsers/rustfmt-toml-parser`.

## 3-12, 17-20. Rust hook-contract packages and one release config package

Affected workspaces:

```text
packages/rs/apparch/g3rs-apparch-hook-contract
packages/rs/arch/g3rs-arch-hook-contract
packages/rs/cargo/g3rs-cargo-hook-contract
packages/rs/clippy/g3rs-clippy-hook-contract
packages/rs/code/g3rs-code-hook-contract
packages/rs/deny/g3rs-deny-hook-contract
packages/rs/deps/g3rs-deps-hook-contract
packages/rs/fmt/g3rs-fmt-hook-contract
packages/rs/garde/g3rs-garde-hook-contract
packages/rs/release/g3rs-release-config-checks
packages/rs/release/g3rs-release-hook-contract
packages/rs/test/g3rs-test-hook-contract
packages/rs/toolchain/g3rs-toolchain-hook-contract
packages/rs/topology/g3rs-topology-hook-contract
```

Failure:

```text
g3rs-arch/no-path-attr crates/runtime/src/lib.rs
g3rs-test/owned-sidecar-shape crates/runtime/src/lib.rs
```

Plan:

- For each package, read `crates/runtime/src/lib.rs` and the `lib_tests` directory.
- If tests exercise `contract.rs`, move `lib_tests` to `contract_tests` and attach it from `contract.rs`:

```rust
#[cfg(test)]
#[path = "contract_tests/mod.rs"] // reason: owned sidecar tests for contract module.
mod contract_tests;
```

- If tests exercise `run.rs`, move `lib_tests` to `run_tests` and attach it from `run.rs`:

```rust
#[cfg(test)]
#[path = "run_tests/mod.rs"] // reason: owned sidecar tests for run module.
mod run_tests;
```

- Remove all `#[cfg(test)]` and `#[path = "lib_tests/mod.rs"]` declarations from `lib.rs`.
- Verify each workspace individually with `g3rs validate --path <workspace>`.

## 13. `packages/rs/code/g3rs-code-ingestion`

Failure:

```text
g3rs-code/too-many-effective-code-lines crates/assertions/src/run.rs
```

Plan:

- Read `crates/assertions/src/run.rs`.
- Split assertion helpers by semantic assertion target.
- Keep `run.rs` as a facade if callers import from that module.
- Verify `g3rs validate --path packages/rs/code/g3rs-code-ingestion`.

## 14. `packages/rs/hooks/g3rs-hooks-config-checks`

Failures:

```text
g3rs-test/runtime-assertions-split .../rule_tests/golden.rs sidecar escapes owned module boundary
g3rs-test/runtime-assertions-split .../rule_tests/mod.rs sidecar missing owned assertions module
g3rs-test/assertions-modules-prove .../rule_tests/golden.rs sidecar owns semantic result assertion
g3rs-test/real-proof-site .../rule_tests/golden.rs test checks results through local path
```

Plan:

- Read the rule modules under `crates/runtime/src/`.
- Move semantic result assertions out of `rule_tests/golden.rs` into the package assertion crate if one exists.
- If no assertion crate helper exists, add one in `crates/assertions` and call it from tests.
- Ensure each rule owns its tests under a sidecar directory directly attached to the rule file.
- Verify `g3rs validate --path packages/rs/hooks/g3rs-hooks-config-checks`.

## 15. `packages/rs/hooks/g3rs-hooks-contract-types`

Failures:

```text
g3rs-clippy/* missing or wrong managed clippy settings
g3rs-deny/* missing managed deny policy
```

Plan:

- Copy managed `clippy.toml` and `deny.toml` policy shape from a passing pure types package, preferably `packages/rs/hooks/g3rs-hooks-types` or another passing `*-types` package.
- Keep package-specific dependency exceptions only if already required by the package.
- Verify `g3rs validate --path packages/rs/hooks/g3rs-hooks-contract-types`.

## 16. `packages/rs/hooks/g3rs-hooks-source-checks`

Failures:

```text
g3rs-code/path-attr-with-reason crates/runtime/src/run.rs
g3rs-arch/mod-facade-only crates/runtime/src/run_tests/mod.rs
g3rs-arch/no-path-attr crates/runtime/src/run.rs
g3rs-test/owned-sidecar-shape crates/runtime/src/run.rs
g3rs-test/runtime-assertions-split multiple rule_tests
g3rs-test/assertions-modules-prove multiple rule_tests
g3rs-test/real-proof-site multiple rule_tests
```

Plan:

- Read `crates/runtime/src/run.rs`, `run_tests/mod.rs`, and each `rule_tests` directory.
- Remove `#[path]` test attachment from `run.rs` unless `run.rs` owns a valid `run_tests` sidecar and the sidecar shape is accepted by G3RS.
- Split non-facade code out of `run_tests/mod.rs`; it must contain only module declarations.
- Move semantic result assertions into `crates/assertions`.
- Attach each rule test directory from the rule file that owns it.
- Verify `g3rs validate --path packages/rs/hooks/g3rs-hooks-source-checks`.

## 21. `packages/ts/astro/content/g3ts-astro-content-config-checks`

Failure:

```text
g3rs-arch/no-path-attr crates/runtime/src/lib.rs
g3rs-test/owned-sidecar-shape crates/runtime/src/lib.rs
```

Plan:

- Move `lib_tests` to `run_tests` if tests exercise `run::check`.
- Attach tests from `run.rs`.
- Remove test attachment from `lib.rs`.
- Verify `g3rs validate --path packages/ts/astro/content/g3ts-astro-content-config-checks`.

## 22. `packages/ts/astro/content/g3ts-astro-content-file-tree-checks`

Same plan as workspace 21.

## 23. `packages/ts/astro/content/g3ts-astro-content-types`

Failure:

```text
g3rs-code/public-struct-named-fields src/types.rs
```

Plan:

- Read `src/types.rs`.
- Convert public named-field structs into opaque structs with private fields and constructors/accessors, or into enum/newtype shapes already used by passing `g3ts-*types` packages.
- Update callers mechanically.
- Verify `g3rs validate --path packages/ts/astro/content/g3ts-astro-content-types`.

## 24. `packages/ts/astro/i18n/g3ts-astro-i18n-config-checks`

Same plan as workspace 21.

## 25. `packages/ts/astro/mdx/g3ts-astro-mdx-config-checks`

Same plan as workspace 21.

## 26. `packages/ts/astro/media/g3ts-astro-media-config-checks`

Same plan as workspace 21.

## 27. `packages/ts/astro/seo/g3ts-astro-seo-config-checks`

Same plan as workspace 21.

## 28. `packages/ts/astro/seo/g3ts-astro-seo-types`

Failure:

```text
g3rs-code/public-struct-named-fields src/types.rs
```

Plan:

- Read `src/types.rs`.
- Convert public named-field structs into opaque public types or enum/newtype shapes.
- Update callers mechanically.
- Verify `g3rs validate --path packages/ts/astro/seo/g3ts-astro-seo-types`.

## 29. `packages/ts/astro/setup/g3ts-astro-setup-config-checks`

Same plan as workspace 21.

## 30. `packages/ts/astro/setup/g3ts-astro-setup-file-tree-checks`

Same plan as workspace 21.

## 31. `packages/ts/astro/state/g3ts-astro-state-file-tree-checks`

Same plan as workspace 21.

# Execution Order

1. Fix the simple `lib.rs` test ownership packages first because they share one mechanical pattern.
2. Fix clippy/deny config drift packages.
3. Fix too-large assertion modules.
4. Fix hooks source/config test ownership and assertion split.
5. Fix public named-field type packages.
6. Re-run all 156 workspaces.
