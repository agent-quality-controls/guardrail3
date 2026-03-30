# TS-TSCONFIG

Status: current family contract, legacy-grouped implementation, weaker than the Rust `toolchain` plus `cargo` split.

Implementation roots:

- `apps/guardrail3/crates/app/ts/validate/tsconfig_check.rs`

Current source of truth:

- this file for family planning/status
- `.plans/todo/checks/ts/tsconfig.md` as the detailed family ledger until the cutover is complete

Current state:

- compiler-config policy exists as a distinct validator file, but not as a family surface
- the code already carries a larger rule inventory than the original placeholder implied
- compared with Rust, this family currently mixes repo/base compiler-floor policy with local app/package strictness policy

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

Target design split inside the family:

- base/runtime floor rules
  - own parseable base config, runtime/tooling baseline keys, and repo-wide compiler-floor semantics
- local strictness/inheritance rules
  - own per-app/package `tsconfig.json` presence, `extends` behavior, local weakening, and strict compiler policy

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
- the current design still lacks an explicit ownership split comparable to:
  - `RS-TOOLCHAIN` for repo/runtime floor
  - `RS-CARGO` for per-policy-root compiler/lint baseline
- if the implementation rejects valid JSONC-style `tsconfig` syntax, that is design drift from the real TypeScript tool surface and should be treated as a family bug, not as policy strictness
- `target`, `module`, `moduleResolution`, and `esModuleInterop` are currently bundled with local strictness keys even though they behave more like runtime/toolchain-floor policy

Historical/supplemental references:

- `.plans/todo/checks/ts/tsconfig.md`
- `.plans/by_file/ts/tsconfig.md`
- `.plans/by_family/rs/toolchain.md`
- `.plans/by_family/rs/cargo.md`

Next planning focus:

- split the overloaded `T9` contract into separate rules for:
  - config presence
  - parseability
  - core strictness keys
- add explicit per-app/package inheritance rules:
  - local `tsconfig.json` existence
  - required `extends` behavior or explicit standalone allowance
  - no silent weakening of inherited strictness
- make the base-floor versus local-policy split explicit in the family contract before further rule expansion
- parse real `tsconfig` syntax rather than a stricter fake subset if the current runtime still rejects JSONC-style inputs
