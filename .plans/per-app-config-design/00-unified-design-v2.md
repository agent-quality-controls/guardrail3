# Unified Design V2: Per-App Config Architecture

**Date:** 2026-03-18
**Method:** Every claim verified against actual steady-parent files and guardrail3 source code.
**Supersedes:** 00-unified-design.md (had incorrect categorizations: Cargo.toml was wrongly listed as Merge-managed when it should be Validate-only; .githooks/pre-commit was wrongly listed as Fully-owned when steady-parent's 283-line custom hook would be destroyed; .jscpd.json threshold difference was not properly characterized)

---

## Per-File Analysis

---

## 1. clippy.toml

### Instances in steady-parent
- `apps/validator-rust/clippy.toml` (45 lines: 5 thresholds, 23 disallowed-methods, 9 disallowed-types)
- `apps/substack-publisher/clippy.toml` (45 lines: 5 thresholds, 25 disallowed-methods, 9 disallowed-types)
- NO root `clippy.toml` (root workspace members `packages/low-expectations` and `packages/seo-site-files` have no clippy.toml covering them)

### Guardrail content vs project content
**Guardrail (shared between both apps):**
- Thresholds: `too-many-lines-threshold = 75`, `cognitive-complexity-threshold = 15`, `too-many-arguments-threshold = 7`, `type-complexity-threshold = 75`, `max-struct-bools = 3` -- identical in both
- Env methods: `std::env::var`, `var_os`, `vars`, `set_var`, `remove_var` -- identical paths
- Process: `std::process::exit`, `std::thread::sleep` -- identical paths
- Filesystem: all 15 `std::fs::*` methods -- identical paths, different reasons per app
- Types: all 9 identical (HashMap, HashSet, Mutex, RwLock, File, LazyLock, OnceLock, once_cell::sync::OnceCell, once_cell::sync::Lazy)

**Project-specific (differs between apps):**
- validator-rust has `reqwest::Client::builder` and `reqwest::Client::new` bans ("Use shared clients from LiveState"). substack-publisher does NOT.
- substack-publisher has `std::io::stdout` and `std::io::stderr` bans ("Use tracing macros for output"). validator-rust does NOT.
- `std::process::Command::new` reason differs: "Shell execution not permitted in this service" (validator-rust) vs "Use tokio::process::Command for async subprocess execution" (substack-publisher).
- All `std::fs::*` reasons differ: "File writes not expected in a stateless validator service" (validator-rust) vs "All state lives in R2 -- no local filesystem writes" (substack-publisher).

### Current guardrail3 behavior
- **Generate:** `generate_rust_files()` iterates `cfg.rust.apps` map. For each app, calls `clippy::build_clippy_toml(effective_profile, is_pure, local.clippy_methods, local.clippy_types)` with GLOBAL overrides. All apps get identical overrides.
- **Validate:** Checks at `primary_workspace_root()` only. Does NOT check per-app clippy.toml.

### Correct behavior
- **Category: Merge-managed**
- Generate action: Build from profile baseline + global overrides + per-app overrides. Non-destructive merge on existing files (preserve user entries not in baseline, ensure baseline entries present, preserve user reason strings).
- Why: ~80% shared baseline but meaningful project-specific bans and reasons.

### Per-app differences
- YES. validator-rust: 23 methods. substack-publisher: 25 methods. 2 unique to validator-rust (reqwest bans), 2 unique to substack-publisher (stdout/stderr). Different reasons on shared bans.
- guardrail3 MUST support per-app clippy configs.

### Override mechanism
- Per-app additions: `.guardrail3/overrides/apps/{name}/clippy-methods.toml`
- Per-app removals: `.guardrail3/overrides/apps/{name}/clippy-methods-remove.toml`
- Global overrides: `.guardrail3/overrides/clippy-methods.toml` (apply to all apps)

### What breaks right now
- Generate overwrites all custom bans with generic baseline. Project-specific reasons ("Use shared clients from LiveState", "All state lives in R2") replaced with guardrail3 defaults. substack-publisher's unique bans (stdout, stderr) lost entirely. substack-publisher not even in guardrail3.toml (not discovered by init).
- Root packages correctly get NEW clippy.toml with library profile (desirable).

---

## 2. deny.toml

### Instances in steady-parent
- `apps/validator-rust/deny.toml` (109 lines)
- `apps/substack-publisher/deny.toml` (68 lines)
- NO root `deny.toml`

### Guardrail content vs project content
**Guardrail (identical in both):**
- `[graph]`: `all-features = true`, `no-default-features = false`
- `[bans]`: `highlight = "all"`, `allow-wildcard-paths = true`
- 23 crate bans: simd-json, json5, sonic-rs, ureq, surf, isahc, log4rs, env_logger, simple_logger, fern, async-std, smol, anyhow, bincode, rmp-serde, actix-web, rocket, warp, poem, diesel, sea-orm, prost, flatbuffers, openssl
- 12 licenses: MIT, Apache-2.0, BSD-3-Clause, ISC, Unicode-3.0, BSD-2-Clause, BSL-1.0, MPL-2.0, CDLA-Permissive-2.0, OpenSSL, Zlib, CC0-1.0
- `confidence-threshold = 0.8`, `[licenses.private] ignore = true`
- `[sources]`: `unknown-registry = "deny"`, `unknown-git = "deny"`, `allow-registry = ["https://github.com/rust-lang/crates.io-index"]`, `allow-git = []`

**Project-specific (differs):**
- `[bans] multiple-versions`: validator-rust `"warn"` (no comment), substack-publisher `"warn"` with `# EXCEPTION: 18 AWS SDK transitive duplicates`
- `[bans] wildcards`: validator-rust `"allow"`, substack-publisher `"warn"` with `# EXCEPTION: transitive deps may use wildcards`
- anyhow: validator-rust `wrappers = ["texting_robots"]`, substack-publisher `wrappers = []`
- validator-rust: commented-out `[[bans.features]]` tokio section with 4-line note about spider/lychee
- validator-rust: comments about chrono being intentionally allowed, once_cell being a transitive dep
- `[advisories] ignore`: COMPLETELY different. validator-rust: RUSTSEC-2025-0057 (fxhash/scraper), RUSTSEC-2024-0388 (derivative/lychee-lib). substack-publisher: RUSTSEC-2025-0134 (rustls-pemfile/aws-sdk), RUSTSEC-2026-0009 (serde_json/aws-sdk).

### Current guardrail3 behavior
- **Generate:** `build_deny_for_profile()` builds from profile modules + global overrides. Same for all apps.
- **Validate:** Checks at `primary_workspace_root()` only.

### Correct behavior
- **Category: Merge-managed**
- Generate: Profile baseline + global + per-app overrides. Advisory ignores are 100% project-specific; the entire `[advisories] ignore` array should be treated as user-owned.
- Why: Advisory ignores, wrapper exceptions, EXCEPTION comments, and feature ban decisions are all app-specific.

### Per-app differences
- YES, major. Advisory ignores completely different. Anyhow wrappers differ. Wildcards setting differs.

### Override mechanism
- Per-app overrides for bans, skip, feature-bans.
- Advisory ignores: new override type OR treat entire `[advisories] ignore` as user-owned during merge.

### What breaks right now
- Advisory ignores lost (cargo deny will fail on real deps). Anyhow texting_robots wrapper lost. EXCEPTION comments lost. Tokio feature ban reasoning lost.

---

## 3. rustfmt.toml

### Instances in steady-parent
- `apps/validator-rust/rustfmt.toml` (11 lines: 7 stable settings + 3 commented nightly lines + 1 comment header)
- `apps/substack-publisher/rustfmt.toml` (7 lines: 7 stable settings only)
- NO root rustfmt.toml

### Guardrail content vs project content
**Guardrail:** ALL content. Both have identical stable settings: `edition = "2024"`, `max_width = 100`, `tab_spaces = 4`, `use_field_init_shorthand = true`, `use_try_shorthand = true`, `reorder_imports = true`, `reorder_modules = true`. Matches guardrail3's `canonical::RUSTFMT`.

**Project-specific:** Nothing meaningful. Only difference: commented nightly hints present/absent.

### Current guardrail3 behavior
- **Generate:** Writes `canonical::RUSTFMT.content` verbatim.
- **Validate:** Checks existence, verifies edition and max_width.

### Correct behavior
- **Category: Fully-owned**
- Full overwrite safe. No project-specific content.

### What breaks right now
- substack-publisher gains commented nightly lines and header. Harmless.

---

## 4. rust-toolchain.toml

### Instances in steady-parent
- DOES NOT EXIST anywhere.

### Current guardrail3 behavior
- **Generate:** Creates at project root: `[toolchain]\nchannel = "stable"\ncomponents = ["clippy", "rustfmt"]`.

### Correct behavior
- **Category: Fully-owned**
- Create at root. One per repo.

### What breaks right now
- CREATES new file. No destruction. Correct behavior.

---

## 5. Cargo.toml [workspace.lints]

### Instances in steady-parent
- `apps/validator-rust/Cargo.toml` lines 35-103: `[workspace.lints.rust]` (5 entries) + `[workspace.lints.clippy]` (35+ entries including allows with EXCEPTION comments)
- `apps/substack-publisher/Cargo.toml` lines 22-78: `[lints.rust]` (5 entries) + `[lints.clippy]` (35+ entries) -- direct lints, not workspace (standalone crate)
- Root `Cargo.toml`: 6 lines. NO `[workspace.lints]` at all.

### Guardrail content vs project content
**Guardrail:** Both apps have all required lint groups and specific overrides matching guardrail3's `canonical::CARGO_LINTS`.

**Project-specific:**
- validator-rust: `type_complexity = "allow"` with `# Nested BTreeMap<String, BTreeMap<String, T>> mirrors JSON spec structure`
- substack-publisher: `multiple_crate_versions = "allow"` with `# EXCEPTION: aws-sdk pulls duplicate transitive versions we cannot control`
- validator-rust: `missing_debug_implementations = "warn"`. substack-publisher: `"deny"` (stricter).
- guardrail3 baseline has `missing_docs = "deny"` -- NEITHER app has this.

### Current guardrail3 behavior
- **Generate:** Does NOT touch Cargo.toml. Prints "NOTE: Add these workspace lints to your Cargo.toml manually".
- **Validate:** R26-R29 check `[workspace.lints]` at primary_workspace_root -- which is the root Cargo.toml that has NO lints.

### Correct behavior
- **Category: Validate-only**
- toml_edit COULD safely add `[workspace.lints]` sections — the file-structure risk is manageable. The real reason to NOT write: adding lints the project doesn't already have (e.g., `missing_docs = "deny"` which guardrail3 baseline has but NEITHER app has) would break the build. Every undocumented public item becomes a compile error. We can't enforce lints via generation without potentially making the codebase uncompilable. Floor enforcement means we validate that lints exist and warn about missing ones, but the project adds them at their own pace.
- Validate per-workspace/per-crate. Root currently has no lints (intentional for virtual workspace with excluded apps).

### What breaks right now
- Validation checks wrong Cargo.toml (root has no lints). False failures for R26-R29. validator-rust's extensive lints go unchecked. substack-publisher invisible to validation.

---

## 6. Per-crate Cargo.toml (lints inheritance)

### Instances in steady-parent
- validator-rust's 5 member crates: should have `[lints] workspace = true`
- substack-publisher: has `[lints]` directly (standalone, no workspace)

### Correct behavior
- **Category: Validate-only**
- Workspace members: check `[lints] workspace = true`. Standalone: check `[lints]` has required entries.

---

## 7. release-plz.toml

### Instances in steady-parent
- DOES NOT EXIST.

### Guardrail content vs project content
release-plz.toml has 64 config fields (verified from https://release-plz.dev/docs/config):
**Guardrail (~20 fields):** `changelog_update`, `dependencies_update`, `git_release_enable`, `git_tag_enable`, `publish`, `semver_check`, `pr_draft`, `publish_no_verify`, `publish_allow_dirty`, `features_always_increment_minor`, `release`, `release_always`, `publish_timeout`, `sort_commits`, `protect_breaking_commits`, `trim`
**Project-specific (~44 fields):** `repo_url`, `git_release_name` (Tera template), `git_tag_name` (Tera template), `pr_name`/`pr_body`/`pr_labels`, `changelog_config` (cliff path), `release_commits` (regex), `publish_features`, `[[package]]` blocks (per-crate: `name`, `changelog_path`, `changelog_include`, `version_group`), `[changelog]` templates (`header`, `body`, `commit_parsers`, `commit_preprocessors`, `postprocessors`, `link_parsers`, `tag_pattern`)

### Correct behavior
- **Category: Scaffold-once (opt-in)**
- NOT fully-owned. Has project-specific templates, per-package blocks, changelog parsers, repo_url.
- Generate baseline on first run with guardrail defaults. Never overwrite — user will add [[package]] blocks, changelog templates, repo_url.
- Only generate when release checks enabled (`[rust.apps.X.checks] release = true`).

### What breaks right now
- Currently generates if profile is "service" regardless of release checks config. Would create file with guardrail3's template that lacks project's repo_url, package blocks, changelog config.

---

## 8. cliff.toml

### Instances in steady-parent
- DOES NOT EXIST.

### Correct behavior
- **Category: Scaffold-once (opt-in)**
- Same reasoning as release-plz.toml. cliff.toml has project-specific commit parsers, templates, tag patterns. Scaffold baseline, don't overwrite.

---

## 9. eslint.config.mjs (root)

### Instances in steady-parent
- `eslint.config.mjs` (487 lines). 10 plugin imports. Sections: global ignores (58 patterns), base JS+TS, Next.js, React, jsx-a11y strict, TS strict rules (30+ rules), structural health (max-lines, import-x/max-dependencies, import-x/no-cycle), test relaxations, config relaxations, hex arch for admin (boundaries plugin), landing module boundaries, CdnImage enforcement, metadata boundaries, pipeline boundaries, design token bans (53 Tailwind classes).

### Guardrail content vs project content

**Guardrail — guardrail3 dictates these (~70%, ~340 lines):**
- Plugin installations and base configs (js.configs.recommended, tseslint.configs.strictTypeChecked)
- All TS strict rules (30+ rules: no-explicit-any, strict-boolean-expressions, etc.)
- jsx-a11y strict mode + control-has-associated-label (accessibility enforcement)
- React plugin + recommended rules + jsx-no-leaked-render
- Structural health with OUR values: max-lines 400, import-x/max-dependencies (needs adding — not in engine yet), import-x/no-cycle (needs adding)
- Test relaxation rules
- Config file relaxation rules
- Hex arch enforcement for service apps: boundaries plugin with OUR layer naming (domain/application/adapters). We know the app path from guardrail3.toml, we dictate the layer names. If a project uses different layer names, they turn OFF hex arch module and manage themselves. When it's ON, we enforce our structure.
- Content isolation for content apps: no-restricted-imports that blocks ALL workspace packages by default. User adds allowed exceptions via overrides/config.
- Leaf package isolation: no-restricted-imports blocking workspace packages for leaf packages.

NOTE: Hex arch boundary config IS generatable because: (a) we dictate layer names (domain/application/adapters), (b) we know app paths from guardrail3.toml config, (c) the boundaries plugin config template is standard — only `basePattern` changes per app. Steady-parent uses `admin-domain` naming but that's just a namespace prefix that guardrail3 can generate from the app name.

NOTE: Content isolation IS generatable because we can discover workspace package names from package.json workspaces. Default: block all `@{workspace-name}/*`. User configures allowed packages.

NOTE: import-x/max-dependencies and import-x/no-cycle are NOT in the guardrail3 engine yet — steady-parent has them but guardrail3 doesn't generate them. They should be ADDED to the engine with guardrail3's chosen values.

**Project-specific (~30%, ~147 lines):**
- 58 global ignores (project paths: content/**, packages/generator/images/**, tools/**, legacy/**) — guardrail3 has default ignores (node_modules, .next, dist, target) but project adds many more
- CdnImage enforcement (project-specific component rule — no-restricted-syntax for `<img>`)
- Pipeline boundary specifics (which pipeline steps can import which — project architecture)
- Design token deny list (53 Tailwind class names — project's design system)
- Specific allowed-exception packages in content isolation (content-constraints allowed in landing)
- `no-console: 'off'` (steady-parent chose this — guardrail3 generates `no-console: error`. steady-parent overrides it in their layer.)
- `globals.browser` + `globals.node` (project choice)
- Next.js plugin config (project uses Next.js — guardrail3 doesn't mandate a framework)

### Current guardrail3 behavior
- **Generate:** ALWAYS generates via `eslint::build_eslint_config()`. ~100-line starter. NO skip flag in CanonicalConfig.
- **Validate:** Checks rule presence (T1-T8, T36-T51, T60+).

### Correct behavior
- **Category: Shadow-imported**
- Engine to `.guardrail3/generated/eslint-engine.mjs`. User's file imports it. Never overwrite user's file after scaffold.

### What breaks right now
- **CATASTROPHIC.** Replaces 487 lines with ~100 lines. Destroys hex arch enforcement, all module boundaries, design token bans, a11y config, structural health rules, 58 project-specific ignores.

---

## 10. eslint.config.mjs (per-app)

### Instances in steady-parent
- `apps/landing/eslint.config.mjs` (34 lines). Uses eslint-config-next (core-web-vitals + typescript). Module boundary: bans all `@steady-parent/*` except `content-constraints`.

### The enforcement gap problem
ESLint flat config does NOT cascade. When ESLint runs from `apps/landing/` (or with experimental `config_lookup_from_file`), landing's local config COMPLETELY replaces root. ALL guardrails from root are lost:
- No TS strict rules (no-explicit-any, strict-boolean-expressions, etc.)
- No jsx-a11y enforcement
- No structural health (max-lines, no-cycle)
- No test relaxations
- No react plugin rules

This happens when:
- IDE runs ESLint in the app directory (common with VS Code + ESLint extension)
- Project uses per-app linting (`pnpm -r run lint` where each app has its own lint script)
- ESLint experimental `config_lookup_from_file` flag is enabled

In steady-parent, `pnpm lint` runs from root, so root config applies. But other monorepos may run per-app linting, and the gap is real.

### Correct behavior
- **Category: Shadow-imported**
- Shadow-import DOES work here. ESLint flat config is an array — entries coexist. `eslint-config-next` uses the same `typescript-eslint` parser as guardrail3's engine. The engine provides rules for `**/*.ts`/`**/*.tsx`, framework config adds framework-specific rules on top. Later entries in the array override earlier ones where they conflict, but guardrail rules that the framework doesn't touch (no-explicit-any, strict-boolean-expressions, etc.) remain active.
- For Vue/Svelte apps with different parsers: the engine targets `**/*.ts` files, framework targets `**/*.vue`/`**/*.svelte` — different `files` patterns, no conflict.
- Per-app config should import the engine:
  ```js
  import engine from '../../.guardrail3/generated/eslint-engine.mjs';
  import nextVitals from 'eslint-config-next/core-web-vitals';
  export default [...engine, ...nextVitals, { /* app-specific rules */ }];
  ```
- guardrail3 scaffolds this import on first run. On subsequent generates, only `.guardrail3/generated/eslint-engine.mjs` is updated — the per-app file is never touched.
- Validate checks that per-app configs import the engine (or contain all required rules independently).

### What breaks right now
- guardrail3 completely ignores per-app eslint configs. No validation, no enforcement, no warning about the gap.

---

## 11. tsconfig.base.json

### Instances in steady-parent
- `tsconfig.base.json` (26 lines). 14 compilerOptions entries. All guardrail strict flags present.

### Guardrail content vs project content
**Guardrail (strict flags):** All 12 strict boolean flags present and correct in steady-parent.
**Project-specific:**
- `lib: ["ES2022"]` -- guardrail3 generates `["ES2022", "DOM", "DOM.Iterable"]`. Steady-parent omits DOM.
- Steady-parent lacks: `declaration`, `declarationMap`, `noEmit`, `$schema`, `_comment` (all in guardrail3's generated version).

### Correct behavior
- **Category: Merge-managed**
- Only touch compilerOptions strict boolean flags. Leave lib, declaration, etc. alone.

### What breaks right now
- Overwrites: changes lib to include DOM/DOM.Iterable (wrong for Node packages). Adds declaration/declarationMap/noEmit. Adds $schema and _comment keys.

---

## 12. Per-app tsconfig.json

### Instances in steady-parent
- `apps/landing/tsconfig.json` (25 lines): EXTENDS base. Adds target ES2017, DOM libs, jsx, Next.js plugin, path aliases.
- `apps/admin/tsconfig.json` (65 lines): STANDALONE. ALL strict flags duplicated. Adds target ES2017, DOM libs, allowImportingTsExtensions, hex-arch path aliases (@modules/*, @domain/*, @adapters/*, @commands/*).

### Correct behavior
- **Category: Validate-only**
- Check exists. If extends base: OK. If standalone (admin): check all strict flags present.
- Never generate (path aliases, plugins, include/exclude are 100% project-specific).

### What breaks right now
- No per-app tsconfig validation at all.

---

## 13. .stylelintrc.mjs

### Instances in steady-parent
- `.stylelintrc.mjs` (74 lines). Extends standard + tailwindcss. 11 a11y rules. Architecture exceptions (dark mode: `media-prefers-color-scheme: null`; `no-duplicate-selectors: null`). CSS notation (lightness-notation: number, hue-degree-notation: null, alpha-value-notation: number). Quality overrides for Tailwind compatibility. ignoreFiles array.

### Guardrail content vs project content
**Guardrail (~45 lines):** extends (standard + tailwindcss), a11y plugin, 11 a11y rules, architecture exceptions (media-prefers-color-scheme: null — we use class-based dark mode; no-duplicate-selectors: null — we use separate :root blocks for themes), base ignoreFiles (node_modules, .next, dist, target, coverage)
**Project (~29 lines):** CSS notation conventions (lightness-notation, hue-degree-notation, alpha-value-notation — oklch preferences), Tailwind v4 compatibility rules (custom-property-pattern, selector-class-pattern, declaration-block-*), project-specific ignoreFiles (legacy, .velite)

### Correct behavior
- **Category: Shadow-imported**
- Engine to `.guardrail3/generated/stylelint-engine.mjs`. User imports and extends.

### What breaks right now
- **DESTRUCTIVE.** Overwrites 74 lines. Loses dark mode exception, CSS notation preferences, Tailwind compatibility rules. No skip flag exists.

---

## 14. cspell.json

### Instances in steady-parent
- DOES NOT EXIST.

### Correct behavior
- **Category: Shadow-imported (if exists) / Scaffold-once (if new)**
- If file doesn't exist: scaffold a complete cspell.json (current behavior, correct).
- If file already exists with custom words: cspell supports `import` — could shadow-import base to `.guardrail3/generated/cspell-base.json`, user's file adds `"import": [".guardrail3/generated/cspell-base.json"]` plus their `words` array. This cleanly separates guardrail settings (language, version, ignorePaths) from user content (words, project dictionaries).
- Calling this "scaffold-once with merge" was muddled — it's really shadow-import for existing projects and scaffold for new ones.

---

## 15. .npmrc

### Instances in steady-parent
- `.npmrc` (37 lines). All 13 key=value settings match guardrail3 exactly. Detailed comments including undici-types note on trust-policy.

### Correct behavior
- **Category: Merge-managed**
- Ensure settings present. Preserve comments and additional settings.

### What breaks right now
- Overwrites comments (including undici-types context). Values functionally identical.

---

## 16. .jscpd.json

### Instances in steady-parent
- Root `.jscpd.json` (28 lines): threshold 10, minTokens 50, reporters consoleFull, 18 ignore patterns (includes project-specific: `content/**`, `packages/validation/**`, `**/legacy/**`), format ["typescript", "rust"].
- `apps/validator-rust/.jscpd.json` (9 lines): threshold 5, reporters console, language ["rust"], ignore ["target/**", "tests/**"], minLines 5, minTokens 50, absolute true.

### Correct behavior
- **Category: Merge-managed**
- Ensure base ignore patterns present. Do NOT change threshold — the threshold is meaningless without the correct ignore set. guardrail3's default ignore list doesn't include project-specific paths (content/**, legacy/**, packages/validation/**). Setting threshold 0 with an incomplete ignore list floods with false positives. The project chose threshold 10 because it matches THEIR ignore patterns. When we merge, we can add our default ignores but the threshold stays.
- Per-app jscpd: validate-only (project-specific).

### What breaks right now
- Threshold changes from 10 to 0 (reports everything). Loses project-specific ignores. Adds patterns not in project (`**/.claude/**`, `**/test-data/**`).

---

## 17. package.json (root)

### Instances in steady-parent
- `package.json` (52 lines). name "steady-parent", packageManager pnpm@10.32.0, scripts, 1 dependency (zod-to-json-schema), 16 devDependencies (ESLint plugins + tooling), engines node>=24, pnpm.overrides.

### Correct behavior
- **Category: Validate-only**
- Check devDependencies, scripts, banned packages. Never write.

---

## 18. Per-app package.json

### Instances in steady-parent
- `apps/landing/package.json` (89 lines): 40+ deps, devDependencies, scripts.
- `apps/admin/package.json` (40 lines): ~15 deps, devDependencies, scripts, engines node>=22.

### Correct behavior
- **Category: Validate-only**

### What breaks right now
- Per-app package.json not validated at all.

---

## 19. .githooks/pre-commit

### Instances in steady-parent
- `.githooks/pre-commit` (283 lines). Monorepo-aware with: secret scanning (gitleaks), file size check (1MB), migration consistency check, content-constraints staleness check, per-app TS type checking loop, ESLint with --max-warnings 0, Stylelint on CSS, per-app Rust check loop (fmt/clippy/deny/machete/test for each app in apps/*/), structural health (500 lines, 20 uses, no crate-wide allow), copy-paste detection (jscpd --threshold 10), guardrail tamper detection (eslint-disable without reason, #[allow] without justification, config relaxation detection).

### Guardrail content vs project content
**Guardrail (standard steps ~40%):** Secret scanning, file size, cargo fmt/clippy/deny/machete/test, structural health, tamper detection pattern.
**Project-specific (~60%):** Migration consistency, content-constraints staleness, per-app TS type checking, Stylelint integration, jscpd threshold 10 (matching .jscpd.json), specific tamper detection patterns, monorepo-aware iteration loops.

### Current guardrail3 behavior
- **Generate:** `generate_and_install_hooks()` generates via `pre_commit::build_pre_commit_script()`. Produces a generic ~50-line hook. Always overwrites.

### Correct behavior
- **Category: Scaffold-once** (with a significant limitation)
- Generate starter on first run only. Never overwrite if user has customized.
- Detection: if file exists and does NOT have `Generated by guardrail3` marker, refuse to overwrite.
- **Limitation:** scaffold-once means guardrail steps can NEVER be updated in existing hooks. If guardrail3 adds a new check next month, the user's hook doesn't get it. This is the strongest argument for the source-based alternative: guardrail3 generates checks to `.guardrail3/generated/pre-commit-checks.sh`, user's hook `source`s it. guardrail3 can update the generated file; user's project-specific steps remain untouched. The tradeoff: more complex setup vs updateable guardrail steps.

### What breaks right now
- **DESTRUCTIVE.** Replaces 283-line monorepo-aware hook with ~50-line generic script. Loses: migration consistency, content-constraints staleness, per-app TS type checking, Stylelint, monorepo Rust loop, sophisticated tamper detection.

---

## 20. prettier config

### Instances in steady-parent
- DOES NOT EXIST. No prettier config files found.

### Correct behavior
- **Category: Not-managed**

---

## Corrected Category Table

| # | File | Category | Generate? | Per-app? |
|---|------|----------|-----------|----------|
| 1 | clippy.toml | **Merge-managed** | Yes (non-destructive merge) | Yes |
| 2 | deny.toml | **Merge-managed** | Yes (non-destructive merge) | Yes |
| 3 | rustfmt.toml | **Fully-owned** | Yes (full overwrite) | No |
| 4 | rust-toolchain.toml | **Fully-owned** | Yes (root only) | No |
| 5 | Cargo.toml [workspace.lints] | **Validate-only** | No | Per-workspace |
| 6 | Per-crate Cargo.toml [lints] | **Validate-only** | No | Per-crate |
| 7 | release-plz.toml | **Scaffold-once (opt-in)** | First run only, if release=true | No |
| 8 | cliff.toml | **Scaffold-once (opt-in)** | First run only, if release=true | No |
| 9 | eslint.config.mjs (root) | **Shadow-imported** | Engine to shadow | No |
| 10 | eslint.config.mjs (per-app) | **Shadow-imported** | Engine import + scaffold | Per-app |
| 11 | tsconfig.base.json | **Merge-managed** | Merge strict flags only | No |
| 12 | Per-app tsconfig.json | **Validate-only** | No | Per-app |
| 13 | .stylelintrc.mjs | **Shadow-imported** | Engine to shadow | No |
| 14 | cspell.json | **Scaffold-once** | First run + structural merge | No |
| 15 | .npmrc | **Merge-managed** | Merge key=value | No |
| 16 | .jscpd.json | **Merge-managed** | Merge ignores only | Aware |
| 17 | package.json (root) | **Validate-only** | No | N/A |
| 18 | Per-app package.json | **Validate-only** | No | Per-app |
| 19 | .githooks/pre-commit | **Scaffold-once** | First run only | No |
| 20 | prettier config | **Not-managed** | No | N/A |

### Category Definitions

- **Fully-owned:** guardrail3 generates the entire file. Full overwrite on regenerate. No user content expected.
- **Merge-managed:** guardrail3 ensures its baseline entries are present and correct. User entries preserved. Non-destructive editing.
- **Shadow-imported:** guardrail3 generates engine file to `.guardrail3/generated/`. User's config imports it. Engine overwritten freely; user's file never touched after scaffold.
- **Scaffold-once:** guardrail3 creates file on first run. Never overwrites. May merge structural settings later.
- **Validate-only:** guardrail3 never generates or modifies. Only checks during validation.
- **Not-managed:** guardrail3 does not touch or check this file.

### Key corrections from V1

1. **Cargo.toml [workspace.lints]:** V1 said "Merge-managed". WRONG — but not for the originally stated reason ("too much project content"). toml_edit CAN safely add sections. The real reason: adding guardrail3's baseline lints (e.g., `missing_docs = "deny"`) to a project that doesn't have them would break the build. Correct: **Validate-only**. guardrail3 checks and warns, project adds lints at their own pace.

2. **.githooks/pre-commit:** V1 said "Fully-owned". WRONG. steady-parent's 283-line hook is 60% project-specific (migration checks, content-constraints, monorepo loops). Correct: **Scaffold-once**. Generate a starter, never overwrite user's customized hook.

3. **.jscpd.json:** V1 said threshold question was open. RESOLVED: the threshold is a project choice (10 in steady-parent). guardrail3 should merge ignore patterns but leave threshold alone. Correct: **Merge-managed** (ignores only, not threshold).

4. **cspell.json:** V1 said "Merge-managed / Fully-owned". CLARIFIED: Since it doesn't exist in steady-parent, first generation creates it (scaffold). Subsequent runs must preserve `words` array. Correct: **Scaffold-once** with structural merge.

5. **release-plz.toml:** V1 said "Fully-owned (opt-in)". WRONG. Has 64 config fields, ~44 project-specific (repo_url, PR templates, per-package blocks, changelog templates, version groups). Correct: **Scaffold-once (opt-in)**.

6. **eslint.config.mjs guardrail/project split:** V1 said ~40% guardrail / ~60% project. V2 corrected to ~70% guardrail / ~30% project. Hex arch enforcement IS guardrail (we dictate layer naming: domain/application/adapters). Content isolation IS guardrail (block all workspace imports by default). Structural health values (max-lines 400, max-dependencies, no-cycle) ARE guardrail — we're opinionated, we dictate. These are generatable because we know app paths from config and we dictate the conventions.

7. **Per-app eslint.config.mjs:** V2 corrected to **Shadow-imported**. Shadow-import works because ESLint flat config is an array where entries coexist. Framework configs (eslint-config-next, Vue, Svelte) use compatible parsers or target different file patterns. No fundamental conflict.

8. **Pre-commit hook alternative model:** Scaffold-once is the safe default, but worth considering: guardrail3 generates guardrail checks to `.guardrail3/generated/pre-commit-checks.sh`, user's hook sources it. This lets guardrail steps be updated without touching project steps.

---

## Implementation Priority

### P0: Prevent destruction (CRITICAL — immediate)

1. **eslint.config.mjs: Stop overwriting.** TEMPORARY fix: add `eslint: Option<bool>` to `CanonicalConfig` (default false) so existing projects aren't destroyed. This is a band-aid — `ts generate` does nothing useful for ESLint until shadow-import (P2) is implemented. The real fix is P2 item 14.

2. **.stylelintrc.mjs: Stop overwriting.** Same temporary fix: add `stylelint: Option<bool>` to `CanonicalConfig` (default false). Real fix is shadow-import in P2.

3. **pre-commit hook: Stop overwriting.** Check for guardrail3 marker before overwrite. If file exists without marker, refuse and suggest `adopt`.

### P1: Per-app Rust config (HIGH)

4. **Per-app overrides directory.** `.guardrail3/overrides/apps/{name}/` for per-app clippy and deny overrides.

5. **Per-app removals.** `*-remove.toml` for per-app exemptions.

6. **Standalone crate discovery.** Fix init to discover `apps/*/Cargo.toml` with `[package]` but no `[workspace]`.

### P2: Shadow-import for JS configs (HIGH — real fix for P0 band-aids)

7. **Engine files to `.guardrail3/generated/`.** eslint-engine.mjs (with hex arch, content isolation, all rules), stylelint-engine.mjs (with a11y rules, architecture exceptions). This is the permanent solution to the P0 destructive overwrite problem.

8. **Scaffold user files.** Create importing root configs on first run. Scaffold per-app configs that import the engine.

9. **Validate import chain.** Check that root and per-app configs import the engine.

### P3: Non-destructive editing for TOML/JSON/INI (HIGH)

10. **TOML merge (clippy.toml, deny.toml).** toml_edit for comment preservation. Ensure baseline entries present, preserve user entries and reasons.

11. **JSON merge (tsconfig.base.json).** Only touch compilerOptions strict booleans.

12. **JSON merge (.jscpd.json).** Ensure base ignore patterns. Leave threshold.

13. **INI merge (.npmrc).** Ensure settings. Preserve comments.

### P4: Validation fixes (MEDIUM)

14. **Per-workspace validation.** Fix single workspace_root assumption.

15. **Per-app tsconfig validation.** Discover and check per-app tsconfigs.

16. **Per-app package.json validation.** Banned packages check.

### P5: Quality of life (LOW)

17. **Opt-in release-plz/cliff.** Don't generate unless configured.

18. **Advisory ignore overrides.** New override type for deny.toml.

19. **Per-app jscpd awareness.** Discover per-app jscpd configs.

---

## Open Questions

1. **Should clippy.toml merge preserve user reason strings?** When baseline ban has different reason than user's file, preserve user's project-specific reason? (Recommendation: yes -- "Use shared clients from LiveState" is more valuable than guardrail3's generic reason.)

2. **Should deny.toml advisory ignores be managed via overrides or inline?** Advisory ignores change frequently. Options: (a) per-app override file for advisories, (b) treat entire `[advisories] ignore` as user-owned during merge. (Recommendation: option b -- simpler, advisory ignores are 100% project-specific.)

3. **Pre-commit hook scaffold pattern?** Options: (a) only generate if absent, (b) markers with overwrite between markers, (c) generate base to `.guardrail3/generated/` and user's hook sources it. (Recommendation: option a -- simplest, matches how real projects work.)

4. **Shadow-import mandatory for new projects?** Support both "shadow-import" mode and "standalone" mode with `[typescript.eslint] mode = "engine" | "standalone"` config switch? (Recommendation: yes, with "engine" as default for new projects.)

5. **Root workspace clippy.toml:** Steady-parent's root workspace members have NO clippy.toml. guardrail3 would create one with library profile. Desirable? (Recommendation: yes, if `[rust.packages]` exists in guardrail3.toml.)

6. **Cargo.toml lints management:** Should guardrail3 eventually support writing `[workspace.lints]` into Cargo.toml via toml_edit? (Recommendation: defer. Validate-only is safer. Cargo.toml has too much project content to risk.)
