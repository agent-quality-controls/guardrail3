# RS-TOOLCHAIN — rust-toolchain.toml checker (4 rules)

**Input:** rust-toolchain.toml / rust-toolchain (one per workspace)
**Parser:** TOML
**Current code:** `config_files.rs` (existence), `toolchain_check.rs` (settings)

## Rules

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-TOOLCHAIN-01 | R24 | Error | rust-toolchain.toml exists at workspace root | Implemented |
| RS-TOOLCHAIN-02 | R25 | Error/Warn | Channel = "stable" (error for nightly, info for pinned version), components include clippy + rustfmt | Implemented (bugs: see code_fixes.md) |

## New rules from audit

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-TOOLCHAIN-03 | Warn/Info | MSRV consistency. If `rust-version` in Cargo.toml AND toolchain pins specific version, warn if pinned < MSRV. Library without `rust-version`: Info. | Planned |
| RS-TOOLCHAIN-04 | Warn | Legacy `rust-toolchain` file (no .toml extension) cannot specify components. Warn to migrate. Also warn if both `rust-toolchain` and `rust-toolchain.toml` coexist (ambiguous). | Planned |

## Explicitly rejected

| Finding | Why rejected |
|---------|-------------|
| `profile` field (minimal/default/complete) | Explicit components check (RS-TOOLCHAIN-02) is stronger than implicit profile defaults. |
| Edition vs toolchain version compatibility | cargo catches at build time ("edition 2024 requires Rust 1.85+"). |
| Toolchain file gitignored | Failure is obvious and immediate (wrong toolchain). |
| Unknown/typo'd keys | Consequences caught by existing checks (missing channel, missing components). |
