## Goal

Implement the first working `ts/astro` slice plus the repo-owned `eslint-plugin-astro-pipeline`.

End state:
- Astro apps can be selected as a first-class TS family in `apps/guardrail3-ts`
- `ts/astro` has working types, ingestion, file-tree checks, config checks, and CLI registration
- the first `ts/astro` rules enforce the initial high-value Astro setup contract from the family plan
- a standalone Node package `packages/ts/eslint-plugin-astro-pipeline` exists with the first source-policy rules, tests, and recommended config
- verification includes unit tests plus an adversarial review against `.plans/2026-04-23-151845-ts-astro-family-plan.md`

## Approach

### 1. ESLint plugin package

Create `packages/ts/eslint-plugin-astro-pipeline` as a standalone Node package.

Implementation:
- add `package.json`
- add `src/index.ts`
- add `src/configs/recommended.ts`
- add `src/utils/*`
- add initial rules:
  - `no-authored-content-fs-read`
  - `no-authored-content-glob`
  - `no-direct-astro-content-in-routes`
  - `no-runtime-mdx-eval`
- add package-local tests using ESLint `RuleTester`

Why this slice:
- these rules directly cover the bypasses already observed in `steady-parent`
- they are concrete enough to test without waiting on more repo-wide Astro policy

### 2. `ts/astro` Rust family scaffold

Create:
- `packages/ts/astro/g3ts-astro-types`
- `packages/ts/astro/g3ts-astro-ingestion`
- `packages/ts/astro/g3ts-astro-config-checks`
- `packages/ts/astro/g3ts-astro-file-tree-checks`

Implement the first rule slice from the family plan:
- `TS-ASTRO-FILETREE-01`
- `TS-ASTRO-FILETREE-02`
- `TS-ASTRO-FILETREE-04`
- `TS-ASTRO-FILETREE-05`
- `TS-ASTRO-CONFIG-02`
- `TS-ASTRO-CONFIG-03`
- `TS-ASTRO-CONFIG-04`
- `TS-ASTRO-CONFIG-06`
- `TS-ASTRO-CONFIG-07`

Use shared parsers only:
- `package-json-parser`
- `eslint-config-parser`

Ingestion responsibilities:
- detect Astro apps
- discover `astro.config.*`
- discover `src/content.config.*`
- classify build-collection apps from config/content presence
- do not implement `TS-ASTRO-FILETREE-05` via raw string scanning
- keep `TS-ASTRO-FILETREE-05` as a typed/manual rule input only until a parser-owned or plugin-owned source fact exists
- read parsed `package.json` and `eslint.config.*` facts through existing parser packages
- classify minimal facts needed for the first rule slice

### 3. TS app registration

Update `apps/guardrail3-ts`:
- add `SupportedFamily::Astro`
- update CLI family selection names
- add workspace deps for new Astro packages
- wire Astro family into config/structure runners according to the implemented lanes
- include package reporting if needed by current package registry conventions

### 4. Verification

Run:
- package-local plugin tests
- `cargo test` for new `ts/astro` packages
- `cargo test` for `apps/guardrail3-ts`
- targeted grep checks for expected rule IDs / family names / plugin exports

### 5. Adversarial review

Use a background attack review against:
- `.plans/2026-04-23-151845-ts-astro-family-plan.md`
- this implementation plan
- the resulting code

Any missing planned item or architectural mismatch becomes a follow-up task before reporting completion.

## Key decisions

### Plugin before source guardrails

Decision:
- source-policy rules live in the ESLint plugin first

Why:
- user wants real validation delegated out of guardrails when feasible
- these checks are source-policy lint rules, not guardrail-only semantics

Alternative rejected:
- create `g3ts-astro-source-checks` first

Reason rejected:
- duplicates validator behavior inside guardrails and violates the intended split

### Standalone Node package

Decision:
- the plugin is a standalone package under `packages/ts`

Why:
- this repo currently has no Node workspace root
- a package-local test/tooling setup is the smallest working path

Alternative rejected:
- invent a new monorepo JS workspace first

Reason rejected:
- expands scope without helping the first implementation slice

### First rule slice only

Decision:
- implement the concrete first slice from the Astro family plan

Why:
- enough to prove the family shape and wiring end-to-end
- avoids speculative broadening before ingestion facts and plugin utilities settle

Alternative rejected:
- implement the full rule inventory immediately

Reason rejected:
- too much surface area before the first end-to-end architecture is proven

## Files to modify

Planned new files:
- `.plans/2026-04-23-163555-ts-astro-implementation.md`
- `packages/ts/eslint-plugin-astro-pipeline/**`
- `packages/ts/astro/g3ts-astro-types/**`
- `packages/ts/astro/g3ts-astro-ingestion/**`
- `packages/ts/astro/g3ts-astro-config-checks/**`
- `packages/ts/astro/g3ts-astro-file-tree-checks/**`

Planned existing files:
- `apps/guardrail3-ts/Cargo.toml`
- `apps/guardrail3-ts/crates/types/app-types/src/supported_family.rs`
- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/selection.rs`
- `apps/guardrail3-ts/crates/logic/family-runner-config/**`
- `apps/guardrail3-ts/crates/logic/family-runner-structure/**`
- `.plans/todo/checks/ts/README.md`
