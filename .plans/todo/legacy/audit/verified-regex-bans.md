# Verified: Regex/Grep Ban Enforcement

**Date:** 2026-03-19
**Auditor:** Claude

## 1. Rust deny.toml Bans (crate-level)

### Banned crates expected: `regex`, `fancy-regex`, `onig`, `pcre2`, `grep-cli`, `grep-regex`, `grep-matcher`

**EXPECTED_BANS** (in `deny_audit.rs` lines 10-37):
```
simd-json, json5, sonic-rs, openssl, openssl-sys, ureq, surf, isahc,
log4rs, env_logger, simple_logger, fern, async-std, smol, anyhow,
actix-web, rocket, warp, poem, chrono, diesel, sea-orm, bincode,
rmp-serde, prost, flatbuffers
```

| Crate | In EXPECTED_BANS? | In deny.rs modules? | Status |
|-------|-------------------|---------------------|--------|
| `regex` | NO | NO | **GAP** |
| `fancy-regex` | NO | NO | **GAP** |
| `onig` | NO | NO | **GAP** |
| `pcre2` | NO | NO | **GAP** |
| `grep-cli` | NO | NO | **GAP** |
| `grep-regex` | NO | NO | **GAP** |
| `grep-matcher` | NO | NO | **GAP** |

**Finding:** guardrail3's own `deny.toml` may ban these crates (self-enforcement), but `EXPECTED_BANS` does NOT include them. This means:
- guardrail3 will NOT flag a target project whose `deny.toml` is missing regex bans
- guardrail3 will NOT generate regex bans for new projects (no deny module exists for regex bans)
- A project can pass all guardrail3 checks while allowing `regex`, `fancy-regex`, `onig`, `pcre2`, and all `grep-*` crates

**Self-enforcement:** guardrail3's own `deny.toml` is generated from the same modules in `deny.rs`. None of the 10 ban category modules (`DENY_BANS_JSON`, `DENY_BANS_TLS`, etc.) include regex crates. So guardrail3 does NOT even ban them on itself through the generated deny.toml (it may have them via `.guardrail3/overrides/deny-bans.toml` but that's project-specific, not enforced on targets).

**Severity: CRITICAL** -- The entire regex ban philosophy (use structured parsers, not regex) is a documentation-only policy with zero enforcement at the Rust crate level.

### Required fix:
1. Add a new module `DENY_BANS_REGEX` in `deny.rs` banning: `regex`, `fancy-regex`, `onig`, `pcre2`, `grep-cli`, `grep-regex`, `grep-matcher`
2. Add it to `service_profile_ban_entries()` and `library_profile_ban_entries()`
3. Add all 7 crate names to `EXPECTED_BANS` in `deny_audit.rs`
4. Run `guardrail3 generate` to regenerate own deny.toml

---

## 2. TypeScript ESLint RegExp Ban (T-ESLP-15)

### Check implementation (eslint_check.rs lines 128-159):

```rust
let has_regexp_ban = content.contains("RegExp") && content.contains("no-restricted");
```

**Wired into orchestrator?** YES -- `check_regex_ban()` is called from `check_eslint_config()` at line 46.

**Does it check for both `no-restricted-globals` (RegExp constructor) AND `no-restricted-syntax` (regex literals)?**

NO. The check only verifies:
- The string `"RegExp"` appears somewhere in the ESLint config
- The string `"no-restricted"` appears somewhere in the ESLint config

It does NOT verify:
- That `no-restricted-globals` specifically bans `RegExp` (it could be banning `process` under `no-restricted-globals` and `RegExp` could appear in a comment)
- That `no-restricted-syntax` with `Literal[regex]` selector exists
- The two strings don't need to be related -- `RegExp` in a comment + `no-restricted-imports` (for axios) would pass

**Could someone pass by having "RegExp" in a comment?** YES. A comment like `// TODO: ban RegExp` plus any `no-restricted-imports` rule would pass T-ESLP-15.

**Severity: MEDIUM** -- The check runs and catches the obvious case (no RegExp mention at all), but is bypassable. Since this is config validation (not adversarial security), the risk is accidental misconfiguration slipping through, which is moderate.

### Required fix:
Use AST-based checking (tree-sitter-javascript) to verify:
1. `no-restricted-globals` rule exists AND contains a `RegExp` entry
2. `no-restricted-syntax` rule exists AND contains a `Literal[regex]` selector

At minimum, check that `RegExp` appears near `no-restricted-globals` (not just anywhere) and that `Literal[regex]` appears in the config.

---

## 3. TypeScript Banned Packages (T17 + T59)

### T17 banned deps (package_check.rs lines 133-156):

`xregexp` -- YES, present at line 154
`regexp-tree` -- YES, present at line 155

### T59 banned imports in node_modules (source_scan.rs lines 323-346):

`xregexp` -- YES, present at line 344
`regexp-tree` -- YES, present at line 345

**Status: PASS** -- Both regex-related npm packages are banned in both the package.json dependency check (T17) and the node_modules presence check (T59).

---

## 4. Canonical/Generated Modules

### Generated deny.toml (deny.rs):

No `DENY_BANS_REGEX` module exists. The generated deny.toml for new projects will NOT include regex crate bans. See Section 1.

**Status: GAP** -- Same gap as Section 1.

### Generated ESLint config (canonical.rs ESLINT_STARTER, lines 251-353):

The starter ESLint config DOES include:
- `no-restricted-globals` with `RegExp` ban (lines 327-330)
- `no-restricted-syntax` with `Literal[regex]` selector (lines 333-339)
- `no-restricted-imports` with `xregexp` and `regexp-tree` (lines 313-314)

**Status: PASS** -- New projects generated by guardrail3 will have the ESLint regex ban. However, the T-ESLP-15 validation check (Section 2) that verifies this config is weak.

---

## Summary Matrix

| Ban | Self-enforced? | Checked on targets? | Generated for new projects? | Status |
|-----|---------------|--------------------|-----------------------------|--------|
| **Rust: regex crate** | NO (not in modules) | NO (not in EXPECTED_BANS) | NO (no module) | **GAP** |
| **Rust: fancy-regex** | NO | NO | NO | **GAP** |
| **Rust: onig** | NO | NO | NO | **GAP** |
| **Rust: pcre2** | NO | NO | NO | **GAP** |
| **Rust: grep-cli** | NO | NO | NO | **GAP** |
| **Rust: grep-regex** | NO | NO | NO | **GAP** |
| **Rust: grep-matcher** | NO | NO | NO | **GAP** |
| **TS: RegExp constructor** | YES (eslint) | WEAK (T-ESLP-15 bypassable) | YES (ESLINT_STARTER) | **PARTIAL** |
| **TS: regex literals** | YES (eslint) | WEAK (T-ESLP-15 bypassable) | YES (ESLINT_STARTER) | **PARTIAL** |
| **TS: xregexp pkg** | YES (T17+T59) | YES (T17+T59) | YES (ESLINT_STARTER imports) | **PASS** |
| **TS: regexp-tree pkg** | YES (T17+T59) | YES (T17+T59) | YES (ESLINT_STARTER imports) | **PASS** |

## Required Actions

### CRITICAL: Add Rust regex crate bans
1. Create `DENY_BANS_REGEX` module in `src/domain/modules/deny.rs`
2. Add to `service_profile_ban_entries()` and `library_profile_ban_entries()`
3. Add all 7 crates to `EXPECTED_BANS` in `src/app/rs/validate/deny_audit.rs`
4. Regenerate own deny.toml

### MEDIUM: Strengthen T-ESLP-15
1. Check that `no-restricted-globals` contains `RegExp` (not just that both strings appear anywhere)
2. Check that `no-restricted-syntax` contains `Literal[regex]` selector
3. Ideally use tree-sitter-javascript AST parsing for this check
