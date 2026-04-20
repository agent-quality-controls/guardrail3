Goal

- Tighten the `ts/eslint` family where the attack pass found real gaps:
  - ignored-file-aware probe selection
  - stronger source-probe semantics
  - TSX coverage when a real TSX source file exists
  - stronger JS carve-out proof
  - exact-result assertions in tests

Approach

- Fix ingestion probe selection in `g3ts-eslint-ingestion`:
  - skip ignored and unreadable files
  - prefer real source files over scripts
  - add `TsxSource` only when a real `.tsx` file exists
  - prefer real JS source before script fallbacks
- Add failing/covering tests for:
  - ignored probe paths are skipped
  - unreadable config returns `Unreadable`
  - TSX probe presence when a real `.tsx` file exists
- Tighten `g3ts-eslint-config-checks`:
  - add TSX parity enforcement only when a real `TsxSource` probe exists
  - strengthen JS carve-out to prove representative typed-lint rules do not leak onto JS
  - make run-test assertions exact instead of subset-only
- Re-run package tests, formatting, `g3rs validate`, and `g3ts` against the two app roots.

Key decisions

- Keep the effective-config design.
  - Reason: this family is about what ESLint actually enforces, not whether the config source used one exact preset spelling.
- Do not widen `g3ts` into nested-root discovery in this pass.
  - Reason: current app contract is to validate the explicit root passed by the user.
- Make TSX parity conditional on a real TSX probe.
  - Reason: projects without real TSX source should not fail TSX-specific parity checks.

Files to modify

- `packages/ts/eslint/g3ts-eslint-ingestion/**`
- `packages/ts/eslint/g3ts-eslint-config-checks/**`
- `packages/parsers/eslint-config-parser/**` tests if needed
