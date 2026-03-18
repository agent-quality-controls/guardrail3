# Override Architecture for guardrail3

**Date:** 2026-03-18
**Status:** Design proposal

## Problem Statement

guardrail3 generates config files (clippy.toml, deny.toml, eslint.config.mjs, stylelint, cspell, etc.) and overwrites them on `generate`. Users need to:

1. Add project-wide custom entries (e.g., extra clippy bans shared across all apps)
2. Add per-app customizations (e.g., validator-rust bans `reqwest::Client::new` but substack-publisher does not)
3. Remove or weaken specific entries for specific apps (e.g., substack-publisher needs `std::fs` which is globally banned)
4. Not lose any of these customizations when regenerating

The current system only supports global overrides via `.guardrail3/overrides/` flat files, with no per-app scoping and no TypeScript support.

---

## 1. Override Directory Structure Analysis

### Option A: Flat global (current)

```
.guardrail3/overrides/
  clippy-methods.toml
  clippy-types.toml
  deny-bans.toml
  deny-skip.toml
  deny-feature-bans.toml
```

- **Discovery:** Convention-based, single directory. Simple glob.
- **UX:** Clear for simple projects. Breaks down when different apps need different rules.
- **Scalability:** Does not scale. With 10 apps, you either over-ban everything or under-ban everything.
- **Tooling:** Fine. Single directory, easy to review.
- **Maintenance:** Good for single-app. Impossible to see per-app differences.

### Option B: Per-app subdirectories

```
.guardrail3/overrides/
  clippy-methods.toml              # global
  validator-rust/
    clippy-methods.toml            # validator-rust only
  substack-publisher/
    clippy-methods.toml            # substack-publisher only
    clippy-methods-remove.toml     # removals for this app
```

- **Discovery:** `overrides/{app-name}/` matches `[rust.apps.{app-name}]` config key.
- **UX:** Intuitive. "I want to customize validator-rust" -> go to `overrides/validator-rust/`.
- **Scalability:** Each app gets its own directory. 10 apps = 10 directories, each self-contained.
- **Tooling:** Git handles subdirectories well. IDE tree shows clear hierarchy.
- **Maintenance:** Excellent. `ls overrides/` shows all customized apps at a glance. `diff overrides/validator-rust/ overrides/substack-publisher/` shows differences.

### Option C: Per-app naming convention

```
.guardrail3/overrides/
  clippy-methods.toml
  clippy-methods.validator-rust.toml
  clippy-methods.substack-publisher.toml
```

- **Discovery:** Parse filename: `{type}.{app-name}.toml` vs `{type}.toml` (global).
- **UX:** Workable but visually noisy. Hard to see what belongs to which app when there are many files.
- **Scalability:** Gets messy. 10 apps x 5 file types = 50+ files in one directory.
- **Tooling:** Flat directory, easy to glob. But no visual grouping.
- **Maintenance:** Poor. You can't easily see "what's customized for validator-rust?" without filtering.

### Option D: Co-located (next to app)

```
apps/validator-rust/.guardrail3/
  clippy-methods.toml
apps/substack-publisher/.guardrail3/
  clippy-methods.toml
```

- **Discovery:** Walk app directories, look for `.guardrail3/` in each.
- **UX:** Feels natural for monorepo developers. "Customization lives next to the code it affects."
- **Scalability:** Scales naturally. Each app owns its overrides.
- **Tooling:** Git handles it fine. IDEs show it in context.
- **Maintenance:** Hard to get a cross-project view. "What bans apply everywhere?" requires checking every app directory.

### Option E: Hybrid (global defaults + per-app overrides)

```
.guardrail3/overrides/
  clippy-methods.toml              # applied to ALL apps
  clippy-types.toml                # applied to ALL apps
  deny-bans.toml                   # applied to ALL apps
  deny-skip.toml                   # applied to ALL apps
  deny-feature-bans.toml           # applied to ALL apps
  eslint-rules.json                # applied to root eslint
  cspell-words.json                # applied to root cspell
  stylelint-rules.json             # applied to root stylelint
  apps/
    validator-rust/
      clippy-methods.toml          # ADD entries for this app
      clippy-methods-remove.toml   # REMOVE entries for this app
      clippy-types.toml
      deny-bans.toml
      deny-skip.toml
    substack-publisher/
      clippy-methods-remove.toml   # remove std::fs bans
    landing/
      eslint-rules.json            # extra ESLint rules for landing
      cspell-words.json            # extra cspell words for landing
    admin/
      eslint-rules.json            # hex arch rules for admin
```

- **Discovery:** `overrides/` for global, `overrides/apps/{name}/` for per-app. App names match guardrail3.toml keys.
- **UX:** Best of both worlds. Global customizations in one place, per-app in subdirectories. Clear hierarchy.
- **Scalability:** Excellent. Global overrides are shared. Per-app only where needed. Most apps may have no per-app overrides.
- **Tooling:** Clean git diffs. IDE shows grouped structure.
- **Maintenance:** `ls overrides/apps/` shows which apps are customized. `overrides/` root shows global policy.

### Decision: Option E (Hybrid)

Option E is the clear winner. It preserves backward compatibility (existing `overrides/` files keep working as global), adds per-app support cleanly, and the directory structure makes the override hierarchy visually obvious.

The `apps/` subdirectory mirrors the `[rust.apps.*]` / `[typescript.apps.*]` config keys, creating a consistent mental model.

---

## 2. Override Merge Semantics

### Merge Order and Precedence

The merge pipeline for each generated config file:

```
Generated base (from profile + modules)
    + Global overrides (.guardrail3/overrides/{file})         → ADDITIVE
    + Per-app overrides (.guardrail3/overrides/apps/{app}/{file})  → ADDITIVE
    - Per-app removals (.guardrail3/overrides/apps/{app}/{file}-remove) → SUBTRACTIVE
    = Final config written to disk
```

**Precedence rules:**
1. Generated base is the foundation (never weakened by global overrides)
2. Global overrides ADD entries to the base for ALL apps
3. Per-app overrides ADD entries to this specific app only
4. Per-app removals REMOVE specific entries from the base+global for this app only
5. If global and per-app both add the same entry, it appears once (deduplication, as current system does)

### TOML configs (clippy.toml, deny.toml)

TOML array entries are identified by their `path` (clippy) or `name` (deny) key. This is the merge key.

**Adding entries (clippy-methods.toml, clippy-types.toml, deny-bans.toml):**
```toml
# .guardrail3/overrides/apps/validator-rust/clippy-methods.toml
{ path = "reqwest::Client::builder", reason = "Use shared clients from LiveState" },
{ path = "std::env::var", reason = "Use config module" },
```

These are appended to the `disallowed-methods` array after the base + global entries.

**Removing entries (clippy-methods-remove.toml):**
```toml
# .guardrail3/overrides/apps/substack-publisher/clippy-methods-remove.toml
# Entries listed here are REMOVED from the generated clippy.toml for this app.
# Match is by `path` value only.
{ path = "std::fs::read_to_string" },
{ path = "std::fs::write" },
{ path = "std::fs::read_dir" },
```

The removal file uses the same TOML format but only the `path` (or `name` for deny) field matters. The `reason` field is optional and ignored during removal matching -- but can be used to document WHY the removal is needed.

**Deny skip entries (deny-skip.toml):**
Skip entries are inherently additive -- you're adding crates to the "allowed duplicates" list. No removal semantic needed.

**Deny feature bans (deny-feature-bans.toml):**
Feature bans are `[[bans.features]]` table arrays. Per-app can add new feature bans. Removal is supported by matching on `name`.

### Why removals need a separate file

Mixing additions and removals in the same file creates ambiguity. Does `{ path = "std::fs::write" }` in a per-app override mean "add this ban" or "remove this ban"? The separate `-remove` suffix eliminates ambiguity completely:

- `clippy-methods.toml` = entries to ADD
- `clippy-methods-remove.toml` = entries to REMOVE

This is explicit, auditable, and impossible to confuse.

### Threshold overrides

Clippy thresholds (`too-many-lines-threshold`, etc.) are scalar values, not array entries. They need a different mechanism.

```toml
# .guardrail3/overrides/apps/substack-publisher/clippy-thresholds.toml
too-many-lines-threshold = 100
cognitive-complexity-threshold = 20
```

Only values present in the override file replace the base. Missing values keep the base defaults. This is a simple key-value overlay.

---

## 3. JavaScript Configs: The Two-File Model

### Why single-file fails for JS

TOML configs are data. You can mechanically append entries to an array. JavaScript configs are *code*. You cannot mechanically insert arbitrary JS into a programmatically-generated config without creating a fragile template engine.

The current system generates `eslint.config.mjs` as a complete file and overwrites it on regenerate. There is no override mechanism for JS.

### The shadow-base pattern

guardrail3 generates a base config to a *shadow location*. The user's config (at the tool-expected path) imports and extends it:

```
.guardrail3/generated/
  eslint.base.config.mjs        # GENERATED — guardrail3 owns this
  .stylelintrc.base.mjs         # GENERATED — guardrail3 owns this
  cspell.base.json              # GENERATED — guardrail3 owns this

eslint.config.mjs               # USER-OWNED — imports the base, adds custom rules
.stylelintrc.mjs                # USER-OWNED — imports the base, adds custom rules
cspell.json                     # USER-OWNED — extends the base, adds custom words
```

**Generated base (guardrail3 writes, regenerate-safe):**
```js
// .guardrail3/generated/eslint.base.config.mjs
// GENERATED by guardrail3 — do not edit manually
// Regenerate with: guardrail3 generate
import js from "@eslint/js";
import tseslint from "typescript-eslint";
// ... full generated config
export default tseslint.config(
  js.configs.recommended,
  // ... all the generated rules
);
```

**User config (user owns, never overwritten):**
```js
// eslint.config.mjs
// This file is USER-MANAGED — guardrail3 will not overwrite it.
// Edit freely. The base config is regenerated separately.
import base from "./.guardrail3/generated/eslint.base.config.mjs";

export default [
  ...base,
  // Project-specific rules:
  {
    files: ["apps/admin/**/*.ts"],
    rules: {
      "boundaries/element-types": [2, { /* hex arch rules */ }],
    },
  },
  {
    files: ["apps/landing/**/*.ts"],
    rules: {
      "no-restricted-imports": [2, { patterns: ["@steady-parent/*", "!@steady-parent/content-constraints"] }],
    },
  },
];
```

### How `generate` handles the two-file model

**First run (`guardrail3 generate` or `guardrail3 ts init`):**
1. Write `.guardrail3/generated/eslint.base.config.mjs` (always)
2. If `eslint.config.mjs` does NOT exist, generate a starter user config that imports the base
3. If `eslint.config.mjs` DOES exist, leave it alone (user owns it)

**Subsequent runs (`guardrail3 generate`):**
1. Overwrite `.guardrail3/generated/eslint.base.config.mjs` (always safe -- guardrail3 owns it)
2. Never touch `eslint.config.mjs` (user owns it)

### JSON configs (cspell.json, .jscpd.json)

JSON supports a similar pattern but with native extension mechanisms:

**cspell.json (user-owned):**
```json
{
  "import": [".guardrail3/generated/cspell.base.json"],
  "words": ["substack", "openviking", "guardrail3"],
  "ignorePaths": ["apps/legacy/**"]
}
```

cspell natively supports `"import"` for extending base configs. The user's file imports the generated base and adds project-specific words and ignore paths.

**.jscpd.json** does not support imports/extends, but it rarely needs customization. Keep it as a single generated file with override support via a structured JSON override:

```json
// .guardrail3/overrides/jscpd-overrides.json
{
  "ignore": ["apps/legacy/**"]
}
```

The generate command deep-merges this into the base .jscpd.json (arrays are concatenated, scalars are replaced).

### Stylelint

Same as ESLint -- shadow-base pattern:

```js
// .guardrail3/generated/.stylelintrc.base.mjs — GENERATED
export default {
  extends: ["stylelint-config-standard", "stylelint-config-tailwindcss"],
  plugins: ["@double-great/stylelint-a11y"],
  rules: { /* generated rules */ },
};
```

```js
// .stylelintrc.mjs — USER-OWNED
import base from "./.guardrail3/generated/.stylelintrc.base.mjs";
export default {
  ...base,
  rules: {
    ...base.rules,
    // Project-specific:
    "declaration-property-value-no-unknown": null,
    "color-function-notation": ["modern", { disableFix: true }],
  },
};
```

---

## 4. Per-App Override Scenarios

### Scenario A: validator-rust needs extra clippy bans that substack-publisher does not

**User action:**
```bash
mkdir -p .guardrail3/overrides/apps/validator-rust
cat > .guardrail3/overrides/apps/validator-rust/clippy-methods.toml << 'EOF'
{ path = "reqwest::Client::builder", reason = "Use shared clients from LiveState" },
{ path = "std::env::var", reason = "Use config module" },
EOF
```

**How generate applies it:**
1. Build base clippy.toml from profile modules
2. Append global overrides from `.guardrail3/overrides/clippy-methods.toml`
3. For validator-rust specifically, also append entries from `.guardrail3/overrides/apps/validator-rust/clippy-methods.toml`
4. Deduplicate (if global and per-app both add the same entry)
5. Write to `apps/validator-rust/clippy.toml`

substack-publisher's clippy.toml gets the base + global overrides only (no per-app overrides exist for it).

**Dry-run report:**
```
apps/validator-rust/clippy.toml
  base: service profile (7 method modules, 4 type modules)
  + global overrides: 0 entries
  + per-app overrides: 2 entries (clippy-methods.toml)
  - per-app removals: 0 entries
  = 25 disallowed methods, 9 disallowed types

apps/substack-publisher/clippy.toml
  base: service profile (7 method modules, 4 type modules)
  + global overrides: 0 entries
  + per-app overrides: none
  - per-app removals: none
  = 23 disallowed methods, 9 disallowed types
```

### Scenario B: landing app needs different ESLint rules than admin

Both use the same generated base ESLint config. Per-app ESLint customization happens in the **user-owned** `eslint.config.mjs` via file-scoped rule blocks:

```js
// eslint.config.mjs (USER-OWNED)
import base from "./.guardrail3/generated/eslint.base.config.mjs";

export default [
  ...base,
  // Landing: restrict cross-package imports
  {
    files: ["apps/landing/**/*.{ts,tsx}"],
    rules: {
      "no-restricted-imports": [2, {
        patterns: [{ group: ["@steady-parent/*", "!@steady-parent/content-constraints"] }]
      }],
    },
  },
  // Admin: enforce hex arch boundaries
  {
    files: ["apps/admin/**/*.{ts,tsx}"],
    plugins: { boundaries },
    rules: {
      "boundaries/element-types": [2, { /* layer definitions */ }],
    },
  },
];
```

This is not an "override" in guardrail3's sense -- it's standard ESLint config composition. The user owns the file and adds app-specific blocks. guardrail3 never touches it after initial generation.

guardrail3's **validate** command still checks that the base rules are present (by inspecting the import chain or checking rule presence). If the user deletes the base import, validate flags it.

### Scenario C: Package needs custom cspell dictionary

**Option 1: Root cspell.json with per-package word lists**

```json
// cspell.json (USER-OWNED)
{
  "import": [".guardrail3/generated/cspell.base.json"],
  "words": ["substack", "openviking"],
  "overrides": [
    {
      "filename": "packages/generator/**",
      "words": ["pipeline", "taxonomy", "embeddings"]
    }
  ]
}
```

cspell natively supports `overrides` with filename globs. This is the preferred approach -- single config, per-path word lists.

**Option 2: Per-package cspell.json**

Not recommended. cspell walks up to find config and the inheritance model is messy. Keep one root config with overrides sections.

guardrail3 does NOT manage the per-package word lists. It generates the base config; the user adds words in the user-owned root `cspell.json`.

### Scenario D: Admin's tsconfig does not extend base

**Current behavior:** guardrail3 generates `tsconfig.base.json`. It does NOT generate per-app `tsconfig.json` files. Per-app tsconfigs are user-managed.

**Problem:** validate check T9 expects per-app tsconfigs to extend the base. Admin has a standalone tsconfig with `target: "ES2017"`.

**Solution: Exemption list in guardrail3.toml**

```toml
[typescript.apps.admin]
type = "service"

[typescript.apps.admin.checks]
# Exempt admin from tsconfig base enforcement
tsconfig_extends_base = false
```

validate reads this config and skips T9 for admin. The exemption is explicit, auditable, and lives in the project config (not hidden in override files).

**Generalized pattern:** Any check can be exempted per-app via `[*.apps.{name}.checks]`. This is already partially supported for Rust (`architecture`, `garde`, `tests`, `release`). Extend to TypeScript and add per-check granularity.

### Scenario E: App wants to REMOVE a global clippy ban

**User action:**
```bash
mkdir -p .guardrail3/overrides/apps/substack-publisher
cat > .guardrail3/overrides/apps/substack-publisher/clippy-methods-remove.toml << 'EOF'
# substack-publisher writes files to disk — allow std::fs operations
{ path = "std::fs::read_to_string", reason = "File-based publishing pipeline" },
{ path = "std::fs::write", reason = "File-based publishing pipeline" },
{ path = "std::fs::read_dir", reason = "File-based publishing pipeline" },
{ path = "std::fs::read", reason = "File-based publishing pipeline" },
{ path = "std::fs::read_link", reason = "File-based publishing pipeline" },
{ path = "std::fs::remove_file", reason = "File-based publishing pipeline" },
{ path = "std::fs::remove_dir_all", reason = "File-based publishing pipeline" },
{ path = "std::fs::create_dir_all", reason = "File-based publishing pipeline" },
{ path = "std::fs::rename", reason = "File-based publishing pipeline" },
{ path = "std::fs::copy", reason = "File-based publishing pipeline" },
{ path = "std::fs::metadata", reason = "File-based publishing pipeline" },
{ path = "std::fs::symlink_metadata", reason = "File-based publishing pipeline" },
{ path = "std::fs::canonicalize", reason = "File-based publishing pipeline" },
{ path = "std::fs::set_permissions", reason = "File-based publishing pipeline" },
{ path = "std::fs::hard_link", reason = "File-based publishing pipeline" },
EOF
```

**How generate processes removals:**
1. Build base + global overrides (full method list)
2. Parse the `-remove.toml` file, extract the `path` values
3. Filter out any entry from the method list whose `path` matches a removal
4. Write the filtered clippy.toml

The `reason` field in removal files is for documentation only. It appears in the dry-run report and serves as an audit trail for WHY the ban was removed.

**Dry-run report:**
```
apps/substack-publisher/clippy.toml
  base: service profile (7 method modules, 4 type modules)
  + global overrides: 0 entries
  + per-app overrides: 0 entries
  - per-app removals: 15 entries (clippy-methods-remove.toml)
    removed: std::fs::read_to_string — "File-based publishing pipeline"
    removed: std::fs::write — "File-based publishing pipeline"
    ... (13 more)
  = 8 disallowed methods, 9 disallowed types
  WARNING: 15 base bans removed for this app. Review removal reasons.
```

**validate behavior:** When validate sees a clippy.toml with fewer bans than expected, it currently flags this as an error. With per-app removals, validate must:
1. Read the removal file for this app
2. Adjust the "expected bans" count downward
3. Only flag genuinely missing bans (not intentionally removed ones)

---

## 5. Files guardrail3 Does Not Own

### Classification

| File | Who owns it | guardrail3 role |
|------|-------------|-----------------|
| `clippy.toml` | guardrail3 | Generates, overwrites |
| `deny.toml` | guardrail3 | Generates, overwrites |
| `rustfmt.toml` | guardrail3 | Generates, overwrites |
| `rust-toolchain.toml` | guardrail3 | Generates, overwrites |
| `.guardrail3/generated/eslint.base.config.mjs` | guardrail3 | Generates, overwrites |
| `.guardrail3/generated/cspell.base.json` | guardrail3 | Generates, overwrites |
| `.guardrail3/generated/.stylelintrc.base.mjs` | guardrail3 | Generates, overwrites |
| `eslint.config.mjs` | User | Generates starter only. Validates presence of base import. |
| `cspell.json` | User | Generates starter only. Validates import of base. |
| `.stylelintrc.mjs` | User | Generates starter only. Validates import of base. |
| `.jscpd.json` | guardrail3 | Generates with JSON override merge |
| `.npmrc` | guardrail3 | Generates, overwrites |
| `tsconfig.base.json` | guardrail3 | Generates, overwrites |
| Per-app `tsconfig.json` | User | Validates settings, respects exemptions |
| Per-app `package.json` | User | Validates scripts/deps, respects exemptions |

### Validate behavior for user-managed files

validate checks user-managed files against expected standards but respects exemptions:

```toml
# guardrail3.toml
[typescript.apps.admin.checks]
tsconfig_extends_base = false  # admin has standalone tsconfig

[typescript.apps.admin.checks]
architecture = false           # admin doesn't follow hex arch
```

When validate encounters a check failure on a user-managed file:
1. Check if the app has an exemption for this specific check
2. If exempted: report as INFO (not ERROR), include the exemption reason
3. If not exempted: report as ERROR as usual

This keeps validate honest (it always checks) while allowing intentional exceptions.

---

## 6. Override Discovery and Reporting

### `generate --dry-run` report

```
guardrail3 generate --dry-run

Override discovery:
  Global overrides: .guardrail3/overrides/
    clippy-methods.toml: 2 entries
    deny-bans.toml: 1 entry
  Per-app overrides:
    validator-rust:
      + clippy-methods.toml: 3 entries (ADD)
    substack-publisher:
      - clippy-methods-remove.toml: 15 entries (REMOVE)

Files to generate:

  apps/validator-rust/clippy.toml [CHANGED]
    base: 23 methods, 9 types
    + global: 2 methods
    + per-app: 3 methods
    = final: 28 methods, 9 types

  apps/validator-rust/deny.toml [UNCHANGED]

  apps/substack-publisher/clippy.toml [CHANGED]
    base: 23 methods, 9 types
    + global: 2 methods
    - per-app removed: 15 methods
    = final: 10 methods, 9 types
    WARNING: 15 base bans removed

  .guardrail3/generated/eslint.base.config.mjs [CHANGED]
    (diff shown)

  eslint.config.mjs [USER-OWNED — not touched]
  cspell.json [USER-OWNED — not touched]
```

### `validate` report with overrides

```
guardrail3 validate

--- apps/validator-rust ---
R4  PASS  clippy.toml: 28/28 expected method bans present
R5  PASS  clippy.toml: 9/9 expected type bans present
    INFO  Per-app overrides: +3 methods (from .guardrail3/overrides/apps/validator-rust/clippy-methods.toml)

--- apps/substack-publisher ---
R4  PASS  clippy.toml: 10/10 expected method bans present (15 removed by override)
R5  PASS  clippy.toml: 9/9 expected type bans present
    WARN  15 base bans removed by per-app override. Removal reasons:
          std::fs::read_to_string — "File-based publishing pipeline"
          ... (14 more)
```

When per-app removals weaken guardrails, validate reports them as WARN (not ERROR). The removal is intentional (the user explicitly created a removal file), but it should be visible in every validate run so the team knows which apps have weaker guardrails.

---

## 7. Proposed Architecture

### Directory structure

```
.guardrail3/
  generated/                              # guardrail3-owned output (shadow bases)
    eslint.base.config.mjs                # ESLint base — regenerate-safe
    cspell.base.json                      # cspell base — regenerate-safe
    .stylelintrc.base.mjs                 # stylelint base — regenerate-safe
  overrides/                              # user-owned customizations
    clippy-methods.toml                   # GLOBAL: extra clippy method bans
    clippy-types.toml                     # GLOBAL: extra clippy type bans
    clippy-thresholds.toml                # GLOBAL: threshold overrides
    deny-bans.toml                        # GLOBAL: extra crate bans
    deny-skip.toml                        # GLOBAL: extra skip entries
    deny-feature-bans.toml               # GLOBAL: extra feature bans
    jscpd-overrides.json                  # GLOBAL: jscpd merge overrides
    apps/                                 # PER-APP overrides
      validator-rust/
        clippy-methods.toml               # ADD method bans
        clippy-types.toml                 # ADD type bans
        clippy-thresholds.toml            # threshold overrides
        deny-bans.toml                    # ADD crate bans
      substack-publisher/
        clippy-methods-remove.toml        # REMOVE method bans
        clippy-types-remove.toml          # REMOVE type bans
      landing/
        # (no Rust overrides — it's a TS app)
      admin/
        # (no Rust overrides — it's a TS app)
```

TS per-app customization happens in the user-owned `eslint.config.mjs` via standard ESLint file-scoped rule blocks, not via override files. This is a deliberate design choice: ESLint's own composition model is superior to any override format guardrail3 could invent.

### Override file formats

**TOML addition files** (`clippy-methods.toml`, `clippy-types.toml`, `deny-bans.toml`):
```toml
# Lines must match { path = "..." } or { name = "..." } pattern
# Comments and blank lines are stripped
{ path = "some::method", reason = "Use alternative instead" },
```

**TOML removal files** (`clippy-methods-remove.toml`, `clippy-types-remove.toml`, `deny-bans-remove.toml`):
```toml
# Match by path/name only. Reason is for documentation.
{ path = "std::fs::write", reason = "This app needs filesystem access" },
```

**TOML threshold files** (`clippy-thresholds.toml`):
```toml
# Key = value pairs. Only listed keys are overridden.
too-many-lines-threshold = 100
```

**JSON override files** (`jscpd-overrides.json`):
```json
{
  "ignore": ["apps/legacy/**"]
}
```

### Merge algorithm

```
fn build_clippy_for_app(profile, app_name, global_overrides, per_app_overrides):
    # Step 1: Base from profile modules
    base_methods = profile_method_modules(profile)
    base_types = profile_type_modules(profile)
    base_thresholds = default_thresholds()

    # Step 2: Global additions
    methods = base_methods + global_overrides.clippy_methods
    types = base_types + global_overrides.clippy_types
    thresholds = base_thresholds.overlay(global_overrides.clippy_thresholds)

    # Step 3: Per-app additions
    if per_app_overrides.exists(app_name):
        methods += per_app_overrides[app_name].clippy_methods
        types += per_app_overrides[app_name].clippy_types
        thresholds = thresholds.overlay(per_app_overrides[app_name].clippy_thresholds)

    # Step 4: Per-app removals
    if per_app_overrides.has_removals(app_name):
        removal_paths = per_app_overrides[app_name].clippy_methods_remove
        methods = methods.filter(|m| m.path not in removal_paths)
        removal_types = per_app_overrides[app_name].clippy_types_remove
        types = types.filter(|t| t.path not in removal_types)

    # Step 5: Deduplicate
    methods = deduplicate_by_path(methods)
    types = deduplicate_by_path(types)

    return build_clippy_toml(thresholds, methods, types)
```

### How generate uses overrides

```
guardrail3 generate:
  1. Load guardrail3.toml
  2. Load global overrides from .guardrail3/overrides/
  3. Scan .guardrail3/overrides/apps/ for per-app directories
  4. For each app in [rust.apps.*]:
     a. Build base from profile
     b. Apply global overrides (add)
     c. Apply per-app overrides if directory exists (add)
     d. Apply per-app removals if *-remove files exist (subtract)
     e. Deduplicate
     f. Write to app's config path
  5. For TypeScript:
     a. Write .guardrail3/generated/ base files (always)
     b. If user-owned files don't exist, generate starters
     c. If user-owned files exist, skip them
  6. Report what was written
```

### How validate respects overrides

```
guardrail3 validate:
  For Rust apps:
    1. Load guardrail3.toml
    2. Load per-app removal files (if any)
    3. For each check that counts expected bans:
       - Base expected count from profile
       - Subtract removals for this app
       - Compare against actual config
    4. Report removals as WARN (visible but not failing)

  For TypeScript:
    1. Check that user-owned files import the generated base
    2. Check that generated base is not stale (same as `check` command)
    3. Per-app check exemptions from guardrail3.toml
```

### How dry-run reports on overrides

The dry-run report has three sections:

1. **Override discovery** -- what override files were found, how many entries each contains
2. **Per-file generation plan** -- for each output file, shows base + additions + removals = final counts
3. **Warnings** -- any removal files that weaken guardrails, any override files with parse errors

### Migration path from current system

**Phase 1: Backward compatible (no breaking changes)**
- Existing `.guardrail3/overrides/*.toml` files continue to work as global overrides (this is the current behavior)
- Add support for `.guardrail3/overrides/apps/{name}/` directories
- Add removal file support (`*-remove.toml`)
- Add threshold override support

**Phase 2: JS shadow-base model**
- Generate JS configs to `.guardrail3/generated/` instead of directly to tool-expected paths
- On first run, if user-owned files don't exist, generate starter configs that import the base
- On subsequent runs, only regenerate the shadow base
- Existing projects: if `eslint.config.mjs` exists and does NOT import a base, warn and suggest migration

**Phase 3: Validate integration**
- validate reads override/removal files to adjust expected counts
- Per-app check exemptions in guardrail3.toml
- Removal warnings in validate output

**Phase 4: JSON override support**
- cspell.json shadow-base with native `"import"` extension
- .jscpd.json JSON merge overrides

### Config changes (guardrail3.toml)

No new top-level config needed. Override files are discovered by convention (filesystem layout). Per-app check exemptions use the existing `[*.apps.{name}.checks]` pattern:

```toml
# Existing pattern -- already supported for Rust:
[rust.apps.guardrail3.checks]
architecture = true
garde = true

# Extended for TypeScript:
[typescript.apps.admin.checks]
tsconfig_extends_base = false

# Per-check granularity (new):
[rust.apps.substack-publisher.checks]
# When an app has removal overrides, validate automatically adjusts
# expected counts. No additional config needed.
```

The override filesystem IS the config for additions/removals. guardrail3.toml only handles check exemptions (which are validation-time, not generation-time concerns).

---

## 8. Edge Cases and Design Decisions

### What if a removal file references an entry that doesn't exist in the base?

**Decision:** Warn, don't error. The removal file might reference entries that were removed from the base in a newer guardrail3 version. Failing hard would break upgrades.

```
WARNING: clippy-methods-remove.toml references 'some::method' which is not in the base. This removal has no effect.
```

### What if global and per-app both add the same entry with different reasons?

**Decision:** Per-app reason wins (more specific). The entry appears once. This follows the standard "most specific wins" precedent.

### What if a per-app removal tries to remove a per-app addition?

**Decision:** The removal wins. Removals are processed after all additions. This handles the case where a global override adds something that a specific app doesn't want.

### What about CI? Should `.guardrail3/generated/` be committed?

**Decision:** Yes. Generated shadow bases should be committed so CI can run without running `guardrail3 generate` first. The `guardrail3 check` command verifies they're not stale.

This matches the current behavior where generated configs (clippy.toml, deny.toml) are committed.

### What about `.guardrail3/` in .gitignore?

**Decision:** `.guardrail3/` should NOT be in .gitignore. Both `overrides/` (user customizations) and `generated/` (shadow bases) must be version-controlled. The overrides are a critical part of the project's guardrail policy.

### Should validate fail if it finds hand-edits in guardrail3-owned files?

**Decision:** Yes. If `clippy.toml` contains entries not traceable to the base + global + per-app overrides, validate should flag it as "unmanaged entries detected -- use .guardrail3/overrides/ instead of editing directly." This is already partially implemented via the `check` command's staleness detection.

### How does this interact with the `diff` command?

`guardrail3 diff` shows what `generate` would change. With the override system:
- For TOML files: shows the full diff including overrides applied
- For JS shadow bases: shows diff of the generated base only
- For user-owned JS files: shows nothing (guardrail3 doesn't touch them)

### Performance: is scanning `overrides/apps/` expensive?

No. It's a single directory listing + a few file reads. The number of per-app override directories is bounded by the number of apps in the project (typically < 20). Each override file is a few lines of TOML. Total overhead: negligible.
