# Fix clippy-toml lint issues

**Date:** 2026-04-03 22:45

## Summary
Added docs to private BanEntryDetail fields, made extra() const fn.
Attempted to fix redundant_pub_crate by making module pub(crate) —
doesn't work because lib.rs is crate root, so pub(crate) mod is itself
redundant. The lint catch-22 is genuine: pub triggers unreachable_pub,
pub(crate) triggers redundant_pub_crate. Allow is unavoidable.
