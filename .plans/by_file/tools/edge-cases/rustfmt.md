# rustfmt.toml Config Resolution Edge Cases

**Date:** 2026-03-18
**Research method:** Web search + rustfmt source code analysis (rust-lang/rustfmt master)

---

## 1. Intermediate directory without Cargo.toml

**Question:** If `rustfmt.toml` is at `crates/` (intermediate dir, no Cargo.toml there) -- does the walk-up find it?

**Answer: Yes.** Rustfmt's config resolution has **nothing to do with Cargo.toml**. The `resolve_project_file` function walks up from the input file's directory using `current.pop()` in a loop, checking each directory for `.rustfmt.toml` or `rustfmt.toml`. It does not look for or care about `Cargo.toml` at any point.

The walk-up is purely filesystem-based: start at the file's directory, check for config, go to parent, repeat until filesystem root.

**Source code** (`src/config/mod.rs`):
```rust
fn resolve_project_file(dir: &Path) -> Result<Option<PathBuf>, Error> {
    let mut current = /* canonicalized dir */;
    loop {
        match get_toml_path(&current) {
            Ok(Some(path)) => return Ok(Some(path)),
            Err(e) => return Err(e),
            _ => (),
        }
        if !current.pop() { break; }
    }
    // then home_dir, then config_dir fallbacks
}
```

So a `crates/rustfmt.toml` will be found by any `.rs` file under `crates/` or its subdirectories.

---

## 2. `cargo fmt` vs `rustfmt` directly

**Answer: They differ in two important ways.**

### a) Config search starting directory

- **`rustfmt <file>`**: Searches for config starting from the **file's directory**, walking up.
- **`cargo fmt`**: Searches for config starting from the **workspace/package root** (where `Cargo.toml` lives). It does NOT search from each individual file's directory.

This means:
- A `rustfmt.toml` placed in a subdirectory like `src/foo/` will be found by `rustfmt src/foo/mod.rs` but **ignored by `cargo fmt`**.
- This is a confirmed bug/behavior documented in [rust-lang/rustfmt#5814](https://github.com/rust-lang/rustfmt/issues/5814).

### b) Edition inference

- **`cargo fmt`**: Automatically reads the `edition` from `Cargo.toml` and passes it to rustfmt.
- **`rustfmt` directly**: Defaults to edition 2015 unless `edition` is set in `rustfmt.toml` or via `--edition` flag.

**Recommendation:** Always set `edition` explicitly in `rustfmt.toml` to avoid discrepancies.

---

## 3. Crate-level rustfmt.toml shadowing workspace root

**Question:** If a crate directory has its own `rustfmt.toml`, does it shadow the workspace root config completely?

**Answer: Yes, completely.** The walk-up stops at the **first** config file found. There is no merging, no inheritance, no layering. If `crates/my-crate/rustfmt.toml` exists, the workspace-root `rustfmt.toml` is never read for files in that crate.

This is a common gotcha: if you want per-crate overrides while keeping workspace defaults, you must **duplicate all desired settings** in the per-crate config file.

**Caveat with `cargo fmt`:** Due to the behavior in point 2, `cargo fmt` may not even find the per-crate config. It finds config based on the workspace root, not per-file. So a per-crate `rustfmt.toml` may only work when invoking `rustfmt` directly on that crate's files.

---

## 4. `.rustfmt.toml` vs `rustfmt.toml`

**Answer: Both work identically, but `.rustfmt.toml` has higher priority.**

The source code defines:
```rust
const CONFIG_FILE_NAMES: [&str; 2] = [".rustfmt.toml", "rustfmt.toml"];
```

The search iterates this array in order. If **both** files exist in the same directory, `.rustfmt.toml` wins because it is checked first. If only one exists, that one is used.

There is no functional difference in how the files are parsed -- same format, same options.

---

## 5. Unknown keys in rustfmt.toml

**Answer: Warning, not error. Formatting continues.**

When rustfmt encounters an unknown key in the TOML config, it emits a warning to stderr but continues processing:

```rust
for key in table.keys() {
    if !Config::is_valid_name(key) {
        let msg = &format!("Warning: Unknown configuration option `{key}`\n");
        err.push_str(msg)
    }
}
```

The unknown key is silently ignored (default value used), and formatting proceeds normally. **Exit code is still 0** (assuming no other errors).

However, the `override_value()` method (used for `--config` CLI overrides) will **panic** on unknown keys:
```rust
_ => panic!("Unknown config key in override: {}", key)
```

So: unknown keys in the file = warning; unknown keys via `--config` flag = panic/crash.

---

## 6. Nightly-only options on stable rustfmt

**Answer: Warning emitted, option silently reverted to default. Formatting continues.**

When stable rustfmt encounters an unstable option in `rustfmt.toml`:
```
Warning: can't set `imports_granularity = Crate`, unstable features are only available in nightly channel.
```

The unstable option is **ignored** (default value used), and formatting proceeds. The exit code remains 0.

There are two levels of instability:
- **Unstable option**: The entire config key is nightly-only (e.g., `imports_granularity`).
- **Unstable variant**: The key is stable, but a specific value is nightly-only.

Both produce warnings and revert to defaults on stable.

**Known bug:** Passing unstable options via `--config` CLI flag bypasses this check entirely, allowing unstable features to work on stable. This is tracked in [rust-lang/rustfmt#5511](https://github.com/rust-lang/rustfmt/issues/5511) and [#6534](https://github.com/rust-lang/rustfmt/issues/6534).

---

## 7. Home directory fallback

**Answer: Yes, `$HOME/rustfmt.toml` (or `$HOME/.rustfmt.toml`) applies as fallback.**

The full search order is:

1. **File's directory** (or workspace root for `cargo fmt`)
2. **Each parent directory**, walking up to filesystem root
3. **Home directory** (`$HOME/` or `~`)
4. **Global config directory**:
   - Linux: `$XDG_CONFIG_HOME/rustfmt/` (default `~/.config/rustfmt/`)
   - macOS: `~/Library/Preferences/rustfmt/`
   - Windows: `%AppData%\rustfmt\`

If no config is found anywhere, built-in defaults are used.

A `$HOME/.rustfmt.toml` will apply to **every Rust project** that doesn't have its own config file anywhere in its directory tree. This can cause surprising behavior -- a personal preference file affecting CI builds, for example.

---

## Summary Table

| Scenario | Behavior |
|---|---|
| Config at intermediate dir (no Cargo.toml) | Found by walk-up (rustfmt doesn't care about Cargo.toml) |
| `cargo fmt` vs `rustfmt` config search | Different starting points; `cargo fmt` starts at workspace root |
| Per-crate config shadowing | Complete shadow, no merging with parent configs |
| `.rustfmt.toml` vs `rustfmt.toml` | Identical; `.rustfmt.toml` wins if both exist in same dir |
| Unknown key in config file | Warning to stderr, key ignored, formatting continues |
| Unknown key via `--config` flag | Panic (crash) |
| Nightly option on stable (in file) | Warning, option ignored, formatting continues |
| Nightly option on stable (via `--config`) | Silently applied (bug) |
| `$HOME/rustfmt.toml` | Applied as fallback if no project config found |

---

## Sources

- [rustfmt Configurations.md (main)](https://github.com/rust-lang/rustfmt/blob/main/Configurations.md)
- [rustfmt source: src/config/mod.rs](https://github.com/rust-lang/rustfmt/blob/master/src/config/mod.rs)
- [rustfmt source: src/config/config_type.rs](https://github.com/rust-lang/rustfmt/blob/master/src/config/config_type.rs)
- [Issue #5814: cargo fmt ignores rustfmt.toml in subdirectory](https://github.com/rust-lang/rustfmt/issues/5814)
- [Issue #5511: Unstable features usable on stable via --config](https://github.com/rust-lang/rustfmt/issues/5511)
- [Issue #6534: Unstable features via --config on stable](https://github.com/rust-lang/rustfmt/issues/6534)
- [Issue #5498: Unknown configuration option fn_params_layout](https://github.com/rust-lang/rustfmt/issues/5498)
- [PR #3280: Look for a global rustfmt.toml](https://github.com/rust-lang/rustfmt/pull/3280)
- [Issue #2710: Fixed location for global rustfmt.toml](https://github.com/rust-lang/rustfmt/issues/2710)
- [Rustfmt official docs](https://rust-lang.github.io/rustfmt/)
