# Cleanup: remove leftover dirs, move golden-tests

**Date:** 2026-03-18 17:44
**Task:** Remove stale directories, relocate golden-tests

## Changes
1. Delete `apps/guardrail3/local/` — old override dir, replaced by `.guardrail3/overrides/`
2. Delete `apps/guardrail3/guardrail3/` — leftover from wrong path resolution
3. Move `apps/guardrail3/golden-tests/` → `apps/guardrail3/tests/golden-tests/`
4. Update any references to these paths
