# Adversarial Audit: tsconfig, npmrc, and jscpd Validation

## Files Audited
- `apps/guardrail3/src/app/ts/validate/tsconfig_check.rs`
- `apps/guardrail3/src/app/ts/validate/npmrc_check.rs`
- `apps/guardrail3/src/app/ts/validate/jscpd_check.rs`
- `apps/guardrail3/src/domain/modules/canonical.rs`

---

## tsconfig_check.rs Findings

### TSC-01: JSONC / trailing commas not handled — tsconfig files silently rejected
**Severity:** HIGH
**Lines:** 108-126

`serde_json::from_str` is used to parse tsconfig. Real `tsconfig.json` files support JSONC (comments and trailing commas) because TypeScript's own parser handles them. A project with `// comment` lines in tsconfig.json will get a parse error from guardrail3, and then **all subsequent checks are skipped** (early return on line 126). The error message mentions JSONC but doesn't actually parse it.

**Exploit:** Add a single `// comment` to tsconfig.json. All 20+ checks are silently bypassed. The user sees one "invalid JSON" error and may dismiss it as a guardrail3 bug, leaving all strict settings unchecked.

**Fix:** Use a JSONC-aware parser (e.g., `serde_jsonc`, or strip comments/trailing commas before parsing).

### TSC-02: No validation of child tsconfig files that DON'T extend the base
**Severity:** HIGH
**Lines:** 63-86

The check looks for `tsconfig.base.json` first, then falls back to `tsconfig.json`. But in a monorepo, individual packages have their own `tsconfig.json` that may `extends` the base — or may NOT. A package-level `tsconfig.json` that does NOT extend the base can set `"strict": false` and guardrail3 will never see it, because it only checks the root-level file.

**Exploit:** Create `apps/my-app/tsconfig.json` with `{"compilerOptions": {"strict": false}}` and no `extends`. The root `tsconfig.base.json` passes all checks. The app silently runs without strict mode.

### TSC-03: No detection of setting overrides via `extends` chain
**Severity:** HIGH
**Lines:** 128 onward

The word "extends" does not appear anywhere in `tsconfig_check.rs`. The check reads the base tsconfig and validates its `compilerOptions`. But TypeScript's `extends` mechanism means a child tsconfig can override any setting from the base. A child tsconfig with `"strict": false` will override `"strict": true` from the base, and guardrail3 will report everything as passing.

**Exploit:** Root `tsconfig.base.json` has `"strict": true`. Child `tsconfig.json` has `{"extends": "./tsconfig.base.json", "compilerOptions": {"strict": false}}`. Guardrail3 reports T9 as passing.

### TSC-04: BOM (Byte Order Mark) will cause JSON parse failure
**Severity:** MEDIUM
**Lines:** 108

If the file starts with a UTF-8 BOM (`\xEF\xBB\xBF`), `serde_json::from_str` will fail to parse it. This is common with files created by Windows editors. Same consequence as TSC-01: all checks silently skipped.

### TSC-05: Check ID collision — T60 used for both `noPropertyAccessFromIndexSignature` AND content import restriction
**Severity:** MEDIUM
**Lines:** 141 (tsconfig_check.rs), 241 (jscpd_check.rs)

`T60` is used as the check ID for `noPropertyAccessFromIndexSignature` in `additional_required_bools` AND for the content import restriction check in `check_content_import_restriction` (jscpd_check.rs line 241). Same ID for completely unrelated checks makes reporting ambiguous.

### TSC-06: Check ID collision — T61 used for both `noImplicitOverride` AND Velite config
**Severity:** MEDIUM
**Lines:** 142 (tsconfig_check.rs), 269 (jscpd_check.rs)

`T61` is used for `noImplicitOverride` (tsconfig_check.rs) AND for the Velite config check (jscpd_check.rs `check_velite_config`). Same collision problem as TSC-05.

### TSC-07: `sourceMap` not validated
**Severity:** LOW
**Lines:** 129-157

The canonical `TSCONFIG_BASE` in canonical.rs does NOT include `sourceMap: true`, and the check does not validate it. Source maps are important for debugging production issues. This may be intentional but is worth noting.

### TSC-08: Canonical has `"target": "ES2022"` (uppercase) but check compares case-insensitively
**Severity:** INFO
**Lines:** 353-357, canonical.rs line 188

The canonical module uses `"ES2022"` (uppercase) while the check expects `"es2022"` (lowercase) in the `required_strings` array. The comparison uses `eq_ignore_ascii_case` so this works correctly at runtime, but the expected value in the check code doesn't match the canonical module's casing. Not a bug, but inconsistent.

### TSC-09: No validation of `lib`, `jsx`, `paths`, `baseUrl` values
**Severity:** LOW
**Lines:** 353-408

Only `target`, `module`, and `moduleResolution` are checked as required string values. The canonical module specifies `"lib": ["ES2022", "DOM", "DOM.Iterable"]` but no check validates that `lib` is set or has reasonable values.

---

## npmrc_check.rs Findings

### NPM-01: CRITICAL — `find()` returns FIRST match, pnpm uses LAST match (duplicate key exploit)
**Severity:** CRITICAL
**Lines:** 93

`settings.iter().find(|(k, _)| k == key)` returns the **first** matching key. But `.npmrc` (like most INI-style configs) resolves duplicate keys by using the **last** value. An attacker (or careless agent) can write:

```ini
strict-peer-dependencies=true
# ... 50 lines of legitimate settings ...
strict-peer-dependencies=false
```

Guardrail3 sees `true` (first match) and reports passing. pnpm sees `false` (last match) and runs permissively. This is the exact exploit called out in the audit scope and it is **NOT fixed**.

**Fix:** Use `settings.iter().rfind(|(k, _)| k == key)` (reverse find) or, better, also flag duplicate keys as an error.

### NPM-02: Inline comments not stripped
**Severity:** HIGH
**Lines:** 56-66

The parser checks for lines starting with `#` or `;` to skip comment lines. But it does NOT strip inline comments. In `.npmrc`:

```ini
strict-peer-dependencies=true # This is important
```

The parsed value will be `"true # This is important"`, which does NOT equal `"true"`. Guardrail3 will report T13 (wrong value) even though pnpm correctly reads `true`. This is a false positive that trains users to ignore guardrail3 output.

Conversely:
```ini
strict-peer-dependencies=false # actually true
```
pnpm reads `false`. Guardrail3 reads `"false # actually true"` which doesn't equal `"true"` — reports wrong value (correct behavior, but for the wrong reason).

**Note:** pnpm's `.npmrc` parsing (via `ini` npm package) DOES strip inline comments after `#` or `;`. Guardrail3's parser does not match pnpm's behavior.

### NPM-03: Quoted values not handled
**Severity:** MEDIUM
**Lines:** 63

If a value is quoted: `save-prefix=""`, the parsed value is `""` (with quotes), not `` (empty string). For the `save-prefix=` check where expected is `""` (empty string in Rust), a file with `save-prefix=""` would be parsed as value `"\"\""` (two quote characters) which does not equal `""` (empty string). This causes a false positive.

Similarly `public-hoist-pattern=""` would fail the check.

**Fix:** Strip surrounding quotes from values after splitting on `=`.

### NPM-04: No duplicate key detection
**Severity:** HIGH
**Lines:** 53-68

Related to NPM-01. Even if `rfind` is used, duplicate keys in `.npmrc` should be flagged as suspicious. A legitimate `.npmrc` has no reason for duplicate keys. Their presence indicates either a merge conflict artifact or a deliberate exploit.

### NPM-05: Missing pnpm settings that SHOULD be enforced
**Severity:** MEDIUM

Settings present in canonical `NPMRC` module but NOT in the expected list... actually they match. Let me check the reverse — settings that modern pnpm supports and would strengthen security but are NOT in either list:

- **`side-effects-cache=false`** — disables side-effects caching which can mask build issues
- **`auto-install-peers=false`** — should be false when `strict-peer-dependencies=true` to avoid auto-installing wrong versions
- **`resolution-mode=highest`** — controls version resolution strategy; `highest` (default) vs `time-based` for reproducibility
- **`dedupe-injected-deps=true`** — deduplicates injected workspace deps

These are debatable but worth documenting as intentional omissions.

### NPM-06: `save-prefix` and `public-hoist-pattern` expected as empty string — fragile
**Severity:** LOW
**Lines:** 87-88

The expected values are `""` (empty Rust string). In the canonical module, these are written as `save-prefix=` and `public-hoist-pattern=` (no value after `=`). The parser extracts everything after `=` and trims it, producing an empty string. This works, but only because the canonical module uses the bare `key=` form. If anyone writes `key=''` or `key=""`, it breaks (see NPM-03).

---

## jscpd_check.rs Findings

### JSCPD-01: `$schema` field not validated
**Severity:** MEDIUM
**Lines:** 43-217

The canonical `JSCPD` module includes `"$schema": "https://json.schemastore.org/jscpd.json"` but the validation code never checks for it. The `$schema` field enables IDE validation and autocompletion. Its absence should at least be a warning.

### JSCPD-02: `reporters` field not validated — silent output exploit
**Severity:** HIGH
**Lines:** 43-217

The canonical module specifies `"reporters": ["consoleFull"]`. The validation code **never checks the `reporters` field**. If someone sets `"reporters": ["json"]`, jscpd will only write to a JSON file and produce no console output. In CI, this means duplication violations are silently swallowed — the exit code may still be 0 and no human will see the report.

**Exploit:** Set `"reporters": []` or `"reporters": ["json"]`. jscpd runs but nobody sees the output. Duplication flies under the radar.

### JSCPD-03: JSON parse error silently swallowed — all checks bypassed
**Severity:** HIGH
**Lines:** 43-46

```rust
let json: serde_json::Value = match serde_json::from_str(&content) {
    Ok(v) => v,
    Err(_) => return,
};
```

If `.jscpd.json` has invalid JSON (or JSONC comments, trailing commas, BOM), the function silently returns with NO error result. The user gets the "config exists" inventory item (T19) but zero validation. Compare with tsconfig_check.rs which at least reports a parse error.

**Fix:** Push a CheckResult with the parse error before returning.

### JSCPD-04: No glob syntax validation for ignore patterns
**Severity:** LOW
**Lines:** 116-136

Ignore patterns are reported as inventory items but never validated for correct glob syntax. A typo like `**/node_moduels/**` will silently exclude nothing, causing false positives on vendored code. Similarly, `node_modules/**` (missing leading `**/`) would only match at root level.

### JSCPD-05: JSONC not supported (same as TSC-01)
**Severity:** MEDIUM
**Lines:** 43

`serde_json::from_str` used for parsing. `.jscpd.json` doesn't typically use comments, but some editors add them. Combined with JSCPD-03, this means a single comment silently disables all validation.

### JSCPD-06: Required ignore patterns use exact string match — no normalization
**Severity:** LOW
**Lines:** 186-187

```rust
if !configured_ignores.iter().any(|p| p == required) {
```

This is an exact string comparison. `**/node_modules` (without trailing `/**`) would fail the check even though it's functionally equivalent for many glob implementations. Similarly, `**/node_modules/*` vs `**/node_modules/**` differ subtly but both exclude node_modules content.

### JSCPD-07: Canonical module has more ignore patterns than the required list
**Severity:** INFO
**Lines:** 172-178, canonical.rs 225-243

The required patterns are:
- `**/node_modules/**`, `**/.next/**`, `**/dist/**`, `**/target/**`, `**/components/ui/**`

The canonical module has 13 additional patterns (drizzle, tests, mocks, coverage, plans, worklogs, claude, etc.) that are NOT enforced by the check. This means `guardrail3 generate` creates a comprehensive config, but `guardrail3 validate` only checks 5 of the 18 patterns. A user who manually creates `.jscpd.json` would miss 13 recommended exclusions with no warning.

### JSCPD-08: `minTokens` check — missing value triggers both T21 and T-JSCPD-01
**Severity:** INFO
**Lines:** 96-152

If `minTokens` is present and equals 50, T21 check is skipped (no output). If `minTokens` is absent, only T-JSCPD-01 fires (warn). If `minTokens` is present and NOT 50, only T21 fires (info). This logic is correct but the T21 info message for non-50 values doesn't indicate whether the value is too high or too low, making it less actionable.

---

## Cross-Cutting Findings

### CROSS-01: Check ID collisions across files
**Severity:** HIGH

| ID | File 1 | File 2 | Meaning 1 | Meaning 2 |
|---|---|---|---|---|
| T60 | tsconfig_check.rs:141 | jscpd_check.rs:241 | `noPropertyAccessFromIndexSignature` | Content import restriction |
| T61 | tsconfig_check.rs:142 | jscpd_check.rs:269 | `noImplicitOverride` | Velite config check |

These collisions mean filtering by check ID in reports will mix unrelated results.

### CROSS-02: Canonical module values vs validation check consistency
**Severity:** INFO

All canonical NPMRC settings match the expected list in `npmrc_check.rs`. All canonical tsconfig compilerOptions match the checked settings. The canonical JSCPD module's `reporters`, `$schema`, and most `ignore` patterns are NOT validated (see JSCPD-01, JSCPD-02, JSCPD-07).

### CROSS-03: No BOM handling anywhere
**Severity:** MEDIUM

None of the three files strip a UTF-8 BOM before parsing. Files saved by Windows Notepad or some other editors include a BOM. For JSON files (tsconfig, jscpd), this causes a parse error. For npmrc, the BOM bytes would be prepended to the first key name, causing it to not match any expected key.

---

## Summary: Findings by Severity

| Severity | Count | IDs |
|---|---|---|
| CRITICAL | 1 | NPM-01 |
| HIGH | 6 | TSC-01, TSC-02, TSC-03, NPM-02, NPM-04, JSCPD-02, JSCPD-03 |
| MEDIUM | 5 | TSC-04, TSC-05, TSC-06, NPM-03, JSCPD-01, JSCPD-05, CROSS-03 |
| LOW | 4 | TSC-07, TSC-09, NPM-06, JSCPD-04, JSCPD-06 |
| INFO | 4 | TSC-08, NPM-05, JSCPD-07, JSCPD-08, CROSS-01, CROSS-02 |

## Priority Fix Order

1. **NPM-01** — `find()` vs `rfind()` is a one-line fix that closes a real exploit
2. **JSCPD-03** — Silent swallow of parse errors is a one-line fix (add error result)
3. **TSC-01 + JSCPD-05** — JSONC support (use `serde_jsonc` or strip comments)
4. **NPM-02** — Strip inline comments (small parser change)
5. **JSCPD-02** — Validate `reporters` field includes `consoleFull`
6. **NPM-04** — Detect and flag duplicate keys
7. **TSC-02 + TSC-03** — Extended tsconfig validation (larger scope, may need workspace discovery)
8. **TSC-05 + TSC-06 / CROSS-01** — Fix check ID collisions
9. **CROSS-03** — BOM stripping (utility function, apply to all parsers)
10. **NPM-03** — Quote stripping
