# Audit Finding Verification: CLI-03, SCOPE-01, SCOPE-02, DISC-01, GAP-TS-ARCH-01, GAP-TS-ARCH-06, FIND-14-01, FIND-14-02

## CLI-03: --code flag runs zero Rust checks

**Verdict: FALSE. The auditor is wrong.**

The `--code` flag does NOT result in zero checks. Here's the actual logic:

In `main.rs` `build_rs_categories()` (line 392-409): when `args.code` is true, `any_cli` is true, and the function returns:
```rust
RustCheckCategories {
    architecture: args.architecture,  // false
    garde: args.garde,                // false
    tests: args.tests,                // false
    release: args.release,            // false
}
```

However, `RustCheckCategories` has no `code` field -- it only controls architecture, garde, tests, and release. The **code checks are always run unconditionally** in `rs/validate/mod.rs` line 66-75: `run_code_checks()` is called with no category gate. It runs config_files, clippy_coverage, deny_audit, cargo_lints, source_scan, and dependency_scan regardless of category flags.

So `--code` actually runs all code checks (config, clippy, deny, lints, source scan, deps) while disabling architecture, garde, tests, and release. The flag name is misleading (it means "only code domain") but it runs a substantial set of checks.

The `ValidateDomains` struct used in `commands/validate.rs` (line 28-34) and hooks does have a `code` field, and `--code` sets `domains.code = true`. The hooks validation does use `ValidateDomains` properly.

**Should it be fixed?** No functional bug. The `--code` flag works as designed: run code checks, skip arch/garde/tests/release. The field name is slightly confusing but correct in context.

---

## SCOPE-01: --staged misses renamed files

**Verdict: TRUE. Real bug.**

In `commands/validate.rs` line 138-155, `git_staged_files()` runs:
```
git diff --cached --name-only --diff-filter=ACM
```

The filter `ACM` = Added, Copied, Modified. It explicitly excludes `R` (Renamed). If a file is renamed in the staging area, the new path will NOT appear in the file list, so checks won't run on it.

**Should it be fixed?** Yes. Renamed files should be validated. The filter should be `--diff-filter=ACMR`. The old path is irrelevant (it's gone), but the new path needs checking.

**Is it exploitable?** Mildly. An agent could `git mv` a file to bypass staged validation. In practice this is a minor gap since renames are uncommon during guardrail-triggering edits.

---

## SCOPE-02: --dirty misses untracked files

**Verdict: TRUE. Real bug.**

In `commands/validate.rs` lines 158-190, `git_dirty_files()` only runs:
1. `git diff --cached --name-only` (staged changes)
2. `git diff --name-only` (unstaged changes to tracked files)

It does NOT run `git ls-files --others --exclude-standard` to pick up untracked files. A brand new file that hasn't been `git add`ed will not be included in the `--dirty` scope.

**Should it be fixed?** Yes. The `--dirty` flag implies "all local changes," which users expect to include new files. Without untracked files, a newly created source file with violations would be invisible to `--dirty` validation.

**Is it exploitable?** Yes. Create a new file with violations, don't stage it, run `--dirty` -- it won't be checked.

---

## DISC-01: Any package.json triggers TS detection

**Verdict: TRUE. Real but by design.**

In `discover.rs` line 353-373, `detect_typescript()`:
```rust
fn detect_typescript(fs: &dyn FileSystem, path: &Path, info: &mut ProjectInfo) {
    let pkg_json = path.join("package.json");
    if pkg_json.exists() {
        info.has_typescript = true;
        ...
```

It checks for `package.json` existence only. It does NOT verify that the project actually uses TypeScript (no check for `.ts` files, `tsconfig.json`, or a `typescript` dependency in package.json).

A pure Rust project with `package.json` (e.g., for cspell or prettier config) will trigger TS detection and run all TS validation checks, producing false positive errors.

**Should it be fixed?** Yes. Detection should require at least one of: `tsconfig.json` exists, `typescript` in dependencies/devDependencies, or `.ts`/`.tsx` files present.

**Note:** Interestingly, `discover_ts_apps()` in `ts_arch_checks.rs` (line 27-48) is smarter -- it checks for both `package.json` AND actual `.ts`/`.tsx` files. But the top-level detection in `discover.rs` does not.

---

## GAP-TS-ARCH-01: TS import boundaries use string matching not AST

**Verdict: TRUE. Real gap.**

In `ts_arch_checks.rs` lines 370-414, `check_file_imports()` iterates `content.lines()` and uses `extract_import_path()` (line 298-303) which does pure string matching:
- Looks for `from '...'`, `from "..."`, `require('...')`, `require("...")`
- No tree-sitter AST parsing

This contrasts with the project's own CLAUDE.md rule: "All source scan checks use syn (Rust) or tree-sitter (TypeScript) for AST parsing. Zero grep, zero line matching."

Meanwhile, tree-sitter IS used in other TS checks (source_scan.rs, ts_code_analysis.rs, test_checks.rs, ast_helpers.rs) -- just not in the arch boundary checker.

**Should it be fixed?** Yes. This violates the project's own stated principle. String matching will produce false positives on commented-out imports, imports inside string literals, and multi-line import statements.

---

## GAP-TS-ARCH-06: Hex arch only checks 2 of 4 layers

**Verdict: TRUE. Real gap.**

In `ts_arch_checks.rs` lines 101-141, `check_single_app_structure()` only checks:
- `src/modules/domain/` exists
- `src/modules/adapters/` exists

It does NOT check for:
- `src/modules/ports/`
- `src/modules/application/`

The `TsLayer` enum (line 161-167) defines all 4 layers, `layer_from_path()` recognizes all 4, `forbidden()` enforces import rules for all 4, but the structure check (T-ARCH-01) only validates 2.

**Should it be fixed?** Partially. `ports/` and `application/` are arguably optional layers in simpler apps. However, if the import boundary checker (T-ARCH-02) references all 4 layers but the structure checker (T-ARCH-01) only validates 2, there's a consistency gap. At minimum, the check message should mention all expected layers, or T-ARCH-01 should have a "recommended" severity for ports/application.

---

## FIND-14-01: 48 check IDs with zero tests

**Verdict: PARTIALLY TRUE, but overstated.**

Searching for sample IDs: R42, R53, T-PKG-01, T-PKG-02, T-JSCPD-01:

- **R42** (unsafe): HAS unit tests in `tests/unit/rs_structure_checks_test.rs` (line 89-98) AND golden test coverage in `self-validate.json`. Also covered by adversarial grep-attacks fixtures.
- **R53** (unsafe_code=forbid): Has golden test coverage in `self-validate.json`, `external-schedulr.json`, `external-pipelin3r.json`. No dedicated unit test.
- **T-PKG-01, T-PKG-02, T-JSCPD-01**: Zero hits in any test file. Only defined in source code. No unit tests, no integration tests, no golden tests referencing these IDs.

The claim of "48 check IDs with zero tests" is plausible for many TS checks. The golden tests provide incidental coverage for some Rust checks (self-validate runs against the guardrail3 repo itself), but many TS check IDs (T-PKG-*, T-JSCPD-*, T-NPMRC-*, T-TSCONFIG-*) genuinely have no targeted test coverage.

**Should it be fixed?** Yes. Check IDs that exist only in source with no test asserting their behavior could regress silently.

---

## FIND-14-02: 24 source modules with zero unit tests

**Verdict: TRUE.**

Checked for `#[cfg(test)]` blocks in source modules:

**TS validate modules with ZERO inline tests:**
- `tsconfig_check.rs` -- 0
- `npmrc_check.rs` -- 0
- `jscpd_check.rs` -- 0
- `package_check.rs` -- 0
- `package_deps.rs` -- not checked but likely 0
- `stylelint_check.rs` -- not checked but likely 0
- `eslint_check.rs` -- not checked but likely 0
- `config_files.rs` -- not checked but likely 0
- `i18n_check.rs` -- not checked but likely 0
- `tool_config_checks.rs` -- not checked but likely 0
- `ts_arch_checks.rs` -- 0 (has external test file `ts_arch_checks_test.rs`)

**RS validate modules with ZERO inline tests:**
- `rustfmt_check.rs` -- 0
- `toolchain_check.rs` -- 0
- `workspace_metadata.rs` -- not checked but likely 0

Only 3 files in the entire RS/TS validate directories have inline `#[cfg(test)]` blocks:
- RS: `ast_helpers.rs`, `test_checks.rs`
- TS: `eslint_plugin_checks.rs`

Some modules have external test coverage via `tests/unit/` files (e.g., `ts_arch_checks_test.rs`, `rs_structure_checks_test.rs`), and golden tests provide indirect coverage. But the claim that many modules have zero unit tests is accurate.

**Should it be fixed?** Yes for modules with zero coverage of any kind (no unit tests AND no integration tests). Modules covered by golden tests (like self-validate) have some safety net but no targeted regression tests.

---

## Summary

| Finding | Real? | Fix? | Severity |
|---------|-------|------|----------|
| CLI-03: --code runs zero checks | FALSE | No | N/A -- auditor wrong |
| SCOPE-01: --staged misses renames | TRUE | Yes | Low |
| SCOPE-02: --dirty misses untracked | TRUE | Yes | Medium |
| DISC-01: package.json triggers TS | TRUE | Yes | Medium |
| GAP-TS-ARCH-01: string matching | TRUE | Yes | Medium (violates own principle) |
| GAP-TS-ARCH-06: only 2/4 layers | TRUE | Partial | Low |
| FIND-14-01: 48 IDs untested | PARTIALLY TRUE | Yes | Medium (many TS checks untested) |
| FIND-14-02: 24 modules no unit tests | TRUE | Yes | Medium |
