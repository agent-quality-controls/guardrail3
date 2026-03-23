# Verified Findings: F09, NPM-01, FINDING-H-01, FINDING-H-02, COV-CRIT-01, COV-CRIT-02, CLI-01

## F09: ESLint zone definition check — generic word matching

**File:** `apps/guardrail3/src/app/ts/validate/eslint_audit.rs`, lines 28-59

**Verdict: REAL, but severity is LOW, not MEDIUM.**

The check at line 29-31 is:
```rust
let has_zones = content.contains("element-types")
    || content.contains("domain")
        && (content.contains("commands") || content.contains("adapters"));
```

The auditor is correct that this searches for generic substrings. The word "domain" could appear in a comment, a URL, or an unrelated config object. Same for "adapters" and "commands". However:

1. The check's purpose is to detect whether ESLint boundaries plugin zone definitions exist. The string `"element-types"` is actually specific to the `eslint-plugin-boundaries` configuration and would be a strong signal by itself.
2. The `"domain" && "adapters"` branch is the weak fallback. In practice, an ESLint config file that contains both "domain" and "adapters" almost certainly has zone definitions — these aren't common words in ESLint configs unless you're defining architectural zones.
3. There is also an **operator precedence issue**: `||` has lower precedence than `&&` in Rust, so this reads as `contains("element-types") || (contains("domain") && (contains("commands") || contains("adapters")))`. This is actually correct behavior — if "element-types" is present, zones are detected. The fallback checks for domain+commands/adapters. No bug here, just could be clearer with explicit parentheses.

**Should fix:** Yes, but low priority. The check could be tightened to look for more specific patterns (e.g., regex for `element-types` in a boundaries config context). But false positives here only produce an Info (inventory) result, not suppress an error — the worst case is guardrail3 says "zones configured" when they aren't, missing a true negative. Worth improving but not urgent.

---

## NPM-01: npmrc duplicate key exploit

**File:** `apps/guardrail3/src/app/ts/validate/npmrc_check.rs`, lines 53-68, 92-93

**Verdict: REAL bug. Should fix.**

The parser at line 53-68 (`parse_npmrc_settings`) collects ALL key=value pairs into a `Vec<(String, String)>`. This correctly preserves duplicates. However, the comparison function `check_expected_settings` at line 93 uses:

```rust
let found = settings.iter().find(|(k, _)| k == key);
```

This is `.find()` which returns the **first** match. If `.npmrc` contains:
```
strict-peer-dependencies=true
strict-peer-dependencies=false
```

guardrail3 would see `true` (first match) and report "correct". But pnpm processes .npmrc as an INI file where **last value wins** — so pnpm would actually use `false`.

This is exploitable: an attacker (or careless agent) could add a duplicate key with a weaker value below the correct one, and guardrail3 would not detect it.

**Fix:** Either:
- (a) Use `.rfind()` to match pnpm's last-wins behavior, OR
- (b) Better: detect duplicate keys and report them as an error (a duplicate key in .npmrc is always suspicious)

Option (b) is strictly better — it catches the issue AND prevents confusion.

---

## FINDING-H-01: No --no-verify bypass detection

**Files:** `apps/guardrail3/src/app/hooks/hook_checks.rs`, `apps/guardrail3/src/app/hooks/validate.rs`, `apps/guardrail3/src/app/hooks/hook_script_checks.rs`

**Verdict: AUDITOR IS WRONG. This is not detectable and not a valid finding.**

`git commit --no-verify` is a runtime flag passed to the `git` CLI at commit time. It tells git to skip running hooks entirely. This happens at the git level, not at the project level.

guardrail3 is a **static analysis tool**. It reads files on disk. It cannot:
1. Intercept git commands at runtime
2. Detect past uses of `--no-verify` (git does not log this)
3. Prevent future uses of `--no-verify`

The only way to detect `--no-verify` bypasses is:
- A CI pipeline that re-runs all pre-commit checks on pushed code (which guardrail3's `validate` command enables — that's the intended workflow)
- A git wrapper/alias that blocks `--no-verify` (outside guardrail3's scope)

This is not a gap in guardrail3. The tool already provides the solution: run `guardrail3 validate` in CI. The `--no-verify` flag becomes irrelevant because CI re-validates everything.

**Should fix:** No. This is working as designed. The CLAUDE.md already documents that the pre-commit hook runs `guardrail3 validate --staged`, and the intended CI workflow is to run `guardrail3 validate` on every push.

---

## FINDING-H-02: No set -e validation

**Files:** All hook check files searched.

**Verdict: REAL. Should fix (LOW priority).**

I searched all hook-related files (`hook_checks.rs`, `hook_script_checks.rs`, `validate.rs`). There is NO check for `set -e`, `set -uo pipefail`, `set -euo pipefail`, or any shell strictness flags in hook scripts.

This matters because without `set -e`, a failing command in the middle of a hook script does not abort the hook — subsequent commands still run, and the hook may exit 0 even though a check failed. For example, if `cargo clippy` fails but the script doesn't have `set -e`, the hook continues and may exit successfully.

However, the severity depends on context:
- If the generated hook uses explicit `|| exit 1` after each command, `set -e` is redundant
- If the hook is a dispatcher that sources scripts, each sourced script should have its own error handling

**Should fix:** Yes. Add a check (e.g., H-SHELL-01) that verifies the pre-commit script starts with `set -e` or `set -euo pipefail`. This is a defensive check — even if the generated hook handles errors correctly, user-modified hooks might not.

---

## COV-CRIT-01: String::starts_with in shadow detection

**File:** `apps/guardrail3/src/commands/coverage/engine.rs`, line 195

**Verdict: REAL bug. Should fix.**

Line 195:
```rust
if i != j && dir_i != dir_j && dir_i.starts_with(dir_j.as_str()) {
```

`dir_i` and `dir_j` are `String` values (from `c.covers.first()` which returns `String`). This uses `String::starts_with`, NOT `Path::starts_with`.

The difference:
- `String::starts_with("apps/web")` matches "apps/web-admin/src" (character prefix match)
- `Path::starts_with("apps/web")` would NOT match "apps/web-admin/src" (component-level match)

So if you have configs at `apps/web/eslint.config.mjs` and `apps/web-admin/eslint.config.mjs`, the shadow detection would incorrectly report that `apps/web-admin` is a shadow of `apps/web`, because the string "apps/web-admin" starts with the string "apps/web".

**Fix:** Convert to `Path` before comparison:
```rust
if i != j && dir_i != dir_j && Path::new(dir_i).starts_with(Path::new(dir_j)) {
```

This is a correctness bug that would produce wrong shadow relationships in monorepos with similarly-named app directories.

---

## COV-CRIT-02: Non-walk-up resolution finds first match

**File:** `apps/guardrail3/src/commands/coverage/engine.rs`, lines 106-110

**Verdict: REAL, but severity is LOWER than claimed.**

Lines 106-110:
```rust
config_files
    .iter()
    .filter_map(|cf| cf.parent())
    .find(|config_dir| dir.starts_with(config_dir))
    .map(Path::to_path_buf)
```

This uses `.find()` which returns the first config whose directory is an ancestor of the source dir. The order of iteration depends on `config_files`, which comes from `tool.config_files(crawl)` — the crawl result.

The issue: if config_files are ordered by path (which `BTreeSet`/sorted collections would do), then for a source dir like `apps/web/src/components/`, if configs exist at both `apps/` and `apps/web/`, `.find()` would match `apps/` first (alphabetically earlier), not `apps/web/` (the more specific/nearest ancestor).

However, this code path is for **non-walk-up** tools (like jscpd). The comment says "find the nearest ancestor config directory." But `.find()` returns the first match in iteration order, not the nearest ancestor.

**Fix:** Replace `.find()` with logic that selects the longest matching prefix (deepest ancestor):
```rust
config_files
    .iter()
    .filter_map(|cf| cf.parent())
    .filter(|config_dir| dir.starts_with(config_dir))
    .max_by_key(|config_dir| config_dir.components().count())
    .map(Path::to_path_buf)
```

Note: `dir.starts_with(config_dir)` here uses `Path::starts_with` (both are `Path`), so it does component-level matching correctly. The only bug is the ordering/selection issue.

**Also note:** This `starts_with` is `Path::starts_with` (on `PathBuf` values), which is correct. Unlike COV-CRIT-01 where `String::starts_with` is used incorrectly.

---

## CLI-01: Scope flags not mutually exclusive

**File:** `apps/guardrail3/src/cli.rs`, lines 143-213 (`ValidateArgs`)

**Verdict: REAL, but LOW severity.**

The scope flags in `ValidateArgs` are:
- `--staged` (line 152)
- `--dirty` (line 157)
- `--commits N` (line 162)
- `--files` (line 167)
- `--code` (line 177)
- `--architecture` (line 182)
- `--release` (line 187)
- `--tests` (line 191)
- `--garde` (line 197)

There is NO `clap::ArgGroup` or `conflicts_with` attribute on any of these flags. A user can pass `--staged --dirty --commits 5` simultaneously, which is contradictory.

However, the actual impact depends on how `commands/validate.rs` handles these flags. The behavior when multiple flags are set is whatever the code does — likely one takes priority over others in an if/else chain, making the extra flags silently ignored rather than causing errors.

**Should fix:** Yes. Add a clap `ArgGroup` for the mutually exclusive file-scope flags (`staged`, `dirty`, `commits`, `files`) so clap rejects contradictory combinations at parse time. The domain flags (`code`, `architecture`, `release`, `tests`, `garde`) are likely intended to be combinable (run only these check domains), so they should NOT be in a mutually exclusive group.

---

## Summary

| ID | Real? | Fix? | Severity | Notes |
|----|-------|------|----------|-------|
| F09 | Yes | Yes (low priority) | LOW | Generic substring matching, but worst case is false inventory, not missed error |
| NPM-01 | Yes | Yes | MEDIUM | `.find()` vs last-wins semantics — exploitable duplicate key bypass |
| FINDING-H-01 | No | No | N/A | Not detectable by static analysis; CI validation is the correct mitigation |
| FINDING-H-02 | Yes | Yes (low priority) | LOW | Missing `set -e` check for hook scripts |
| COV-CRIT-01 | Yes | Yes | HIGH | `String::starts_with` vs `Path::starts_with` — wrong shadow detection for similar dir names |
| COV-CRIT-02 | Yes | Yes | MEDIUM | `.find()` returns first match, not nearest ancestor — wrong config resolution |
| CLI-01 | Yes | Yes (low priority) | LOW | Contradictory scope flags not rejected by clap |
