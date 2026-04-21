Goal

Fix the concrete TS family architecture bugs found by the attack review so the TS package structure better matches the established RS seam:

- parser packages own parsing and parser-specific helpers
- ingestion owns discovery and normalization
- config-checks consume narrow family facts
- public family types stay minimal

Approach

1. Fix the confirmed `tsconfig` correctness bug first.
   - Add a failing test for valid `extends` arrays that share a common ancestor.
   - Fix cycle detection in `g3ts-tsconfig-ingestion` so it tracks the active traversal stack instead of a repo-wide seen set.
   - Re-run ingestion and config-check tests plus `g3rs validate` on the `tsconfig` slice.

2. Narrow the public `ts/eslint` family input.
   - Replace the public `Parsed` payload from full parser document to a smaller family-owned snapshot/fact surface.
   - Keep the parser package unchanged unless the fix proves a parser boundary hole.
   - Update ingestion, config-check helpers, and tests to use the narrower input.

3. Narrow the public `ts/tsconfig` family input.
   - Remove raw parser document from the public parsed state if checks only need normalized facts.
   - Keep the parser package as the document boundary and keep parser helpers inside it.
   - Update config-check helpers and tests accordingly.

4. Reassess root-awareness drift in `ts/eslint` and `ts/tsconfig`.
   - Do not widen scope into a full local-config multi-root model unless the current app-root runner cannot stay correct.
   - If the current plan is too broad for the actual app-root validator shape, record the exact mismatch in the worklog rather than leaving it implicit.

Key decisions

- Fix the real check bug in `tsconfig` first.
  - Reason: shared-ancestor `extends` arrays are a real false positive, not just a design complaint.
- Treat “family types should not expose full parser documents” as the main seam fix for this pass.
  - Reason: this is the clearest, local, architecture-improving correction with low risk to the current runner behavior.
- Do not redesign `g3ts` around repo-global multi-root ownership in this pass.
  - Reason: the active runner validates one explicit target root at a time. Full multi-root ownership is a larger design shift and should not be smuggled into a bug-fix pass without tighter requirements.

Files to modify

- `packages/ts/eslint/g3ts-eslint-types/**`
- `packages/ts/eslint/g3ts-eslint-ingestion/**`
- `packages/ts/eslint/g3ts-eslint-config-checks/**`
- `packages/ts/tsconfig/g3ts-tsconfig-types/**`
- `packages/ts/tsconfig/g3ts-tsconfig-ingestion/**`
- `packages/ts/tsconfig/g3ts-tsconfig-config-checks/**`
- `.worklogs/*fix-ts-family-boundaries*.md`

Tests to add first

- `g3ts-tsconfig-ingestion`
  - valid `extends` array with shared parent does not report circular chain
- `g3ts-eslint-*`
  - family input no longer exposes parser document shape directly
- `g3ts-tsconfig-*`
  - family input no longer requires raw parser document for checks
