# Audit Finding Verification: CRIT-01 through F04

**Date:** 2026-03-19
**Verifier:** Verification agent (code-level review)

---

## CRIT-01: `missing_docs` and `missing_debug_implementations` not in EXPECTED_RUST_LINTS

**Verdict: REAL finding, SHOULD be fixed.**

The canonical module at `canonical.rs:49-56` defines the recommended `[workspace.lints.rust]` and includes:
```toml
missing_docs = "deny"
missing_debug_implementations = "warn"
```

However, `EXPECTED_RUST_LINTS` in `cargo_lints.rs:12-38` only contains 5 entries:
- `warnings` = deny
- `unsafe_code` = forbid
- `dead_code` = deny
- `unused_results` = deny
- `unused_crate_dependencies` = deny

`missing_docs` and `missing_debug_implementations` are **not checked**. This means a project could remove these two lints from its `[workspace.lints.rust]` and guardrail3 would not flag it.

**Exploitable?** Yes. A project using the canonical template that later removes `missing_docs = "deny"` would pass validation silently. The canonical module recommends them, but validation doesn't enforce them.

**Fix:** Add both lints to `EXPECTED_RUST_LINTS` in `cargo_lints.rs`.

---

## CRIT-02: `_profile` param ignored in `clippy_coverage::check`

**Verdict: REAL finding, but LOW practical impact currently.**

In `clippy_coverage.rs:58-62`:
```rust
pub fn check(
    fs: &dyn FileSystem,
    workspace_root: &Path,
    _profile: Option<&str>,
) -> Vec<CheckResult> {
```

The `_profile` parameter is prefixed with `_` and completely unused. Lines 111-114 show:
```rust
// All profiles (service, library) use the same expected bans.
// Unknown/missing profiles default to service (the most comprehensive set).
let expected_methods = EXPECTED_METHOD_BANS;
let expected_types = EXPECTED_TYPE_BANS;
```

This is an **intentional design decision**, documented in the code. Both service and library profiles currently use the same method/type bans. The parameter exists as a forward-looking hook for when profiles diverge (e.g., library profile might not need HTTP client bans).

**Exploitable?** No. Both profiles use the strictest ban set. If anything, the library profile could arguably be even more restrictive in the future, not less. The `_profile` prefix is idiomatic Rust for "accepted but intentionally unused."

**Should it be fixed?** Not urgently. The code comment explains the design. When profile-specific ban lists are needed, the parameter is already wired through. This is a style/cleanliness issue, not a correctness issue.

---

## CRIT-03: `path.exists()` bypassing FileSystem trait

**Verdict: REAL finding, but PERVASIVE and BY DESIGN.**

`config_files.rs` has 4 direct `.exists()` calls (lines 16, 43, 71, 112). However, this is NOT specific to `config_files.rs`. A grep across the entire `rs/validate/` directory reveals **21 direct `.exists()` calls** across 12 files:
- `config_files.rs` (4 calls)
- `cargo_lints.rs` (2 calls)
- `clippy_coverage.rs` (1 call)
- `deny_audit.rs` (1 call)
- `structure_checks.rs` (1 call)
- `workspace_metadata.rs` (1 call)
- `allow_checks.rs` (1 call)
- `code_quality_checks.rs` (1 call)
- `release_repo_checks.rs` (3 calls)
- `release_crate_checks.rs` (2 calls)
- `test_checks.rs` (1 call)
- `test_quality_checks.rs` (2 calls)
- `hex_arch_checks.rs` (1 call)

The `FileSystem` trait does NOT have an `exists()` method. It provides `read_file()` and `read_file_err()`. The pattern throughout the codebase is: use `.exists()` for path existence checks, use `fs.read_file()` for content access.

**Exploitable?** In theory, `.exists()` calls bypass the `FileSystem` trait abstraction, making these code paths harder to test with mock filesystems. However, in practice: (a) the real filesystem is the only implementation used in production, (b) tests that need to mock file absence can mock `read_file()` to return `None`, and (c) adding `exists()` to the trait would require updating all 21 call sites plus all test mocks.

**Should it be fixed?** This is a legitimate architectural observation, but fixing it is a significant refactor (add `exists()` to `FileSystem` trait, update 12+ files, update all mock implementations). The current pattern works correctly. Low priority.

---

## CRIT-R42: `check_unsafe` dead code

**Verdict: REAL. The function is defined but never called from production code.**

`check_unsafe` is defined at `structure_checks.rs:72-93` as `pub fn check_unsafe(...)`.

A codebase-wide grep for `check_unsafe` returns only:
1. The definition itself (`structure_checks.rs:72`)
2. `check_unsafe_code_forbid` definition (`structure_checks.rs:96`) — different function

The function is NOT called from `source_scan.rs` (the orchestrator for per-file checks). Looking at `source_scan.rs:32-63`, the per-file loop calls:
- `allow_checks::check_crate_level_allow`
- `allow_checks::check_item_level_allow`
- `allow_checks::check_garde_skip`
- `allow_checks::check_cfg_attr_allow`
- `structure_checks::check_file_length`
- `structure_checks::check_use_count`
- `code_quality_checks::check_direct_fs_usage`

No call to `structure_checks::check_unsafe`.

**However:** This is likely **intentional dead code**, not a bug. Per the CLAUDE.md philosophy section: "guardrail3 does NOT scan source for what clippy already catches — that would be redundant." The `unsafe_code = "forbid"` lint in `[workspace.lints.rust]` (verified by R26/R53) already catches `unsafe` blocks at compile time. The `check_unsafe` function is redundant with clippy enforcement. It may have been written as an early implementation before the "enforce configuration, not violations" philosophy was finalized, or kept as a utility for edge cases.

**Should it be fixed?** The function should either be (a) removed as dead code, or (b) wired into the source scan if there's a reason clippy's `unsafe_code = forbid` is insufficient (there isn't one — `forbid` cannot be overridden with `#[allow]`). Recommend removal.

---

## CRIT-R53: `check_unsafe_code_forbid` dead code

**Verdict: REAL. The function is defined but never called from production code.**

`check_unsafe_code_forbid` is defined at `structure_checks.rs:96-149`. The codebase-wide grep confirms it appears ONLY at its definition site.

It is NOT called from `source_scan.rs`, `mod.rs`, or any other orchestrator.

**Same analysis as CRIT-R42:** This function checks that `unsafe_code = "forbid"` is set in `[workspace.lints.rust]`. However, `cargo_lints.rs` already checks `EXPECTED_RUST_LINTS` which includes `unsafe_code = "forbid"` (line 19-22). So R53 is redundant with R26's check of `unsafe_code` in the expected lints list.

The difference is that R53 specifically checks for `forbid` vs `deny` and provides a more targeted error message about the `#[allow]` bypass risk. This is a higher-signal check than what R26 provides (R26 just checks the level matches, R53 explains *why* `forbid` matters over `deny`).

**Should it be fixed?** Yes — this function provides value that R26 doesn't (the `deny` → `forbid` upgrade warning). It should be wired into the Rust validation orchestrator (`mod.rs`). It's a real gap: if a project sets `unsafe_code = "deny"` instead of `"forbid"`, R26 would flag it as wrong level, but R53's more specific message about `#[allow]` bypass is more actionable.

---

## F01: ESLint override blindness (later config blocks can disable rules)

**Verdict: REAL limitation, but LOW exploitability in practice.**

The ESLint checks use two patterns:
1. `content.contains(rule_name)` — for presence checks (e.g., `check_eslint_rule_presence` at `eslint_rule_infra.rs:217`)
2. `check_rule_value()` — for value checks, which scans lines for the rule name and extracts the first number within 5 lines

Both operate on the entire file content as a flat string. Neither understands ESLint's flat config block structure.

**Can a later block override a rule?** Yes, in ESLint flat config:
```js
export default [
  { rules: { "max-lines": ["error", { max: 400 }] } },
  { rules: { "max-lines": "off" } },  // silently disables it
];
```

guardrail3 would see `"max-lines"` present AND the value `400`, and report T2 as passing. The later `"off"` override would not be detected.

**However:** The T7 check (`check_relaxed_rules`) explicitly scans every line for `"off"` and `"warn"` and reports them as Info inventory items. So while the T2 check would pass incorrectly, T7 would flag the `"off"` line separately. The override is not invisible — it just doesn't block T2 from passing.

**Practical exploitability:** Low. This requires someone to intentionally add a second config block that disables a rule. The T7 inventory check would surface it. And the CLAUDE.md already acknowledges this as a known limitation: "ESLint rules checked by pattern matching... cannot detect if a rule's configuration was changed."

**Should it be fixed?** Long-term, yes — proper AST parsing of the ESLint config would eliminate this class of false negatives. Short-term, the T7 inventory provides a safety net. Not urgent.

---

## F04: max-lines expected value mismatch (400 vs 300)

**Verdict: AUDITOR IS WRONG. 400 is intentional.**

In `eslint_check.rs:57-65`:
```rust
check_eslint_rule(
    content, eslint_path, "T2", "max-lines", Some("400"),
    Severity::Error, results,
);
```

The expected value is `400`. The canonical ESLint starter template in `canonical.rs:283` also specifies `max: 400`.

The `check_rule_value` function in `eslint_rule_infra.rs:250-281` uses a "stricter-or-equal passes" comparison: if the actual value is `<= expected`, it passes. So a project with `max-lines: 300` would PASS (300 <= 400, meaning stricter). A project with `max-lines: 500` would FAIL.

The auditor claims the value should be `300`. But `400` is the guardrail3 **baseline** — the maximum acceptable value. Projects are free to set stricter values (300, 200, etc.) and they will pass. The baseline is not "the recommended value" — it's "the loosest acceptable value."

This is working as designed. The value `400` matches both the code and the canonical template. If a third-party spec recommends `300`, that project can set `300` in their ESLint config and guardrail3 will pass it (300 <= 400). guardrail3 just won't *require* `300`.

**Should it be fixed?** No. The auditor misunderstands the semantics. `400` is the ceiling, not the target.

---

## Summary Table

| Finding | Real? | Should Fix? | Priority |
|---------|-------|-------------|----------|
| CRIT-01: missing_docs/debug_impls not in EXPECTED_RUST_LINTS | YES | YES | Medium — canonical module recommends them but validation doesn't enforce |
| CRIT-02: _profile param unused in clippy_coverage | YES (by design) | LOW | Low — intentional, documented in code comment |
| CRIT-03: path.exists() bypassing FileSystem | YES | DEFER | Low — pervasive pattern (21 sites), works correctly, large refactor |
| CRIT-R42: check_unsafe dead code | YES | YES (remove) | Low — redundant with clippy unsafe_code=forbid |
| CRIT-R53: check_unsafe_code_forbid dead code | YES | YES (wire in) | Medium — provides unique value over R26 |
| F01: ESLint override blindness | YES (known) | DEFER | Low — T7 inventory provides safety net, acknowledged limitation |
| F04: max-lines 400 vs 300 | NO | NO | N/A — auditor misunderstands baseline semantics |
