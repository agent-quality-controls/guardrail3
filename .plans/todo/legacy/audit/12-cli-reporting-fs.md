# Audit 12: CLI, Reporting, Filesystem Module, Discovery

**Scope:** cli.rs, main.rs, fs.rs, discover.rs, report/{types,text,json,markdown}.rs, commands/validate.rs

---

## CLI (cli.rs + main.rs)

### CLI-01: No mutual exclusion on scope flags (ERROR)
**File:** `apps/guardrail3/src/cli.rs` lines 149-167
`--staged`, `--dirty`, `--commits N`, and `--files` are all independent boolean/option flags with no mutual exclusion. A user can pass `--staged --dirty --commits 5 --files foo.rs` simultaneously. The resolution in `commands/validate.rs` (`resolve_scoped_files`) uses a priority cascade (`files` > `staged` > `dirty` > `commits`), but the user receives NO warning that conflicting flags were silently ignored. This is confusing and error-prone — a user who passes `--staged --dirty` expects both semantics, but only staged runs.

### CLI-02: --garde flag is silently ignored for TypeScript (WARN)
**File:** `apps/guardrail3/src/main.rs` lines 432-445
The `--garde` flag is defined on `ValidateArgs` (shared between `rs validate` and `ts validate`). For TypeScript, `build_ts_categories` checks `args.garde` as part of `any_cli` (line 432) but never maps it to a TS category. Result: passing `--garde` to `ts validate` sets `any_cli = true`, which causes ALL TS checks except `--garde` to be skipped (they default to false). So `guardrail3 ts validate --garde` runs ZERO checks. This is a silent correctness failure.

### CLI-03: --code flag has no effect on Rust categories (ERROR)
**File:** `apps/guardrail3/src/main.rs` lines 393-408, `apps/guardrail3/src/commands/validate.rs` lines 243-258
When `--code` is passed, `any_cli = true` but `RustCheckCategories` has no `code` field. So `--code` triggers the "filter mode" branch but only enables categories explicitly mapped from CLI args (architecture, garde, tests, release). The `--code` flag itself maps to... nothing in `RustCheckCategories`. Result: `guardrail3 rs validate --code` runs ZERO Rust category checks (architecture=false, garde=false, tests=false, release=false). This is actively harmful.

### CLI-04: domains_from_args ignores --garde for ValidateDomains (WARN)
**File:** `apps/guardrail3/src/main.rs` lines 352-360
`domains_from_args` computes `run_all` by checking `!args.code && !args.architecture && !args.release && !args.tests && !args.garde`, but `ValidateDomains` has no `garde` field. So `--garde` suppresses the default `run_all=true` behavior but doesn't enable anything in domains. For `hooks-validate` which uses `ValidateDomains`, `--garde` means all domains are false. Zero hook checks run.

### CLI-05: profile argument not validated in rs init (WARN)
**File:** `apps/guardrail3/src/cli.rs` lines 84-89
`--profile` accepts any string with default "service", but there's no validation that the value is "service" or "library". Invalid values like `--profile banana` are accepted silently. The `garde::Validate` derive is present but no garde annotation validates the profile string.

### CLI-06: Map command has no validate subcommand (INFO)
**File:** `apps/guardrail3/src/cli.rs` lines 39-77, `apps/guardrail3/src/main.rs` lines 60-107
The `map` command's `path` argument is NOT canonicalized — it's passed raw as `&path` (a `&String`). Compare with `validate` commands which call `resolve_path()` to canonicalize. If the path doesn't exist, `crawl()` will silently fail or produce garbage results with no user-facing error message.

### CLI-07: Duplicate code between main.rs and commands/validate.rs (WARN)
**Files:** `apps/guardrail3/src/main.rs` lines 370-446 vs `apps/guardrail3/src/commands/validate.rs` lines 219-290
`load_config`, `build_rs_categories`, and `build_ts_categories` are fully duplicated between main.rs and commands/validate.rs. These are NOT re-exports — they are independent, parallel implementations. If one is updated without the other, `guardrail3 rs validate` and `guardrail3 validate` will silently diverge in behavior.

### CLI-08: `guardrail3 validate` (top-level) command not in CLI enum (INFO)
**File:** `apps/guardrail3/src/cli.rs`
The CLAUDE.md documents `guardrail3 validate [path]` as auto-detecting stacks, but `Commands` enum only has `Rs`, `Ts`, `DumpGuide`, and `Map`. There is no top-level `Validate` variant. The `commands/validate.rs::run()` function exists and handles both stacks, but it appears to only be called from the `rs validate` path via main.rs — the auto-detect path may be dead code or wired elsewhere.

**UPDATE:** Looking again at main.rs, `run_rs_validate` calls `commands::validate::resolve_scoped_files_pub` but NOT `commands::validate::run`. The top-level combined validation logic in `commands/validate.rs::run()` appears to be unused from main.rs. This is either dead code or wired from a different entry point not in scope.

---

## Scope Flags & Git Resolution (commands/validate.rs)

### SCOPE-01: --staged misses renamed files (ERROR)
**File:** `apps/guardrail3/src/commands/validate.rs` lines 138-155
`git_staged_files` uses `--diff-filter=ACM` (Added, Copied, Modified). This explicitly EXCLUDES:
- **R** (Renamed) — a renamed file's new path is not picked up. If you rename `foo.rs` to `bar.rs` and stage it, `--staged` will NOT check `bar.rs`.
- **T** (Type change) — e.g., symlink to regular file.

### SCOPE-02: --dirty misses untracked new files (ERROR)
**File:** `apps/guardrail3/src/commands/validate.rs` lines 158-190
`git_dirty_files` only runs `git diff --cached --name-only` and `git diff --name-only`. Neither captures UNTRACKED files. A brand-new file that hasn't been `git add`ed is invisible to `--dirty`. Users reasonably expect `--dirty` to mean "everything that's different from HEAD," which includes untracked files.

### SCOPE-03: --dirty missing diff-filter means deleted files are included (WARN)
**File:** `apps/guardrail3/src/commands/validate.rs` lines 158-190
Unlike `--staged` which uses `--diff-filter=ACM`, `--dirty` has NO diff filter. This means deleted files (filter D) are included in the file list. Guardrail3 will then try to validate files that no longer exist on disk.

### SCOPE-04: --commits misses renamed file destinations (WARN)
**File:** `apps/guardrail3/src/commands/validate.rs` lines 193-217
`git_commit_files` uses `--diff-filter=ACM` same as staged. It also uses `git log --name-only` which only shows one name per file. With renames, `--name-only` shows the NEW name but `--diff-filter=ACM` excludes R. The interaction is subtle — `git log --name-only --diff-filter=ACM` may or may not include the new name of a renamed file depending on git version. This is fragile.

### SCOPE-05: --files paths are not canonicalized or validated (WARN)
**File:** `apps/guardrail3/src/commands/validate.rs` lines 118-120
When `--files` is used, the paths are passed through verbatim (`args.files.clone()`). They are NOT:
- Canonicalized to absolute paths
- Checked for existence
- Resolved relative to the project root

But `git_staged_files` and `git_dirty_files` produce absolute paths via `to_abs_path`. This means scope filtering downstream may fail to match `--files` entries against git-produced paths because one is relative and the other is absolute.

### SCOPE-06: git command failures silently disable scoping (WARN)
**File:** `apps/guardrail3/src/commands/validate.rs` lines 138-217
All three git functions (`git_staged_files`, `git_dirty_files`, `git_commit_files`) return `None` on any failure (command not found, not a git repo, etc.). `None` means "no scope filter" which means "run all checks on all files." So if git is not installed or the project isn't a git repo, `--staged` silently behaves like no flag was passed. The user thinks they're scoping to staged files; they're actually running full validation.

### SCOPE-07: --commits 0 runs git log -0 (INFO)
**File:** `apps/guardrail3/src/commands/validate.rs` line 198
`n: usize` allows 0. `git log -0` outputs nothing, resulting in an empty file list. This is probably harmless but confusing.

---

## Discovery (discover.rs)

### DISC-01: detect_typescript false positive on package.json without TypeScript (ERROR)
**File:** `apps/guardrail3/src/app/discover.rs` lines 353-373
`detect_typescript` sets `has_typescript = true` if ANY `package.json` exists — even for a pure JavaScript project with no TypeScript files, no tsconfig, no `typescript` dependency. The presence of `package.json` does NOT mean TypeScript. This causes all TS checks to run on non-TS projects, producing false error reports.

### DISC-02: Fallback cascade can lose data (WARN)
**File:** `apps/guardrail3/src/app/discover.rs` lines 60-77
The fallback logic (check `crates/`, then `apps/backend/`) does `info.has_rust = false; info.workspaces.clear()` before trying the fallback. If the primary detection found SOME data (e.g., has_rust=true but no members because it's a non-workspace Cargo.toml), the fallback CLEARS that data and then tries `crates/Cargo.toml`. If `crates/Cargo.toml` doesn't exist, info is now empty — worse than before the fallback.

Specifically line 62: `if crates_path.join("Cargo.toml").exists()` — this uses `Path::exists()` directly, NOT the `FileSystem` trait. This bypasses the testable filesystem abstraction.

### DISC-03: Path::exists() used directly, bypassing FileSystem trait (WARN)
**File:** `apps/guardrail3/src/app/discover.rs` lines 62, 72, 93, 136, 184, 283, 310, 355, 365
Multiple calls to `.exists()` on `Path` objects, bypassing the `FileSystem` trait that the module accepts as a parameter. This makes these code paths untestable with mock filesystems and inconsistent with the project's centralized fs philosophy.

### DISC-04: discover_nested_workspaces uses DirEntry::path().clone() redundantly (INFO)
**File:** `apps/guardrail3/src/app/discover.rs` line 114
`let entry_path = entry.path().clone();` — `entry.path()` already returns a `PathBuf` (owned). The `.clone()` is unnecessary allocation.

### DISC-05: Glob patterns from Cargo.toml workspace members are unsanitized (WARN)
**File:** `apps/guardrail3/src/app/discover.rs` lines 132-134, 277-280
Workspace member patterns from `Cargo.toml` are passed directly to `glob::glob()`. A malicious `Cargo.toml` with `members = ["../../../etc/*"]` would cause discovery to traverse outside the project root. While this is unlikely in practice, it's a path traversal vector in untrusted codebases.

### DISC-06: Nested workspace discovery only checks apps/ (INFO)
**File:** `apps/guardrail3/src/app/discover.rs` lines 91-177
`discover_nested_workspaces` only scans `apps/` for nested workspaces. Workspaces under `packages/`, `crates/`, `services/`, or any other directory naming convention are invisible. This is a design limitation that may produce incomplete results for non-standard monorepo layouts.

---

## Filesystem Module (fs.rs)

### FS-01: read_file silently swallows all errors (WARN)
**File:** `apps/guardrail3/src/fs.rs` lines 12-14
`read_file` returns `None` for permission denied, encoding errors, I/O errors — all indistinguishable from "file doesn't exist." Callers cannot distinguish "file missing" from "file exists but is unreadable" from "file exists but contains invalid UTF-8." This leads to silent data loss in validation — a file that exists but can't be read is treated as non-existent, potentially causing false "file missing" errors instead of "file unreadable" errors.

### FS-02: No symlink handling (WARN)
**File:** `apps/guardrail3/src/fs.rs`
Neither `read_file` nor `metadata` handles symlinks explicitly. `std::fs::read_to_string` follows symlinks by default, which means:
- Symlinks pointing outside the project root can be read (path traversal)
- Circular symlinks will cause errors (swallowed by `.ok()`)
- `metadata` follows symlinks (uses `fs::metadata` not `fs::symlink_metadata`)

### FS-03: No file size limits on read_file (WARN)
**File:** `apps/guardrail3/src/fs.rs` lines 12-14
`read_file` reads the entire file into memory with no size limit. A malicious or accidentally large file (e.g., a 10GB Cargo.lock, a binary accidentally named .toml) will cause OOM. The pre-commit hook has a 1MB file size check, but the validation path has no such guard.

### FS-04: write_file has no path sanitization (WARN)
**File:** `apps/guardrail3/src/fs.rs` lines 51-56
`write_file` creates parent directories and writes content with no validation that the path is within an expected directory. Combined with user-provided paths from CLI args, this could write to arbitrary locations. For example, `guardrail3 dump-guide` writes to a hardcoded relative path `GUARDRAIL3_GUIDE.md` (safe), but if any code path passes an attacker-controlled path to `write_file`, it's exploitable.

### FS-05: list_dir returns empty Vec on error, indistinguishable from empty directory (INFO)
**File:** `apps/guardrail3/src/fs.rs` lines 27-32
Same pattern as FS-01. Permission denied on a directory looks the same as an empty directory. Callers can't distinguish.

### FS-06: No atomic write (INFO)
**File:** `apps/guardrail3/src/fs.rs` lines 51-56
`write_file` does `create_dir_all` then `fs::write`. If the process crashes between these two calls, or if `fs::write` is interrupted, the file may be left in a corrupted state. For config generation, this could leave a project with a partial clippy.toml.

---

## Reporting (report.rs, text.rs, json.rs, markdown.rs)

### RPT-01: JSON output ignores _show_inventory parameter (WARN)
**File:** `apps/guardrail3/src/report/json.rs` line 12
The `_show_inventory` parameter is accepted but unused (prefixed with `_`). The comment says "JSON always includes ALL results." This is a reasonable design choice, BUT the `--inventory` flag has no effect on JSON output, and the user isn't told this. More importantly, the JSON `summary.info` count includes inventory items, while the text/markdown `info` count also includes them. If a consumer uses `summary.errors > 0` for CI, this is fine. But `summary.info` being inflated by hidden inventory items is misleading.

### RPT-02: JSON output missing --verbose flag entirely (INFO)
**File:** `apps/guardrail3/src/report/json.rs` line 12
`print_report` accepts `(report, show_inventory)` but NOT `verbose`. The JSON format doesn't support summarization at all — it always dumps everything. This is fine for machine consumption, but means the JSON and text outputs are structurally different: text may summarize N results into "5 entries", while JSON shows all 5. Consumers switching formats may see different apparent structure.

### RPT-03: JSON schema has no version field (WARN)
**File:** `apps/guardrail3/src/report/json.rs` lines 68-77
The JSON output has no `"version"` or `"schema_version"` field. If the schema changes (adding fields, renaming fields, changing types), consumers have no way to detect the version they're parsing. For a tool designed for CI pipelines, this is a stability risk.

### RPT-04: JSON uses `unwrap_or_else` for serialization failure (INFO)
**File:** `apps/guardrail3/src/report/json.rs` line 81
`serde_json::to_string_pretty(&output).unwrap_or_else(...)` — if serialization fails, it prints `{"error": "..."}` to stdout. But the exit code is still determined by `report.error_count()`, not by the serialization failure. A serialization failure produces invalid/misleading output with a potentially zero exit code.

### RPT-05: error_count includes inventory items (WARN)
**File:** `apps/guardrail3/src/domain/report.rs` lines 159-165
`count_by_severity` (used by `error_count`, `warn_count`, `info_count`) counts ALL results regardless of `inventory` flag. This means if an inventory item has `Severity::Error` (which shouldn't happen by convention but nothing enforces it), it would count toward the exit code. The `as_inventory` method doesn't constrain severity.

### RPT-06: Markdown table cells not fully escaped (WARN)
**File:** `apps/guardrail3/src/report/markdown.rs` lines 143-145
Pipe characters (`|`) are escaped in title, message, and location. But the `id` field (line 148-149) is NOT escaped. If a check ID contains a pipe character, the markdown table breaks. While unlikely with current IDs (R1, T23, etc.), there's no compile-time guarantee.

### RPT-07: Text report uses Unicode icons that may not render in all terminals (INFO)
**File:** `apps/guardrail3/src/report/text.rs` lines 92-96
Unicode characters `\u{2717}` (CROSS MARK), `\u{26a0}` (WARNING SIGN), `\u{2139}` (INFORMATION SOURCE) require Unicode support. Some CI environments, Windows terminals, and log aggregators don't render these correctly.

### RPT-08: Summary counts in JSON may disagree with visible counts in text (INFO)
The JSON summary counts ALL results (including inventory). The text summary counts ALL results but hides inventory from display. So `summary.errors` in JSON matches the exit code determination, but the visual count in text may differ from what the user sees if they count visible items. This is confusing but not strictly a bug.

### RPT-09: Markdown shows "No checks in this section" for empty sections but text skips them (INFO)
**File:** `apps/guardrail3/src/report/markdown.rs` lines 33-39 vs `apps/guardrail3/src/report/text.rs` lines 37-40
Inconsistent behavior between formats. Markdown shows a section header with "No checks in this section" when `section.results.is_empty()`. Text silently skips such sections. JSON includes them as empty arrays. A consumer comparing outputs across formats will see structural differences.

---

## Exit Codes

### EXIT-01: Inconsistent exit codes (WARN)
**File:** `apps/guardrail3/src/main.rs`
- Parse failure: exit 2 (line 53)
- Validation error (garde): exit 2 (line 337)
- Path resolution failure: exit 1 (line 348)
- Report with errors: exit 1 (line 328)
- File write error (dump-guide): exit 1 (line 169)

Exit code 1 is used for BOTH "validation found errors" and "tool failed." A CI pipeline cannot distinguish "guardrail3 found violations" from "guardrail3 crashed trying to read the path." Consider exit code 1 for violations and exit code 2 for tool errors (currently only used for parse/validation errors).

### EXIT-02: No exit code on warnings-only (INFO)
**File:** `apps/guardrail3/src/main.rs` line 327
`print_report` only exits non-zero on `error_count() > 0`. If the report has 100 warnings and 0 errors, exit code is 0. This is by design, but there's no `--strict` or `--warn-as-error` flag to let users opt into treating warnings as failures. For guardrails meant to enforce standards, this seems like a missing feature.

---

## Path Traversal / Security

### SEC-01: No project root sandboxing (WARN)
**File:** `apps/guardrail3/src/commands/validate.rs`, `apps/guardrail3/src/app/discover.rs`
There is no mechanism to constrain filesystem access to the project root. Discovery follows glob patterns from Cargo.toml (DISC-05), git commands return paths relative to the git root (which may differ from the project path), and `--files` accepts arbitrary paths. A crafted `--files ../../../etc/passwd` would be passed to validators.

### SEC-02: Git command injection not possible but git output is trusted (INFO)
Git commands use `Command::new("git").args([...])` with no shell interpolation, so command injection is not possible. However, git output (file paths) is trusted without sanitization. A git repo with a file named with special characters (newlines, null bytes) in the path could cause unexpected behavior since the output is split by lines.

---

## Counting Summary

| Severity | Count |
|----------|-------|
| ERROR | 5 |
| WARN | 18 |
| INFO | 10 |
| **Total** | **33** |

### ERROR findings (must fix):
1. **CLI-01**: No mutual exclusion on scope flags — conflicting flags silently ignored
2. **CLI-03**: `--code` flag runs ZERO checks for Rust validation
3. **SCOPE-01**: `--staged` misses renamed files (`--diff-filter=ACM` excludes R)
4. **SCOPE-02**: `--dirty` misses untracked new files
5. **DISC-01**: Any `package.json` triggers TypeScript detection regardless of actual TS usage
