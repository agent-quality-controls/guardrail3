# Rust Native Surface Audit

## Coverage

- Audited implemented runtime rules from `packages/rs/*/*/crates/runtime/src`.
- Implemented rule count: `249`
- Family counts:
  - `APPARCH`: `10`
  - `ARCH`: `11`
  - `CARGO`: `15`
  - `CLIPPY`: `23`
  - `CODE`: `30`
  - `DENY`: `29`
  - `DEPS`: `11`
  - `FMT`: `8`
  - `GARDE`: `13`
  - `HOOKS`: `38`
  - `RELEASE`: `33`
  - `TEST`: `19`
  - `TOOLCHAIN`: `4`
  - `TOPOLOGY`: `5`

## Dylint substrate to add before any migration

- Exact target surface:
  - use Dylint libraries, not "Clippy plugins"
  - keep Clippy for built-in lints and config only
- Root `Cargo.toml`:

```toml
[workspace.metadata.dylint]
libraries = [{ path = "packages/rs/dylints", pattern = "*" }]

[workspace.lints.rust.unexpected_cfgs]
level = "warn"
check-cfg = ["cfg(dylint_lib, values(any()))"]
```

- Dylint library crate layout:
  - `packages/rs/dylints/g3rs-code-dylints`
  - `packages/rs/dylints/g3rs-garde-dylints`
  - `packages/rs/dylints/g3rs-test-dylints`
  - `packages/rs/dylints/g3rs-arch-dylints`
  - `packages/rs/dylints/g3rs-apparch-dylints`
- One lint module per migrated rule inside the owning Dylint crate.
- Hook or CI command:
  - `cargo dylint --all -- --all-targets`
- Existing cargo rule that already supports the workspace-lints side of this:
  - `g3rs-cargo/workspace-lints-inherited`

## Finding 1 - delete exact duplicate

| Rule | Current file | Current behavior | Action | Exact replacement |
|---|---|---|---|---|
| `g3rs-code/unsafe-code-lint` | `packages/rs/code/g3rs-code-config-checks/crates/runtime/src/rs_code_config_12_unsafe_code_lint/rule.rs` | checks manifest lint policy for `unsafe_code` | delete rule | `g3rs-cargo/workspace-lints`, `g3rs-cargo/lint-levels`, `g3rs-cargo/no-weakened-overrides` already own the manifest lint baseline, lint level, and no-weakened-override checks |

## Finding 2 - only remove `todo!` / `unimplemented!` after stronger Clippy execution proof exists

| Rule | Current file | Current behavior | Action | Exact replacement |
|---|---|---|---|---|
| `g3rs-code/ast-13-todo-macros` | `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_13_todo_macros/rule.rs` | flags `todo!`, `unimplemented!`, and inventories `unreachable!` | do not delete now; if hook coverage is strengthened, narrow this rule to `unreachable!` only | `g3rs-cargo/workspace-lints` and `g3rs-cargo/lint-levels` already require `todo` and `unimplemented` to be denied; source-lane replacement is only safe after hook coverage proves the lint actually runs across the intended targets |
| `g3rs-code/ast-16-panic-macro` | `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_16_panic_macro/rule.rs` | flags `panic!` in non-test code | do not delete now; delete only after hook coverage is strengthened | `g3rs-cargo/workspace-lints` and `g3rs-cargo/lint-levels` already require `clippy::panic = "deny"`; source-lane replacement is only safe after hook coverage proves the lint actually runs across the intended targets |

- Current hook proof is not strong enough:
  - `g3rs-hooks/hook-rs-02-clippy-step-present` proves some `cargo clippy` step exists
  - `g3rs-hooks/hook-rs-09-clippy-denies-warnings` proves one invocation denies warnings
  - neither proves full target or feature coverage
- Exact precondition to remove the source rules:
  - tighten `g3rs-hooks/hook-rs-09-clippy-denies-warnings` so the active hook must run `cargo clippy --workspace --all-targets --all-features -- -D warnings`, or an equivalent command shape that proves the same coverage

## Finding 3 - move pure source rules to Dylint

| Rule | Current behavior | Exact Dylint target | Exact action |
|---|---|---|---|
| `g3rs-code/ast-01-crate-level-allow` | crate-level or inline-module `#![allow(...)]`, except approved `unused_crate_dependencies` | `g3_code_no_crate_level_allow` in `g3rs-code-dylints` | move the detection to Dylint; keep `g3rs-code/ast-02-unused-crate-dependencies-allow` as the approved-exemption inventory |
| `g3rs-code/ast-09-too-many-effective-code-lines` | non-test file exceeds 500 effective code-bearing lines | `g3_code_effective_line_cap` in `g3rs-code-dylints` | move the source counting rule to Dylint; delete `g3rs-code/ast-09-too-many-effective-code-lines` after parity tests |
| `g3rs-code/ast-10-too-many-use-imports` | non-test file has more than 20 top-level `use` imports | `g3_code_use_import_cap` in `g3rs-code-dylints` | move the hard cap to Dylint; keep warn threshold in the paired lint below or emit warn/error from one lint crate |
| `g3rs-code/ast-11-many-use-imports` | non-test file has 16-20 top-level `use` imports | `g3_code_use_import_cap` in `g3rs-code-dylints` | move the warn threshold to the same Dylint rule as `g3rs-code/ast-10-too-many-use-imports`; delete `g3rs-code/ast-11-many-use-imports` after parity tests |
| `g3rs-code/ast-15-direct-fs-usage` | direct `std::fs` imports and direct `std::fs::*` calls outside filesystem-boundary modules | `g3_code_no_direct_std_fs` in `g3rs-code-dylints` | move to Dylint with exact boundary-module exemptions; delete `g3rs-code/ast-15-direct-fs-usage` after parity tests |
| `g3rs-code/ast-17-impl-allow-blast-radius` | blanket `#[allow]` or `#[expect]` on impl blocks | `g3_code_no_impl_level_suppression` in `g3rs-code-dylints` | move to Dylint; delete `g3rs-code/ast-17-impl-allow-blast-radius` after parity tests |
| `g3rs-code/ast-18-always-true-cfg-attr-bypass` | `cfg_attr` suppression that is effectively unconditional | `g3_code_no_always_true_cfg_attr_suppression` in `g3rs-code-dylints` | move to Dylint; delete `g3rs-code/ast-18-always-true-cfg-attr-bypass` after parity tests |
| `g3rs-code/ast-19-large-type-inventory` | large struct or enum inventory | `g3_code_large_type_inventory` in `g3rs-code-dylints` | move the size inventory to Dylint; delete `g3rs-code/ast-19-large-type-inventory` after parity tests |
| `g3rs-code/ast-20-extern-allow` | `#[allow]` or `#[expect]` on `extern` blocks | `g3_code_no_extern_block_suppression` in `g3rs-code-dylints` | move to Dylint; delete `g3rs-code/ast-20-extern-allow` after parity tests |
| `g3rs-code/ast-21-fs-glob-import` | `use std::fs::*` glob import bypass, including std alias forms | `g3_code_no_std_fs_glob_import` in `g3rs-code-dylints` | move to Dylint; delete `g3rs-code/ast-21-fs-glob-import` after parity tests |
| `g3rs-code/ast-23-include-bypass` | `include!()` bypass and traversal-bearing include paths | `g3_code_no_include_bypass` in `g3rs-code-dylints` | move to Dylint; delete `g3rs-code/ast-23-include-bypass` after parity tests |
| `g3rs-code/ast-29-large-trait-surface` | traits above warn and error method-count caps | `g3_code_trait_surface_cap` in `g3rs-code-dylints` | move to Dylint; delete `g3rs-code/ast-29-large-trait-surface` after parity tests |
| `g3rs-code/ast-33-public-weak-error-forms` | public `Result<_, String | &str | anyhow::Error | Box<dyn Error>>` | `g3_code_typed_public_error` in `g3rs-code-dylints` | move to Dylint; delete `g3rs-code/ast-33-public-weak-error-forms` after parity tests |
| `g3rs-code/ast-34-generic-parameter-cap` | more than 6 type or const generic parameters | `g3_code_generic_parameter_cap` in `g3rs-code-dylints` | move to Dylint; delete `g3rs-code/ast-34-generic-parameter-cap` after parity tests |
| `g3rs-code/ast-36-string-dispatch-cap` | string dispatch site above 10 literal branches | `g3_code_string_dispatch_cap` in `g3rs-code-dylints` | move to Dylint; delete `g3rs-code/ast-36-string-dispatch-cap` after parity tests |
| `g3rs-garde/ast-01-struct-derive-validate` | boundary structs deserialize without `Validate` | `g3_garde_structs_require_validate` in `g3rs-garde-dylints` | move to Dylint; delete `g3rs-garde/ast-01-struct-derive-validate` after parity tests |
| `g3rs-garde/ast-02-manual-deserialize-impl` | manual `Deserialize` impl bypassing derive-based validation | `g3_garde_no_manual_deserialize_bypass` in `g3rs-garde-dylints` | move to Dylint; delete `g3rs-garde/ast-02-manual-deserialize-impl` after parity tests |
| `g3rs-garde/ast-03-enum-derive-validate` | boundary enums deserialize without `Validate` | `g3_garde_enums_require_validate` in `g3rs-garde-dylints` | move to Dylint; delete `g3rs-garde/ast-03-enum-derive-validate` after parity tests |
| `g3rs-garde/ast-04-query-as-inventory` | `sqlx::query_as!` and `query_as_unchecked!` inventory | `g3_garde_query_as_review` in `g3rs-garde-dylints` | move to Dylint; delete `g3rs-garde/ast-04-query-as-inventory` after parity tests |
| `g3rs-garde/ast-05-field-level-constraints` | validated fields missing field-level garde validators | `g3_garde_field_requires_validator` in `g3rs-garde-dylints` | move to Dylint; delete `g3rs-garde/ast-05-field-level-constraints` after parity tests |
| `g3rs-garde/ast-06-nested-validation-dive` | nested validated fields missing `#[garde(dive)]` | `g3_garde_nested_requires_dive` in `g3rs-garde-dylints` | move to Dylint; delete `g3rs-garde/ast-06-nested-validation-dive` after parity tests |
| `g3rs-garde/ast-07-context-validation-surface` | ctx-using validators without `#[garde(context(...))]` | `g3_garde_context_surface_complete` in `g3rs-garde-dylints` | move to Dylint; delete `g3rs-garde/ast-07-context-validation-surface` after parity tests |
| `g3rs-test/inline-test-bodies` | inline `#[cfg(test)] mod ... { ... }` bodies | `g3_test_no_inline_cfg_test_bodies` in `g3rs-test-dylints` | move to Dylint; delete `g3rs-test/inline-test-bodies` after parity tests |
| `g3rs-test/should-panic-expected` | `#[should_panic]` without `expected = "..."` | `g3_test_should_panic_requires_expected` in `g3rs-test-dylints` | move to Dylint; delete `g3rs-test/should-panic-expected` after parity tests |
| `g3rs-test/tautological-assertions` | tautological literal-vs-literal assertions | `g3_test_no_tautological_assertions` in `g3rs-test-dylints` | move to Dylint; delete `g3rs-test/tautological-assertions` after parity tests |
| `g3rs-test/weak-matches-assert` | weak `matches!` assertions with `_` payload wildcards | `g3_test_no_weak_matches` in `g3rs-test-dylints` | move to Dylint; delete `g3rs-test/weak-matches-assert` after parity tests |
| `g3rs-arch/no-path-attr` | `#[path = ...]` facade bypass except owned sidecar shape | `g3_arch_no_path_attr` in `g3rs-arch-dylints` | move to Dylint; delete `g3rs-arch/no-path-attr` after parity tests |
| `g3rs-apparch/io-traits-in-types` | public traits defined in io crates | `g3_apparch_no_io_public_traits` in `g3rs-apparch-dylints` | move to Dylint; delete `g3rs-apparch/io-traits-in-types` after parity tests |
| `g3rs-apparch/types-public-surface` | public behavior exposed from `types` crates | `g3_apparch_types_surface_passive_only` in `g3rs-apparch-dylints` | move to Dylint; delete `g3rs-apparch/types-public-surface` after parity tests |

## Finding 4 - move detection to Dylint but keep guardrail3 for repo-specific reason or waiver policy

| Rule | Current behavior | Exact Dylint target | Keep in guardrail3 |
|---|---|---|---|
| `g3rs-code/ast-03-item-level-allow-without-reason` | item-level `#[allow]` or `#[expect]` without useful same-line `// reason:` | `g3_code_item_level_suppression` in `g3rs-code-dylints` | same-line `// reason:` parsing and strength policy |
| `g3rs-code/ast-04-item-level-allow-with-reason` | documented `#[allow]` and `#[expect]` inventory | `g3_code_item_level_suppression` in `g3rs-code-dylints` | documented-reason inventory |
| `g3rs-code/ast-05-garde-skip-without-comment` | `#[garde(skip)]` without comment | `g3_code_garde_skip_requires_reason` in `g3rs-code-dylints` | exact comment placement policy |
| `g3rs-code/ast-06-garde-skip-with-comment` | `#[garde(skip)]` with weak or valid reason | `g3_code_garde_skip_requires_reason` in `g3rs-code-dylints` | reason-strength validation |
| `g3rs-code/ast-22-deny-forbid-without-reason` | `#[deny]` and `#[forbid]` without useful same-line `// reason:` | `g3_code_local_lint_level_override` in `g3rs-code-dylints` | same-line `// reason:` parsing and strength policy |
| `g3rs-code/ast-24-path-attr-with-reason` | `#[path]` with escape or weak reason, with exact sidecar exemption | `g3_code_path_attr_policy` in `g3rs-code-dylints` | exact sidecar exemption and reason-governance |
| `g3rs-code/ast-31-public-struct-named-fields` | public named field bags, with shared-crate special cases | `g3_code_public_named_field_bag` in `g3rs-code-dylints` | shared-crate carveouts and waiver routing |
| `g3rs-code/ast-32-test-expect-message-quality` | weak test `expect(...)` message quality | `g3_code_test_expect_message` in `g3rs-code-dylints` | repo-specific literal quality thresholds if you still want them separate from the lint |
| `g3rs-test/ignore-reason` | `#[ignore]` reason quality | `g3_test_ignore_requires_reason` in `g3rs-test-dylints` | previous-line and inline comment parsing plus reason-strength policy |

## Audited and not migration targets

- No native-tool or Dylint move candidates found in:
  - all `FILETREE` rules
  - all `HOOKS` rules
  - all `TOPOLOGY` rules
  - all `FMT` rules
  - all `TOOLCHAIN` rules
  - all `CLIPPY` rules
  - all `DENY` rules
  - all `DEPS` allowlist and lockfile rules
  - all `RELEASE` filetree and repo-workflow rules
  - `g3rs-test/real-proof-site`, `g3rs-test/assertions-modules-prove`, `g3rs-test/external-harnesses-use-assertions`
  - `g3rs-arch/lib-facade-only`, `g3rs-arch/mod-facade-only`, `g3rs-arch/feature-gated-exports`
- Reason:
  - these rules are repo-topology, file-placement, workflow-structure, waiver-policy, or cross-file governance rules rather than compiler-native code semantics

## Execution order

1. Delete `g3rs-code/unsafe-code-lint`.
2. Tighten `g3rs-hooks/hook-rs-09-clippy-denies-warnings` so the active hook must prove `cargo clippy --workspace --all-targets --all-features -- -D warnings`, or an equivalent full-coverage invocation.
3. Add the Dylint substrate in root `Cargo.toml`.
4. Create the five Dylint library crates under `packages/rs/dylints`.
5. Migrate the pure-source rules in Finding 3.
6. Migrate the hybrid rules in Finding 4.
7. Delete the replaced guardrail3 rules only after parity tests prove no weaker coverage.
