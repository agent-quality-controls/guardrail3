# Enforce regex bans on target projects

**Date:** 2026-03-19 19:27
**Scope:** deny_audit.rs, deny.rs, mod.rs

## Summary
Added 7 regex crates to EXPECTED_BANS and created DENY_BANS_REGEX module. Target projects are now required to ban regex/fancy-regex/onig/pcre2/grep-cli/grep-regex/grep-matcher in their deny.toml. The `generate` command produces these bans for new projects.

## Verification
Ran against steady-parent: coverage map shows required_total=33, present=26, missing=7 — correctly flags the 7 missing regex bans.
