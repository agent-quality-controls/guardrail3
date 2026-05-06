Summary
- Fixed the second adversarial review round for independent G3RS and G3TS verifier hooks.
- Removed inline TypeScript/package-manager gates from this repo pre-commit hook so the Rust verifier is not blocked by unrelated TypeScript checks.
- Tightened verifier source checks around exact paths, scoped categories, real command execution, cargo dupes completeness, and fail-open verifier commands.
- Fixed the follow-up adversarial findings around configured verifier scope matching, repo-root package discovery, npx/bunx false positives, required-tool aliases, and mode-specific Rust verifier checks.
- Fixed the third adversarial findings around `$SCOPE` pre-commit escapes, duplicate G3TS verifier flags, absolute repo-root scope normalization, fixed G3RS pre-commit scope, and required `--diff-filter=ACM` staged-file reads.
- Fixed the fourth adversarial findings around root `--scope .` escapes, `builtin alias` bypasses, and required G3RS verifier commands softened with fail-open wrappers.
- Fixed the fifth adversarial findings around runtime pre-commit repo-root scope, mixed separate/equal flags, and `command alias` bypasses.
- Fixed the sixth adversarial findings around canonical repo-root comparison and aliases inside called verifier functions.

Decisions made
- G3TS verifier now uses concrete `pnpm` command invocations instead of generic helper indirection because source checks must prove real tools run, not only helper names.
- G3TS ingestion derives enabled categories from the configured verifier scope, not from every app in a repo-root crawl.
- G3TS runtime scope discovery now covers root packages, nested `apps/**/package.json`, and nested `packages/**/package.json`; an empty scope fails instead of silently passing.
- G3TS source checks reject mismatched verifier scopes, equals-form verifier flags that the runtime script rejects, `npx echo <tool>`/`bunx echo <tool>` false positives, and aliases for required tools.
- G3TS source checks now require pre-commit verifier scopes to match configured app/package roots directly; `$SCOPE` remains valid only inside the verifier's own `g3ts validate --path "$SCOPE"` command.
- G3TS verifier rejects duplicate `--mode` and `--scope` flags so static source checks and runtime parsing cannot disagree on first-vs-last values.
- G3TS source checks reject `--scope .` as a pre-commit verifier scope even when a root package exists, and detect both `alias` and `builtin alias` for required verifier tools.
- G3TS runtime rejects repo-root scope in pre-commit mode, static checks reject mixed separate/equal verifier flags, and alias detection covers `command alias`.
- G3TS canonicalizes `REPO_ROOT` before comparing it to `SCOPE`, and alias detection scans top-level verifier commands plus function bodies.
- G3RS cargo dupes threshold and test-exclusion rules now both require the same complete cargo dupes invocation so two partial commands cannot satisfy the contract.
- G3RS verifier scripts now receive fail-open checks because a verifier command softened with `|| true` breaks the hook contract.
- G3RS verifier source checks now require required commands in both `pre-commit` and `workspace` mode branches and require pre-commit mode to read staged files.
- G3RS pre-commit source checks now require the fixed `apps/guardrail3-rs` verifier scope and require staged-file reads to include `--diff-filter=ACM`.
- G3RS verifier source checks now reject fail-open wrappers on every required verifier command, including cargo metadata, fmt, and mutants commands that are not part of the older generic critical-command set.

Key files for context
- `.githooks/pre-commit`
- `scripts/g3rs/verify`
- `scripts/g3ts/verify`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/run.rs`
- `packages/ts/hooks/g3ts-hooks-ingestion/crates/runtime/src/run.rs`
- `packages/ts/hooks/g3ts-hooks-source-checks/crates/runtime/src/commands.rs`
- `packages/ts/hooks/g3ts-hooks-source-checks/crates/runtime/src/run.rs`

Next steps
- Run another adversarial review pass against `.plans/2026-05-06-130854-independent-verifier-guarantees.md`.
- Commit only after the new pass reports no verifier contract gaps or after its findings are fixed.
