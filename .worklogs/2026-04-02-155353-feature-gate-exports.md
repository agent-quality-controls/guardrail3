# RS-ARCH-08: Feature-gate facade exports (89 → 4)

**Date:** 2026-04-02 15:53

## Summary
Added feature gating to 44 crates. Each gets [features] with all + default
and #[cfg(feature = "...")] on pub exports. RS-ARCH-08 violations reduced
from 89 to 4.

## Remaining 4
- CLI adapter (3): has its own product feature system, doesn't fit all/api
- Family mapper (1): FamilyMapper export is infrastructure, not family-facing

## Fix for CLI adapter
Reverted incorrectly added api feature gates that broke internal module
references. CLI adapter keeps its existing product-* feature system.
