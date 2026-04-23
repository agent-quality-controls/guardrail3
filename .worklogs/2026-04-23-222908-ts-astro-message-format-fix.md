## Summary

Rewrote the active `ts/astro` guardrail messages and `eslint-plugin-astro-pipeline` lint messages to match the repo's rule-message contract. The new strings now name the concrete bad thing, the concrete fix surface, and the concrete reason instead of only restating policy.

## Decisions made

- Used the repo-local message format plan as the source of truth.
  - Read `.plans/2026-04-14-220517-rule-error-message-format.md` and compared the active TS messages against representative Rust rules before changing anything.

- Tightened plugin messages to point at configured surfaces instead of abstract "approved modules".
  - Added `message-surfaces.ts` so lint messages can name the configured adapter or loader surface from rule options.
  - Rejected keeping messages like "approved adapter or loader" because the adversarial pass correctly flagged them as too vague.

- Tightened Astro config/filetree messages to use concrete failure reasons.
  - Replaced policy-level why text like "normal app validation path" with concrete consequences such as CI and local validation missing Astro errors.

## Key files for context

- `.plans/2026-04-14-220517-rule-error-message-format.md`
- `.plans/2026-04-23-221719-ts-astro-message-format-fix.md`
- `packages/ts/eslint-plugin-astro-pipeline/src/utils/message-surfaces.ts`
- `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-authored-content-fs-read.ts`
- `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-authored-content-glob.ts`
- `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-direct-astro-content-in-routes.ts`
- `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-runtime-mdx-eval.ts`
- `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-side-loader-imports.ts`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/*.rs`
- `packages/ts/astro/g3ts-astro-file-tree-checks/crates/runtime/src/*.rs`

## Verification

- `npm test` in `packages/ts/eslint-plugin-astro-pipeline`
- `cargo test -q --manifest-path packages/ts/astro/g3ts-astro-config-checks/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/astro/g3ts-astro-file-tree-checks/Cargo.toml --workspace`
- Two adversarial review passes against:
  - `.plans/2026-04-14-220517-rule-error-message-format.md`
  - representative Rust rule files
  - touched TS/Astro message files
  - final result: `No concrete findings.`

## Next steps

- Keep future TS family messages pinned to the same format before adding more active TS rules.
- If the Astro plugin grows new rules, make the configured destination surface available in the message data up front instead of adding vague fallback wording later.
