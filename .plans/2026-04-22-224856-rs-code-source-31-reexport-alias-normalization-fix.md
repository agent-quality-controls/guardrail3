Goal:
Fix `RS-CODE-SOURCE-31` so re-export aliases imported through `use super::Alias` resolve to the underlying public struct target instead of the alias path, and reversed alias import order still normalizes to the concrete struct.

Approach:
- Add red-first regressions in the shared rule tests using valid nested-module Rust with `pub use self::Input as Alias;` followed by `use super::Alias; impl Alias { ... }`, plus a reversed alias import order that still has to resolve through the same boundary.
- Patch the binding/normalization boundary in `rs_code_ast_31_public_struct_named_fields/rule.rs` so alias paths are resolved to their concrete struct target before the inherent-impl comparison and local `use` bindings start from the module-visible alias map.
- Keep the fix out of the final matcher comparison and avoid any parser-wide changes.
- Run the `g3rs-code-source-checks` package tests and validate the package path with `g3rs validate`.

Key decisions:
- Re-export alias resolution belongs in the shared-struct normalization path because the parser already has the module/binding facts needed.
- The red regressions should exercise valid nested-module alias imports, not synthetic comparison-only cases.
- The fix should happen while normalizing bindings, not by special-casing the final comparison.

Files to modify:
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule_tests/shared.rs`
- `.worklogs/<dated>-rs-code-source-31-reexport-alias-normalization-fix.md`
