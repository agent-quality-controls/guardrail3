# Remove Accidental Target Artifacts

**Date:** 2026-03-28 12:04
**Scope:** `.worklogs/2026-03-28-120454-remove-target-artifacts.md`, `apps/guardrail3/target/**`

## Summary
Removed accidentally committed `apps/guardrail3/target/` build artifacts from the previous bundle commit. The source bundle itself was valid, but staging after verification recreated the app target directory and `git add -A` captured it.

## Context & Problem
The prior bundle checkpoint intentionally committed all remaining source work, but the test/build verification recreated `apps/guardrail3/target/` after I had cleaned generated artifacts earlier in the turn. Because the repo root `.gitignore` already ignores `/target/`, the correct fix is simply to drop the tracked subtree from the index and keep the source commit intact.

## Decisions Made

### Remove tracked target output instead of rewriting the source bundle
- **Chose:** Make a follow-up cleanup commit that removes `apps/guardrail3/target/` from tracking.
- **Why:** The source changes in the bundle commit are still the right checkpoint. The only mistake was tracking generated build output.
- **Alternatives considered:**
  - Rewrite the previous commit interactively — rejected because a small explicit cleanup commit is safer and clearer.
  - Leave the tracked artifacts in place — rejected because generated build output does not belong in the repo.

## Architectural Notes
No source semantics changed here. This is repository hygiene only. The root `.gitignore` already ignores `/target/`, so removing the tracked subtree restores the intended state.

## Information Sources
- `.gitignore`
- `git ls-tree -r --name-only HEAD | rg '^apps/guardrail3/target/'`
- prior commit `a3c7ddf`

## Open Questions / Future Considerations
- None for this cleanup itself. The follow-up work remains whatever comes next from the bundled Rust-family checkpoint.

## Key Files for Context
- `.gitignore` — confirms `target/` is intentionally ignored
- `.worklogs/2026-03-28-120301-bundle-rs-family-progress.md` — the source bundle this cleanup follows

## Next Steps / Continuation Plan
1. Remove `apps/guardrail3/target/` from tracking.
2. Commit the cleanup worklog plus the artifact removal.
3. Continue from the bundled source checkpoint without generated debris in the repo.
