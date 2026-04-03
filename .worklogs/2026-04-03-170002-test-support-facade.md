# Split test_support lib.rs into facade + support.rs

**Date:** 2026-04-03 17:00

## Summary
12 test_support crates: moved implementation from lib.rs to support.rs,
lib.rs now facade-only with specific named re-exports. Clippy already had
submodules — replaced broad `pub use X::*` with specific re-exports.

Assertions crates not changed — they only have marker imports and pub mod
declarations, which is a rule-level question (should `use X as _;` be
allowed in lib.rs?).
