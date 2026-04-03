# Fix redundant_pub_crate vs unreachable_pub conflict

**Date:** 2026-04-03 23:02

## Summary
Clippy's `redundant_pub_crate` (nursery) conflicts with rustc's
`unreachable_pub` — clippy#5369, known since 2020. Resolution: keep
rustc's `unreachable_pub` at deny, allow `redundant_pub_crate` at
workspace level in parser crates. Added `redundant_pub_crate` to
CARGO-12's approved allow list so no escape hatches needed. Removed
per-site `#[allow(clippy::redundant_pub_crate)]` from all 3 fs.rs files.
