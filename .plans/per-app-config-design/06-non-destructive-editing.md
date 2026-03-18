# Non-Destructive Config Editing

**Date:** 2026-03-18
**Status:** Design document (not yet implemented)

## Problem Statement

guardrail3's `generate` command currently overwrites entire config files from templates. This destroys user-managed content that coexists with guardrail-managed content in the same file.

Real monorepo config files contain two categories of content:

1. **User-managed** (project-specific, MUST NOT be touched): ESLint module boundary rules, tsconfig path aliases, clippy.toml project-specific method bans, deny.toml project-specific exceptions, .jscpd.json project-specific ignore patterns, .npmrc registry configs.

2. **Guardrail-managed** (MUST be correct): specific plugins, specific rules, strict flags, baseline bans, baseline thresholds.

Current behavior: `generate` replaces category 1 along with category 2. The new model: `generate` ensures category 2 entries are present and correct while leaving category 1 untouched.

## Design Principles

### P1: "Ensure, Don't Replace"

For each guardrail entry:
- If the file doesn't exist: create from template (full generation, current behavior)
- If the file exists: patch — add missing guardrail entries, update incorrect ones, leave everything else alone

### P2: "Stricter Is Always OK"

When a user has a value that is stricter than the guardrail baseline:
- **Thresholds** (lower = stricter): user has `cognitive-complexity-threshold = 10`, guardrail says 15. User is stricter. Leave it.
- **Thresholds** (higher = looser): user has `cognitive-complexity-threshold = 50`, guardrail says 15. User is looser. **Override to guardrail value.**
- **Boolean flags** (true = stricter): user has `noUncheckedIndexedAccess = false`, guardrail says true. **Override to true.**
- **Ban lists**: user has extra bans beyond baseline. Leave them (stricter). User removed a baseline ban. **Re-add it.**
- **Rule severities**: user has `"error"` where guardrail says `"warn"`. Stricter. Leave it. User has `"off"` where guardrail says `"error"`. **Override to guardrail value.**

Rationale: guardrail3's job is to enforce a floor, not a ceiling. Anything at or above the floor is compliant. Anything below the floor gets corrected.

### P3: "Ownership Tracking via Content, Not Metadata"

We do NOT need a separate registry of "which entries are ours vs theirs." Instead:
- guardrail3 knows its own baseline entries (they're compiled into the binary as `Module` constants)
- Any entry matching a baseline key is guardrail-owned for value enforcement
- Any entry NOT matching a baseline key is user-owned and untouched
- This is exactly how `validate` already works — it checks specific known keys

### P4: "First Run = Full Generation, Subsequent Runs = Patch"

The behavior changes based on file existence:
- File absent: write the full template (identical to current behavior)
- File present: parse, merge guardrail entries, write back

This means the migration path is: nothing changes for new projects. Existing projects get non-destructive behavior automatically.

---

## Per-File-Type Strategy

### 1. TOML Files: clippy.toml, deny.toml

**Parser:** `toml` crate (already a dependency). Parse into `toml::Value`, merge, serialize back.

**Strategy:** Structured merge at the TOML value level.

#### clippy.toml

**Guardrail-owned entries:**

| Entry | Type | Strictness direction | Merge behavior |
|-------|------|---------------------|----------------|
| `too-many-lines-threshold` | integer | lower = stricter | Keep if user value <= guardrail value; override if user value > guardrail value |
| `cognitive-complexity-threshold` | integer | lower = stricter | Same |
| `too-many-arguments-threshold` | integer | lower = stricter | Same |
| `type-complexity-threshold` | integer | lower = stricter | Same |
| `max-struct-bools` | integer | lower = stricter | Same |
| `disallowed-methods` | array of objects | more entries = stricter | Union merge: ensure all baseline entries exist (by `path` key). Add missing ones. Leave user extras. Never remove user entries. |
| `disallowed-types` | array of objects | more entries = stricter | Same as disallowed-methods |

**User-owned entries (examples, left untouched):**
- Any threshold not in the guardrail baseline
- Any `disallowed-methods` entry whose `path` is not in the baseline set
- Any `disallowed-types` entry whose `path` is not in the baseline set
- Comments (see below)

**Merge algorithm for array-of-objects (disallowed-methods/types):**

```
for each baseline_entry in guardrail_baseline:
    key = baseline_entry.path
    existing = find entry in file where entry.path == key
    if existing is None:
        append baseline_entry to array
    else if existing.reason != baseline_entry.reason:
        update existing.reason to baseline_entry.reason
        (the path matches, so the ban exists, but the reason may have been updated)
    else:
        leave it alone
```

User entries (path not in baseline) are never touched.

**Merge algorithm for thresholds:**

```
for each (key, guardrail_value) in baseline_thresholds:
    existing = file[key]
    if existing is None:
        set file[key] = guardrail_value
    else if existing > guardrail_value:  // user is looser
        set file[key] = guardrail_value
    else:  // user is stricter or equal
        leave it alone
```

**Comment preservation:** TOML comments are not preserved by `toml::ser`. This is a known limitation. Options:
1. Accept that comments are lost on merge (the `# GENERATED by guardrail3` header and section comments disappear). Regenerated comments from guardrail3's own template replace them.
2. Use `toml_edit` crate instead of `toml` — it preserves comments and formatting. **This is the recommended approach** for non-destructive editing.

**Recommendation:** Use `toml_edit` for clippy.toml and deny.toml. It preserves comments, formatting, and ordering. The merge logic operates on `toml_edit::DocumentMut` instead of `toml::Value`.

#### deny.toml

**Guardrail-owned sections and entries:**

| Section | Entry | Merge behavior |
|---------|-------|----------------|
| `[graph]` | `all-features`, `no-default-features` | Ensure present with correct values |
| `[bans]` | `multiple-versions`, `wildcards`, `allow-wildcard-paths`, `highlight` | Ensure present with correct values |
| `[bans].deny` | array of `{ name, wrappers }` | Union merge by `name` key. Baseline crates always present. User extras preserved. |
| `[bans].skip` | array | Preserve user entries. guardrail3 adds none by default (only via overrides). |
| `[[bans.features]]` | tokio feature ban | Ensure the tokio entry exists with correct `deny`/`allow` lists |
| `[licenses]` | `allow` list, `confidence-threshold` | Ensure baseline licenses present. User extras preserved. `confidence-threshold` enforced at minimum. |
| `[licenses.private]` | `ignore` | Ensure `true` |
| `[advisories]` | `unmaintained`, `yanked`, `ignore` | Ensure values match. User `ignore` entries preserved (union merge). |
| `[sources]` | `unknown-registry`, `unknown-git`, `allow-registry`, `allow-git` | Ensure values match. User `allow-git` entries preserved. |

**User-owned entries (left untouched):**
- Extra crate bans in `[bans].deny` (beyond baseline)
- All entries in `[bans].skip` (project-specific duplicate exceptions)
- Extra `[[bans.features]]` entries (project-specific feature bans)
- Extra licenses in `[licenses].allow` (project-specific license allowances)
- Extra entries in `[advisories].ignore` (project-specific advisory ignores)
- Extra entries in `[sources].allow-git` (project-specific git source allowances)

**Strictness for deny.toml license list:** The license allow-list is special. More licenses = looser, fewer = stricter. guardrail3 should ensure its baseline licenses are present but NOT remove user-added licenses. Removing a license the user explicitly added would break their build. If validation wants to flag extra licenses, that's `validate`'s job, not `generate`'s.

**Strictness for `[advisories].yanked`:** guardrail3 sets `yanked = "warn"`. If user has `yanked = "deny"` (stricter), leave it. If user has `yanked = "allow"` (looser), override to `"warn"`.

Severity ordering for deny.toml values: `deny` > `warn` > `allow`. guardrail3 enforces a floor.

---

### 2. JavaScript/MJS Files: eslint.config.mjs, .stylelintrc.mjs

**Decision: Option C (Separate file import) for eslint.config.mjs. Option B (Section markers) as fallback.**

The analysis of all four options:

#### Option A: AST-based patching
Rejected. ESLint flat config is arbitrary JavaScript. The config is a function call (`tseslint.config(...)`) with spread operators, conditionals, computed properties, and imported values. Reliably patching this AST is a multi-month project that would be more complex than guardrail3 itself. Every ESLint config variation (monorepo overrides, Next.js plugin configs, custom parsers, conditional env-based rules) would need handling.

#### Option B: Section markers
Viable as a secondary mechanism. guardrail3 injects a clearly marked section:

```js
// --- guardrail3 managed rules (DO NOT EDIT between markers) ---
{
  rules: {
    "@typescript-eslint/no-explicit-any": "error",
    // ... all guardrail rules
  },
},
// --- end guardrail3 ---
```

On regenerate, only content between markers is replaced. Everything outside is preserved.

**Pros:** Simple text-based replacement. No AST parsing. Predictable.
**Cons:** Markers can be accidentally deleted. Only works for rule sections, not imports/plugins. Cannot ensure plugins are imported (user must add imports manually or markers must wrap the entire export).

#### Option C: Separate file import (RECOMMENDED for ESLint)
guardrail3 generates a standalone file that the user's config imports:

```
.guardrail3/eslint-guardrails.mjs    ← guardrail3 owns this entirely
eslint.config.mjs                     ← user owns this, imports guardrails
```

The generated file exports config blocks:

```js
// .guardrail3/eslint-guardrails.mjs
// GENERATED by guardrail3 — regenerate with: guardrail3 generate
import unicorn from "eslint-plugin-unicorn";
import regexp from "eslint-plugin-regexp";
import sonarjs from "eslint-plugin-sonarjs";
// ... all guardrail plugin imports

export const guardrailPlugins = [
  { ...unicorn.configs["flat/recommended"], rules: { /* overrides */ } },
  { ...regexp.configs["flat/recommended"], rules: { /* overrides */ } },
  { plugins: { sonarjs }, rules: { /* sonarjs rules */ } },
];

export const guardrailRules = [
  { rules: { /* core rules: max-lines, no-console, eqeqeq, etc. */ } },
  { rules: { /* TS strict rules: no-explicit-any, etc. */ } },
];

export const guardrailTestRelaxation = [
  { files: ["**/*.test.ts", "**/*.spec.ts", /* ... */], rules: { /* relaxations */ } },
];

// Content-app specific (only if has_content_app)
export const guardrailA11y = [ /* jsx-a11y config */ ];
export const guardrailTailwindBan = [ /* tailwind-ban config */ ];

// Service-app specific (only if has_service_app)
export const guardrailBoundaries = [ /* boundaries config */ ];
```

The user's `eslint.config.mjs` imports and spreads:

```js
import { guardrailPlugins, guardrailRules, guardrailTestRelaxation } from "./.guardrail3/eslint-guardrails.mjs";

export default tseslint.config(
  js.configs.recommended,
  ...tseslint.configs.strictTypeChecked,
  ...guardrailPlugins,
  ...guardrailRules,
  // User's own rules below — never touched by guardrail3
  { rules: { "boundaries/element-types": "warn" } },
  { settings: { /* project-specific boundary settings */ } },
  ...guardrailTestRelaxation,
);
```

**Pros:**
- Clean separation. guardrail3 NEVER touches eslint.config.mjs after initial setup.
- User can reorder, add rules before/after guardrails, override specific rules.
- ESLint flat config's spread semantics mean later rules override earlier ones — user can override any guardrail rule.
- No parsing needed. guardrail3 generates the import file from its existing `Module` constants.

**Cons:**
- Requires one-time setup: user must add the import. `guardrail3 ts init` should scaffold this.
- If user deletes the import, guardrails are silently dropped. Mitigated by `validate` checking for the import.

**Migration path:**
1. `generate` creates `.guardrail3/eslint-guardrails.mjs`
2. If `eslint.config.mjs` doesn't exist: generate a new one with the import (current behavior adapted)
3. If `eslint.config.mjs` exists AND was previously generated by guardrail3 (contains `GENERATED by guardrail3` header): replace it with the new import-based version + preserve no user content (there was none, it was all generated)
4. If `eslint.config.mjs` exists AND was NOT generated by guardrail3: print a message telling the user to add the import. Do NOT modify their file.
5. `validate` gains a new check: verify that `eslint.config.mjs` imports from `.guardrail3/eslint-guardrails.mjs`

#### Option D: Validate-only
Rejected as the sole strategy. Automated fixing is the entire point of `generate`. But validate-only remains the correct approach for user-owned files that guardrail3 should never touch (like the user's eslint.config.mjs in Option C).

#### .stylelintrc.mjs

**Decision: Option C (Separate file) for stylelint too.**

Same pattern as ESLint:
- guardrail3 generates `.guardrail3/stylelint-guardrails.mjs`
- User's `.stylelintrc.mjs` imports and extends

The generated file:

```js
// .guardrail3/stylelint-guardrails.mjs
export default {
  extends: ["stylelint-config-standard", "stylelint-config-tailwindcss"],
  plugins: ["@double-great/stylelint-a11y"],
  rules: {
    "a11y/content-property-no-static-value": true,
    "a11y/font-size-is-readable": true,
    // ... all a11y rules
  },
};
```

User's `.stylelintrc.mjs`:

```js
import guardrails from "./.guardrail3/stylelint-guardrails.mjs";
export default {
  ...guardrails,
  rules: {
    ...guardrails.rules,
    // User's additional rules
    "color-function-notation": "modern",
    "lightness-notation": "percentage",
    "no-duplicate-selectors": null,  // user override
  },
};
```

---

### 3. JSON Files: tsconfig.base.json, .jscpd.json, cspell.json

**Parser:** `serde_json` (already a dependency). Parse into `serde_json::Value`, merge, serialize back with `serde_json::to_string_pretty`.

**Strategy:** Structured key-level merge.

#### tsconfig.base.json

**Guardrail-owned entries:**

| Key path | Type | Merge behavior |
|----------|------|----------------|
| `compilerOptions.strict` | bool | Ensure `true` |
| `compilerOptions.noImplicitReturns` | bool | Ensure `true` |
| `compilerOptions.noUnusedLocals` | bool | Ensure `true` |
| `compilerOptions.noUnusedParameters` | bool | Ensure `true` |
| `compilerOptions.noUncheckedIndexedAccess` | bool | Ensure `true` |
| `compilerOptions.exactOptionalPropertyTypes` | bool | Ensure `true` |
| `compilerOptions.noPropertyAccessFromIndexSignature` | bool | Ensure `true` |
| `compilerOptions.noImplicitOverride` | bool | Ensure `true` |
| `compilerOptions.noFallthroughCasesInSwitch` | bool | Ensure `true` |
| `compilerOptions.forceConsistentCasingInFileNames` | bool | Ensure `true` |
| `compilerOptions.isolatedModules` | bool | Ensure `true` |
| `compilerOptions.allowUnreachableCode` | bool | Ensure `false` |
| `compilerOptions.allowUnusedLabels` | bool | Ensure `false` |

**User-owned entries (left untouched):**
- `compilerOptions.target` (project choice)
- `compilerOptions.lib` (project choice)
- `compilerOptions.module` (project choice)
- `compilerOptions.moduleResolution` (project choice)
- `compilerOptions.jsx` (framework-specific)
- `compilerOptions.baseUrl`, `compilerOptions.paths` (path aliases)
- `compilerOptions.outDir`, `compilerOptions.rootDir`
- `compilerOptions.plugins` (Next.js etc.)
- `compilerOptions.customConditions`
- `include`, `exclude`, `extends`, `references`
- Any other key not in the guardrail-owned list

**Merge algorithm:**

```
parsed = serde_json::from_str(existing_content)
co = parsed["compilerOptions"] (create if missing)
for each (key, required_value) in guardrail_booleans:
    co[key] = required_value  // always set, regardless of current value
// All other keys in co are untouched
// All other top-level keys (include, exclude, extends, references) are untouched
write_back(parsed)
```

Note: for tsconfig, ALL guardrail entries are booleans with a fixed required value. There's no "stricter" direction — the value must be exactly what guardrail3 requires. This is because these flags are binary safety switches, not tunable thresholds.

**Comment preservation:** JSON doesn't have comments, but tsconfig supports JSONC (JSON with Comments). If we need to support JSONC, we'd need a JSONC parser. For now, `serde_json` handles standard JSON. If the user's tsconfig uses comments, we should detect this (presence of `//` or `/*`) and either:
- Use a JSONC-aware parser (add `json_comments` crate or similar)
- Warn and skip modification

**Recommendation:** Use `serde_json` for standard JSON files. Add a pre-check: if the file contains `//` or `/*` comments (outside of string values), use the JSONC stripping approach from `json_comments` before parsing, then... we lose comments. Better approach: detect JSONC and warn the user, offering to create a `.guardrail3/tsconfig-strict.json` that they `extends` from (similar to the ESLint Option C pattern).

Actually, the cleanest approach for tsconfig is also **Option C**:

```json
// tsconfig.base.json (user-owned)
{
  "extends": "./.guardrail3/tsconfig-strict.json",
  "compilerOptions": {
    "target": "ES2022",
    "lib": ["ES2022", "DOM"],
    "paths": { "@/*": ["./src/*"] }
    // user settings here — they override the base
  }
}
```

```json
// .guardrail3/tsconfig-strict.json (guardrail3-owned)
{
  "$schema": "https://json.schemastore.org/tsconfig",
  "_comment": "GENERATED by guardrail3 — do not edit",
  "compilerOptions": {
    "strict": true,
    "noImplicitReturns": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noUncheckedIndexedAccess": true,
    "exactOptionalPropertyTypes": true,
    "noPropertyAccessFromIndexSignature": true,
    "noImplicitOverride": true,
    "noFallthroughCasesInSwitch": true,
    "forceConsistentCasingInFileNames": true,
    "isolatedModules": true,
    "allowUnreachableCode": false,
    "allowUnusedLabels": false,
    "esModuleInterop": true,
    "resolveJsonModule": true,
    "skipLibCheck": true,
    "declaration": true,
    "declarationMap": true,
    "noEmit": true
  }
}
```

**However**, tsconfig `extends` has surprising merge behavior — arrays are replaced, not merged, and some settings interact oddly. For tsconfig specifically, **direct JSON merge is safer and more predictable** than `extends`, because:
1. The user already has a tsconfig.base.json or tsconfig.json
2. We only touch `compilerOptions` boolean flags
3. We never touch `include`, `exclude`, `paths`, `references`, or any other section
4. `serde_json` preserves all keys it doesn't modify

**Final decision for tsconfig: Direct JSON merge.** Parse the existing file, set the guardrail boolean flags in `compilerOptions`, write back. User content is preserved because we only set specific known keys and leave everything else.

#### .jscpd.json

**Guardrail-owned entries:**

| Key | Type | Merge behavior |
|-----|------|----------------|
| `threshold` | number | Ensure `0`. If user has higher (looser), override to 0. If user has 0, leave it. |
| `minTokens` | number | Ensure `50` if missing. If user has different value, **leave it** — this is a tuning knob, not a safety floor. |

**User-owned entries:**
- `ignore` array — union merge. Ensure guardrail baseline patterns exist. User extras preserved.
- `reporters` — leave untouched
- `format` — leave untouched
- `absolute` — leave untouched
- Any other key — leave untouched

**Merge algorithm for `ignore` array:**

```
existing_ignore = parsed["ignore"] as array, or empty
for each baseline_pattern in guardrail_ignore_patterns:
    if baseline_pattern not in existing_ignore:
        append baseline_pattern to existing_ignore
// User patterns that aren't in baseline are untouched
```

The guardrail baseline ignore patterns (from `canonical::JSCPD`):
```
**/node_modules/**, **/.next/**, **/dist/**, **/target/**,
**/components/ui/**, **/components/pro-blocks/**, **/drizzle/**,
**/__tests__/**, **/__mocks__/**, **/test/**,
**/*.generated.*, **/coverage/**, **/.plans/**,
**/.worklogs/**, **/lib/utils.ts, **/.claude/**,
**/test-data/**, **/golden/**
```

#### cspell.json

**Guardrail-owned entries:**

| Key | Merge behavior |
|-----|----------------|
| `version` | Ensure `"0.2"` |
| `language` | Ensure `"en"` |
| `ignorePaths` | Union merge (ensure baseline patterns present, user extras preserved) |
| `$schema` | Ensure present |

**User-owned entries (NEVER touch):**
- `words` — this is the user's dictionary. Overwriting it would delete all their custom words.
- `flagWords`, `ignoreWords` — user-managed
- `dictionaries`, `dictionaryDefinitions` — user-managed
- `overrides` — user-managed
- `import` — user-managed
- Any other key — leave untouched

This is a critical example of why non-destructive editing matters. The current `generate` produces a cspell.json with `"words": []`, which would wipe out a user's carefully curated word list.

---

### 4. INI-like Files: .npmrc

**Parser:** Line-by-line key=value parsing (already implemented in `npmrc_check.rs`).

**Strategy:** Key-level merge. This is the simplest case.

**Guardrail-owned entries:**

| Key | Value | Merge behavior |
|-----|-------|----------------|
| `strict-peer-dependencies` | `true` | Ensure present with correct value |
| `disallow-workspace-cycles` | `true` | Ensure present |
| `save-workspace-protocol` | `rolling` | Ensure present |
| `engine-strict` | `true` | Ensure present |
| `package-manager-strict-version` | `true` | Ensure present |
| `strict-dep-builds` | `true` | Ensure present |
| `verify-deps-before-run` | `error` | Ensure present |
| `minimum-release-age` | `1440` | Ensure present. If user has higher (stricter), leave it. If lower, override. |
| `block-exotic-subdeps` | `true` | Ensure present |
| `trust-policy` | `warn` | Ensure present |
| `save-prefix` | (empty) | Ensure present with empty value |
| `public-hoist-pattern` | (empty) | Ensure present with empty value |
| `shamefully-hoist` | `false` | Ensure present |

**User-owned entries (left untouched):**
- Registry configurations (`registry=`, `@scope:registry=`)
- Auth tokens (`//registry.npmjs.org/:_authToken=`)
- Any other key not in the guardrail baseline

**Merge algorithm:**

```
lines = read existing .npmrc
settings = parse key=value pairs
for each (key, required_value) in guardrail_settings:
    if key in settings:
        if settings[key] != required_value:
            // Check strictness direction
            if key == "minimum-release-age" and parse_int(settings[key]) > parse_int(required_value):
                continue  // user is stricter
            replace the line in-place
    else:
        append "key=required_value" at the end (or in a guardrail section)
write back all lines
```

**Comment preservation:** .npmrc comments (`#` lines) are preserved because we operate on lines, not on a parsed data structure. Lines that are comments or blank are kept in place.

**Formatting preservation:** The existing lines keep their exact formatting (spacing around `=`). Only modified or added lines use guardrail3's formatting.

---

### 5. Shell Scripts: .githooks/pre-commit

**Decision: Full replacement (current behavior is correct).**

Pre-commit hooks are entirely guardrail3-managed. There is no meaningful user content in the generated hook — it's a sequence of tool invocations that guardrail3 controls. Users who need custom hook steps should use a multi-hook system (like pre-commit framework) or chain scripts.

The current approach of generating the full hook is correct. No change needed.

**One improvement:** If the user has added content AFTER the generated hook (e.g., appended custom steps), we could detect this and preserve it. But this is low priority and adds complexity. The existing behavior (warn on overwrite) is sufficient.

---

### 6. TOML Config Files: rustfmt.toml, rust-toolchain.toml

**rustfmt.toml:**

All entries are guardrail-managed. There are no user-specific rustfmt settings that guardrail3 doesn't know about. Full replacement is fine.

However, if a user has uncommented the nightly-only settings, we should preserve that. The merge approach:
- Ensure all stable settings match guardrail values
- Leave any nightly-only settings the user has uncommented/added
- Leave any settings not in the guardrail baseline

**rust-toolchain.toml:**

Similarly all guardrail-managed. Full replacement is fine.

Exception: if user has added extra components (e.g., `miri`, `rust-analyzer`), we should preserve them. Merge approach:
- Ensure `channel = "stable"` (or whatever guardrail requires)
- Ensure `clippy` and `rustfmt` are in `components`
- Preserve any additional user components

---

## Implementation Architecture

### New Module: `merge` (or `patch`)

Create a new module `src/domain/merge/` with per-format merge logic:

```
src/domain/merge/
    mod.rs              — MergeResult type, MergeStrategy enum
    toml_merge.rs       — TOML merge using toml_edit
    json_merge.rs       — JSON merge using serde_json
    ini_merge.rs        — INI/npmrc merge (line-based)
    js_import.rs        — JS file import injection (for initial setup)
```

### Core Types

```rust
/// What happened to a specific entry during merge.
enum EntryAction {
    Unchanged,              // Already correct
    Added,                  // Was missing, added
    Updated { old: String }, // Was wrong, corrected
    Preserved,              // User content, left alone
}

/// Result of merging one file.
struct MergeResult {
    path: String,
    content: String,           // The merged content to write
    actions: Vec<(String, EntryAction)>,  // Per-entry actions for dry-run display
    was_created: bool,         // true if file didn't exist before
}
```

### Changes to `generate` Command

The `generate` command currently calls `crate::fs::write_file` for each generated file. The new flow:

```
for each guardrail file:
    if file doesn't exist:
        create from template (MergeResult with was_created = true)
    else:
        read existing content
        merge guardrail entries into existing content
        produce MergeResult with per-entry actions

    if not dry_run:
        write merged content

    report actions (for --dry-run or verbose output)
```

### Changes to `diff` Command

Currently `diff` shows line-level diffs between generated and existing files. With non-destructive editing:
- `diff` shows the **merge result** vs the existing file
- This means only the guardrail entries that would change are shown
- User content doesn't appear in the diff (because it's unchanged)

### Changes to `check` Command

Currently `check` verifies generated files match exactly. With non-destructive editing:
- `check` verifies that all guardrail entries are present and correct
- It does NOT verify that the entire file matches a template
- This is closer to what `validate` does, but for generated config files specifically

This raises a question: **should `check` and `validate` merge?** Currently:
- `validate` = config-free, checks known patterns
- `check` = requires guardrail3.toml, verifies generated files are current

With non-destructive editing, `check` becomes "verify guardrail entries are present" which is identical to the relevant subset of `validate`. Consider: `check` could call the `validate` checks for config files, plus verify that the guardrail3.toml-driven entries (profile-specific bans, app-specific configs) are present.

---

## Override System Interaction

The existing `.guardrail3/overrides/` system adds entries to guardrail files:
- `clippy-methods.toml` — extra disallowed methods
- `clippy-types.toml` — extra disallowed types
- `deny-bans.toml` — extra crate bans
- `deny-skip.toml` — duplicate crate skip list
- `deny-feature-bans.toml` — feature bans

With non-destructive editing, overrides work the same way — they're additional entries that get merged in alongside the baseline. The `deduplicated_override` function already handles preventing duplicates.

**New capability enabled by non-destructive editing:** Overrides become less necessary. Currently, if a user wants to add a clippy method ban, they put it in `.guardrail3/overrides/clippy-methods.toml` because editing clippy.toml directly would be overwritten by `generate`. With non-destructive editing, the user can add the ban directly to clippy.toml and `generate` will preserve it.

This means `.guardrail3/overrides/` becomes optional — a convenience for users who prefer to keep their additions separate from the generated content. Both approaches (direct edit + override file) produce the same result.

---

## Dry-Run Output Changes

Currently dry-run shows line-level diffs. With non-destructive editing, show per-entry changes:

```
$ guardrail3 diff

clippy.toml:
  ✓ cognitive-complexity-threshold = 15 (unchanged)
  ✓ too-many-lines-threshold = 75 (unchanged)
  + disallowed-methods: { path = "std::env::var" } (added — was missing)
  ~ disallowed-types: { path = "std::collections::HashMap" } reason updated
  · disallowed-methods: { path = "mylib::custom_method" } (user entry, preserved)

deny.toml:
  ✓ [bans].multiple-versions = "deny" (unchanged)
  + [bans].deny: { name = "chrono" } (added — was missing)
  · [bans].skip: { name = "windows-sys", version = "0.48" } (user entry, preserved)

tsconfig.base.json:
  ~ compilerOptions.noUncheckedIndexedAccess: false → true
  ✓ compilerOptions.strict = true (unchanged)
  · compilerOptions.paths (user entry, preserved)

.npmrc:
  + minimum-release-age=1440 (added — was missing)
  · @myco:registry=https://npm.pkg.github.com (user entry, preserved)
```

This is much more informative than a raw diff and clearly shows which entries are guardrail-managed vs user-managed.

---

## The `.guardrail3/` Directory

With non-destructive editing, the `.guardrail3/` directory gains new significance:

```
.guardrail3/
    overrides/                      — user override files (existing)
        clippy-methods.toml
        clippy-types.toml
        deny-bans.toml
        deny-skip.toml
        deny-feature-bans.toml
    eslint-guardrails.mjs           — generated ESLint guardrail rules (NEW)
    stylelint-guardrails.mjs        — generated Stylelint guardrail rules (NEW)
    tsconfig-strict.json            — generated tsconfig strict flags (NEW, optional)
```

The `eslint-guardrails.mjs` and `stylelint-guardrails.mjs` files are fully generated and can be safely overwritten. The user's config files import from them.

---

## Migration Path

### Phase 1: Separate import files for JS configs
1. Add `.guardrail3/eslint-guardrails.mjs` generation
2. Add `.guardrail3/stylelint-guardrails.mjs` generation
3. `generate` creates these files alongside (not instead of) the current full-file generation
4. New projects get the import-based pattern. Existing projects continue with full generation.
5. Add `validate` check: warn if eslint.config.mjs is fully generated (has the `GENERATED by guardrail3` header) and suggest migrating to the import pattern.

### Phase 2: JSON merge for tsconfig, jscpd, cspell
1. Add `toml_edit` dependency
2. Implement JSON merge for tsconfig.base.json (only strict flags)
3. Implement JSON merge for .jscpd.json (threshold + ignore union)
4. Implement JSON merge for cspell.json (schema + version + ignorePaths union, NEVER touch words)
5. `generate` uses merge when file exists, full generation when file absent

### Phase 3: TOML merge for clippy.toml, deny.toml
1. Implement TOML merge using `toml_edit` (preserves comments and formatting)
2. `generate` uses merge when file exists
3. Override files become optional (direct edits are preserved)

### Phase 4: INI merge for .npmrc
1. Implement line-based merge for .npmrc
2. `generate` uses merge when file exists

### Phase 5: Unify check and validate for config files
1. `check` delegates to `validate` for config file checks
2. Remove the "whole file comparison" approach from `check`
3. `check` only verifies guardrail3.toml-derived expectations (profile-specific entries)

---

## Dependency Changes

| Crate | Purpose | Phase |
|-------|---------|-------|
| `toml_edit` | Comment-preserving TOML merge | Phase 3 |

`serde_json` is already a dependency. No new JSON dependencies needed.

---

## Questions Resolved

**1. For each file type, which edit strategy?**
- TOML (clippy.toml, deny.toml): Structured merge via `toml_edit`
- JS (eslint.config.mjs, .stylelintrc.mjs): Separate import file (Option C)
- JSON (tsconfig.base.json, .jscpd.json, cspell.json): Structured merge via `serde_json`
- INI (.npmrc): Line-based merge
- Shell (.githooks/pre-commit): Full replacement (no change)
- TOML configs (rustfmt.toml, rust-toolchain.toml): Structured merge (preserve user additions)

**2. For each guardrail entry type, what's the right "direction" policy?**
- Thresholds: enforce floor (override if user is looser, leave if stricter or equal)
- Booleans: enforce exact value (safety switches are non-negotiable)
- Ban lists: enforce superset (baseline entries always present, user extras preserved)
- Rule severities: enforce floor (error > warn > off; never downgrade)
- License allow-lists: enforce superset (baseline licenses present, user extras preserved)

**3. How does generate change?**
- File absent: full generation (no change)
- File present: merge guardrail entries, preserve everything else

**4. How does dry-run output change?**
- Per-entry change display instead of line-level diff
- Clear distinction between guardrail-managed and user-managed entries

**5. Migration path?**
- Phased: JS imports first (highest user impact), then JSON merge, then TOML merge, then INI merge
- Backward compatible: new projects get new behavior, existing projects can opt in

**6. How does the override system interact?**
- Overrides continue to work as additional entries merged alongside baseline
- Direct edits to config files are now also preserved (overrides become optional convenience)

**7. First run vs subsequent runs?**
- First run (file absent): full template generation
- Subsequent runs (file present): merge with existing content

**8. How do we track which entries are "ours" vs "user's"?**
- No explicit tracking needed. guardrail3 knows its baseline entries (compiled-in). Anything matching a baseline key is guardrail-managed for value enforcement. Anything else is user-managed and untouched.
