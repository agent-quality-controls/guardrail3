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
  - `RS-CARGO-CONFIG-08`

## Finding 1 - delete exact duplicate

| Rule | Current file | Current behavior | Action | Exact replacement |
|---|---|---|---|---|
| `RS-CODE-CONFIG-12` | `packages/rs/code/g3rs-code-config-checks/crates/runtime/src/rs_code_config_12_unsafe_code_lint/rule.rs` | checks manifest lint policy for `unsafe_code` | delete rule | `RS-CARGO-CONFIG-01`, `RS-CARGO-CONFIG-02`, `RS-CARGO-CONFIG-09` already own the manifest lint baseline, lint level, and no-weakened-override checks |

## Finding 2 - only remove `todo!` / `unimplemented!` after stronger Clippy execution proof exists

| Rule | Current file | Current behavior | Action | Exact replacement |
|---|---|---|---|---|
| `RS-CODE-SOURCE-13` | `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_13_todo_macros/rule.rs` | flags `todo!`, `unimplemented!`, and inventories `unreachable!` | do not delete now; if hook coverage is strengthened, narrow this rule to `unreachable!` only | `RS-CARGO-CONFIG-01` and `RS-CARGO-CONFIG-02` already require `todo` and `unimplemented` to be denied; source-lane replacement is only safe after hook coverage proves the lint actually runs across the intended targets |
| `RS-CODE-SOURCE-16` | `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_16_panic_macro/rule.rs` | flags `panic!` in non-test code | do not delete now; delete only after hook coverage is strengthened | `RS-CARGO-CONFIG-01` and `RS-CARGO-CONFIG-02` already require `clippy::panic = "deny"`; source-lane replacement is only safe after hook coverage proves the lint actually runs across the intended targets |

- Current hook proof is not strong enough:
  - `RS-HOOKS-SOURCE-04` proves some `cargo clippy` step exists
  - `RS-HOOKS-SOURCE-10` proves one invocation denies warnings
  - neither proves full target or feature coverage
- Exact precondition to remove the source rules:
  - tighten `RS-HOOKS-SOURCE-10` so the active hook must run `cargo clippy --workspace --all-targets --all-features -- -D warnings`, or an equivalent command shape that proves the same coverage

## Finding 3 - move pure source rules to Dylint

| Rule | Current behavior | Exact Dylint target | Exact action |
|---|---|---|---|
| `RS-CODE-SOURCE-01` | crate-level or inline-module `#![allow(...)]`, except approved `unused_crate_dependencies` | `g3_code_no_crate_level_allow` in `g3rs-code-dylints` | move the detection to Dylint; keep `RS-CODE-SOURCE-02` as the approved-exemption inventory |
| `RS-CODE-SOURCE-09` | non-test file exceeds 500 effective code-bearing lines | `g3_code_effective_line_cap` in `g3rs-code-dylints` | move the source counting rule to Dylint; delete `RS-CODE-SOURCE-09` after parity tests |
| `RS-CODE-SOURCE-10` | non-test file has more than 20 top-level `use` imports | `g3_code_use_import_cap` in `g3rs-code-dylints` | move the hard cap to Dylint; keep warn threshold in the paired lint below or emit warn/error from one lint crate |
| `RS-CODE-SOURCE-11` | non-test file has 16-20 top-level `use` imports | `g3_code_use_import_cap` in `g3rs-code-dylints` | move the warn threshold to the same Dylint rule as `RS-CODE-SOURCE-10`; delete `RS-CODE-SOURCE-11` after parity tests |
| `RS-CODE-SOURCE-15` | direct `std::fs` imports and direct `std::fs::*` calls outside filesystem-boundary modules | `g3_code_no_direct_std_fs` in `g3rs-code-dylints` | move to Dylint with exact boundary-module exemptions; delete `RS-CODE-SOURCE-15` after parity tests |
| `RS-CODE-SOURCE-17` | blanket `#[allow]` or `#[expect]` on impl blocks | `g3_code_no_impl_level_suppression` in `g3rs-code-dylints` | move to Dylint; delete `RS-CODE-SOURCE-17` after parity tests |
| `RS-CODE-SOURCE-18` | `cfg_attr` suppression that is effectively unconditional | `g3_code_no_always_true_cfg_attr_suppression` in `g3rs-code-dylints` | move to Dylint; delete `RS-CODE-SOURCE-18` after parity tests |
| `RS-CODE-SOURCE-19` | large struct or enum inventory | `g3_code_large_type_inventory` in `g3rs-code-dylints` | move the size inventory to Dylint; delete `RS-CODE-SOURCE-19` after parity tests |
| `RS-CODE-SOURCE-20` | `#[allow]` or `#[expect]` on `extern` blocks | `g3_code_no_extern_block_suppression` in `g3rs-code-dylints` | move to Dylint; delete `RS-CODE-SOURCE-20` after parity tests |
| `RS-CODE-SOURCE-21` | `use std::fs::*` glob import bypass, including std alias forms | `g3_code_no_std_fs_glob_import` in `g3rs-code-dylints` | move to Dylint; delete `RS-CODE-SOURCE-21` after parity tests |
| `RS-CODE-SOURCE-23` | `include!()` bypass and traversal-bearing include paths | `g3_code_no_include_bypass` in `g3rs-code-dylints` | move to Dylint; delete `RS-CODE-SOURCE-23` after parity tests |
| `RS-CODE-SOURCE-29` | traits above warn and error method-count caps | `g3_code_trait_surface_cap` in `g3rs-code-dylints` | move to Dylint; delete `RS-CODE-SOURCE-29` after parity tests |
| `RS-CODE-SOURCE-33` | public `Result<_, String | &str | anyhow::Error | Box<dyn Error>>` | `g3_code_typed_public_error` in `g3rs-code-dylints` | move to Dylint; delete `RS-CODE-SOURCE-33` after parity tests |
| `RS-CODE-SOURCE-34` | more than 6 type or const generic parameters | `g3_code_generic_parameter_cap` in `g3rs-code-dylints` | move to Dylint; delete `RS-CODE-SOURCE-34` after parity tests |
| `RS-CODE-SOURCE-36` | string dispatch site above 10 literal branches | `g3_code_string_dispatch_cap` in `g3rs-code-dylints` | move to Dylint; delete `RS-CODE-SOURCE-36` after parity tests |
| `RS-GARDE-SOURCE-01` | boundary structs deserialize without `Validate` | `g3_garde_structs_require_validate` in `g3rs-garde-dylints` | move to Dylint; delete `RS-GARDE-SOURCE-01` after parity tests |
| `RS-GARDE-SOURCE-02` | manual `Deserialize` impl bypassing derive-based validation | `g3_garde_no_manual_deserialize_bypass` in `g3rs-garde-dylints` | move to Dylint; delete `RS-GARDE-SOURCE-02` after parity tests |
| `RS-GARDE-SOURCE-03` | boundary enums deserialize without `Validate` | `g3_garde_enums_require_validate` in `g3rs-garde-dylints` | move to Dylint; delete `RS-GARDE-SOURCE-03` after parity tests |
| `RS-GARDE-SOURCE-04` | `sqlx::query_as!` and `query_as_unchecked!` inventory | `g3_garde_query_as_review` in `g3rs-garde-dylints` | move to Dylint; delete `RS-GARDE-SOURCE-04` after parity tests |
| `RS-GARDE-SOURCE-05` | validated fields missing field-level garde validators | `g3_garde_field_requires_validator` in `g3rs-garde-dylints` | move to Dylint; delete `RS-GARDE-SOURCE-05` after parity tests |
| `RS-GARDE-SOURCE-06` | nested validated fields missing `#[garde(dive)]` | `g3_garde_nested_requires_dive` in `g3rs-garde-dylints` | move to Dylint; delete `RS-GARDE-SOURCE-06` after parity tests |
| `RS-GARDE-SOURCE-07` | ctx-using validators without `#[garde(context(...))]` | `g3_garde_context_surface_complete` in `g3rs-garde-dylints` | move to Dylint; delete `RS-GARDE-SOURCE-07` after parity tests |
| `RS-TEST-SOURCE-01` | inline `#[cfg(test)] mod ... { ... }` bodies | `g3_test_no_inline_cfg_test_bodies` in `g3rs-test-dylints` | move to Dylint; delete `RS-TEST-SOURCE-01` after parity tests |
| `RS-TEST-SOURCE-05` | `#[should_panic]` without `expected = "..."` | `g3_test_should_panic_requires_expected` in `g3rs-test-dylints` | move to Dylint; delete `RS-TEST-SOURCE-05` after parity tests |
| `RS-TEST-SOURCE-06` | tautological literal-vs-literal assertions | `g3_test_no_tautological_assertions` in `g3rs-test-dylints` | move to Dylint; delete `RS-TEST-SOURCE-06` after parity tests |
| `RS-TEST-SOURCE-08` | weak `matches!` assertions with `_` payload wildcards | `g3_test_no_weak_matches` in `g3rs-test-dylints` | move to Dylint; delete `RS-TEST-SOURCE-08` after parity tests |
| `RS-ARCH-SOURCE-09` | `#[path = ...]` facade bypass except owned sidecar shape | `g3_arch_no_path_attr` in `g3rs-arch-dylints` | move to Dylint; delete `RS-ARCH-SOURCE-09` after parity tests |
| `RS-APPARCH-SOURCE-04` | public traits defined in io crates | `g3_apparch_no_io_public_traits` in `g3rs-apparch-dylints` | move to Dylint; delete `RS-APPARCH-SOURCE-04` after parity tests |
| `RS-APPARCH-SOURCE-05` | public behavior exposed from `types` crates | `g3_apparch_types_surface_passive_only` in `g3rs-apparch-dylints` | move to Dylint; delete `RS-APPARCH-SOURCE-05` after parity tests |

## Finding 4 - move detection to Dylint but keep guardrail3 for repo-specific reason or waiver policy

| Rule | Current behavior | Exact Dylint target | Keep in guardrail3 |
|---|---|---|---|
| `RS-CODE-SOURCE-03` | item-level `#[allow]` or `#[expect]` without useful same-line `// reason:` | `g3_code_item_level_suppression` in `g3rs-code-dylints` | same-line `// reason:` parsing and strength policy |
| `RS-CODE-SOURCE-04` | documented `#[allow]` and `#[expect]` inventory | `g3_code_item_level_suppression` in `g3rs-code-dylints` | documented-reason inventory |
| `RS-CODE-SOURCE-05` | `#[garde(skip)]` without comment | `g3_code_garde_skip_requires_reason` in `g3rs-code-dylints` | exact comment placement policy |
| `RS-CODE-SOURCE-06` | `#[garde(skip)]` with weak or valid reason | `g3_code_garde_skip_requires_reason` in `g3rs-code-dylints` | reason-strength validation |
| `RS-CODE-SOURCE-22` | `#[deny]` and `#[forbid]` without useful same-line `// reason:` | `g3_code_local_lint_level_override` in `g3rs-code-dylints` | same-line `// reason:` parsing and strength policy |
| `RS-CODE-SOURCE-24` | `#[path]` with escape or weak reason, with exact sidecar exemption | `g3_code_path_attr_policy` in `g3rs-code-dylints` | exact sidecar exemption and reason-governance |
| `RS-CODE-SOURCE-31` | public named field bags, with shared-crate special cases | `g3_code_public_named_field_bag` in `g3rs-code-dylints` | shared-crate carveouts and waiver routing |
| `RS-CODE-SOURCE-32` | weak test `expect(...)` message quality | `g3_code_test_expect_message` in `g3rs-code-dylints` | repo-specific literal quality thresholds if you still want them separate from the lint |
| `RS-TEST-SOURCE-04` | `#[ignore]` reason quality | `g3_test_ignore_requires_reason` in `g3rs-test-dylints` | previous-line and inline comment parsing plus reason-strength policy |

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
  - `RS-TEST-SOURCE-07`, `RS-TEST-SOURCE-16`, `RS-TEST-SOURCE-17`
  - `RS-ARCH-SOURCE-02`, `RS-ARCH-SOURCE-04`, `RS-ARCH-SOURCE-08`
- Reason:
  - these rules are repo-topology, file-placement, workflow-structure, waiver-policy, or cross-file governance rules rather than compiler-native code semantics

## Execution order

1. Delete `RS-CODE-CONFIG-12`.
2. Tighten `RS-HOOKS-SOURCE-10` so the active hook must prove `cargo clippy --workspace --all-targets --all-features -- -D warnings`, or an equivalent full-coverage invocation.
3. Add the Dylint substrate in root `Cargo.toml`.
4. Create the five Dylint library crates under `packages/rs/dylints`.
5. Migrate the pure-source rules in Finding 3.
6. Migrate the hybrid rules in Finding 4.
7. Delete the replaced guardrail3 rules only after parity tests prove no weaker coverage.
