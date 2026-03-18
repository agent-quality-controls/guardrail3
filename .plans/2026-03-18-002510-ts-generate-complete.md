# Make `ts generate` produce complete config files

**Date:** 2026-03-18 00:25
**Task:** `ts generate` currently only produces .npmrc, tsconfig.base.json, .jscpd.json, and a minimal eslint starter. It needs to produce ALL config files that `ts validate` checks for — otherwise generate is pointless.

## Goal

After running `guardrail3 ts generate`, the project should have every config file needed to pass ALL T-ESLP-*, T-STYL-*, T-TOOL-* validation checks. The generated ESLint config must include unicorn, regexp, sonarjs, react, built-in TS rules, test relaxations, boundary configs, and app-type-specific plugins (a11y, tailwind-ban for content apps).

## Scope

### Files `ts generate` must produce

| File | Current | Needed |
|------|---------|--------|
| `.npmrc` | Yes | Yes (no change) |
| `tsconfig.base.json` | Yes | Yes (needs all strict flags from T9/T52-T64) |
| `.jscpd.json` | Yes | Yes (threshold=0 per T20) |
| `eslint.config.mjs` | Minimal starter | **Complete config with ALL plugin rules** |
| `cspell.json` | No | **Yes** (T-TOOL-07) |
| `.stylelintrc.mjs` | No | **Yes, content apps only** (T-STYL-01..06) |

### What the ESLint config must include

**Always (all app types):**
- Base: `@eslint/js` recommended + `typescript-eslint` strict type checking
- Plugin: `unicorn` flat/recommended + UNICORN_DISABLED (8 rules off) + UNICORN_EXTRA (4 rules on)
- Plugin: `regexp` flat/recommended + REGEXP_EXTRA (6 rules)
- Plugin: `sonarjs` recommended + SONARJS_RULES (24 rules)
- Plugin: `react` + REACT_EXTRA (10 rules)
- Built-in: BUILTIN_RULES (17 rules) including naming-convention with selector, jsx-no-leaked-render with validStrategies
- Core rules: max-lines (400), max-lines-per-function (100), complexity (25), no-console, eqeqeq, no-restricted-imports, no-restricted-globals (process.env)
- All T60-T83 rules: no-floating-promises, no-explicit-any, strict-boolean-expressions, switch-exhaustiveness-check, etc.
- Test relaxation block: TEST_RELAXATION_RULES (5 rules) + standard test relaxations
- Boundaries plugin for hex arch (if service app)
- Global ignores
- `projectService: true` for type-aware linting

**Content apps additionally:**
- `jsx-a11y` strict mode (T-ESLP-07/08)
- `tailwind-ban` with denyList (T-ESLP-12)

### What the stylelint config must include (content apps only)
- `stylelint-config-standard` extends (T-STYL-02)
- `stylelint-config-tailwindcss` extends (T-STYL-03)
- `@double-great/stylelint-a11y` plugin (T-STYL-04)
- All 11 a11y rules enabled (T-STYL-05)
- Architecture exceptions disabled (T-STYL-06)

### What cspell.json must include
- Language setting
- ignorePaths (node_modules, .next, dist, coverage, .plans, .worklogs)
- Empty words array for project-specific additions

## Approach

### Step 1: Build ESLint config module (`apps/guardrail3/src/domain/modules/eslint.rs`)

New module that builds a complete `eslint.config.mjs` string. Takes parameters:
- `has_content_app: bool` — include a11y, tailwind-ban sections
- `has_service_app: bool` — include boundaries plugin section
- `max_lines: u32` — file length limit (default 400)

The config is built as string concatenation of sections (same pattern as clippy.rs builds clippy.toml). Each section is a const or a function that returns a string.

### Step 2: Build stylelint config module

Small module producing `.stylelintrc.mjs` with standard + tailwind + a11y.

### Step 3: Build cspell config module

Small module producing `cspell.json`.

### Step 4: Update tsconfig.base.json canonical module

Add missing strict flags: `noUncheckedIndexedAccess`, `exactOptionalPropertyTypes`, `noPropertyAccessFromIndexSignature`, `noImplicitOverride`, `noFallthroughCasesInSwitch`, `allowUnreachableCode: false`, `allowUnusedLabels: false`.

### Step 5: Update .jscpd.json canonical module

Change threshold from current value to 0 (per T20).

### Step 6: Wire into `generate_ts_files()`

- Always generate eslint.config.mjs (remove the `mode` gate — if you have a TS section, you get the config)
- Generate stylelint config if any app has content type
- Generate cspell.json always
- Pass app type info to the ESLint builder

### Step 7: Update `generate_expected_ts()` for diff/dry-run

Same changes as generate — must produce the same file list.

## Key Decisions

- **ESLint config is always generated** — removing the `mode: "generate"/"starter"` gate. If you opted into guardrail3 for TS, you want the full config.
- **Config is a single flat file** — not split per-app. ESLint flat config handles per-directory overrides with `files` globs internally.
- **App type detection flows from guardrail3.toml** — `generate_ts_files` reads the `[typescript.apps.*]` config to determine which app types exist and what sections to include.
- **Stylelint only generated if content app exists** — no point generating it for pure service apps.

## Files to Modify

- `apps/guardrail3/src/domain/modules/eslint.rs` — **NEW**: complete ESLint config builder
- `apps/guardrail3/src/domain/modules/stylelint.rs` — **NEW**: stylelint config builder
- `apps/guardrail3/src/domain/modules/cspell.rs` — **NEW**: cspell config builder
- `apps/guardrail3/src/domain/modules/canonical.rs` — update TSCONFIG_BASE and JSCPD content
- `apps/guardrail3/src/domain/modules/mod.rs` — register new modules
- `apps/guardrail3/src/commands/generate.rs` — wire new files into `generate_ts_files()` and `generate_expected_ts()`
- `apps/guardrail3/src/domain/config/types.rs` — possibly remove `EslintConfig.mode` or keep as override
