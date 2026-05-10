## Goal

Build `guardrail3-ts` as the smallest possible separate app:

- separate CLI app
- validate only
- almost exact structural copy of `guardrail3-rs`
- TS family packages first
- no generation, no init, no diff, no write path

## Correction to prior thinking

The wrong idea was to wire TS into `guardrail3-rs`.

The correct target is:

- `guardrail3-rs`
  - Rust validator app
- `guardrail3-ts`
  - TypeScript validator app

Same concept, same architecture style, separate products.

## Core constraint

Everything under `packages/ts/**` is still implemented in Rust.

So each TS family package must obey Rust guardrails:

- Rust package shape
- Rust config families
- Rust source / filetree / arch checks
- self-hosting under `guardrail3-rs validate`

This is not optional follow-up cleanup.
It is part of the package design from the start.

## Product surface

`guardrail3-ts` should be minimal.

It should support only:

- `validate`

It should not support:

- generate
- init
- diff
- check
- hooks install
- any write path

If those ever come later, they should be added intentionally.
They are not part of the first TS migration.

## App target shape

Make `guardrail3-ts` almost an exact copy of `guardrail3-rs`.

That means mirroring the same app-level package layout:

- `crates/types/app-types`
- `crates/logic/family-runner-*`
- `crates/logic/validate-command/crates/runtime`
- `crates/logic/validate-command/crates/assertions`
- `crates/io/inbound/cli/crates/runtime`
- `crates/io/inbound/cli/crates/assertions`
- `crates/io/outbound/report/crates/runtime`
- `crates/io/outbound/report/crates/assertions`
- `crates/io/outbound/packages`

But with TS family names and TS package dependencies.

## App design rule

Copy the Rust app structure, not the Rust family set.

Keep the app minimal:

- family enum
- validate request
- workspace crawl trait
- family runner trait
- report renderer trait
- CLI parse + run
- report rendering

Do not add app-local complexity for TS.

## Package target shape

Create:

- `packages/ts/{family}/...`

Use the same package-family style as Rust:

- `g3ts-<family>-types`
- `g3ts-<family>-ingestion`
- pure check packages where justified

Likely lanes:

- `-config-checks`
- `-source-checks`
- `-filetree-checks`

But only when the family really needs that lane.

## Highest-impact first wave

Start with the strongest config-policy foundation:

1. `ts/eslint`
2. `ts/tsconfig`
3. `ts/npmrc`
4. `ts/package`

Reason:

- these are the highest-impact TS guardrail surfaces
- they force the important shared decisions early
- they give the first useful `guardrail3-ts validate` surface fast

## What these first families should prove

### `ts/eslint`

Must prove:

- config discovery
- parseability
- plugin package presence
- plugin wiring
- rule baseline
- local config ownership

### `ts/tsconfig`

Must prove:

- typed config parsing
- base vs local config handling
- compiler strictness checks
- inheritance handling

### `ts/npmrc`

Must prove:

- package-manager root ownership
- root config parsing
- root-only policy checks

### `ts/package`

Must prove:

- `package.json` typed parsing
- package-manager root vs package/app root ownership
- explicit key ownership boundaries with other families

## Shared boundary rules to preserve

### 1. Whole typed config files

Use the same preference as Rust:

- parse once
- type once
- pass whole typed config files across family boundaries
- do not centrally slice configs into mini family views

Examples:

- `eslint.config.*`
- `tsconfig*.json`
- `.npmrc`
- `package.json`
- `.jscpd.json`
- stylelint config

Inside a family, checks can still consume smaller local typed inputs.

### 2. Explicit `package.json` ownership

This must be settled early:

- `ts/package`
  - generic package policy
  - packageManager
  - engines
  - preinstall
  - prepare
  - workspace policy
  - generic scripts
  - `pnpm.*`
- `ts/eslint`
  - eslint plugin package presence
- `ts/fmt`
  - formatter package presence
- `ts/spelling`
  - cspell package presence
- `ts/typecov`
  - type coverage package presence
- `ts/size`
  - size-limit package presence
- `ts/css`
  - stylelint package presence

Do not let `ts/package` absorb tool-family ownership.

### 3. Nearest local config ownership where appropriate

Preserve local-config ownership for:

- `ts/eslint`
- `ts/tsconfig`
- `ts/fmt`
- `ts/spelling`
- `ts/typecov`
- `ts/size`
- `ts/jscpd`
- `ts/css`

Do not apply that model to:

- `ts/package`
- `ts/code`
- `ts/arch`
- `ts/hexarch`
- `ts/libarch`
- `ts/content`
- `ts/i18n`
- `ts/seo`
- `ts/tests`

## Recommended migration order

Phase 1 - app shell + config-policy foundation:

1. create `apps/guardrail3-ts` as a minimal validate-only app shell copied from `guardrail3-rs`
2. create `packages/ts/eslint/...`
3. create `packages/ts/tsconfig/...`
4. create `packages/ts/npmrc/...`
5. create `packages/ts/package/...`
6. wire those families into `guardrail3-ts validate`

Phase 2 - adjacent config/tool families:

7. `ts/fmt`
8. `ts/jscpd`
9. `ts/css`
10. `ts/spelling`
11. `ts/typecov`
12. `ts/size`

Phase 3 - source and tests:

13. `ts/code`
14. `ts/tests`

Phase 4 - architecture families:

15. `ts/arch`
16. `ts/libarch`
17. `ts/hexarch`

Phase 5 - product-shape families:

18. `ts/content`
19. `ts/i18n`
20. `ts/seo`

## Minimal app implementation strategy

Do not invent a new app architecture.

Instead:

1. copy the `guardrail3-rs` app layout
2. rename crates and package dependencies for TS
3. shrink to validate-only if any Rust-only pieces do not apply
4. keep the same command boundary style

This should give:

- less design churn
- less app-level code
- easier parity across the two validator apps

## Legacy TS code treatment

Use:

- `legacy/apps/guardrail3-current/crates/app/ts/validate/**`

only as migration input.

Do not revive it as the active runtime.
Do not build new grouped validator buckets around it.

Normalize it into family packages.

## Concrete next steps

1. Decide exact TS package naming:
   - likely `g3ts-*`
2. Create the `apps/guardrail3-ts` workspace shell by copying the minimal `guardrail3-rs` structure
3. Write the first detailed package plan for:
   - `ts/eslint`
4. Write the first detailed package plan for:
   - `ts/tsconfig`
5. Write the first detailed package plan for:
   - `ts/npmrc`
6. Write the first detailed package plan for:
   - `ts/package`
7. Write the app-wiring plan for the first four TS families into `guardrail3-ts validate`

## Non-final questions

Open items:

1. exact top-level package path:
   - `packages/ts/{family}/...`
   - this is the current best direction, not yet implemented
2. exact package prefix:
   - `g3ts-*`
   - current best guess, not final
3. whether there should be one shared TS workspace crawl package before family migration starts
4. which legacy TS checks should be deleted instead of migrated

## Files to modify later

Likely future anchors:

- new app root under `apps/guardrail3-ts`
- new TS family packages under `packages/ts/`
- `.plans/todo/checks/ts/*.md`
- legacy TS validator code under `legacy/apps/guardrail3-current/crates/app/ts/validate/**`
