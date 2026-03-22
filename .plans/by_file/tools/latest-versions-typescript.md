# Latest Stable Versions & Breaking Changes (March 2026)

Research date: 2026-03-19

## 1. ESLint

- **Latest:** v10.0.3 (2026-03-06)
- **Breaking changes (v10.0.0, 2026-02-06):**
  - Dropped Node.js < 20.19.0, v21.x, v23.x
  - Removed deprecated `LintMessage#nodeType` and `TestCaseError#type`
  - Dropped support for jiti < 2.2.0
  - Updated `eslint:recommended` config; removed `v10_*` and inactive `unstable_*` flags
  - `no-shadow-restricted-names` now reports `globalThis` by default
- **Action:** Review migration guide. Ensure Node >= 20.19.0. Update `eslint:recommended` usage.

## 2. typescript-eslint

- **Latest:** v8.57.1 (2026-03-17)
- **Still on v8.x** (no v9 released yet). Supports ESLint ^8.57.0 || ^9.0.0 || ^10.0.0.
- **Notable recent additions:**
  - `no-base-to-string`: new `checkUnknown` option
  - `prefer-promise-reject-errors`: new `allowTypeOrValueSpecifier` option
  - `no-useless-default-assignment`: reports unnecessary defaults in ternaries
- **v8 breaking changes recap (still relevant):**
  - Removed formatting/layout rules (use `eslint.style` instead)
  - Split `ban-types` into `no-empty-object-type`, `no-restricted-types`, `no-unsafe-function-type`, `no-wrapper-object-types`
  - `prefer-nullish-coalescing` default `ignoreConditionalTests` changed to `true`
- **Action:** Safe to stay on v8.x. Consider enabling `checkUnknown` on `no-base-to-string`.

## 3. TypeScript

- **Latest stable:** v5.9 (mid-2025). v6.0 RC announced 2026-03-06 (not yet GA).
- **v5.8 new compiler options:**
  - `--erasableSyntaxOnly` -- ensures only erasable syntax (compatible with Node's `--experimental-strip-types`)
  - `--module node18` -- stable flag for Node 18 users (disallows `require()` of ESM)
  - `--rewriteRelativeImportExtensions` -- rewrites .ts/.tsx imports to .js/.jsx
- **v5.9 new compiler options:**
  - `--strictInference` now enabled under `--strict` (catches unchecked generics/conditional types)
  - `tsc --init` defaults now include `noUncheckedIndexedAccess` and `exactOptionalPropertyTypes`
  - Decorator metadata (Stage 3) stable
  - `import defer` syntax support
- **v6.0 (RC, not yet stable):**
  - Last JS-based compiler release. Foundation for Go-based TypeScript 7.
  - Feature-stable, no new features expected before GA.
- **Action:** Upgrade to 5.9. Enable `--strictInference`, `noUncheckedIndexedAccess`, `exactOptionalPropertyTypes`. Wait for 6.0 GA before adopting.

## 4. Stylelint

- **Latest:** v17.4.0 (2026-02-27)
- **Breaking changes (v17.0.0):**
  - Removed CommonJS Node.js API (ESM only)
  - Removed `output` property from Node.js API resolved object
  - Dropped Node.js < 20.19.0
  - Removed GitHub formatter
  - Changed default fix mode to `strict`
  - Removed `resolveNestedSelectors` from `selector-class-pattern`
  - Removed `checkContextFunctionalPseudoClasses` from `selector-max-id`
- **Action:** Ensure ESM imports for Node API usage. Update Node to >= 20.19.0.

## 5. Prettier

- **Latest:** v3.8.1 (~2026-01)
- **No major version bump.** Still on v3.x.
- **v3.5 (2025-02):** New `objectWrap` option, `--experimental-operator-position` flag
- **v3.7 (2025-11):** Improved TypeScript/Flow class/interface formatting consistency
- **Action:** Safe upgrade. Review `objectWrap` option if relevant.

## 6. cspell

- **Latest:** v9.7.0 (2026-02-25)
- **No major breaking changes noted.** Steady incremental releases.
- **Action:** Upgrade to 9.7.0.

## 7. pnpm

- **Latest:** v10.32.1 (2026-03-11)
- **Major security features (v10.x):**
  - `preinstall`/`postinstall` scripts **no longer run by default**
  - `allowBuilds` (v10.26+): granular per-package script permissions
  - `minimumReleaseAge`: blocks packages younger than N hours (e.g., 24h)
  - `trustPolicy: no-downgrade`: prevents installing packages with decreased trust level
  - Stricter defaults for git-hosted dependencies
  - Registry/auth in `.npmrc` (INI); pnpm-specific settings in `pnpm-workspace.yaml` (YAML)
- **Action:** Upgrade to 10.x. Configure `allowBuilds` for packages needing install scripts. Consider `minimumReleaseAge` and `trustPolicy`.

## 8. eslint-plugin-unicorn

- **Latest:** v63.0.0 (~2026-02)
- **100+ rules.** Frequent major version bumps (rule additions/removals).
- **Action:** Upgrade and run with `recommended` config. Review any new rules added since last version.

## 9. eslint-plugin-sonarjs

- **Latest:** v4.0.2 (2026-03-12)
- **Breaking changes (v2.0+):** Moved into SonarJS monorepo. Now exports ALL SonarJS rules (not a subset).
- **v3.0.0:** Another major version with repo reorganization.
- **v4.0.0:** Latest major. Compatible with ESLint v9 and v10.
- **Known issues:** v2.x had crashes on ESLint v9 due to `context.getScope` removal. Fixed in later versions.
- **Action:** Upgrade to 4.0.2. Test thoroughly -- the plugin has had stability issues across major versions.

## 10. eslint-plugin-regexp

- **Latest:** v3.0.0 (~2026-02)
- **Major version bump from v2.x.** 80 rules for regex mistakes and style violations.
- **Action:** Upgrade to 3.0.0. Review breaking changes in v3 migration guide.

---

## Summary Table

| Tool                     | Version   | Major Since | Node Req     |
|--------------------------|-----------|-------------|--------------|
| ESLint                   | 10.0.3    | v10 (Feb 26)| >= 20.19.0   |
| typescript-eslint        | 8.57.1    | v8          | --           |
| TypeScript               | 5.9 (6.0 RC) | v5.9 (mid-25) | --        |
| Stylelint                | 17.4.0    | v17 (2026)  | >= 20.19.0   |
| Prettier                 | 3.8.1     | v3          | --           |
| cspell                   | 9.7.0     | v9          | --           |
| pnpm                     | 10.32.1   | v10         | --           |
| eslint-plugin-unicorn    | 63.0.0    | frequent    | --           |
| eslint-plugin-sonarjs    | 4.0.2     | v4 (2026)   | --           |
| eslint-plugin-regexp     | 3.0.0     | v3 (2026)   | --           |

## Key Guardrail-Relevant Takeaways

1. **Node.js >= 20.19.0 is now required** by ESLint 10 and Stylelint 17.
2. **TypeScript 5.9's `--strictInference`** under `--strict` is the biggest new type-safety win. Also `noUncheckedIndexedAccess` and `exactOptionalPropertyTypes` are now defaults.
3. **pnpm 10's security defaults** (no auto-scripts, `allowBuilds`, `minimumReleaseAge`, `trustPolicy`) are significant supply-chain hardening.
4. **eslint-plugin-sonarjs v4** is a full rewrite from the v1.x days -- treat as a new plugin when configuring.
5. **Stylelint 17 is ESM-only** -- no more CommonJS API.
