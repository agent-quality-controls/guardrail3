# cargo-deny

## What it does
Checks dependencies: bans, licenses, advisories, sources.

## Config file
`deny.toml` or `.deny.toml` or `.cargo/deny.toml` (checked in that order at each directory level)

## Config discovery (verified from cargo-deny source: common.rs, common/cfg.rs)
1. If `--config <path>` passed (subcommand flag, NOT top-level): use that file, NO walk-up. Resolves relative to CWD.
2. Else: start from manifest directory (where Cargo.toml is), walk UP parent directories
3. At each level, check: `deny.toml`, `.deny.toml`, `.cargo/deny.toml`
4. First found wins. No merging. Walk stops.
5. If nothing found: warn "unable to find a config path", use default config

**IMPORTANT CORRECTION:** Earlier claimed "CWD only, no walk-up." WRONG. Verified from source: cargo-deny DOES walk up from manifest directory. Behaves like clippy.

Without `--manifest-path`: uses CWD's Cargo.toml. Errors if no Cargo.toml in CWD (does NOT search parent dirs for Cargo.toml — different from `cargo` itself).

## Shadowing
YES. A deny.toml next to a member crate's Cargo.toml shadows the workspace root deny.toml. First found in walk-up wins. This means per-crate deny.toml can silently override all workspace bans — same risk as clippy.

## Exceptions file
`deny.exceptions.toml` / `.deny.exceptions.toml` / `.cargo/deny.exceptions.toml` — same walk-up. Only supports license exceptions. Additive (appended to main config's list).

## Virtual workspaces
All members automatically included as graph roots. Equivalent to implicit `--workspace`.

## How to invoke
```bash
cd <workspace-root> && cargo deny check
```
Per workspace.

## Guardrail3's role
- **Generate/merge:** Ensure baseline bans present, preserve advisories/wrappers
- **Validate:** Check bans complete, sections present
- **Hook:** Run per discovered workspace
- **Coverage map:** Walk-up simulation per crate (SAME as clippy — not scope-level only). Need to fix current implementation which doesn't do per-crate walk-up.
