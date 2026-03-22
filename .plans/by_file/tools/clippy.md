# clippy

## What it does
Rust linter. Checks code quality, enforces method/type bans via clippy.toml.

## Config file
`clippy.toml` or `.clippy.toml` (dot variant checked first, wins if both exist with a warning)

## Config discovery (verified from clippy source: clippy_config/src/conf.rs)
1. Check `CLIPPY_CONF_DIR` env var → start walk-up from there
2. Else check `CARGO_MANIFEST_DIR` → start walk-up from there
3. Else use CWD canonicalized
4. Walk UP parent directories, checking each for `clippy.toml` or `.clippy.toml`
5. First found wins. No merging. Walk stops.
6. If nothing found: check `$HOME`, `$XDG_CONFIG_HOME/clippy/`
7. If still nothing: use `Conf::default()` (no bans, default thresholds)

When `cargo clippy --workspace` runs, `CARGO_MANIFEST_DIR` is set PER CRATE being compiled. Each crate independently resolves its config.

## Shadowing
YES. An intermediate directory config (e.g., `crates/adapters/clippy.toml` between workspace root and `crates/adapters/outbound/`) IS found and completely shadows the workspace root. All workspace bans lost for that crate.

## Error handling
Syntax errors: non-fatal. Clippy emits diagnostics, continues with defaults. Unknown keys: warning with edit-distance suggestion, key ignored.

## How to invoke
```bash
cd <workspace-root> && cargo clippy --workspace --all-targets --all-features -- -D warnings
```
Per workspace. Each workspace is independent.

## Guardrail3's role
- **Generate/merge:** Ensure baseline bans present per workspace root
- **Validate:** Check bans complete, thresholds not looser, ERROR on rogue per-crate clippy.toml
- **Hook:** Run per discovered workspace
- **Coverage map:** Walk-up simulation per crate, detect shadows
