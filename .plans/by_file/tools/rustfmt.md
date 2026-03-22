# rustfmt

## What it does
Formats Rust code. Enforces consistent style.

## Config file
`rustfmt.toml` or `.rustfmt.toml` (dot variant checked first, wins if both exist)

## Config discovery (verified from rustfmt docs + research)

**CRITICAL: `cargo fmt` and `rustfmt` have DIFFERENT resolution.**

**`rustfmt` directly:** Walk-up from each SOURCE FILE being formatted. Checks parent dirs for rustfmt.toml. Nearest wins, completely shadows. Falls back to `$HOME/.rustfmt.toml`, then `$XDG_CONFIG_HOME/rustfmt/`.

**`cargo fmt`:** Starts from WORKSPACE ROOT (not per-file). Resolves config from entry points only — ignores subdirectory configs. Also: reads `edition` from Cargo.toml, while `rustfmt` defaults to edition 2015.

In practice with `cargo fmt --all` (which is what hooks use), the workspace root config applies uniformly. Per-subdirectory configs are NOT picked up by `cargo fmt`.

## Shadowing
YES for `rustfmt` direct invocation. NO for `cargo fmt` (starts at workspace root, doesn't see subdirectory configs).

**But:** IDE format-on-save often calls `rustfmt` directly, which DOES walk up and finds subdirectory configs. So a developer's IDE could format differently than CI's `cargo fmt`.

## Error handling
Unknown keys: warning to stderr, key ignored. Nightly-only options on stable: warning, reverted to default. BUT: unknown keys via `--config` CLI flag cause a panic (known bug).

## Home directory fallback
YES. `$HOME/.rustfmt.toml` applies if no project config found. A developer's personal config could affect formatting. Guardrail3 should ensure project always has rustfmt.toml so the home fallback never activates.

## How to invoke
```bash
cd <workspace-root> && cargo fmt --all -- --check
```
Per workspace. `--check` for CI/hooks.

## Guardrail3's role
- **Generate:** Write rustfmt.toml at workspace root (fully owned — no user content)
- **Validate:** Check existence, verify edition/max_width
- **Hook:** Run `cargo fmt --all -- --check` per discovered workspace
- **Coverage map:** Walk-up simulation per crate (consistent with clippy), even though `cargo fmt` doesn't walk up — because `rustfmt` in IDE does
