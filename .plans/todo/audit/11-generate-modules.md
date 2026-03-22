# Audit 11: Config Generation, Staleness Checking, and Embedded Modules

## Scope

- `commands/generate.rs`, `commands/generate_helpers.rs`
- `commands/check.rs`, `commands/diff.rs`
- `domain/modules/clippy.rs`, `deny.rs`, `canonical.rs`, `mod.rs`, `pre_commit.rs`
- Cross-reference: `rs/validate/deny_audit.rs`, `deny_bans.rs`, `clippy_coverage.rs`
- Cross-reference: `commands/coverage/deny.rs`, `commands/coverage/clippy.rs`

---

## FINDING-11-01: `all_modules()` missing 3 modules (MEDIUM)

**Location:** `domain/modules/mod.rs` lines 19-78

`all_modules()` is the registry used by `list-modules` and `show-module`. It is missing:

1. **`deny::DENY_BANS_LIBRARY_IO`** -- defined at `deny.rs:189`, used by `library_profile_ban_entries()`, but never registered in `all_modules()`. Running `guardrail3 show-module deny/bans/library-io` will return nothing.
2. **`stylelint::STYLELINT`** -- defined at `stylelint.rs:26`, never registered in `all_modules()`. Running `guardrail3 show-module canonical/stylelint` will return nothing.
3. **`guide` module** -- `mod.rs` declares `pub mod guide` but `all_modules()` does not register it. (May be intentional since it's not a config file module -- but it's inconsistent that it's a `pub mod` in the module registry file.)

**Impact:** Users cannot inspect these modules via `list-modules`/`show-module`. Library IO bans are invisible to the user even though they're actively generated.

---

## FINDING-11-02: `EXPECTED_TYPE_BANS` missing `TYPE_GLOBAL_STATE` entries (MEDIUM)

**Location:** `clippy_coverage.rs:46-56`

`EXPECTED_TYPE_BANS` contains:
- `std::collections::HashMap`, `std::collections::HashSet` (from `TYPE_COLLECTIONS`)
- `std::sync::Mutex`, `std::sync::RwLock` (from `TYPE_SYNC`)
- `std::fs::File` (from `TYPE_FILESYSTEM`)
- `axum::extract::Json`, `axum::Json`, `axum::extract::Query`, `axum::extract::Form` (from `TYPE_GARDE_EXTRACTORS`)

**Missing:** The 4 entries from `TYPE_GLOBAL_STATE`:
- `std::sync::LazyLock`
- `std::sync::OnceLock`
- `once_cell::sync::Lazy`
- `once_cell::sync::OnceCell`

These are included in `library_profile_types()` and in `service_profile_types()` when `is_pure_layer = true`. But validation via `EXPECTED_TYPE_BANS` never checks for them. A library profile project or a pure-layer crate could have its global-state bans silently removed from `clippy.toml` and validation would not catch it.

**Impact:** Global-state type bans can be silently removed without validation detecting it.

---

## FINDING-11-03: `generate_expected_ts` inconsistent hook generation vs `generate_all_files` (MEDIUM)

**Location:** `generate.rs:354-370` vs `generate.rs:250-289`

`generate_expected_ts()` (used by `diff run_ts`) builds the pre-commit hook as:
```rust
let hook_content = pre_commit::build_pre_commit_script(has_rust, true);
```
This does NOT apply the `GUARDRAIL3_RUST_WORKSPACE` replacement that `generate_all_files()` applies at lines 272-282.

Meanwhile `generate_all_files()` does:
```rust
.replace("GUARDRAIL3_RUST_WORKSPACE:-.}", ...)
```

This means `guardrail3 diff ts` compares against a different hook content than what `guardrail3 generate` would actually produce in a full generate. Specifically, in a monorepo with `workspace_root != "."`, the TS diff will show the hook as up-to-date when it's actually stale (or vice versa).

**Impact:** `check` and `diff ts` can give false positives/negatives for the pre-commit hook in monorepos with non-default workspace roots.

---

## FINDING-11-04: `run_rs` and `run_ts` generate hooks without workspace_root replacement (MEDIUM)

**Location:** `generate.rs:117-120` and `generate.rs:161-163`

Both `run_rs()` and `run_ts()` call `generate_and_install_hooks()` which uses `build_pre_commit_script()` directly. Neither applies the `GUARDRAIL3_RUST_WORKSPACE` replacement from the config. Only `generate_all_files()` (used by the full `generate` command) applies this replacement.

This means:
- `guardrail3 rs generate` installs a hook with default `GUARDRAIL3_RUST_WORKSPACE:-.}`
- `guardrail3 generate` installs a hook with the correct workspace root from config

**Impact:** `rs generate` and `ts generate` produce a different hook than `generate` in monorepo configurations.

---

## FINDING-11-05: `generate_all_files` hook replacement is fragile string manipulation (LOW)

**Location:** `generate.rs:279-282`

```rust
.replace(
    "GUARDRAIL3_RUST_WORKSPACE:-.}",
    &format!("GUARDRAIL3_RUST_WORKSPACE:-{rust_workspace_root}}}"),
)
```

This replaces a substring in the generated shell script. The search string `GUARDRAIL3_RUST_WORKSPACE:-.}` must exactly match the text in `PRE_COMMIT_BASE`. The actual text in `pre_commit.rs:15` is:

```bash
RUST_WORKSPACE="${GUARDRAIL3_RUST_WORKSPACE:-.}"
```

The search pattern `GUARDRAIL3_RUST_WORKSPACE:-.}` will match the substring `GUARDRAIL3_RUST_WORKSPACE:-.}` within `${GUARDRAIL3_RUST_WORKSPACE:-.}"`. This works, but:

1. If `PRE_COMMIT_BASE` ever changes this line's quoting or variable syntax, the replacement silently fails (no match = no replacement = wrong hook).
2. The closing `}}}` in the format string is confusing (one `}` for the bash variable, two `}}` to escape a literal `}` in Rust format string). A refactor could easily break this.

**Impact:** Silent breakage if the pre-commit template changes.

---

## FINDING-11-06: `deny.toml` for per-app crates uses workspace profile, not effective profile (MEDIUM)

**Location:** `generate_helpers.rs:293-298`

```rust
let deny_content = build_deny_for_profile(
    profile,  // <-- workspace-level profile, NOT effective_profile
    &local.deny_bans,
    &local.deny_skip,
    &local.deny_feature_bans,
);
```

But `effective_profile` was computed at line 271-275 and is used for `clippy.toml`. The `deny.toml` always uses the workspace-level `profile` instead. This means a crate with `type = "library"` in a `service` workspace gets a service-profile `deny.toml` (missing the library-io bans) but a library-profile `clippy.toml`.

**Impact:** Library crates in a service workspace get inconsistent deny.toml (missing library-specific I/O crate bans like axum, tokio, reqwest, sqlx, hyper).

---

## FINDING-11-07: `warn_if_overwriting` only warns, does not preserve custom content (LOW)

**Location:** `generate.rs:240-248`

When `generate` overwrites an existing file that differs, it prints a warning directing users to `.guardrail3/overrides/`. But:

1. Only `clippy.toml` and `deny.toml` support overrides. Files like `eslint.config.mjs`, `cspell.json`, `.npmrc`, `.stylelintrc.mjs`, `tsconfig.base.json`, `.jscpd.json` have NO override mechanism.
2. The warning says "Use .guardrail3/overrides/ for project-specific customization" but this is misleading for non-TOML files.

**Impact:** Users who customize non-TOML generated files (especially eslint.config.mjs) will lose changes with no recovery path except version control.

---

## FINDING-11-08: `check.rs` staleness comparison is byte-exact but generation is non-deterministic for per-app configs (LOW)

**Location:** `check.rs:16-29` and `generate_helpers.rs:238-398`

`check.rs` compares `actual != *expected_content` (byte-exact). The generation in `generate_rust_files` depends on `detect_project()` (line 250) which does filesystem discovery. If the filesystem state changes (e.g., a new crate is added but not yet in config), `detect_project()` might return different `app_path_map` results, changing the generated file paths.

This is not a bug per se, but it means `guardrail3 check` can report files as stale purely because the workspace structure changed, even if no config was modified.

**Impact:** Confusing false-positive staleness reports after workspace structure changes.

---

## FINDING-11-09: `diff.rs` `collect_toml_entries` section detection is incomplete (LOW)

**Location:** `diff.rs:191-206`

The section detection hardcodes only three patterns:
- `disallowed-methods` -> "methods"
- `disallowed-types` -> "types"
- `deny` -> "deny"

But `deny.toml` also has sections like `[bans]`, `[licenses]`, `[advisories]`, `[sources]`, `[graph]`, and `[[bans.features]]`. The `deny` detection at line 199 (`trimmed.starts_with("deny")`) will match `deny = [` but NOT `[bans]` which is where the deny array actually lives in TOML structure.

The `]` handling at line 203 resets `current_section` on ANY line that is just `]`, which could match the closing of a TOML inline array.

**Impact:** Custom entry detection in `diff` can produce incorrect results for deny.toml, potentially showing entries as "custom" that are actually in the generated base (or missing custom entries because section context was lost).

---

## FINDING-11-10: `validate_override_content` allows multiline entries to slip through (LOW)

**Location:** `generate_helpers.rs:56-75`

The validation only checks single lines for `{path=` or `{name=` patterns. If an override file contains a multiline TOML entry:
```toml
{ name = "some-crate",
  wrappers = [] },
```

Only the first line passes validation (it starts with `{name=`). The second line (`wrappers = [] },`) is rejected with a warning. The resulting override content has a broken TOML entry: `{ name = "some-crate",\n` without the closing brace.

**Impact:** Multiline override entries produce broken TOML that will cause either a parse error or silent data loss when injected into the generated config.

---

## FINDING-11-11: `deduplicated_override` uses substring matching, not TOML-aware comparison (LOW)

**Location:** `generate_helpers.rs:79-100`

```rust
let key = trimmed.trim_end_matches(',');
if base.contains(key) {
    continue;
}
```

`base.contains(key)` searches the entire accumulated output string. This means:
1. A comment containing the same text would trigger a false dedup match.
2. A partial path match could trigger false dedup. E.g., if the base has `std::fs::read` and the override has `std::fs::read_dir`, the check `base.contains("std::fs::read_dir")` would still work correctly. But if the override has `{ path = "std::fs" }` and the base has `std::fs::read`, `base.contains("std::fs")` would be true, incorrectly deduplicating.

**Impact:** Unlikely but possible incorrect deduplication of override entries.

---

## FINDING-11-12: Pre-commit hook missing `set -e` (MEDIUM)

**Location:** `pre_commit.rs:11`

The shebang line is `set -uo pipefail` but notably absent is `-e` (exit on error). The script relies on explicit `if ! command; then exit 1; fi` patterns for each check, which is correct for the commands that have this pattern. However, any unguarded command failure between checks will be silently ignored.

For example, at line 89:
```bash
PKG_CHANGED=$(echo "$STAGED_FILES" | grep -cE 'package\.json$' || true)
```

This is correctly guarded with `|| true`. But the `git config` command at `generate.rs:192`:
```rust
let _ = std::process::Command::new("git")
    .args(["config", "core.hooksPath", ".githooks"])
```

This is a Rust-side silent failure, not a hook issue. The hook itself appears to handle errors correctly through explicit checking. However, the lack of `set -e` means any new check added without explicit error handling will silently pass.

**Impact:** Any future addition to the hook that forgets explicit error handling will silently pass.

---

## FINDING-11-13: Pre-commit hook prettier check doesn't handle filenames with spaces (LOW)

**Location:** `pre_commit.rs:142`

```bash
if ! pnpm exec prettier --check $(echo "$STAGED_FILES" | grep -E '\.(ts|tsx|mjs|json|css)$') 2>/dev/null; then
```

The `$(echo "$STAGED_FILES" | ...)` expansion is unquoted, so filenames with spaces will be word-split. Same issue at lines 149, 157, 175 for ESLint, cspell, and stylelint.

**Impact:** Pre-commit checks break silently on files with spaces in names.

---

## FINDING-11-14: Pre-commit hook CSS check has precedence bug (LOW)

**Location:** `pre_commit.rs:174`

```bash
if command -v pnpm &> /dev/null && [ -f ".stylelintrc.mjs" ] || [ -f ".stylelintrc.json" ] || [ -f "stylelint.config.mjs" ]; then
```

Due to operator precedence, this evaluates as:
```
(pnpm exists AND .stylelintrc.mjs exists) OR .stylelintrc.json exists OR stylelint.config.mjs exists
```

If `.stylelintrc.json` exists but pnpm is not installed, the condition is true and the `pnpm exec stylelint` command will fail. The intent was likely to check that pnpm exists AND that any config file exists.

**Impact:** Stylelint runs without pnpm when `.stylelintrc.json` or `stylelint.config.mjs` exists, causing a confusing error.

---

## FINDING-11-15: `service_profile_types()` does not include `TYPE_GLOBAL_STATE` but `library_profile_types()` does -- validator ignores this distinction (LOW)

**Location:** `clippy.rs:161-168` vs `clippy_coverage.rs:46-56`

The service profile types are:
- `TYPE_COLLECTIONS`, `TYPE_SYNC`, `TYPE_FILESYSTEM`, `TYPE_GARDE_EXTRACTORS`

The library profile types add `TYPE_GLOBAL_STATE`.

But `EXPECTED_TYPE_BANS` is a single list used for ALL profiles (both service and library). It contains only the service profile types. This means:
- For service profile: validation checks exactly the right bans (correct)
- For library profile: validation does NOT check that global-state bans are present (gap)

This is the validator-side consequence of FINDING-11-02.

**Impact:** Library profile projects pass validation even with missing global-state type bans.

---

## FINDING-11-16: `generate_and_install_hooks` duplicates code with `run_hooks` (COSMETIC)

**Location:** `generate.rs:168-195` vs `generate.rs:198-236`

`generate_and_install_hooks()` and `run_hooks()` both create `.githooks/pre-commit`, set permissions, and configure git. But they differ in error handling:
- `generate_and_install_hooks` silently ignores `create_dir_all` failure (`let _`)
- `run_hooks` properly reports and exits on `create_dir_all` failure

And only `run_hooks` prints the manual `git config` instruction. `generate_and_install_hooks` silently runs `git config` but does not tell the user about it.

**Impact:** Inconsistent error handling between the two code paths.

---

## FINDING-11-17: `eslint.config.mjs` generated vs `ESLINT_STARTER` module are completely different (LOW)

**Location:** `canonical.rs:251-338` vs `eslint.rs`

The `ESLINT_STARTER` module (in canonical.rs) is a stripped-down ESLint config with basic rules. The actual generated `eslint.config.mjs` (from `eslint.rs:build_eslint_config`) is a comprehensive config with unicorn, regexp, sonarjs, react, boundaries, a11y, tailwind-ban plugins and dozens more rules.

`ESLINT_STARTER` is registered in `all_modules()` and can be viewed via `show-module canonical/eslint-starter`. But what the user actually gets from `guardrail3 generate` is the full config from `eslint.rs`. The "starter" is misleading -- it's never generated anywhere, it just sits in the module registry as a historical artifact.

**Impact:** `show-module canonical/eslint-starter` shows content that does not match what `generate` produces. Users inspecting the module get wrong expectations.

---

## FINDING-11-18: `release-plz.toml` contains placeholder `your-crate-name` (LOW)

**Location:** `release.rs:17-18`

```toml
[[package]]
name = "your-crate-name"
```

This is generated as-is into the project. If the user runs `guardrail3 generate` and doesn't notice, `release-plz` will fail or do nothing because `your-crate-name` doesn't match any actual crate.

**Impact:** Generated release-plz.toml is non-functional out of the box. Should either be auto-populated from workspace discovery or clearly marked as requiring user action.

---

## FINDING-11-19: `check.rs` and `diff.rs` don't check for extra files on disk (LOW)

**Location:** `check.rs`, `diff.rs`

Both commands only iterate over the expected generated files and check if they match. Neither checks for the reverse: files that WERE generated previously but are no longer expected (e.g., after switching from service to library profile, `release-plz.toml` and `cliff.toml` should be removed but won't be flagged).

**Impact:** Stale generated files from a previous profile/config persist on disk without warning.

---

## FINDING-11-20: No race condition protection between `generate` and `check` (LOW)

**Location:** `generate.rs` and `check.rs`

If `guardrail3 generate` and `guardrail3 check` run concurrently (e.g., in a CI pipeline), there's no file locking. A `check` could read a partially-written file from a concurrent `generate`. In practice this is unlikely in CI (they'd be sequential), but in a development environment with file watchers, it's possible.

**Impact:** Theoretical race condition producing false staleness reports.

---

## Summary

| ID | Severity | Finding |
|---|---|---|
| 11-01 | MEDIUM | `all_modules()` missing DENY_BANS_LIBRARY_IO, STYLELINT |
| 11-02 | MEDIUM | `EXPECTED_TYPE_BANS` missing 4 TYPE_GLOBAL_STATE entries |
| 11-03 | MEDIUM | `generate_expected_ts` skips workspace_root hook replacement |
| 11-04 | MEDIUM | `run_rs`/`run_ts` skip workspace_root hook replacement |
| 11-05 | LOW | Hook workspace_root replacement is fragile string manipulation |
| 11-06 | MEDIUM | Per-app deny.toml uses workspace profile, not effective profile |
| 11-07 | LOW | No override mechanism for non-TOML generated files |
| 11-08 | LOW | Staleness check sensitive to workspace structure changes |
| 11-09 | LOW | diff.rs TOML section detection incomplete for deny.toml |
| 11-10 | LOW | Multiline override entries produce broken TOML |
| 11-11 | LOW | Dedup uses substring matching, not TOML-aware comparison |
| 11-12 | MEDIUM | Pre-commit hook missing `set -e` |
| 11-13 | LOW | Pre-commit unquoted expansion breaks on spaces in filenames |
| 11-14 | LOW | CSS check operator precedence bug |
| 11-15 | LOW | Validator ignores profile distinction for type bans |
| 11-16 | COSMETIC | Duplicated hook installation code with inconsistent error handling |
| 11-17 | LOW | ESLINT_STARTER module doesn't match actual generated eslint config |
| 11-18 | LOW | release-plz.toml contains placeholder crate name |
| 11-19 | LOW | check/diff don't detect stale files from previous config |
| 11-20 | LOW | No race condition protection between generate and check |

**MEDIUM findings: 5** (11-01, 11-02, 11-03, 11-04, 11-06)
**LOW findings: 13**
**COSMETIC findings: 1**
