# Per-App Config Architecture: TypeScript Config Scoping

**Date:** 2026-03-18
**Scope:** Design document for how guardrail3 handles config file generation, validation, and overrides across monorepo apps

---

## 1. The Fundamental Tension

guardrail3 occupies an uncomfortable position: it must generate config files that are generic enough to work for any project, but real projects need project-specific rules. A generated `eslint.config.mjs` with 50 plugins is useful. A generated `eslint.config.mjs` that knows "landing can only import from content-constraints" is not possible without the user teaching guardrail3 about their dependency graph.

This creates two distinct roles:

1. **Engine** -- plugins installed, base rules configured, strictness levels set. This is guardrail3's domain. It answers: "What tools should be active and how strict should they be?"
2. **Policy** -- module boundaries, import restrictions, per-app overrides, project-specific ignores. This is the user's domain. It answers: "What architectural rules does THIS project enforce?"

The entire design below follows from this split. Every file type is analyzed through the lens of: what part is engine, what part is policy, and how do the two compose without destroying each other.

---

## 2. File-by-File Analysis

### 2.1 eslint.config.mjs

#### Tool resolution semantics

ESLint flat config (eslint.config.mjs) has NO walk-up, NO cascade, NO inheritance. One config at cwd. Period. With the experimental `unstable_config_lookup_from_file` flag (shipping in ESLint v9.x), ESLint walks up from the file being linted to find the nearest config -- but this flag is not yet stable and changes the entire mental model.

In a pnpm monorepo, there are two viable patterns:

**Pattern A: Single root config with file-pattern scoping.** One `eslint.config.mjs` at root. Per-app rules use `files: ["apps/landing/**"]` to scope. This is what steady-parent does today (487 lines). Pros: single source of truth, all rules visible in one place. Cons: grows unboundedly, mixing engine and policy in one file.

**Pattern B: Per-app configs that import a shared base.** Each app has its own `eslint.config.mjs` that imports from a shared package or relative path. ESLint flat config supports this via `import ... from '../shared-eslint.mjs'`. Pros: each app owns its rules. Cons: duplication of boilerplate, drift between apps, harder to validate completeness.

**Pattern C: Root config imports per-app policy files.** Root `eslint.config.mjs` imports from `apps/landing/eslint-policy.mjs`, `apps/admin/eslint-policy.mjs` etc. and spreads them in. Pros: clean separation. Cons: ESLint doesn't natively support this pattern (it works, but it's unusual and surprising).

#### What steady-parent actually does

Single root config (Pattern A). 487 lines including:
- Global ignores (~50 patterns, project-specific)
- Per-app module boundary rules via `files:` scoping (different for landing, admin, metadata, generator)
- Hex arch enforcement (admin only, via `files: ["apps/admin/**"]`)
- Design token bans (landing only)
- Test relaxations
- All plugins (boundaries, jsx-a11y, tailwind-ban, etc.)

#### What guardrail3 currently generates

A single root `eslint.config.mjs` (Pattern A) with:
- All base plugins (typescript-eslint, unicorn, regexp, sonarjs, react, react-hooks)
- Conditional content plugins (jsx-a11y, tailwind-ban) if any app is content-type
- Conditional service plugins (boundaries) if any app is service-type
- Core rules, TS strict rules, test relaxations
- The boundaries section includes placeholder settings ("Customize boundary rules per project")
- The route wrapper section is entirely commented out

This is already the right pattern. But the gap is: guardrail3 generates the engine (plugins + base rules), and the user needs to add policy (boundaries, per-app scoped rules) -- but the generated file says "DO NOT EDIT" and `guardrail3 generate` overwrites it.

#### Scoping levels

| Level | Possible? | Needed? | Notes |
|---|---|---|---|
| Root only | Yes | Default | One config at monorepo root |
| Per-app | Yes (ESLint supports it) | Only if apps need fundamentally different plugin sets | Not recommended -- forces running ESLint per-app instead of once from root |
| Per-package | Technically yes | No | Packages inherit root config |
| Per-pipeline-stage | Technically yes | No | Generator pipeline stages are internal to one package |

**Recommendation: Root only.** Per-app ESLint configs are an escape hatch, not the default. The flat config's `files:` scoping handles per-app rules within a single root config.

#### Content differences between apps

| Dimension | Landing | Admin | Packages | Generator |
|---|---|---|---|---|
| **Plugins needed** | jsx-a11y, tailwind-ban | boundaries | Base only | Base only |
| **Module boundaries** | Can only import content-constraints | Hex arch (domain -> app -> adapters) | N/A | Only ../../lib/* and content-constraints |
| **Special rules** | Design token bans, SEO patterns | Route wrapper enforcement | Stricter no-console | Isolated, no cross-stage imports |
| **Test relaxations** | Standard | Standard | Standard | Standard |

The plugin sets (jsx-a11y for content, boundaries for service) are already handled by guardrail3's conditional generation. The module boundary rules, import restrictions, and per-app overrides are policy -- they cannot be generated.

#### The override problem

This is the hardest design problem in guardrail3's TypeScript story. Options:

**Option A: "DO NOT EDIT" + regenerate overwrites everything.**
Current behavior. User loses all manual edits on `guardrail3 generate`. Terrible for ESLint because the real value is in the policy rules that guardrail3 cannot generate.

**Option B: guardrail3 generates to a shadow location, user's config imports it.**
```
.guardrail3/generated/eslint-engine.mjs  <-- guardrail3 generates this
eslint.config.mjs                         <-- user owns this, imports from above
```
The user's `eslint.config.mjs` would look like:
```js
import engine from "./.guardrail3/generated/eslint-engine.mjs";
export default [...engine, { /* per-app policy rules */ }];
```
Pros: Clean separation. guardrail3 can regenerate the engine without touching policy. The user's file is the composition point.
Cons: Extra level of indirection. User must understand the import pattern. The `files:` scoping in the engine config must be compatible with the user's additions.

**Option C: User has a policy file that guardrail3 merges at generation time.**
```
.guardrail3/overrides/eslint-policy.mjs   <-- user writes this
eslint.config.mjs                          <-- guardrail3 generates (engine + merged policy)
```
Cons: ESLint config is JavaScript, not JSON/TOML. You can't merge JavaScript files programmatically without an AST transform. This is not viable for arbitrary JS.

**Option D: Marker-based preservation.**
guardrail3 generates `eslint.config.mjs` with markers:
```js
// --- guardrail3:engine:start ---
// ... generated content ...
// --- guardrail3:engine:end ---

// --- user:policy:start ---
// ... user content preserved across regeneration ...
// --- user:policy:end ---
```
Cons: Fragile. Markers can be accidentally deleted. The user is editing inside a generated file, which is confusing. Interleaving generated and user-written JS is error-prone (scope, variable references, import order).

**Option E: guardrail3 generates the engine as a package, user imports it.**
A dedicated workspace package (e.g., `packages/eslint-config-guardrail3/`) that exports the engine config. The root `eslint.config.mjs` imports from this package.
Cons: Overkill for most projects. Adds a package to the workspace. But this is actually how large monorepos typically handle shared ESLint configs (e.g., `@company/eslint-config`).

#### Recommended approach: Option B (shadow location)

The generated config goes to `.guardrail3/generated/eslint-engine.mjs`. The user's `eslint.config.mjs` at root imports it. guardrail3 generates BOTH files on first run (`ts init` or `ts generate`), but only regenerates the engine file on subsequent runs. The user's `eslint.config.mjs` is marked as user-owned and never overwritten after initial creation.

```
.guardrail3/generated/eslint-engine.mjs  <-- guardrail3 owns, regenerates freely
eslint.config.mjs                         <-- guardrail3 scaffolds once, user owns thereafter
```

The scaffolded `eslint.config.mjs` would be:
```js
// @ts-check
import engine from "./.guardrail3/generated/eslint-engine.mjs";

export default [
  ...engine,

  // -------------------------------------------------------------------
  // Project-specific rules below. guardrail3 will NOT overwrite this file.
  // Add module boundary rules, per-app overrides, and custom ignores here.
  // -------------------------------------------------------------------

  // Example: per-app module boundaries
  // {
  //   files: ["apps/landing/**"],
  //   rules: { "no-restricted-imports": ["error", { paths: [...] }] },
  // },
];
```

**Validation implications:** guardrail3's `ts validate` checks for rule presence in `eslint.config.mjs` (the user's file). Since the engine rules come via `...engine` spread, the validator cannot see them by grepping the user's file. Two options:

1. **Validate the generated engine file directly.** guardrail3 knows what's in it (it generated it). Only validate user-policy rules in the user's file.
2. **Resolve the full config.** Read both files, understand that `...engine` means "all engine rules are present." This requires the validator to understand the import relationship.

Option 1 is simpler and correct. The engine file IS the guardrail3-controlled config -- validate it directly. The user's file is policy -- guardrail3 validates structural properties (e.g., "does the user's file import the engine?") but does not dictate policy content.

**What guardrail3 validates in the user's eslint.config.mjs:**
- T1: File exists
- T-ENGINE-IMPORT: File imports from `.guardrail3/generated/eslint-engine.mjs` (new check)
- T6: Boundary enforcement present (policy check -- user's responsibility)
- T7: Relaxed rules inventory (audit trail)
- T8: File-specific overrides inventory
- T49: Test relaxations inventory
- T50: Route wrapper enforcement (policy)
- T51: process.env ban (should be in engine, but user might add exceptions)

**What guardrail3 validates in the engine file:**
- All rule presence checks (T2-T5, T40-T48, T60-T83)
- Plugin presence
- Rule values/thresholds

This cleanly separates concerns. The engine is guardrail3's domain and fully validated. The policy is the user's domain and only audited.

#### The "one root config" vs "per-app config" question

For ESLint, guardrail3 should ALWAYS generate a single root engine config. If a project needs per-app ESLint configs (e.g., apps/landing has its own eslint.config.mjs), that is policy -- the user creates it. guardrail3 does not need to support per-app ESLint generation because:

1. ESLint flat config's `files:` scoping handles most per-app needs within a single root config
2. Per-app configs are rare and indicate the apps are so different they might as well be separate repositories
3. Generating per-app ESLint configs would require guardrail3 to know each app's dependency graph, which is policy

**Edge case: app with its own eslint.config.mjs that contradicts root.** If `apps/landing/eslint.config.mjs` exists, ESLint (without the experimental flag) ignores it entirely -- only the root config is used when linting from root. This is confusing but correct behavior. guardrail3 should warn if per-app ESLint configs exist (they do nothing when linting from root):

> T-LINT-ORPHAN: `apps/landing/eslint.config.mjs` exists but ESLint flat config only uses the root config when run from workspace root. This file has no effect. Move its rules to the root `eslint.config.mjs` with `files: ["apps/landing/**"]` scoping, or run ESLint separately from within `apps/landing/`.

---

### 2.2 tsconfig.base.json / tsconfig.json

#### Tool resolution semantics

TypeScript's `extends` performs a deep merge of `compilerOptions` and replaces `include`/`exclude`/`files` arrays. Each app needs its own `tsconfig.json`. The base config provides shared strictness settings; per-app configs add paths, includes, plugins, and framework-specific options.

#### Scoping levels

| Level | Possible? | Needed? | Notes |
|---|---|---|---|
| Root (tsconfig.base.json) | Yes | Yes | Shared strictness settings |
| Per-app (tsconfig.json) | Yes | Required | Every app needs its own for paths, includes, plugins |
| Per-package (tsconfig.json) | Yes | Common | Packages often extend base |
| Per-pipeline-stage | Yes | Exists in steady-parent | Generator's 13 stages each have own tsconfig |

#### What steady-parent actually does

- `tsconfig.base.json` at root: strict settings, ES2022, bundler resolution
- `apps/landing/tsconfig.json`: extends base, adds Next.js plugins, path aliases, specific include/exclude
- `apps/admin/tsconfig.json`: STANDALONE (does NOT extend base), ES2017, its own path aliases (@modules, @domain, @adapters)
- `packages/content-constraints/tsconfig.json`: extends base
- `packages/spec/tsconfig.json`: standalone (no extends)
- `packages/validator-types/tsconfig.json`: standalone
- `packages/generator/tsconfig.json`: extends base
- `packages/generator/pipeline/*/tsconfig.json`: extends generator's tsconfig (13 stages)

Key observation: not all packages extend base. Some are standalone. This is intentional -- `spec` and `validator-types` have different compilation targets or constraints.

#### What guardrail3 should generate

**tsconfig.base.json: YES.** This is pure engine -- strict compiler options that every app should inherit (or consciously deviate from). guardrail3 already generates this and it's correct.

**Per-app tsconfig.json: NO.** These are entirely policy:
- Path aliases (`@modules`, `@domain`, etc.) depend on project structure
- `include`/`exclude` depend on project structure
- Framework plugins (Next.js `next/typescript`) are framework-specific
- Some apps intentionally don't extend base
- Some apps target different ES versions

guardrail3 cannot generate per-app tsconfig.json files without intimate knowledge of each app's structure.

#### What guardrail3 should validate

Currently, `check_tsconfig` looks for `tsconfig.base.json` or falls back to `tsconfig.json` at the project root and checks strict settings. This is correct for validating the base config.

The harder question: should guardrail3 also validate per-app tsconfig.json files?

**Yes, but with different rules.** Per-app tsconfig.json validation should check:

1. **T-TSC-EXTENDS: Does it extend base?** If not, that's an audit finding (Info severity), not an error. Some apps legitimately don't extend base (admin in steady-parent). But the user should be aware.
2. **T-TSC-STRICT: Are strict settings consistent?** If an app extends base, the strict settings are inherited -- good. If it doesn't extend base, check that it at least has `"strict": true`. An app that neither extends base nor has its own strict settings is a real error.
3. **T-TSC-EXISTS: Does each discovered app have a tsconfig.json?** A TypeScript app without tsconfig.json is an error.

**What guardrail3 should NOT validate per-app:**
- Path aliases (policy)
- Include/exclude patterns (policy)
- Framework plugins (policy)
- Target/module settings (these vary legitimately between apps)

#### The admin problem

`apps/admin` has a standalone tsconfig that doesn't extend base, uses ES2017 (not ES2022), and has its own strict settings. guardrail3 should handle this gracefully:

- Discover that admin has a tsconfig.json (good)
- Note that it doesn't extend tsconfig.base.json (Info audit finding)
- Check that it has `strict: true` independently (pass or fail)
- NOT flag ES2017 as wrong (that's the app's choice)

This means the validator needs two modes:
1. **Base validation** (current): check tsconfig.base.json for full strictness
2. **Per-app validation** (new): for each discovered app, check minimal requirements (tsconfig exists, either extends base or has strict:true)

#### Generator pipeline stages

The 13 pipeline stages under `packages/generator/pipeline/*/tsconfig.json` are a special case. These extend the generator's tsconfig, not the root base. guardrail3 should NOT try to discover or validate these. They are internal to the generator package.

**Rule:** guardrail3 discovers apps under `apps/` and packages under `packages/`. It does NOT recurse into package internals. If `packages/generator/` has sub-tsconfigs, that's the package's business.

However, if a package's root tsconfig.json has issues, guardrail3 should flag that. The question is whether guardrail3 should validate package-level tsconfigs at all.

**Recommendation: Yes, but opt-in.** By default, guardrail3 validates:
- Root tsconfig.base.json (always)
- Per-app tsconfig.json under `apps/*/` (always, for discovered TS apps)
- Package tsconfig.json under `packages/*/` (only if `[typescript.checks.packages]` is enabled in guardrail3.toml)

---

### 2.3 .stylelintrc.mjs

#### Tool resolution semantics

Stylelint uses cosmiconfig: walks up from the file being linted, nearest config wins, no merge. Has `extends` for explicit inheritance.

#### Scoping levels

| Level | Possible? | Needed? | Notes |
|---|---|---|---|
| Root only | Yes | Default for most projects | One config at root |
| Per-app | Yes (cosmiconfig walk-up) | Only if apps have fundamentally different CSS needs | Rare |
| Per-package | Not relevant | Packages typically don't have CSS | |

#### What steady-parent does

Single root `.stylelintrc.mjs` with a11y rules and CSS notation rules. Only relevant for apps with CSS (landing, admin). Packages don't have CSS.

#### What guardrail3 should generate and validate

**Generate: Root only, conditional on content app.** Already correct. guardrail3 generates `.stylelintrc.mjs` only when a content-type app is detected. This is engine (what plugins, what a11y rules).

**Per-app stylelint: NOT guardrail3's domain.** If landing needs different CSS rules than admin (e.g., landing bans certain Tailwind classes, admin doesn't), that's policy. The user handles it via stylelint's `overrides` field or per-app configs.

**Edge case: admin has CSS but no a11y requirements.** Stylelint walk-up means admin picks up the root config, which has a11y rules. If admin's CSS legitimately doesn't need a11y (e.g., admin dashboard used only by sighted staff), the user can:
1. Add `overrides: [{ files: ["apps/admin/**"], rules: { "a11y/*": null } }]` in root config
2. Create `apps/admin/.stylelintrc.mjs` that doesn't extend the root

Both are policy. guardrail3 doesn't need to support this in generation.

**Validation:** Same shadow-location approach as ESLint.
- Engine file: `.guardrail3/generated/stylelint-engine.mjs`
- User file: `.stylelintrc.mjs` (imports engine, adds overrides)
- Or: keep current approach (root-only, guardrail3 generates directly) since stylelint configs are much simpler than ESLint and rarely need project-specific policy. The a11y rules are universal, not project-specific.

**Recommendation: Keep generating root .stylelintrc.mjs directly.** Unlike ESLint, stylelint configs are almost entirely engine (standard config, tailwind config, a11y plugin, a11y rules). The policy surface area is tiny. If a user needs to customize, they can edit the generated file and guardrail3's overwrite warning is sufficient.

---

### 2.4 cspell.json

#### Tool resolution semantics

cspell walks up from the file being checked. Nearest config wins, no merge. Has `import` for explicit inheritance from other cspell configs.

#### Scoping levels

| Level | Possible? | Needed? | Notes |
|---|---|---|---|
| Root only | Yes | Default | One dictionary for the whole project |
| Per-app | Yes (via walk-up or import) | Sometimes | Landing has content words (MDX jargon), admin has domain words (hex arch terms) |
| Per-package | Yes | Rare | Packages might have specialized vocabulary |

#### What guardrail3 should generate

**Root cspell.json: YES.** This is engine -- standard ignore paths, language setting, schema.

**Per-app cspell configs: NO.** Per-app dictionaries are policy (project-specific words, domain jargon).

#### The words problem

cspell.json has a `words` array that is inherently project-specific. guardrail3 generates it empty (`"words": []`). Users add their project's vocabulary. On regeneration, guardrail3 would overwrite it, losing all added words.

**Solution: Same shadow pattern.**
- `.guardrail3/generated/cspell-engine.json` -- guardrail3 owns (ignore paths, language, schema)
- `cspell.json` -- user owns, imports from engine:
```json
{
  "import": [".guardrail3/generated/cspell-engine.json"],
  "words": ["velite", "contentlayer", "drizzle", ...]
}
```

This is clean because cspell's `import` feature is designed for exactly this composition pattern. The engine provides structure; the user provides vocabulary.

**Validation:**
- Check that cspell.json exists (T-TOOL-07, already implemented)
- Check that it imports the engine config (new check)
- Do NOT validate the words list (that's entirely user domain)

---

### 2.5 .npmrc

#### Tool resolution semantics

Root-level only during workspace install. pnpm ignores .npmrc files in subdirectories during workspace operations.

#### Scoping

Root only. Always. pnpm workspaces use one .npmrc.

#### What guardrail3 should do

**Generate: Yes, root only.** Already correct. The .npmrc is pure engine -- strict-peer-dependencies, engine-strict, etc. No project-specific policy.

**Override concern: Minimal.** Users rarely need to customize .npmrc beyond what guardrail3 generates. If they do (e.g., adding a registry URL), the shadow pattern works, but it's overkill.

**Recommendation: Keep generating .npmrc directly.** If the user needs project-specific settings, they can add them below the generated content. guardrail3 should check that the required settings are present (already implemented in T11-T14) rather than checking for exact file match. This means: generate with "DO NOT EDIT", but validate by checking individual settings, not file identity.

This is actually what the validator already does -- `npmrc_check.rs` checks for individual settings like `strict-peer-dependencies=true`, not for file identity. So overwriting is safe as long as the user adds their settings below the generated block.

**However:** If the user changes a guardrail3 setting (e.g., `strict-peer-dependencies=false`), regeneration will overwrite their change AND the validator will flag it. This is the correct behavior -- the user is weakening a guardrail, and guardrail3 should both detect and revert it.

---

### 2.6 .jscpd.json

#### Tool resolution semantics

cwd-only, no walk-up.

#### Scoping

Root only. jscpd is run from the project root.

#### What guardrail3 should do

**Generate: Yes, root only.** Pure engine -- threshold, min tokens, ignore patterns. Already correct.

**Override concern: The ignore list.** The generated .jscpd.json includes ignore patterns that are partially project-specific (`**/components/ui/**`, `**/components/pro-blocks/**`). These are shadcn/ui patterns that guardrail3 assumes.

**Recommendation:** The current approach is fine. jscpd config is simple enough that direct generation works. Users can edit the ignore list and the validator checks structural properties (T19-T22: threshold, reporters, ignore patterns exist) rather than exact file content.

---

### 2.7 prettier config

#### Tool resolution semantics

cosmiconfig walk-up from file. Nearest wins, no merge.

#### What guardrail3 should do

guardrail3 does NOT currently generate or validate prettier config. Should it?

**Analysis:** Prettier config is almost entirely engine (print width, tab width, semicolons, trailing commas). There's very little policy. But prettier is contentious -- teams have strong opinions about formatting.

**Recommendation: Not in scope for V1.** Prettier config is:
1. Simpler than ESLint (just a few options)
2. More contentious (teams fight over semicolons)
3. Not a safety concern (formatting doesn't prevent bugs)

guardrail3's philosophy is enforcing safety guardrails, not formatting preferences. Leave prettier to the user.

---

## 3. The Unified Override Architecture

### 3.1 Current state

For Rust, guardrail3 already has `.guardrail3/overrides/` with TOML files:
- `clippy-methods.toml` -- extra disallowed methods
- `clippy-types.toml` -- extra disallowed types
- `deny-bans.toml` -- extra crate bans
- `deny-skip.toml` -- duplicate crate skip list
- `deny-feature-bans.toml` -- feature bans

These are TOML and can be mechanically merged. ESLint config is JavaScript -- it cannot be merged the same way.

### 3.2 Proposed TypeScript override architecture

```
.guardrail3/
  generated/                         # guardrail3 regenerates freely
    eslint-engine.mjs                # ESLint plugins + base rules
    cspell-engine.json               # cspell ignore paths + structure
  overrides/                         # Already exists for Rust
    clippy-methods.toml              # (existing)
    clippy-types.toml                # (existing)
    deny-bans.toml                   # (existing)
    deny-skip.toml                   # (existing)
    deny-feature-bans.toml           # (existing)
```

**User-owned files (never overwritten after initial scaffold):**
```
eslint.config.mjs                    # Imports engine, adds project policy
cspell.json                          # Imports engine, adds project words
```

**guardrail3-owned files (always overwritten on generate):**
```
.guardrail3/generated/eslint-engine.mjs
.guardrail3/generated/cspell-engine.json
.npmrc                               # Direct generation (no user policy needed)
tsconfig.base.json                   # Direct generation (pure engine)
.jscpd.json                          # Direct generation (minimal policy)
.stylelintrc.mjs                     # Direct generation (minimal policy)
.githooks/pre-commit                 # Direct generation
```

### 3.3 The "first generate" vs "subsequent generate" distinction

| File | First `ts generate` | Subsequent `ts generate` |
|---|---|---|
| `.guardrail3/generated/eslint-engine.mjs` | Created | Overwritten |
| `eslint.config.mjs` | Scaffolded with import + examples | **NOT touched** (skip if exists) |
| `.guardrail3/generated/cspell-engine.json` | Created | Overwritten |
| `cspell.json` | Scaffolded with import | **NOT touched** (skip if exists) |
| `.npmrc` | Created | Overwritten |
| `tsconfig.base.json` | Created | Overwritten |
| `.jscpd.json` | Created | Overwritten |
| `.stylelintrc.mjs` | Created (if content app) | Overwritten |

For the scaffolded files (eslint.config.mjs, cspell.json), guardrail3 prints a message on subsequent runs:

```
  skipped: eslint.config.mjs (user-owned, not regenerated)
  skipped: cspell.json (user-owned, not regenerated)
```

If the user wants to re-scaffold from scratch, they delete the file and run `ts generate` again.

### 3.4 How overrides work for ESLint specifically

The user's `eslint.config.mjs` is the composition point. They can:

1. **Add per-app rules:**
```js
import engine from "./.guardrail3/generated/eslint-engine.mjs";
export default [
  ...engine,
  {
    files: ["apps/landing/**"],
    rules: {
      "no-restricted-imports": ["error", {
        paths: [{ name: "@steady-parent/admin", message: "Landing cannot import admin" }],
      }],
    },
  },
];
```

2. **Override engine rules:**
```js
export default [
  ...engine,
  {
    // Relax for this project -- EXCEPTION: legacy code
    rules: { "@typescript-eslint/no-explicit-any": "warn" },
  },
];
```

3. **Add project-specific ignores:**
```js
export default [
  ...engine,
  { ignores: ["apps/legacy/**", "scripts/**"] },
];
```

guardrail3's validator can detect overrides that weaken engine rules (item 2) and flag them as audit findings.

---

## 4. Validation Scope Matrix

### 4.1 What validates where

| Check | Validates which file | Scope |
|---|---|---|
| T1 (ESLint exists) | `eslint.config.mjs` | Root |
| T2-T5 (ESLint rule values) | `.guardrail3/generated/eslint-engine.mjs` | Root |
| T6 (Boundary enforcement) | `eslint.config.mjs` | Root (user policy) |
| T7 (Relaxed rules audit) | `eslint.config.mjs` | Root (user policy) |
| T9 (tsconfig strict) | `tsconfig.base.json` | Root |
| T9-per-app (tsconfig exists + strict) | `apps/*/tsconfig.json` | Per-app |
| T11-T14 (.npmrc settings) | `.npmrc` | Root |
| T19-T22 (jscpd config) | `.jscpd.json` | Root |
| T40-T83 (ESLint rule presence) | `.guardrail3/generated/eslint-engine.mjs` | Root |
| T-STYL-01..06 (stylelint) | `.stylelintrc.mjs` | Root |
| T-TOOL-07 (cspell exists) | `cspell.json` | Root |

### 4.2 New checks needed

| Check ID | Description | Severity |
|---|---|---|
| T-ENGINE-01 | `eslint.config.mjs` imports from `.guardrail3/generated/eslint-engine.mjs` | Error |
| T-ENGINE-02 | `.guardrail3/generated/eslint-engine.mjs` exists and is not stale | Error |
| T-ENGINE-03 | `cspell.json` imports from `.guardrail3/generated/cspell-engine.json` | Warn |
| T-TSC-APP-01 | Each discovered TS app has a `tsconfig.json` | Error |
| T-TSC-APP-02 | Per-app tsconfig either extends base or has `strict: true` | Warn |
| T-LINT-ORPHAN | Per-app `eslint.config.mjs` exists but has no effect from root | Warn |
| T-POLICY-WEAKEN | User's eslint.config.mjs overrides an engine rule to off/warn | Info (audit) |

### 4.3 Staleness detection

`guardrail3 check` currently compares generated file content with expected content. With the shadow pattern:
- `.guardrail3/generated/*` files: check for staleness (exact content match with expected)
- User-owned files (`eslint.config.mjs`, `cspell.json`): check structural properties only (imports engine, engine is not stale)

---

## 5. guardrail3.toml Config Shape

### 5.1 Current TypeScript section

```toml
[typescript]
migrations = "drizzle/migrations"

[typescript.apps.landing]
type = "content"

[typescript.apps.admin]
type = "service"

[typescript.checks]
architecture = true
content = true
tests = true

[typescript.eslint]
mode = "engine"   # or "standalone" for legacy

[typescript.canonical]
npmrc = true
tsconfig_base = true
jscpd = true
```

### 5.2 Proposed additions

```toml
[typescript]
migrations = "drizzle/migrations"

[typescript.apps.landing]
type = "content"

[typescript.apps.admin]
type = "service"

[typescript.checks]
architecture = true
content = true
tests = true
packages = false    # NEW: validate per-package tsconfig.json (default: false)

[typescript.eslint]
mode = "engine"     # "engine" = shadow pattern, "standalone" = guardrail3 generates eslint.config.mjs directly (legacy)

[typescript.canonical]
npmrc = true
tsconfig_base = true
jscpd = true
stylelint = true    # NEW: explicit control (currently auto-detected from content apps)
cspell = true       # NEW: explicit control (currently always generated)
```

The `mode = "engine"` vs `mode = "standalone"` distinction allows migration:
- New projects get `engine` mode (shadow pattern)
- Existing projects that already have a guardrail3-generated `eslint.config.mjs` stay in `standalone` mode until they opt in
- `standalone` mode is the current behavior (generate directly, overwrite on regenerate)

---

## 6. Edge Cases

### 6.1 App with its own eslint.config.mjs that contradicts root

As discussed in section 2.1: ESLint flat config ignores per-app configs when run from root. guardrail3 should warn (T-LINT-ORPHAN). The user must either:
- Move rules to root config with `files:` scoping
- Run ESLint separately from within the app directory

### 6.2 Package with no tsconfig at all

A TypeScript package (has .ts files or package.json with typescript in deps) without tsconfig.json is a real error. TypeScript will use default settings (no strict, no type checking). guardrail3 should flag this when `[typescript.checks.packages]` is enabled.

### 6.3 Package with standalone tsconfig (doesn't extend base)

Audit finding (Info), not an error. Some packages legitimately need different settings (e.g., `spec` package that only exports types and targets a different module system).

### 6.4 Multiple cspell.json files

cspell walks up from file. If `apps/landing/cspell.json` exists, it takes precedence for files in landing, and the root cspell.json is ignored for those files. This is intentional and correct -- landing might have MDX-specific vocabulary.

guardrail3 should NOT flag per-app cspell.json files as errors. But it should inventory them:

> T-SPELL-MULTI: `apps/landing/cspell.json` found. This overrides the root cspell.json for files in `apps/landing/`. Ensure it imports the root config if you want shared vocabulary: `"import": ["../../cspell.json"]`.

### 6.5 tools/ directory

guardrail3 currently discovers apps under `apps/`. The `tools/` directory (e.g., `tools/freebie-renderer`) is not discovered.

**Should it be?** No. Tools are utilities, not apps. They don't need hex arch validation, content checks, or the full ESLint rule set. If a tool has TypeScript, the root ESLint config covers it. If it has its own tsconfig.json, that's its business.

**Exception:** If the user explicitly configures a tool in guardrail3.toml:
```toml
[typescript.apps.freebie-renderer]
type = "library"
path = "tools/freebie-renderer"   # NEW: explicit path for non-standard locations
```

### 6.6 Generator pipeline stages (13 sub-tsconfigs)

guardrail3 should NOT discover or validate these. They are internal to the `packages/generator` package. The package's root `tsconfig.json` is what guardrail3 validates (if package validation is enabled).

If the user wants guardrail3 to validate pipeline stages, they can configure them explicitly:
```toml
[typescript.apps.generator-stage-2]
type = "library"
path = "packages/generator/pipeline/2_extract_knowledge"
```

But this is unlikely to be useful. The pipeline stages are tightly coupled to the generator package and don't benefit from independent validation.

### 6.7 legacy/ directory

guardrail3 should completely ignore any `legacy/` directory. This should be added to the default ignore list for app discovery and source scanning.

### 6.8 What if the user has never run `guardrail3 generate`?

guardrail3's `validate` command works without `generate`. It checks whatever config files exist. If the user has a hand-written `eslint.config.mjs` without the engine import pattern, the validator should still work -- it just checks the file directly (current behavior).

The engine import check (T-ENGINE-01) should only fire if `[typescript.eslint].mode = "engine"` is set in guardrail3.toml. If there's no config or mode is "standalone", use current behavior.

---

## 7. What guardrail3 Should and Should NOT Generate

### Definitive list

| File | Generate? | Override mechanism | Rationale |
|---|---|---|---|
| `tsconfig.base.json` | YES | Direct (overwrite) | Pure engine, no policy |
| `.npmrc` | YES | Direct (overwrite) | Pure engine, no policy |
| `.jscpd.json` | YES | Direct (overwrite) | Minimal policy (ignore list) |
| `.guardrail3/generated/eslint-engine.mjs` | YES | Direct (overwrite) | Engine portion of ESLint |
| `eslint.config.mjs` | SCAFFOLD ONCE | User-owned after first gen | Policy composition point |
| `.guardrail3/generated/cspell-engine.json` | YES | Direct (overwrite) | Engine portion of cspell |
| `cspell.json` | SCAFFOLD ONCE | User-owned after first gen | Policy (vocabulary) |
| `.stylelintrc.mjs` | YES (if content app) | Direct (overwrite) | Mostly engine (a11y rules) |
| `.githooks/pre-commit` | YES | Direct (overwrite) | Pure engine |
| Per-app tsconfig.json | NO | N/A | Pure policy |
| Per-app eslint configs | NO | N/A | Pure policy |
| Module boundary rules | NO | N/A | Pure policy |
| Import restriction rules | NO | N/A | Pure policy |
| Project-specific ignores | NO | User adds to eslint.config.mjs | Pure policy |
| prettier config | NO | N/A | Not a safety concern |

### The gray area, resolved

**Plugin installation and rule presence** -- guardrail3's domain. If `eslint-plugin-boundaries` is needed for service apps, the engine config includes it.

**Plugin configuration** (which boundary zones exist, what the zone patterns are) -- user's domain. The engine config sets `"boundaries/element-types": "warn"` with a default zone structure. The user customizes in their eslint.config.mjs.

**Test relaxations** -- guardrail3's domain for standard relaxations (no-explicit-any in tests). User's domain for project-specific test relaxations (e.g., "allow longer files in e2e tests").

**Framework-specific rules** (Next.js core-web-vitals, react-hooks exhaustive-deps) -- guardrail3's domain to install the plugin and enable base rules. User's domain to customize thresholds or scope to specific apps.

---

## 8. Migration Path

### For existing projects (already have guardrail3-generated eslint.config.mjs)

1. `guardrail3 ts generate` detects existing `eslint.config.mjs`
2. If `[typescript.eslint].mode` is not set, default to `"standalone"` (current behavior)
3. User can opt into engine mode by setting `mode = "engine"` in guardrail3.toml
4. On next `guardrail3 ts generate` with `mode = "engine"`:
   - Move existing eslint.config.mjs to eslint.config.mjs.backup
   - Generate `.guardrail3/generated/eslint-engine.mjs`
   - Scaffold new `eslint.config.mjs` with engine import
   - Print instructions: "Merge your custom rules from eslint.config.mjs.backup into the new eslint.config.mjs"

### For new projects

1. `guardrail3 ts init` creates guardrail3.toml with `mode = "engine"` by default
2. `guardrail3 ts generate` creates engine file and scaffolds user file
3. User adds project-specific rules to eslint.config.mjs

---

## 9. Implementation Implications for guardrail3

### Changes to generate

1. New `generate_ts_engine_files()` function that generates to `.guardrail3/generated/`
2. New `scaffold_user_files()` function that creates user-owned files only if they don't exist
3. `eslint.rs` `build_eslint_config()` stays the same but output goes to different path
4. New `build_eslint_scaffold()` function that generates the minimal user file with import

### Changes to validate

1. `eslint_check.rs`: When mode is "engine", validate the engine file for rule presence and the user file for structural properties
2. `tsconfig_check.rs`: Add per-app tsconfig discovery and minimal validation
3. New check IDs: T-ENGINE-01, T-ENGINE-02, T-ENGINE-03, T-TSC-APP-01, T-TSC-APP-02, T-LINT-ORPHAN, T-POLICY-WEAKEN

### Changes to config types

1. Add `packages: Option<bool>` to `TsChecksConfig`
2. Add `stylelint: Option<bool>` and `cspell: Option<bool>` to `CanonicalConfig`
3. Add `path: Option<String>` to `TsAppConfig` for non-standard app locations

### Changes to check (staleness)

1. For engine files: exact content match (current approach)
2. For user files: structural check only (imports engine, engine is not stale)

---

## 10. Open Questions

1. **Should the engine file be .mjs or .js?** ESLint flat config requires the file to be `eslint.config.mjs` (or `.js` with `"type": "module"`). The engine file can be any name with any extension since it's imported, not auto-discovered. `.mjs` is safer (always treated as ESM regardless of package.json).

2. **Should guardrail3 validate that required ESLint plugins are in package.json devDependencies?** Currently it checks for some plugins (eslint_plugin_checks.rs). With the engine pattern, the engine file imports plugins that must be installed. If they're not in devDependencies, ESLint will fail at runtime. guardrail3 should verify this.

3. **Should the engine file export named sections instead of a flat array?** e.g., `export { baseRules, tsStrictRules, reactRules, testRelaxations }` so the user can selectively import. This is more flexible but more complex for the user to compose.

4. **How does this interact with `guardrail3 check` in CI?** CI runs `guardrail3 check` to verify generated files are current. With the shadow pattern, `check` only compares `.guardrail3/generated/*` files, not user-owned files. The user-owned files are validated by `guardrail3 ts validate`, not by `check`.

5. **Should guardrail3 support eslint-plugin-import's `no-cycle` rule at the engine level?** Cycle detection requires type information and is slow. Many projects disable it. It's currently checked (T45) but might be better as opt-in.
