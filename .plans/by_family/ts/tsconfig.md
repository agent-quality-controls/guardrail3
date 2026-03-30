# TS-TSCONFIG

Status: current family contract, legacy-grouped implementation.

Implementation roots:

- `apps/guardrail3/crates/app/ts/validate/tsconfig_check.rs`

Current source of truth:

- this file for family planning/status
- `.plans/todo/checks/ts/tsconfig.md` as the detailed family ledger until the cutover is complete

Current state:

- compiler-config policy exists as a distinct validator file, but not as a family surface
- the code already carries a larger rule inventory than the original placeholder implied

Rule inventory:

- `T9` config existence, parseability, and core strictness baseline
  - Should require a TS config file and enforce the core strictness settings currently bundled under `T9`: `strict`, `noImplicitReturns`, `noUnusedLocals`, `noUnusedParameters`, and `forceConsistentCasingInFileNames`.
  - It is for making sure the compiler baseline exists and catches the most important type-safety and code-hygiene failures.
- `T52` `noUncheckedIndexedAccess`
  - Should require `noUncheckedIndexedAccess = true`.
  - It is for forcing callers to handle `undefined` on dynamic lookups instead of assuming happy-path access.
- `T53` `exactOptionalPropertyTypes`
  - Should require `exactOptionalPropertyTypes = true`.
  - It is for preventing the common confusion between missing and explicitly undefined properties.
- `T54` `isolatedModules`
  - Should require `isolatedModules = true`.
  - It is for keeping the project compatible with fast one-file-at-a-time transpilers.
- `T-TSC-60` `noPropertyAccessFromIndexSignature`
  - Should require `noPropertyAccessFromIndexSignature = true`.
  - It is for making uncertain index-signature access visually explicit.
- `T-TSC-61` `noImplicitOverride`
  - Should require `noImplicitOverride = true`.
  - It is for preventing accidental method shadowing in inheritance hierarchies.
- `T62` `noFallthroughCasesInSwitch`
  - Should require `noFallthroughCasesInSwitch = true`.
  - It is for catching switch fallthrough bugs.
- `T63` `allowUnreachableCode = false`
  - Should require unreachable code to stay disallowed.
  - It is for preventing dead-code drift.
- `T64` `allowUnusedLabels = false`
  - Should require unused labels to stay disallowed.
  - It is for preventing misleading legacy/control-flow leftovers.
- `T65` `target = es2022`
  - Should require the target baseline string value.
  - It is for standardizing emitted JS expectations.
- `T66` `module = esnext`
  - Should require the module baseline string value.
  - It is for standardizing module semantics across the toolchain.
- `T67` `moduleResolution = bundler`
  - Should require bundler-style module resolution.
  - It is for aligning TypeScript with the intended runtime/build tooling.
- `T68` `esModuleInterop = true`
  - Should require `esModuleInterop = true`.
  - It is for predictable CommonJS/ESM interop behavior.
- `T10` extra compiler option inventory
  - Should inventory compiler options outside the known baseline set.
  - It is for surfacing local deviations and framework-specific add-ons without immediately banning them.

Current implementation mapping:

- `check_tsconfig(...)` selects config roots and owns family orchestration
- `parse_tsconfig_json(...)` covers the existence/parseability half of `T9`
- `emit_required_true_bool_checks(...)` covers `T9`, `T52`, `T53`, `T54`, `T-TSC-60`, `T-TSC-61`, `T62`, and `T68`
- `emit_required_false_bool_checks(...)` covers `T63` and `T64`
- `emit_required_string_checks(...)` covers `T65`, `T66`, and `T67`
- extra option inventory is emitted as `T10`

Known reconciliation notes:

- `T9` is overloaded: it currently covers file existence, JSON parseability, and several core strictness settings
- the design docs discuss per-app `tsconfig` presence and `extends` relationships, but current code only checks the selected root/base config files
- there is no current per-app rule for "app tsconfig exists" or "app tsconfig extends base vs standalone by explicit choice"

Historical/supplemental references:

- `.plans/todo/checks/ts/tsconfig.md`
- `.plans/by_file/ts/tsconfig.md`

Next planning focus:

- split the overloaded `T9` contract into clearer family-scoped rule surfaces
- reconcile base-config versus per-app-config ownership against the real TS project shapes
