# TS-ESLINT — eslint.config.mjs checker (63 rules)

**Input:** eslint.config.mjs (one per project, tree-sitter parsed)
**Current code:** `eslint_check.rs`, `eslint_audit.rs`, `eslint_plugin_checks.rs`

## Config existence + structure (8 rules)

| New ID | Old ID | Rule | Status |
|--------|--------|------|--------|
| TS-ESLINT-01 | T1 | eslint.config.mjs exists | Implemented |
| TS-ESLINT-02 | T2 | `max-lines` rule (400) | Implemented |
| TS-ESLINT-03 | T3 | `max-lines-per-function` rule (100) | Implemented |
| TS-ESLINT-04 | T4 | `complexity` rule (25) | Implemented |
| TS-ESLINT-05 | T5 | `no-restricted-imports` configured | Implemented |
| TS-ESLINT-06 | T6 | Import boundary plugin present | Implemented |
| TS-ESLINT-07 | T7 | Inventory of relaxed rules (off/warn) | Implemented |
| TS-ESLINT-08 | T8 | Inventory of file-specific overrides | Implemented |

## Boundary enforcement (4 rules)

| New ID | Old ID | Rule | Status |
|--------|--------|------|--------|
| TS-ESLINT-09 | T36 | Boundary zone definitions | Implemented |
| TS-ESLINT-10 | T37 | Import direction rules (inward flow) | Implemented |
| TS-ESLINT-11 | T38 | Entry-point barrel enforcement | Implemented |
| TS-ESLINT-12 | T39 | External dependency per-zone bans | Implemented |

## Core TypeScript rules (24 rules)

| New ID | Old ID | ESLint Rule | Status |
|--------|--------|------------|--------|
| TS-ESLINT-13 | T40 | `no-floating-promises` | Implemented |
| TS-ESLINT-14 | T41 | `no-explicit-any` | Implemented |
| TS-ESLINT-15 | T42 | `no-console` | Implemented |
| TS-ESLINT-16 | T43 | `eqeqeq` | Implemented |
| TS-ESLINT-17 | T44 | `no-restricted-globals` | Implemented |
| TS-ESLINT-18 | T45 | `no-cycle` (import) | Implemented |
| TS-ESLINT-19 | T46 | `max-dependencies` | Implemented |
| TS-ESLINT-20 | T47 | `explicit-function-return-type` | Implemented |
| TS-ESLINT-21 | T48 | `strict-boolean-expressions` | Implemented |
| TS-ESLINT-22 | T49 | Test/spec file override inventory | Implemented |
| TS-ESLINT-23 | T50 | Route wrapper enforcement (`withBody`/`withRoute`) | Implemented |
| TS-ESLINT-24 | T51 | `process.env` restriction via ESLint | Implemented |
| TS-ESLINT-25 | T60 | `no-misused-promises` | Implemented |
| TS-ESLINT-26 | T61 | `await-thenable` | Implemented |
| TS-ESLINT-27 | T62 | `consistent-type-imports` | Implemented |
| TS-ESLINT-28 | T63 | `no-non-null-assertion` | Implemented |
| TS-ESLINT-29 | T64 | `switch-exhaustiveness-check` | Implemented |
| TS-ESLINT-30 | T65 | `no-unused-vars` | Implemented |
| TS-ESLINT-31 | T66 | `require-await` | Implemented |
| TS-ESLINT-32 | T67 | `no-param-reassign` | Implemented |
| TS-ESLINT-33 | T68 | `no-unsafe-assignment` | Implemented |
| TS-ESLINT-34 | T69 | `no-unsafe-member-access` | Implemented |
| TS-ESLINT-35 | T70 | `no-unsafe-call` | Implemented |
| TS-ESLINT-36 | T71 | `no-unsafe-return` | Implemented |

## Safety + style rules (12 rules)

| New ID | Old ID | ESLint Rule | Status |
|--------|--------|------------|--------|
| TS-ESLINT-37 | T72 | `no-unsafe-argument` | Implemented |
| TS-ESLINT-38 | T73 | `explicit-module-boundary-types` | Implemented |
| TS-ESLINT-39 | T74 | `promise-function-async` | Implemented |
| TS-ESLINT-40 | T75 | `consistent-type-exports` | Implemented |
| TS-ESLINT-41 | T76 | `consistent-type-definitions` | Implemented |
| TS-ESLINT-42 | T77 | `no-unnecessary-condition` | Implemented |
| TS-ESLINT-43 | T78 | `prefer-nullish-coalescing` | Implemented |
| TS-ESLINT-44 | T79 | `prefer-optional-chain` | Implemented |
| TS-ESLINT-45 | T80 | `no-deprecated` | Implemented |
| TS-ESLINT-46 | T81 | `restrict-template-expressions` | Implemented |
| TS-ESLINT-47 | T82 | `no-throw-literal` | Implemented |
| TS-ESLINT-48 | T83 | `no-empty` | Implemented |

## Plugin configs (15 rules)

| New ID | Old ID | Plugin/Concept | Status |
|--------|--------|---------------|--------|
| TS-ESLINT-49 | T-ESLP-01 | Unicorn flat config import | Implemented |
| TS-ESLINT-50 | T-ESLP-02 | 8 unicorn disabled rules | Implemented |
| TS-ESLINT-51 | T-ESLP-03 | 4 unicorn extra rules | Implemented |
| TS-ESLINT-52 | T-ESLP-04 | RegExp flat config import | Implemented |
| TS-ESLINT-53 | T-ESLP-05 | 6 regexp extra rules | Implemented |
| TS-ESLINT-54 | T-ESLP-06 | 23 sonarjs cherry-picked rules | Implemented |
| TS-ESLINT-55 | T-ESLP-07 | jsx-a11y strict config (content profile) | Implemented |
| TS-ESLINT-56 | T-ESLP-08 | jsx-a11y control-has-associated-label | Implemented |
| TS-ESLINT-57 | T-ESLP-09 | 10 React extra rules (non-a11y) | Implemented |
| TS-ESLINT-58 | T-ESLP-10 | 16 built-in ESLint/TS rules | Implemented |
| TS-ESLINT-59 | T-ESLP-11 | 5 test relaxation rules | Implemented |
| TS-ESLINT-60 | T-ESLP-12 | Tailwind-ban plugin (content profile) | Implemented |
| TS-ESLINT-61 | T-ESLP-13 | strictTypeChecked preset | Implemented |
| TS-ESLINT-62 | T-ESLP-14 | stylisticTypeChecked preset | Implemented |
| TS-ESLINT-63 | T-ESLP-15 | RegExp constructor ban | Implemented |
