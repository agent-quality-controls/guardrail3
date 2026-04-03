# Split test_support lib.rs to facade-only

**Date:** 2026-04-03 16:59

## Summary
Converted all 12 test_support crate lib.rs files to facade-only pattern
by extracting implementation into sibling modules. ARCH-02 requires
lib.rs to contain only mod/use declarations and specific named
re-exports (broad `pub use X::*` is flagged).

## Decisions
- **Named re-exports over glob**: ARCH-02 flags `pub use X::*` as
  broad re-exports even in lib.rs. Used specific `pub use support::{A, B}`
  for all crates.
- **clippy test_support**: Already had submodules (fixtures, fs_ops,
  toml_edit). Replaced `pub use fixtures::*` etc. with specific named
  re-exports from each module. No new support.rs needed.
- **All other test_support crates**: Created `support.rs` with the full
  implementation moved from lib.rs. lib.rs became
  `mod support; pub use support::{...};`.
- **assertions crates left unchanged**: Examined all 14 assertions
  lib.rs files. None have inline helper functions - they only contain
  `use X as _;` marker imports, `mod common;` declarations (which are
  fine per ARCH-02 as non-pub mod without body), and `#[cfg] pub mod`
  declarations. Nothing to extract.

## Key files
- Rule definition: `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/facade/rs_arch_02_lib_facade_only.rs`
- Facade surface parser: `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/facts/facade_surface.rs`
- All `*/test_support/src/lib.rs` and `*/test_support/src/support.rs` under `apps/guardrail3/crates/app/rs/families/`

## Next steps
- The `use X as _;` marker imports in assertions lib.rs files are
  technically "private use" body items per ARCH-02. If these need to
  pass ARCH-02, they'd need to move or the rule would need an exception
  for `use X as _;` patterns.
