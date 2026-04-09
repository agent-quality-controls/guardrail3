## Goal

Run another adversarial coverage pass across the extracted Rust family
packages so config, AST, and ingestion lanes stop carrying obvious fail-open
bugs, weak branch coverage, or pattern drift.

## Scope

Families with config + ingestion lanes:
- cargo
- clippy
- code
- deny
- deps
- fmt
- garde
- release
- test
- toolchain

Families with AST lanes:
- code
- garde
- test

## Approach

1. Audit each family package set locally from the same four attack angles:
   - completeness
   - missing scenarios
   - pattern parity
   - false positives / intent vs implementation

2. Start with package-level structure and boundary checks:
   - does ingestion fail closed where it should?
   - do pipeline tests exist for the real `crawl -> ingest -> check` path?
   - do malformed owned inputs become findings when they should, instead of
     dying early?
   - do rule files keep the one-rule/one-sidecar pattern?

3. For config families, focus on:
   - owned-file selection
   - malformed/unreadable input behavior
   - exact threshold and negative cases
   - full supported filename matrices where applicable

4. For AST families, focus on:
   - parse boundary ownership
   - multi-file proof-catalog branches
   - classification coverage in ingestion
   - direct and pipeline coverage for the hardest branches

5. Fix concrete bugs first, then fill missing tests around the fixed boundary.

## Key decisions

- Treat fail-open boundaries as bugs, not coverage debt.
  - If ingestion prevents a rule from ever firing through the real lane, fix
    the boundary first.

- Prefer local rule-side tests before widening pipeline tests.
  - Rule branches should be pinned close to the rule.
  - Pipeline tests should prove the real lane and ownership boundaries.

- Keep the audit family-by-family, but commit as one repo-wide coverage pass if
  the fixes are cohesive.

## Files likely to change

- `.plans/todo/checks/rs/*.md`
- `packages/rs/*/g3rs-*-config-checks/**`
- `packages/rs/*/g3rs-*-ast-checks/**`
- `packages/rs/*/g3rs-*-ingestion/**`
- `.worklogs/*multi-family-test-attack*.md`
