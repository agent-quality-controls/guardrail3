# Implement hex arch enforcement checks (R-ARCH-01/02/03)

**Date:** 2026-03-16 16:05
**Task:** Build three hex arch enforcement checks per the plan at .plans/2026-03-16-160337-hex-arch-enforcement.md

## Goal
New file `src/app/rs/validate/hex_arch_checks.rs` with three checks, wired into the orchestrator, with unit tests.

## Approach

### StubFs design
Need a multi-file StubFs (BTreeMap<PathBuf, String>) that returns different content per path. For R-ARCH-01 directory existence checks, use `fs.read_file()` on a sentinel path or `fs.metadata()`. Since metadata returns `std::fs::Metadata` (hard to fake), I'll use `fs.read_file()` as a proxy for existence — if `read_file(dir_path)` returns Some, the directory "exists". This is consistent with how other checks work (they check Cargo.toml existence via read_file).

Actually, looking more carefully: R-ARCH-01 needs to check if subdirectories exist. The FileSystem trait has `metadata()` which returns `Option<Metadata>` — None means not found. But `Metadata` is a std type that's hard to construct in tests. Better approach: check if `crates/domain/Cargo.toml` exists (via read_file), which proves both the directory and crate exist.

### Layer detection
For R-ARCH-02, determine layer by:
1. Check guardrail3.toml `crate_configs[name].layer`
2. Infer from path segments: "/domain" → domain, "/ports" → ports, "/app/" → app, "/adapters" → adapters

### Files
- NEW: src/app/rs/validate/hex_arch_checks.rs
- MODIFY: src/app/rs/validate/mod.rs (add module, wire into architecture section)
- MODIFY: src/help_gen.rs (add R-ARCH-01/02/03 to RS_VALIDATE_HELP)
