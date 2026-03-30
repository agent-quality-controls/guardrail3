# TS-ESLINT

Status: current family contract, legacy-grouped implementation.

Implementation roots:

- `apps/guardrail3/crates/app/ts/validate/eslint_check.rs`
- `apps/guardrail3/crates/app/ts/validate/eslint_plugin_checks.rs`
- `apps/guardrail3/crates/app/ts/validate/eslint_parser.rs`
- `apps/guardrail3/crates/app/ts/validate/eslint_rule_infra.rs`
- plugin/package portions of `apps/guardrail3/crates/app/ts/validate/package_deps.rs`

Current source of truth:

- this file for family planning/status
- `.plans/todo/checks/ts/eslint.md` as the detailed family ledger until the cutover is complete

Current state:

- ESLint logic is substantial but still grouped under the old validator layout

Rule inventory:

- `T1` — ESLint config exists.
  - What it should do: require an `eslint.config.*` file at the owning root.
  - What it is for: this is the base lint policy surface; nothing else matters if the config is missing.
- `T2` — `max-lines` is configured to the required baseline.
  - What it should do: require `max-lines` at the expected value.
  - What it is for: keep file-size pressure explicit in the lint layer.
- `T3` — `max-lines-per-function` is configured to the required baseline.
  - What it should do: require `max-lines-per-function` at the expected value.
  - What it is for: keep function complexity pressure explicit in the lint layer.
- `T4` — `complexity` is configured to the required baseline.
  - What it should do: require the baseline complexity rule/value.
  - What it is for: catch excessive branching and local complexity.
- `T5` — `no-restricted-imports` is present.
  - What it should do: require restricted-import policy in ESLint config.
  - What it is for: this is a generic lint-level import restriction surface, separate from TS hexarch zone ownership.
- `T6` — boundary enforcement is configured.
  - What it should do: detect whether `eslint-plugin-boundaries` or equivalent boundary enforcement is active.
  - What it is for: this is the lint-layer prerequisite for import-zone enforcement, even though the actual zone semantics belong with `ts/hexarch`.
- `T7` — non-test relaxed rules are inventoried.
  - What it should do: inventory non-test rules set to `off` or `warn`.
  - What it is for: surface local lint relaxations and make them reviewable.
- `T8` — file-specific overrides are inventoried.
  - What it should do: inventory `files:` / override blocks.
  - What it is for: scoped lint exceptions need visibility because they often hide the real policy surface.
- `T40`..`T48` — required baseline rules are present.
  - What they should do: require the current core rule set:
    - `no-floating-promises`
    - `no-explicit-any`
    - `no-console`
    - `eqeqeq`
    - `no-restricted-globals`
    - `no-cycle`
    - `max-dependencies`
    - `explicit-function-return-type`
    - `strict-boolean-expressions`
  - What they are for: define the minimum generic ESLint/TypeScript rule baseline.
- `T49` — test-file relaxations are inventoried.
  - What it should do: inventory test-specific override sections.
  - What it is for: test relaxations are legitimate but should stay narrow and visible.
- `T50` — route wrapper enforcement exists.
  - What it should do: require lint-level enforcement of canonical route-wrapper usage where that is part of the app contract.
  - What it is for: this keeps route entrypoints consistent and auditable.
- `T51` — direct `process.env` access is banned at the lint layer.
  - What it should do: require ESLint restrictions against direct `process.env`.
  - What it is for: configuration access should go through a central audited surface.
- `T60`..`T83` — advanced baseline rules are present.
  - What they should do: require the broader modern TS/ESLint baseline, including rules such as `no-misused-promises`, `await-thenable`, `consistent-type-imports`, `no-non-null-assertion`, `no-unused-vars`, `no-unsafe-*`, `explicit-module-boundary-types`, `promise-function-async`, `prefer-nullish-coalescing`, `prefer-optional-chain`, and related safety/style rules.
  - What they are for: these carry the deeper lint contract beyond the small top-level baseline.
- `T-ESLP-01` — unicorn preset import exists.
  - What it should do: require the unicorn flat-config import.
  - What it is for: this keeps the unicorn rule family actually wired, not just installed.
- `T-ESLP-02` — required unicorn disabled-rules set is present.
  - What it should do: require the expected explicit unicorn rule disables.
  - What it is for: this encodes the project’s chosen exception surface against the unicorn preset.
- `T-ESLP-03` — required unicorn extra rules are present.
  - What it should do: require the project’s extra unicorn rules beyond the preset.
  - What it is for: this captures the intentional stronger lint surface.
- `T-ESLP-04` — regexp preset import exists.
  - What it should do: require the regexp flat-config import.
  - What it is for: this wires regexp lint policy rather than relying on package presence alone.
- `T-ESLP-05` — regexp extra rules are present.
  - What it should do: require the extra regexp rules beyond the preset.
  - What it is for: this strengthens the parser/validation safety surface.
- `T-ESLP-06` — required sonarjs rules are present.
  - What it should do: require the curated sonarjs rule set.
  - What it is for: this carries cognitive-complexity and logic-smell pressure into ESLint.
- `T-ESLP-07` — jsx-a11y strict config exists for content-profile roots.
  - What it should do: require strict jsx-a11y configuration where content/web UI surfaces are in scope.
  - What it is for: this makes accessibility linting part of the frontend/content contract.
- `T-ESLP-08` — `jsx-a11y/control-has-associated-label` is present.
  - What it should do: require the explicit control-label rule.
  - What it is for: this carries a specific high-value accessibility guarantee.
- `T-ESLP-09` — required React extra rules are present.
  - What it should do: require the curated React rule set.
  - What it is for: this encodes project-specific React quality expectations.
- `T-ESLP-10` — required built-in ESLint/TS rules are present and sufficiently configured.
  - What it should do: require the curated built-in rule group and warn when crucial option shapes are missing.
  - What it is for: some rules are only meaningful with the right option structure, not just the rule name.
- `T-ESLP-11` — test relaxation section exists and carries the expected test-rule set.
  - What it should do: require a real test override section and the expected disabled test rules.
  - What it is for: this makes test-only lint relaxation explicit instead of accidental.
- `T-ESLP-12` — tailwind-ban plugin and deny list exist where content-profile rules require them.
  - What it should do: require the plugin and its deny-list configuration.
  - What it is for: this enforces semantic design-token usage over arbitrary Tailwind escape hatches.
- `T-ESLP-13` — `strictTypeChecked` preset exists.
  - What it should do: require `tseslint.configs.strictTypeChecked`.
  - What it is for: this is the deep type-aware lint baseline.
- `T-ESLP-14` — `stylisticTypeChecked` preset exists.
  - What it should do: require `tseslint.configs.stylisticTypeChecked`.
  - What it is for: this carries the style-oriented type-aware lint baseline.
- `T-ESLP-15` — RegExp is banned appropriately.
  - What it should do: require lint bans on raw RegExp usage where the project expects structured parsing/validation instead.
  - What it is for: this enforces the “no ad hoc regex parsing” policy at the lint layer.

Current code mapping:

- `apps/guardrail3/crates/app/ts/validate/eslint_check.rs`
  - owns `T1`, `T2`..`T8`, `T40`..`T51`, `T60`..`T83`, and `T-ESLP-13`..`T-ESLP-15`
- `apps/guardrail3/crates/app/ts/validate/eslint_plugin_checks.rs`
  - owns `T-ESLP-01`..`T-ESLP-12`
- `apps/guardrail3/crates/app/ts/validate/eslint_parser.rs`
  - parses the flat config into `EslintConfig`
- `apps/guardrail3/crates/app/ts/validate/eslint_rule_infra.rs`
  - provides rule presence/value checking infrastructure
- mixed package-presence surfaces currently live in `apps/guardrail3/crates/app/ts/validate/package_deps.rs`
  - genuinely ESLint-owned package presence includes at least `eslint`, `typescript-eslint`, `eslint-plugin-unicorn`, `eslint-plugin-regexp`, `eslint-plugin-sonarjs`, `eslint-plugin-import-x`, `eslint-import-resolver-typescript`, `eslint-plugin-boundaries`, and content-profile plugins like `eslint-plugin-jsx-a11y`
  - but `knip`, `jscpd`, and `only-allow` are current mixed spillover and should not silently become part of the long-term TS-ESLINT contract

Current doc/code reconciliation notes:

- the old ledger in `.plans/todo/checks/ts/eslint.md` is directionally right but under-specifies the real live rule inventory
- the current implementation is split across config parsing, config rule checks, plugin group checks, and package presence checks; this file should keep those boundaries explicit instead of pretending the family is already cleanly separated
- `T6` is the main intentional boundary ambiguity: it is an ESLint-owned config-surface rule that exists to support `ts/hexarch`, but it should not absorb actual architecture semantics
- the live rule surface is much richer than the old ledger; this family is one of the main places where the by-family plan must be code-led rather than doc-led

Historical/supplemental references:

- `.plans/todo/checks/ts/eslint.md`
- `.plans/by_file/ts/eslint-config-mjs.md`

Rule inventory:

- `T1` — ESLint config exists.
  What it should do: require at least one `eslint.config.*` surface and inventory success when found.
  What it is for: ensure ESLint is actually present as an enforceable tool surface.
- `T2` — `max-lines` configured to the policy threshold.
  What it should do: require `max-lines` at the approved limit.
  What it is for: keep file size bounded at the lint layer.
- `T3` — `max-lines-per-function` configured to the policy threshold.
  What it should do: require `max-lines-per-function` at the approved limit.
  What it is for: constrain local function sprawl.
- `T4` — `complexity` configured to the policy threshold.
  What it should do: require the `complexity` rule at the approved threshold.
  What it is for: cap cyclomatic complexity through lint policy.
- `T5` — `no-restricted-imports` exists.
  What it should do: require restricted-import policy in ESLint.
  What it is for: centralize import restrictions that are not specific to TS architecture zones.
- `T6` — import-boundary enforcement is configured.
  What it should do: require `eslint-plugin-boundaries` or equivalent boundary enforcement wiring.
  What it is for: ensure import-boundary lint infrastructure exists at all.
- `T7` — relaxed rules are inventoried.
  What it should do: inventory non-test rules set to `warn` or `off`.
  What it is for: make lint relaxations visible and reviewable.
- `T8` — file-specific overrides are inventoried.
  What it should do: inventory file-pattern-based override blocks.
  What it is for: surface scoped relaxations and keep them narrow.
- `T40` — `no-floating-promises` required.
  What it should do: require the rule at error severity.
  What it is for: prevent silently dropped async failures.
- `T41` — `no-explicit-any` required.
  What it should do: require the rule at error severity.
  What it is for: keep unsound type escapes explicit and exceptional.
- `T42` — `no-console` required.
  What it should do: require the rule at error severity.
  What it is for: block casual console logging in shipped code.
- `T43` — `eqeqeq` required.
  What it should do: require the rule at error severity.
  What it is for: prevent coercive equality surprises.
- `T44` — `no-restricted-globals` required.
  What it should do: require the rule at error severity.
  What it is for: centralize bans on dangerous globals.
- `T45` — `no-cycle` required.
  What it should do: require the rule at error severity.
  What it is for: prevent dependency cycles in TS module graphs.
- `T46` — `max-dependencies` required.
  What it should do: require the rule at error severity.
  What it is for: flag dependency fan-in/fan-out that is getting too broad.
- `T47` — `explicit-function-return-type` required.
  What it should do: require the rule at error severity.
  What it is for: keep public/local function contracts explicit where policy demands it.
- `T48` — `strict-boolean-expressions` required.
  What it should do: require the rule at error severity.
  What it is for: prevent truthiness-driven type bugs.
- `T49` — test-file relaxation blocks are inventoried.
  What it should do: inventory test-specific override blocks.
  What it is for: keep test-only lint relaxations visible and scoped.
- `T50` — route wrapper enforcement exists.
  What it should do: require lint patterns that enforce canonical route wrappers such as `withBody` or `withRoute`.
  What it is for: make route validation and error handling consistent.
- `T51` — direct `process.env` access is banned in ESLint.
  What it should do: require a lint restriction on `process.env`.
  What it is for: force configuration through a centralized env surface.
- `T60` — `no-misused-promises` required.
  What it should do: require the rule at error severity.
  What it is for: prevent async misuse in sync contexts.
- `T61` — `await-thenable` required.
  What it should do: require the rule at error severity.
  What it is for: catch bogus awaits and logic mistakes.
- `T62` — `consistent-type-imports` required.
  What it should do: require the rule at error severity.
  What it is for: keep type-only imports explicit and hygienic.
- `T63` — `no-non-null-assertion` required.
  What it should do: require the rule at error severity.
  What it is for: block unsound null escapes.
- `T64` — `switch-exhaustiveness-check` required.
  What it should do: require the rule at error severity.
  What it is for: ensure discriminated unions are handled exhaustively.
- `T65` — `no-unused-vars` required.
  What it should do: require the rule at error severity.
  What it is for: keep dead local bindings visible.
- `T66` — `require-await` required.
  What it should do: require the rule at error severity.
  What it is for: block fake async functions.
- `T67` — `no-param-reassign` required.
  What it should do: require the rule at error severity.
  What it is for: avoid mutation-heavy local logic.
- `T68` — `no-unsafe-assignment` required.
  What it should do: require the rule at error severity.
  What it is for: stop `any` and unsound values from contaminating assignments.
- `T69` — `no-unsafe-member-access` required.
  What it should do: require the rule at error severity.
  What it is for: stop unsound property access on untyped values.
- `T70` — `no-unsafe-call` required.
  What it should do: require the rule at error severity.
  What it is for: stop untyped/dangerous calls.
- `T71` — `no-unsafe-return` required.
  What it should do: require the rule at error severity.
  What it is for: prevent unsound values escaping function boundaries.
- `T72` — `no-unsafe-argument` required.
  What it should do: require the rule at error severity.
  What it is for: prevent passing unsound values into typed APIs.
- `T73` — `explicit-module-boundary-types` required.
  What it should do: require the rule at error severity.
  What it is for: keep module entry/exit types explicit.
- `T74` — `promise-function-async` required.
  What it should do: require the rule at error severity.
  What it is for: align promise-returning functions with explicit async semantics.
- `T75` — `consistent-type-exports` required.
  What it should do: require the rule at error severity.
  What it is for: keep type-only exports explicit and consistent.
- `T76` — `consistent-type-definitions` required.
  What it should do: require the rule at error severity.
  What it is for: keep type-definition style consistent.
- `T77` — `no-unnecessary-condition` required.
  What it should do: require the rule at error severity.
  What it is for: catch defensive branches that are never meaningful.
- `T78` — `prefer-nullish-coalescing` required.
  What it should do: require the rule at error severity.
  What it is for: favor safer defaulting semantics over `||`.
- `T79` — `prefer-optional-chain` required.
  What it should do: require the rule at error severity.
  What it is for: prefer clear null-safe access patterns.
- `T80` — `no-deprecated` required.
  What it should do: require the rule at error severity.
  What it is for: keep deprecated APIs from creeping back in.
- `T81` — `restrict-template-expressions` required.
  What it should do: require the rule at error severity.
  What it is for: prevent accidental stringification of unsafe values.
- `T82` — `no-throw-literal` required.
  What it should do: require the rule at error severity.
  What it is for: keep thrown values structured and debuggable.
- `T83` — `no-empty` required.
  What it should do: require the rule at error severity.
  What it is for: block silent empty blocks.
- `T-ESLP-01` — unicorn flat config import exists.
  What it should do: require the unicorn plugin/config import wiring.
  What it is for: make the unicorn rule surface structurally present.
- `T-ESLP-02` — baseline unicorn disabled-rule set exists.
  What it should do: require the expected `unicorn/*` rules to be explicitly disabled.
  What it is for: codify the project’s chosen exceptions to the unicorn baseline.
- `T-ESLP-03` — additional unicorn rules exist.
  What it should do: require the expected extra unicorn rules.
  What it is for: keep the preferred stricter unicorn policy intact.
- `T-ESLP-04` — regexp config import exists.
  What it should do: require regexp plugin/config import wiring.
  What it is for: make regex safety/style enforcement structurally present.
- `T-ESLP-05` — extra regexp rules exist.
  What it should do: require the expected regexp rule set.
  What it is for: strengthen regex correctness beyond the base import.
- `T-ESLP-06` — selected sonarjs rules exist.
  What it should do: require the chosen `sonarjs/*` rule subset.
  What it is for: cover complexity and suspicious-pattern checks not owned elsewhere.
- `T-ESLP-07` — jsx-a11y strict config exists for content profile.
  What it should do: require strict jsx-a11y config wiring when content-profile checks are in scope.
  What it is for: enforce frontend accessibility linting.
- `T-ESLP-08` — `jsx-a11y/control-has-associated-label` exists.
  What it should do: require that concrete accessibility rule.
  What it is for: catch unlabeled interactive controls.
- `T-ESLP-09` — extra React rules exist.
  What it should do: require the chosen `react/*` rule subset.
  What it is for: enforce the project’s stricter React/JSX hygiene policy.
- `T-ESLP-10` — baseline built-in ESLint/TS rules exist, plus required option shapes.
  What it should do: require the built-in/core rules listed in the current runtime and warn when key options like naming-convention selectors or jsx-no-leaked-render strategies are missing.
  What it is for: keep the baseline lint bundle complete and not just nominally present.
- `T-ESLP-11` — test-file relaxation section exists and covers the expected rules.
  What it should do: require a test-override section and require the expected relaxed rules there.
  What it is for: make test relaxations explicit, narrow, and consistent.
- `T-ESLP-12` — tailwind-ban plugin/rule exists for content profile.
  What it should do: require the tailwind-ban plugin and warn if its deny-list policy is absent.
  What it is for: enforce semantic design-token use over arbitrary utility sprawl.
- `T-ESLP-13` — `tseslint.configs.strictTypeChecked` exists.
  What it should do: require the strict type-checked preset.
  What it is for: make type-aware linting part of the baseline.
- `T-ESLP-14` — `tseslint.configs.stylisticTypeChecked` exists.
  What it should do: require the stylistic type-checked preset.
  What it is for: keep stylistic TS rules aligned with the type-aware preset baseline.
- `T-ESLP-15` — RegExp is banned or tightly restricted in ESLint.
  What it should do: require the config patterns that ban or restrict unsafe regex use.
  What it is for: push parsing/validation toward structured approaches instead of ad hoc regex.

Current code mapping:

- `apps/guardrail3/crates/app/ts/validate/eslint_check.rs`
  - owns `T1`, `T2`-`T8`, `T40`-`T51`, `T60`-`T83`, `T-ESLP-13`, `T-ESLP-14`, `T-ESLP-15`.
- `apps/guardrail3/crates/app/ts/validate/eslint_plugin_checks.rs`
  - owns `T-ESLP-01` through `T-ESLP-12`.
- `apps/guardrail3/crates/app/ts/validate/eslint_parser.rs`
  - provides the parsed config surface these checks depend on.
- `apps/guardrail3/crates/app/ts/validate/eslint_rule_infra.rs`
  - provides generic presence/value helpers for many rule checks.
- `apps/guardrail3/crates/app/ts/validate/package_deps.rs`
  - still owns package presence for some ESLint-related plugins; that mixed ownership needs later cleanup.

Current doc/code reconciliation notes:

- the old ledger in `.plans/todo/checks/ts/eslint.md` captures family scope correctly, but it massively understates the live rule inventory
- several current rule IDs (`T40`-`T51`, `T60`-`T83`, `T-ESLP-*`) are only discoverable from code today, not from the old family doc
- the current code still mixes generic ESLint baseline ownership with some architecture-adjacent checks like `T6` and route-wrapper enforcement; that split must be clarified against `ts/hexarch`
- content-profile checks (`T-ESLP-07`, `T-ESLP-08`, `T-ESLP-12`) are conditional and should stay explicit in the family summary

Next planning focus:

- separate baseline ESLint ownership from TS hexarch boundary policy
- split package-presence spillover cleanly out of `package_deps.rs` so TS-ESLINT owns only the ESLint/plugin package surface it actually needs
- decide exactly which current checks stay in `ts/eslint` versus move to `ts/hexarch` during the future TS family split
