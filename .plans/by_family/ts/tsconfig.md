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

- `T9` — TypeScript config exists, parses, and carries the core strict baseline.
  What it should do: require `tsconfig.base.json` or `tsconfig.json`, require valid JSON, and require the core strict options:
  - `strict`
  - `noImplicitReturns`
  - `noUnusedLocals`
  - `noUnusedParameters`
  - `forceConsistentCasingInFileNames`
  What it is for: establish the minimum compiler strictness floor and fail fast on missing/broken TS compiler policy.
- `T52` — `noUncheckedIndexedAccess` is enabled.
  What it should do: require `noUncheckedIndexedAccess = true`.
  What it is for: force explicit handling of possibly-missing indexed access.
- `T53` — `exactOptionalPropertyTypes` is enabled.
  What it should do: require `exactOptionalPropertyTypes = true`.
  What it is for: avoid silent confusion between omitted and explicitly-undefined optional fields.
- `T54` — `isolatedModules` is enabled.
  What it should do: require `isolatedModules = true`.
  What it is for: keep transpilation safe for single-file transforms and modern bundler pipelines.
- `T-TSC-60` — `noPropertyAccessFromIndexSignature` is enabled.
  What it should do: require `noPropertyAccessFromIndexSignature = true`.
  What it is for: make index-signature access explicit instead of pretending arbitrary properties are definitely present.
- `T-TSC-61` — `noImplicitOverride` is enabled.
  What it should do: require `noImplicitOverride = true`.
  What it is for: prevent accidental shadowing in class hierarchies.
- `T62` — `noFallthroughCasesInSwitch` is enabled.
  What it should do: require `noFallthroughCasesInSwitch = true`.
  What it is for: catch accidental fallthrough in control flow.
- `T63` — `allowUnreachableCode` is false.
  What it should do: require `allowUnreachableCode = false`.
  What it is for: surface dead code instead of silently permitting it.
- `T64` — `allowUnusedLabels` is false.
  What it should do: require `allowUnusedLabels = false`.
  What it is for: catch dead or accidental labels.
- `T65` — `target` is `es2022`.
  What it should do: require `compilerOptions.target = "es2022"`.
  What it is for: pin emitted language/runtime assumptions to the approved modern baseline.
- `T66` — `module` is `esnext`.
  What it should do: require `compilerOptions.module = "esnext"`.
  What it is for: keep module semantics aligned with the modern ESM toolchain.
- `T67` — `moduleResolution` is `bundler`.
  What it should do: require `compilerOptions.moduleResolution = "bundler"`.
  What it is for: align TypeScript’s resolution behavior with the expected modern bundler/runtime model.
- `T68` — `esModuleInterop` is enabled.
  What it should do: require `esModuleInterop = true`.
  What it is for: avoid common interop hazards between CommonJS and ESM dependencies.
- `T10` — extra compiler options are inventoried.
  What it should do: emit inventory/info for compiler options outside the baseline allowlist.
  What it is for: make local compiler-policy drift visible without automatically forbidding every additional option.

Current code mapping:

- `apps/guardrail3/crates/app/ts/validate/tsconfig_check.rs`
  - `check_tsconfig(...)` selects config roots and handles missing-config `T9`
  - `parse_tsconfig_json(...)` handles parse failure under `T9`
  - `check_single_tsconfig(...)` fans out the full rule set
  - `emit_required_true_bool_checks(...)` carries `T9`, `T52`, `T53`, `T54`, `T-TSC-60`, `T-TSC-61`, `T62`, `T68`
  - `emit_required_false_bool_checks(...)` carries `T63`, `T64`
  - `emit_required_string_checks(...)` carries `T65`, `T66`, `T67`
  - the extra-key scan at the end carries `T10`

Current doc/code reconciliation notes:

- the old ledger in `.plans/todo/checks/ts/tsconfig.md` is directionally correct but under-specified; it collapses many concrete check IDs into broad categories
- `T9` is overloaded today: file existence, parseability, and several core strictness settings all share one id
- the live code already has a rich compiler-policy surface; this family mainly needs reconciliation and later architectural migration, not rule invention

Historical/supplemental references:

- `.plans/todo/checks/ts/tsconfig.md`
- `.plans/by_file/ts/tsconfig.md`

Next planning focus:

- reconcile nearest-config ownership and root inheritance against real TS project shapes
- decide whether `T9` should be split into existence/parseability versus core strictness rules during the eventual TS family migration
- verify whether the current `tsconfig.base.json` vs local `tsconfig.json` precedence matches the intended TS family ownership model before demoting the old ledger
