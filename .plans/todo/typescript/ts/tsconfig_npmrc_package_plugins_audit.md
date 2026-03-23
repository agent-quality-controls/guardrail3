# Adversarial Audit: tsconfig / npmrc / package / plugins checkers

Audited: 2026-03-21. Covers plan files + implementation source.

---

## 1. TSCONFIG (19 rules)

### BUG: JSONC not supported — tsconfig parsing silently fails on valid files

`check_single_tsconfig` uses `serde_json::from_str` which rejects comments and trailing commas. But `tsconfig.json` is JSONC by spec — the TypeScript compiler itself accepts `//` comments and trailing commas. Any project with comments in tsconfig (extremely common, especially for generated configs and Next.js defaults) gets a parse error and all 18 subsequent checks are skipped. This is not a missing rule — it is a correctness bug in TS-TSCONFIG-01. The error message even says "use `jsonc` format if comments are needed" as if that is a different file, but tsconfig.json IS jsonc.

**Fix:** Strip comments + trailing commas before parsing, or use a JSONC parser. The CLAUDE.md already mandates "strip comments first for JSONC" under structured parsing rules.

**Severity: Critical.** This silently disables the entire tsconfig checker for a large fraction of real projects.

### MISSING: `extends` chain — child tsconfig settings are invisible

The checker validates only the file it reads. But most monorepo projects have:
```
tsconfig.base.json          <-- strict: true, etc.
apps/web/tsconfig.json      <-- { "extends": "../tsconfig.base.json", "compilerOptions": { "jsx": "react-jsx" } }
```

The checker already prefers `tsconfig.base.json` over `tsconfig.json`, which is the right heuristic. But it does NOT:
1. Warn if a child tsconfig overrides a base setting to a weaker value (e.g., `strict: false` in child).
2. Follow `extends` chains to verify the effective merged config.

**Proposed rule — TS-TSCONFIG-20: `extends` override weakening.** If a child tsconfig.json has `"extends"` pointing to a base that we validated, and the child explicitly sets a checked key to a weaker value, emit an error. This catches the common pattern where an agent adds `"strict": false` to an app tsconfig to "fix" type errors.

**Verdict: Add as a new rule.** Without it, a single child tsconfig can silently undo every guardrail in the base config.

### MISSING: `skipLibCheck` warning

`skipLibCheck: true` is in the `known_keys` list (line 476) so it is silently accepted. This is defensible for build speed, but `skipLibCheck` hides type errors in `.d.ts` files, including errors in your own library packages within a monorepo. At minimum, it should be inventory (Info) when true, not invisible.

**Proposed rule — TS-TSCONFIG-21: `skipLibCheck` inventory.** When `skipLibCheck: true`, emit Info inventory noting the tradeoff. Not an error — just visibility.

**Verdict: Low priority.** Acceptable as-is since it is in the known list, but adding visibility aligns with the "total visibility" principle.

### NOT MISSING: `jsx`, `paths`, `baseUrl`

These are project-specific settings with no universal correct value. `jsx` depends on whether React is used and which transform. `paths` and `baseUrl` depend on project structure. They are correctly in the `known_keys` list and excluded from validation. No rule needed.

### NOT MISSING: `resolveJsonModule`, `verbatimModuleSyntax`

Already in `known_keys`. These are project-dependent. Correct as-is.

---

## 2. NPMRC (5 rules)

### ALREADY HANDLED: Inline comments and quoted values

The parser (line 66) correctly strips `#` and `;` comment lines. Quoted values are stripped (lines 73-76: strips surrounding `"`). The plan's 5 rules are complete for the parsing concerns raised.

### MISSING: Inline comments on value lines

The parser does NOT strip inline comments. A line like:
```
strict-peer-dependencies=false  # TODO: re-enable later
```
would parse as value `false  # TODO: re-enable later`, which would fail the `== "false"` check and be reported as wrong value (T13) with a confusing message showing the comment as part of the value. This is a parsing bug, not a missing rule.

**Fix:** After splitting on `=`, strip everything from `#` or `;` onward in the value portion (being careful about `#` inside quoted values, though .npmrc rarely uses those).

**Severity: Medium.** Uncommon in practice but produces a confusing diagnostic when it occurs.

### NOT MISSING: `node-linker`

`node-linker` controls how pnpm installs packages (hoisted vs symlinked vs isolated). The default `node-linker=hoisted` is required for most bundlers and frameworks. Changing it to `isolated` or `pnp` would break many projects. However, the default is already correct without setting it, and the `extra settings` check (T14) would flag it if someone explicitly sets it. No dedicated rule needed.

### NOT MISSING: `hoist-pattern`

`public-hoist-pattern=""` is already checked (line 130). The separate `hoist-pattern` setting is less common and the extra-settings inventory (T14) would catch it. No dedicated rule needed.

### MISSING: BOM handling inconsistency

The parser strips BOM (line 63: `strip_prefix('\u{FEFF}')`). Good. But it uses `unwrap_or` which is fine here. No issue.

---

## 3. PACKAGE (12 rules)

### BUG: Banned deps list duplicated between root check and per-app check

`check_package_json` (lines 143-166) and `check_banned_deps_in_package` (lines 467-489) contain identical copies of the `banned_deps` and `banned_prefixes` arrays. If one is updated and the other isn't, apps get different ban lists than root. This violates DRY and is a latent bug.

**Fix:** Extract to a shared const.

**Severity: Medium.** Not a missing rule, but a code quality issue that will cause a rule gap when the ban list is next updated.

### MISSING: `type: "module"` enforcement

Modern Node.js projects should use `"type": "module"` in package.json to enable ESM by default. Without it, `.js` files are treated as CommonJS, which conflicts with `"module": "esnext"` in tsconfig (TS-TSCONFIG-14). The checker enforces ESM in tsconfig but doesn't check the corresponding package.json field.

**Proposed rule — TS-PACKAGE-13: `type` field must be `"module"`.** Error if missing or set to `"commonjs"`. This completes the ESM enforcement chain: tsconfig says ESM, package.json says ESM, .npmrc says no hoisting shenanigans.

**Verdict: Add.** This is a real gap — enforcing `module: esnext` in tsconfig without `type: module` in package.json leads to confusing runtime behavior where Node.js treats output files as CJS.

### MISSING: Workspace member package.json validation

The checker validates root package.json fully and per-app package.jsons only for banned deps (T17). But workspace member package.jsons can have their own problems:
- Missing `private: true` (accidental publish of internal package)
- Missing `engines` (deployed to wrong Node version)
- Having `dependencies` that should be `peerDependencies` for shared packages

**Proposed rule — TS-PACKAGE-14: Workspace member `private` field.** Any non-root package.json that is NOT in the `pnpm.publishConfig` allowlist should have `"private": true`.

**Verdict: Medium priority.** The most dangerous gap is accidental npm publish of internal packages.

### MISSING: `packageManager` format validation

TS-PACKAGE-04 checks that `packageManager` exists but not its format. A value like `"packageManager": "pnpm"` (without version) or `"packageManager": "npm@10.0.0"` (wrong manager) would pass. Corepack requires the format `pnpm@X.Y.Z`.

**Proposed rule — TS-PACKAGE-15: `packageManager` format.** Validate that the value matches `pnpm@<semver>`.

**Verdict: Add.** Simple check, catches real misconfiguration.

### NOT MISSING: `peerDependencies` check

peerDependencies are project-specific. No universal rule applies. Correctly omitted.

### NOT MISSING: Script content validation

The checker validates script existence (lint, typecheck, knip, preinstall, prepare) but not content. This is correct — script content is project-specific (`tsc --noEmit` vs `tsc -b --noEmit`, `eslint .` vs `eslint src/`). The one exception is `preinstall` which checks for `only-allow pnpm` — and that IS content-validated already (T55, line 232).

### MISSING: `engines.node` format validation

TS-PACKAGE-07 checks that `engines` exists, TS-PACKAGE-12 checks that `engines.pnpm` exists. But neither validates that `engines.node` exists specifically, nor that the value is a valid semver range. A value like `"node": "latest"` would pass.

**Proposed rule — TS-PACKAGE-16: `engines.node` must exist with semver range.** When `engines` exists, `node` key must be present and match a semver range pattern (`>=X`, `^X.Y`, etc.).

**Verdict: Low-medium.** The `engines` existence check already covers the common case of missing the field entirely.

---

## 4. PLUGINS (25 rules)

### BUG: Only checks `devDependencies`, not `dependencies`

`check_dev_dep` (line 14-18) only looks in `devDependencies`. But some packages (notably `eslint`, `typescript`, and framework plugins) are sometimes in `dependencies` instead. The checker would report them as missing even though they're installed.

More importantly: some of these packages should ONLY be in devDependencies and having them in `dependencies` is itself a bug (ships to production). The checker doesn't warn about this.

**Proposed two-part fix:**
1. When checking presence, look in both `dependencies` and `devDependencies`.
2. When found in `dependencies` instead of `devDependencies`, emit a warning: "X is in dependencies but should be in devDependencies — it ships to production unnecessarily."

**Severity: Medium.** False negatives (reporting missing when it's in deps) plus missed misplacement warnings.

### MISSING: `@next/eslint-plugin-next` for Next.js projects

Next.js projects need `@next/eslint-plugin-next` for framework-specific lint rules (no `<img>` without `next/image`, no `<a>` without `next/link`, etc.). The content profile already checks for other framework-specific packages (jsx-a11y, stylelint-config-tailwindcss) but not Next.js ESLint integration.

**Proposed rule — TS-PLUG-20: `@next/eslint-plugin-next` in devDeps (content profile, when next.js detected).** Detection: `next` is in `dependencies`. If so, require `@next/eslint-plugin-next` in devDeps.

**Verdict: Add for content profile.** Next.js is a primary target framework per the tech stack (CLAUDE.md says Next.js). Missing this plugin means no framework-specific lint rules.

### MISSING: Version constraint validation (minimum versions)

All 25 rules check package presence but not version. An ancient `eslint@7.0.0` or `typescript@4.0.0` would pass. This matters because:
- `typescript-eslint` v8+ requires `eslint` v9+ and `typescript` v5+
- `eslint-plugin-import-x` requires `eslint` v9+
- Flat config (which the ESLint checker validates) requires `eslint` v9+

**Proposed rule — TS-PLUG-21: Minimum version constraints.** For packages where a minimum version is architecturally required (eslint >= 9, typescript >= 5, typescript-eslint >= 8), validate the version specifier in package.json is not below the minimum.

**Verdict: Medium priority.** Version mismatches cause cryptic runtime errors. The most important one is eslint >= 9 since flat config is mandatory.

### MISSING: Conflicting/deprecated package detection

Some packages conflict:
- `eslint-plugin-import` (deprecated) vs `eslint-plugin-import-x` (replacement) — having both causes rule conflicts
- `@typescript-eslint/eslint-plugin` + `@typescript-eslint/parser` (v7 style) vs `typescript-eslint` (v8 unified) — having both causes double-parsing
- `tslint` (fully deprecated)

**Proposed rule — TS-PLUG-22: Conflicting/deprecated lint packages.** Error if deprecated packages are present alongside their replacements.

**Verdict: Add.** Agent-managed codebases commonly accumulate both old and new versions of a package after migration attempts. This is a real and common failure mode.

### NOT MISSING: Checking `dependencies` section for tools

The prompt asked about checking `dependencies` not just `devDependencies`. For lint plugins and dev tools, `devDependencies` is the correct location. Finding them in `dependencies` is a misplacement, not a valid alternative location. See the bug note above for the proposed two-part fix.

---

## Summary of proposed additions

| ID | Checker | Rule | Priority |
|----|---------|------|----------|
| -- | tsconfig | **BUG FIX**: JSONC support (comments + trailing commas) | Critical |
| TS-TSCONFIG-20 | tsconfig | `extends` override weakening detection | High |
| TS-TSCONFIG-21 | tsconfig | `skipLibCheck: true` inventory | Low |
| -- | npmrc | **BUG FIX**: Inline comment stripping on value lines | Medium |
| -- | package | **BUG FIX**: Deduplicate banned deps list | Medium |
| TS-PACKAGE-13 | package | `type: "module"` enforcement | High |
| TS-PACKAGE-14 | package | Workspace member `private: true` | Medium |
| TS-PACKAGE-15 | package | `packageManager` format validation (pnpm@semver) | High |
| TS-PACKAGE-16 | package | `engines.node` existence + semver format | Low-medium |
| -- | plugins | **BUG FIX**: Check both deps + devDeps, warn on misplacement | Medium |
| TS-PLUG-20 | plugins | `@next/eslint-plugin-next` (content + next.js) | Medium |
| TS-PLUG-21 | plugins | Minimum version constraints (eslint>=9, ts>=5) | Medium |
| TS-PLUG-22 | plugins | Conflicting/deprecated package detection | High |
