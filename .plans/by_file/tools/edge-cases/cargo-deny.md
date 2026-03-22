# cargo-deny deny.toml Config Resolution Edge Cases

**Date:** 2026-03-18
**Source:** cargo-deny source code (EmbarkStudios/cargo-deny, main branch as of 2026-03-18)
**Key file:** `src/cargo-deny/common.rs` (all resolution logic lives here)

---

## 1. Running `cargo deny check` without `--manifest-path` from a directory with no Cargo.toml

**Answer: It errors. It does NOT search parent directories for Cargo.toml.**

From `main.rs` lines 277-305:

```rust
let manifest_path = if let Some(mpath) = args.ctx.manifest_path {
    mpath
} else {
    let cwd = std::env::current_dir()...;
    let man_path = cwd.join("Cargo.toml");
    anyhow::ensure!(
        man_path.exists(),
        "the directory {} doesn't contain a Cargo.toml file",
        cwd.display()
    );
    man_path.try_into().context("non-utf8 path")?
};
```

cargo-deny does NOT use `cargo locate-project` or walk parent directories. It simply joins `cwd + "Cargo.toml"` and errors if it doesn't exist. This is different from `cargo` itself, which searches parent directories.

**Implication for guardrail3r:** When running `cargo deny check` as a tool, we must either:
- Run it from the directory containing Cargo.toml, OR
- Pass `--manifest-path` explicitly

---

## 2. Can deny.toml be at `.cargo/deny.toml`?

**Answer: YES. It's the third location checked in the parent-walk.**

From `common.rs` `default_config_path()` (lines 60-87), when no `--config` is passed, cargo-deny walks UP from the manifest directory checking three filenames at each level:

1. `<dir>/deny.toml`
2. `<dir>/.deny.toml`
3. `<dir>/.cargo/deny.toml`

It starts at the manifest's parent directory and walks up to the filesystem root. First match wins.

**Full search order example** for manifest at `/workspace/crates/foo/Cargo.toml`:
1. `/workspace/crates/foo/deny.toml`
2. `/workspace/crates/foo/.deny.toml`
3. `/workspace/crates/foo/.cargo/deny.toml`
4. `/workspace/crates/deny.toml`
5. `/workspace/crates/.deny.toml`
6. `/workspace/crates/.cargo/deny.toml`
7. `/workspace/deny.toml`
8. `/workspace/.deny.toml`
9. `/workspace/.cargo/deny.toml`
10. ... continues to filesystem root

**Key insight:** The deny.toml parent-walk is based on `manifest_path`, NOT on `cwd`. So if you pass `--manifest-path /other/place/Cargo.toml`, it searches from `/other/place/` upward.

---

## 3. `--config` flag: check subcommand or top-level?

**Answer: It's on the `check` subcommand (and also on `fetch`, `list`, `init`).**

From `check.rs`:
```rust
/// Path to the config to use
/// Defaults to <cwd>/deny.toml if not specified
#[arg(short, long)]
pub config: Option<PathBuf>,
```

The flag is `-c` / `--config` on each subcommand, NOT on the top-level `cargo deny` command. The top-level only has `--manifest-path`, `--log-level`, `--format`, `--color`, and workspace-related flags.

**Important behavior difference when `--config` IS provided:**
- The path is resolved relative to `cwd` (NOT relative to manifest_path)
- There is NO parent-directory walk. It's a direct path resolution: absolute paths used as-is, relative paths joined with cwd.
- If the resolved path doesn't exist, cargo-deny logs a warning and falls back to **default config** (empty config), NOT an error.

From `common/cfg.rs` lines 33-37:
```rust
Some(cfg_path) => {
    log::warn!("config path '{cfg_path}' doesn't exist, falling back to default config");
    (String::new(), cfg_path)
}
```

**Implication:** `cargo deny check --config deny.toml` and `cargo deny check` (no flag) have DIFFERENT resolution semantics. Without `--config`, it walks parents from manifest dir. With `--config deny.toml`, it resolves from cwd only.

---

## 4. Virtual workspaces (`[workspace]` with no `[package]`)

**Answer: Virtual manifests automatically enable `--workspace` behavior, checking all members.**

From `main.rs` lines 57-61 (the `--workspace` flag docs):
```
/// If passed, all workspace packages are used as roots for the crate graph.
///
/// Automatically assumed if the manifest path points to a virtual manifest.
///
/// Normally, if you specify a manifest path that is a member of a workspace,
/// that crate will be the sole root of the crate graph...
```

Behavior summary:
- **Virtual manifest** (no `[package]`): All workspace members are graph roots automatically
- **Member manifest**: Only that crate is the graph root (plus its workspace deps)
- **Member manifest + `--workspace`**: All workspace members become graph roots
- **`--exclude`**: Can exclude specific members from the graph
- **`--exclude-unpublished`**: Excludes members with `publish = false`

The graph builder (`gather_krates`) passes `self.workspace` to `krates::Builder::workspace()`, and the krates library handles the virtual manifest detection internally.

---

## 5. Workspace member having its own deny.toml

**Answer: NO per-member config support. But the parent-walk means a member's local deny.toml IS found first.**

cargo-deny has a single config for the entire run. There is no per-crate or per-member config override system.

However, because `default_config_path()` walks UP from the manifest directory:
- If you run `cargo deny check --manifest-path crates/foo/Cargo.toml` and `crates/foo/deny.toml` exists, it will be found FIRST (before the workspace root's deny.toml).
- If `crates/foo/deny.toml` does NOT exist, it continues walking up and finds the workspace root's `deny.toml`.

**This is a gotcha:** If a workspace member accidentally has a `deny.toml` (e.g., copied from somewhere), it will shadow the workspace root config when that member is used as the manifest path. But in normal usage (running from workspace root with no `--manifest-path`), the workspace root's `deny.toml` is found and applies to all members.

There is NO merging of member-level and workspace-level deny.toml files. It's first-match-wins.

---

## 6. `deny.exceptions.toml` merge behavior

**Answer: It ONLY supports `[exceptions]` (license exceptions). It APPENDS to the main config's exceptions list.**

### Location resolution

Same parent-walk pattern as deny.toml, starting from manifest_path parent, checking three names per directory:
1. `<dir>/deny.exceptions.toml`
2. `<dir>/.deny.exceptions.toml`
3. `<dir>/.cargo/deny.exceptions.toml`

First match wins. Only ONE exceptions file is loaded (no stacking).

### What it supports

ONLY the `exceptions` field (license exceptions). From `licenses/cfg.rs` `load_exceptions()`:

```rust
let mut th = TableHelper::new(&mut parsed)?;
let exceptions = th.required("exceptions")?;
th.finalize(None)?;  // errors on any OTHER fields
```

`th.finalize(None)` will produce diagnostics for any unrecognized keys. So putting `[bans]`, `[advisories]`, `[sources]`, or even `[licenses]` in the exceptions file will generate warnings/errors.

### Merge behavior

The exceptions are **appended** to the main config's exceptions list:

```rust
cfg.exceptions.reserve(exceptions.len());
for exc in exceptions {
    cfg.exceptions.push(ValidException { ... });
}
```

There is no deduplication, no override, no conflict resolution. Exceptions from deny.exceptions.toml are simply added to whatever exceptions exist in deny.toml.

### Format

```toml
exceptions = [
    { allow = ["CDDL-1.0"], crate = "inferno" },
    { allow = ["BSD-2-Clause"], crate = "cloudabi" },
]
```

### Use case

Corporate environments where a shared/global deny.toml is maintained centrally (e.g., in a shared CI config or template), but individual projects need project-specific license exceptions without forking the global config.

---

## Summary Table

| Question | Answer |
|---|---|
| No Cargo.toml in cwd? | Error (no parent search for Cargo.toml) |
| `.cargo/deny.toml`? | Yes, third location checked per directory in parent-walk |
| `--config` location? | Per-subcommand flag (`check`, `fetch`, `list`, `init`) |
| `--config` resolution? | Relative to cwd, NO parent-walk |
| No `--config` resolution? | Parent-walk from manifest_path dir upward |
| Virtual workspace? | All members checked automatically |
| Member's own deny.toml? | Found first by parent-walk, shadows workspace config (no merge) |
| deny.exceptions.toml? | License exceptions only, appended to main config, same parent-walk |

---

## Sources

- [cargo-deny source: common.rs](https://github.com/EmbarkStudios/cargo-deny/blob/main/src/cargo-deny/common.rs) - Config and exceptions path resolution
- [cargo-deny source: main.rs](https://github.com/EmbarkStudios/cargo-deny/blob/main/src/cargo-deny/main.rs) - Manifest path resolution, CLI flags
- [cargo-deny source: check.rs](https://github.com/EmbarkStudios/cargo-deny/blob/main/src/cargo-deny/check.rs) - Check subcommand with --config flag
- [cargo-deny source: common/cfg.rs](https://github.com/EmbarkStudios/cargo-deny/blob/main/src/cargo-deny/common/cfg.rs) - ValidConfig::load and exceptions merge
- [cargo-deny source: licenses/cfg.rs](https://github.com/EmbarkStudios/cargo-deny/blob/main/src/cargo-deny/licenses/cfg.rs) - load_exceptions() implementation
- [cargo-deny CLI docs: check](https://embarkstudios.github.io/cargo-deny/cli/check.html) - CLI flag documentation
- [cargo-deny CLI docs: common](https://embarkstudios.github.io/cargo-deny/cli/common.html) - Common options
- [cargo-deny license config docs](https://embarkstudios.github.io/cargo-deny/checks/licenses/cfg.html) - Exceptions file documentation
- [GitHub issue #541](https://github.com/EmbarkStudios/cargo-deny/issues/541) - Original feature request for exceptions file
