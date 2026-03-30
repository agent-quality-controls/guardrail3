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
- the live rule surface is much richer than the old ledger; this family is one of the main places where the by-family plan must be code-led rather than doc-led

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
  What it is for: stop unsound invocation of untyped values.
- `T71` — `no-unsafe-return` required.
  What it should do: require the rule at error severity.
  What it is for: stop unsafe values escaping function boundaries.
- `T72` — `no-unsafe-argument` required.
  What it should do: require the rule at error severity.
  What it is for: stop unsafe values from flowing into typed APIs.
- `T73` — `explicit-module-boundary-types` required.
  What it should do: require the rule at error severity.
  What it is for: keep exported/module boundary types explicit.
- `T74` — `promise-function-async` required.
  What it should do: require the rule at error severity.
  What it is for: keep promise-returning functions explicitly async.
- `T75` — `consistent-type-exports` required.
  What it should do: require the rule at error severity.
  What it is for: normalize type-only export style.
- `T76` — `consistent-type-definitions` required.
  What it should do: require the rule at error severity.
  What it is for: keep TS type declaration style consistent.
- `T77` — `no-unnecessary-condition` required.
  What it should do: require the rule at error severity.
  What it is for: catch dead or redundant conditionals.
- `T78` — `prefer-nullish-coalescing` required.
  What it should do: require the rule at error severity.
  What it is for: prefer semantically precise fallback behavior.
- `T79` — `prefer-optional-chain` required.
  What it should do: require the rule at error severity.
  What it is for: prefer concise and safe nullable property access.
- `T80` — `no-deprecated` required.
  What it should do: require the rule at error severity.
  What it is for: stop drift onto deprecated APIs.
- `T81` — `restrict-template-expressions` required.
  What it should do: require the rule at error severity.
  What it is for: prevent unsafe or sloppy string interpolation.
- `T82` — `no-throw-literal` required.
  What it should do: require the rule at error severity.
  What it is for: force throwable error objects instead of bare literals.
- `T83` — `no-empty` required.
  What it should do: require the rule at error severity.
  What it is for: catch swallowed/empty control-flow blocks.
- `T-ESLP-01` — unicorn config import present.
  What it should do: ensure the unicorn flat config is imported/configured.
  What it is for: establish the unicorn ruleset baseline.
- `T-ESLP-02` — expected unicorn-disabled rule set is present.
  What it should do: ensure the approved set of unicorn rule relaxations is explicit.
  What it is for: document intentional deviations from the plugin baseline.
- `T-ESLP-03` — expected extra unicorn rules are present.
  What it should do: ensure the chosen extra unicorn hardening rules are enabled.
  What it is for: strengthen code hygiene beyond the default preset.
- `T-ESLP-04` — regexp config import present.
  What it should do: ensure regexp plugin config is imported.
  What it is for: make regex policy enforceable at all.
- `T-ESLP-05` — expected extra regexp rules are present.
  What it should do: ensure the chosen regexp hardening rules are enabled.
  What it is for: steer regex usage toward safer structured patterns.
- `T-ESLP-06` — expected sonarjs rules are present.
  What it should do: ensure the chosen sonarjs complexity/duplication rules are enabled.
  What it is for: add high-signal code-smell coverage.
- `T-ESLP-07` — jsx-a11y strict config present.
  What it should do: ensure the strict accessibility preset is configured when the content/frontend profile needs it.
  What it is for: make accessibility baseline explicit.
- `T-ESLP-08` — `jsx-a11y/control-has-associated-label` present.
  What it should do: ensure a key accessibility rule is enabled.
  What it is for: protect labeled-control accessibility.
- `T-ESLP-09` — expected extra React rules are present.
  What it should do: ensure the chosen React rule bundle is enabled.
  What it is for: tighten UI correctness beyond the default React preset.
- `T-ESLP-10` — expected built-in ESLint/TS rule bundle is present.
  What it should do: ensure a curated set of built-in rules is present.
  What it is for: maintain a minimum baseline independent of plugin presets.
- `T-ESLP-11` — expected test-file relaxation section is present.
  What it should do: ensure the approved rules are relaxed in test overrides and only there.
  What it is for: keep test deviations explicit and controlled.
- `T-ESLP-12` — tailwind-ban plugin and rule present.
  What it should do: ensure tailwind-ban is installed/configured when needed.
  What it is for: enforce CSS architecture/content policy around Tailwind use.
- `T-ESLP-13` — `strictTypeChecked` preset present.
  What it should do: ensure `tseslint.configs.strictTypeChecked` is configured.
  What it is for: make the type-aware strict baseline mandatory.
- `T-ESLP-14` — `stylisticTypeChecked` preset present.
  What it should do: ensure `tseslint.configs.stylisticTypeChecked` is configured.
  What it is for: make the stylistic TS baseline mandatory.
- `T-ESLP-15` — RegExp bans present.
  What it should do: ensure regex/RegExp use is banned or tightly restricted via ESLint.
  What it is for: push validation/parsing toward structured tools.

Current implementation mapping:

- `apps/guardrail3/crates/app/ts/validate/eslint_check.rs`
  - owns `T1` through `T8`, `T40` through `T51`, and `T60` through `T83`.
- `apps/guardrail3/crates/app/ts/validate/eslint_plugin_checks.rs`
  - owns `T-ESLP-01` through `T-ESLP-12`.
- `apps/guardrail3/crates/app/ts/validate/eslint_parser.rs`
  - provides the parsed config surface these checks depend on.
- `apps/guardrail3/crates/app/ts/validate/eslint_rule_infra.rs`
  - provides generic presence/value helpers for many rule checks.
- `apps/guardrail3/crates/app/ts/validate/package_deps.rs`
  - still owns package presence for some ESLint-related plugins; that mixed ownership needs later cleanup.

Implementation status:

- all listed rule ids above are implemented in some form today

Current doc/code reconciliation notes:

- the old ledger in `.plans/todo/checks/ts/eslint.md` captures family scope correctly, but it massively understates the live rule inventory
- several current rule IDs (`T40`-`T51`, `T60`-`T83`, `T-ESLP-*`) are only discoverable from code today, not from the old family doc
- the current code still mixes generic ESLint baseline ownership with some architecture-adjacent checks like `T6` and route-wrapper enforcement; that split must be clarified against `ts/hexarch`
- content-profile checks (`T-ESLP-07`, `T-ESLP-08`, `T-ESLP-12`) are conditional and should stay explicit in the family summary

Historical/supplemental references:

- `.plans/todo/checks/ts/eslint.md`
- `.plans/by_file/ts/eslint-config-mjs.md`

Next planning focus:

- separate baseline ESLint ownership from TS hexarch boundary policy
- decide exactly which current checks stay in `ts/eslint` versus move to `ts/hexarch` during the future TS family split
- eventually split plugin package presence from config-file rule enforcement so this family can migrate cleanly into a dedicated TS family runtime
