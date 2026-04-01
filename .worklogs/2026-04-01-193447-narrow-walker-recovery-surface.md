# Narrow Walker Recovery Surface

**Date:** 2026-04-01 19:34
**Scope:** `apps/guardrail3/crates/app/core/project_walker.rs`, `apps/guardrail3/crates/app/core/project_walker/tests.rs`, `apps/guardrail3/crates/app/core/project_walker/README.md`

## Summary
Changed the shared project walker so ignored-file recovery no longer restores arbitrary ignored source files repo-wide. The walker now hard-bans never-governed roots such as `.claude/worktrees`, keeps tracked ignored file recovery, and restores only config/structure files needed for discovery and policy.

## Context & Problem
`RS-CODE` was reporting thousands of findings under `.claude/worktrees/...`. The root cause was not in the `code` family or its mapper. `dump-tree` proved the shared `ProjectTree` already contained `.claude/worktrees`, including Rust source, manifests, and policy files.

That happened because the walker had three layers:
1. ignore-aware baseline walk
2. tracked ignored file recovery via `git ls-files`
3. recursive ignored-file recovery for anything considered “relevant”

The “relevant” test restored both config files and source files like `*.rs`/`*.ts` anywhere under the repo root, even when the ignored path was a non-governed scratch tree. This solved earlier fail-open issues for ignored manifests and hidden structure, but it was too broad and reintroduced machine-local copies as governed repo content.

## Decisions Made

### Add hard-banned never-governed roots
- **Chose:** Introduced a shared hard-ban check for `.claude/worktrees` plus directory-name bans for `.git`, `target`, and `node_modules`.
- **Why:** These paths are not repo-governed source and should stay invisible to every walker phase, including tracked-file recovery, ignored-file recovery, and immediate-child preservation.
- **Alternatives considered:**
  - Fix the issue in `RS-CODE` or mapper slicing only — rejected because `dump-tree` proved the leak already existed in the shared walker output.
  - Maintain a larger ad hoc denylist in families — rejected because the boundary belongs in shared discovery, not per-family filtering.

### Stop globally restoring ignored source files
- **Chose:** Narrowed `should_recover_ignored(...)` to config/structure files handled by `should_cache(...)` only.
- **Why:** The walker must not globally reintroduce ignored `.rs`, `.ts`, and similar source files. Source ownership belongs later, after structure and legality decide governed containers.
- **Alternatives considered:**
  - Keep restoring all ignored source and add more hard-banned roots — rejected because it still leaves the walker deciding source ownership by suffix.
  - Stop restoring ignored files entirely — rejected because ignored manifests/config such as `Cargo.toml` and `guardrail3.toml` must remain visible for fail-closed discovery.

### Update tests to reflect the new contract
- **Chose:** Replaced the previous “ignored TS source is recovered” expectation with tests proving ignored untracked Rust/TS source stays out, while ignored manifests still recover and `.claude/worktrees` stays excluded.
- **Why:** The old tests were enforcing the too-broad contract that caused the leak. The new tests lock the shared boundary where it belongs.
- **Alternatives considered:**
  - Leave old tests and special-case `.claude/worktrees` only — rejected because it would preserve the underlying wrong contract for all other ignored source trees.

## Architectural Notes
The walker is now responsible for:
- respecting `.gitignore` as the baseline noise filter
- restoring tracked ignored files because they are real repo state
- restoring ignored config/policy/structure files outside hard-banned roots
- excluding never-governed scratch roots from all phases

The walker is no longer responsible for globally restoring ignored source code. That source must instead be surfaced later by structure/legality/ownership once governed containers are known.

This keeps discovery fail-closed for manifests and policy without letting local machine-state poison the repo snapshot.

## Information Sources
- `apps/guardrail3/crates/app/core/project_walker.rs` — existing staged walker phases and recovery predicates.
- `apps/guardrail3/crates/app/core/project_walker/tests.rs` — explicit previous tests for recovering ignored untracked manifests and source files.
- `apps/guardrail3/crates/app/core/project_walker/README.md` — prior documented contract for ignored-file recovery.
- `cargo run --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- dump-tree .` — used to prove `.claude/worktrees` already existed in `ProjectTree`.
- `cargo run --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 --no-default-features --features family-code -- rs validate apps/guardrail3 --family code --format json` — used to confirm `.claude/worktrees` findings disappeared after the walker change.
- `.worklogs/2026-03-21-214628-project-tree-walker.md` — original walker rationale including tracked ignored recovery.
- `.worklogs/2026-03-23-220029-hexarch-structural-rule-04-05-hardening.md` — prior motivation for not letting ignored structure disappear.

## Open Questions / Future Considerations
- The remaining question is exactly which config/policy files families truly require restored. The current `should_cache(...)` set is still broad and should be audited against family ownership docs and planned TS families.
- If some ignored source files must be visible for a specific family, that should happen after governed-container discovery, not in the walker’s global ignored-file recovery pass.
- The directory-name hard bans for `target` and `node_modules` are shared policy now; if any governed path legitimately needs those names, that should be an explicit policy decision rather than accidental walker inclusion.

## Key Files for Context
- `apps/guardrail3/crates/app/core/project_walker.rs` — shared walker phases and hard-ban / recovery rules.
- `apps/guardrail3/crates/app/core/project_walker/tests.rs` — adversarial contract tests for ignored recovery and hard-banned roots.
- `apps/guardrail3/crates/app/core/project_walker/README.md` — current documented walker contract.
- `.worklogs/2026-03-21-214628-project-tree-walker.md` — original walker implementation context.
- `.worklogs/2026-03-23-220029-hexarch-structural-rule-04-05-hardening.md` — earlier motivation for recovering ignored structural data.

## Next Steps / Continuation Plan
1. Audit all Rust families and planned TypeScript families to enumerate exactly which config/policy files they require from the walker.
2. Compare that list against `should_cache(...)` and remove any file types that are no longer needed for shared discovery/policy.
3. Decide whether additional never-governed roots should be added alongside `.claude/worktrees`, based on actual repo policy rather than family-local coping.
