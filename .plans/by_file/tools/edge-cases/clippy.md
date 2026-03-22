# clippy.toml Configuration Resolution: Edge Cases

**Date:** 2026-03-18
**Sources:**
- [Clippy source: conf.rs](https://github.com/rust-lang/rust-clippy/blob/master/clippy_config/src/conf.rs)
- [Clippy docs: Configuration](https://doc.rust-lang.org/clippy/configuration.html)
- [Issue #7353: project-wide and package-specific clippy.toml](https://github.com/rust-lang/rust-clippy/issues/7353)
- [PR #10592: Fix parent directory bug in lookup_conf_file](https://github.com/rust-lang/rust-clippy/pull/10592)
- [DeepWiki: Configuration Options](https://deepwiki.com/rust-lang/rust-clippy/7.1-configuration-options)
- [Rendered conf.rs source](https://doc.rust-lang.org/stable/nightly-rustc/src/clippy_config/conf.rs.html)

---

## The Algorithm: `lookup_conf_file()`

The canonical source is in `clippy_config/src/conf.rs`. The function signature:

```rust
pub fn lookup_conf_file() -> io::Result<(Option<PathBuf>, Vec<String>)>
```

### Step-by-step behavior:

1. **Determine start directory** (in priority order):
   - `CLIPPY_CONF_DIR` env var (if set)
   - `CARGO_MANIFEST_DIR` env var (if set, and CLIPPY_CONF_DIR is not)
   - `"."` (current working directory, if neither env var is set)

2. **Canonicalize** the start directory (resolves symlinks, relative paths). If canonicalization fails, returns `Err`.

3. **Walk up ancestors** in a loop:
   - In each directory, check for `.clippy.toml` then `clippy.toml` (in that order).
   - For each candidate, call `.canonicalize()` then `fs::metadata()`.
   - Skip `NotFound` errors silently. Return hard on other IO errors. Skip directories (a dir named `clippy.toml` is ignored).
   - If a valid file is found and no prior file was found, store it as `found_config`.
   - If a valid file is found and a prior file was ALREADY found (both `.clippy.toml` and `clippy.toml` in same dir), emit a warning and keep the first one.
   - **After checking both filenames in a directory:** if `found_config` is `Some`, return immediately. Do NOT continue walking up.
   - If `found_config` is still `None`, call `current.pop()` to move to the parent directory. If `pop()` returns false (at filesystem root), return `(None, warnings)`.

**Key insight: the walk stops at the FIRST directory that contains ANY config file. It does NOT merge configs from multiple directories. It does NOT skip intermediate directories.**

---

## Answers to Specific Questions

### 1. Intermediate directory clippy.toml (e.g., `crates/adapters/clippy.toml` between workspace root and `crates/adapters/outbound/`)

**YES, it gets found.** The walk starts from the crate directory (`crates/adapters/outbound/`) and walks upward. It checks `crates/adapters/outbound/` first, then `crates/adapters/`, then `crates/`, then workspace root, etc. The first directory containing a `clippy.toml` or `.clippy.toml` wins.

So if there's a `crates/adapters/clippy.toml`, it will be found when linting `crates/adapters/outbound/` -- and the workspace root's `clippy.toml` will be **completely ignored** (not merged, not even read).

**This is the most dangerous edge case for workspaces.** An intermediate `clippy.toml` shadows the workspace-root config for all crates below it.

### 2. TWO clippy.toml files in the walk-up path

**Only the first one found (closest to the crate) is used. The second is never even discovered.**

The algorithm returns as soon as it finds a config file in ANY directory. It does not continue walking. There is NO merging, NO inheritance, NO override mechanism.

The only case where two files interact is when both `.clippy.toml` AND `clippy.toml` exist in the **same** directory. In that case:
- `.clippy.toml` wins (checked first in the loop)
- `clippy.toml` is noted and a warning is emitted: `"using config file '.clippy.toml', 'clippy.toml' will be ignored"`
- The function returns after processing that single directory

### 3. CARGO_MANIFEST_DIR not set (running clippy-driver directly)

**Falls back to current working directory (`"."`).**

The code is:
```rust
let mut current = env::var_os("CLIPPY_CONF_DIR")
    .or_else(|| env::var_os("CARGO_MANIFEST_DIR"))
    .map_or_else(|| PathBuf::from("."), PathBuf::from)
    .canonicalize()?;
```

When `cargo clippy` runs, Cargo sets `CARGO_MANIFEST_DIR` to the crate's `Cargo.toml` directory. When running `clippy-driver` directly, neither env var is set, so it starts from `"."` (cwd) and walks up from there.

**Historical bug (fixed in PR #10592):** Before the fix, the path was not canonicalized before `pop()` was called, which meant the parent directory walk was broken when using relative paths. The `"."` start dir would fail to walk up because `PathBuf::pop` on a non-canonical relative path doesn't work correctly. This was fixed by adding `.canonicalize()`.

### 4. `.clippy.toml` (dot-prefixed) vs `clippy.toml`

**Both are fully supported and equivalent.** The search checks for both in every directory:

```rust
const CONFIG_FILE_NAMES: [&str; 2] = [".clippy.toml", "clippy.toml"];
```

`.clippy.toml` is checked FIRST, so if both exist in the same directory, `.clippy.toml` wins and a warning is emitted about `clippy.toml` being ignored.

The dot-prefix form is useful for hiding the config file from directory listings (Unix convention).

### 5. Syntax errors in clippy.toml

**Clippy does NOT fail/abort. It emits compiler diagnostic errors and falls back to default configuration.**

The error handling chain:

1. **TOML parse error:** `toml::de::Deserializer` fails -> `TryConf::from_toml_error()` creates a `TryConf` with `Conf::default()` and the error recorded.

2. **Unknown keys:** The `ConfVisitor` (serde deserializer) detects unknown fields and records them as errors with edit-distance suggestions (e.g., "did you mean `foo-bar`?").

3. **Underscore vs hyphen:** If a key uses underscores (e.g., `allow_mixed_uninlined_format_args`), Clippy suggests the kebab-case form (`allow-mixed-uninlined-format-args`).

4. **All errors are non-fatal.** From `read_inner()`:
   ```rust
   // all conf errors are non-fatal, we just use the default conf in case of error
   for error in errors {
       // ... emit as span_err diagnostic
   }
   ```

5. **The result:** Clippy continues linting with default values for any settings that couldn't be parsed. The errors appear as compiler diagnostics (shown to the user), but compilation/linting proceeds.

### 6. `CLIPPY_CONF_DIR` env var

**YES, it overrides the starting directory for the search.** It is checked FIRST, before `CARGO_MANIFEST_DIR`.

```rust
let mut current = env::var_os("CLIPPY_CONF_DIR")
    .or_else(|| env::var_os("CARGO_MANIFEST_DIR"))
    .map_or_else(|| PathBuf::from("."), PathBuf::from)
    .canonicalize()?;
```

**Behavior:** When `CLIPPY_CONF_DIR` is set, the ancestor walk starts from that directory instead of the crate directory. The walk still proceeds upward if no config file is found in the specified directory.

**Important:** This means `CLIPPY_CONF_DIR` doesn't point to the config FILE -- it points to a DIRECTORY that should contain (or whose ancestors should contain) `clippy.toml` or `.clippy.toml`.

**Practical usage via `.cargo/config.toml`:**
```toml
[env]
CLIPPY_CONF_DIR = { value = "path/to/config/dir", relative = true }
```

**Limitation noted in issue #7353:** Using `CLIPPY_CONF_DIR` as a workaround prevents per-crate overrides entirely, since it forces all crates to start their search from the same directory.

---

## Summary Table

| Scenario | Behavior |
|---|---|
| Config at intermediate dir between workspace root and crate | Found and used; workspace root config ignored entirely |
| Two configs in walk-up path | Closest to crate wins; farther one never discovered |
| Both `.clippy.toml` and `clippy.toml` in same dir | `.clippy.toml` wins; warning emitted for the other |
| `CARGO_MANIFEST_DIR` not set | Falls back to `"."` (cwd), walks up from there |
| TOML syntax error | Non-fatal; error diagnostic emitted, defaults used |
| Unknown keys | Non-fatal; error with edit-distance suggestion, defaults used |
| `CLIPPY_CONF_DIR` set | Overrides start directory; walk still proceeds upward |
| `CLIPPY_CONF_DIR` + `CARGO_MANIFEST_DIR` both set | `CLIPPY_CONF_DIR` wins (checked first) |
| Config file is actually a directory | Silently skipped |
| IO error (not NotFound) reading config | Hard error, function returns `Err` |
| No config found anywhere up to filesystem root | Returns `(None, [])`, all defaults used |

---

## Implications for Workspace Tooling

1. **No config merging exists.** You cannot have workspace-wide defaults in root `clippy.toml` and per-crate overrides. The first config found shadows everything above it. Issue #7353 tracks this as a feature request (still open, PR #10929 was closed).

2. **Intermediate directories are hazardous.** A `clippy.toml` at `crates/adapters/` will silently shadow the workspace root config for all crates under `crates/adapters/`. This is especially dangerous if someone creates one for a subset of crates without realizing it blocks the root config.

3. **`CLIPPY_CONF_DIR` is all-or-nothing.** It forces a single start directory for all crates -- useful for ensuring a shared config, but prevents any per-crate customization.

4. **Syntax errors are survivable.** Clippy will continue with defaults, but the user sees diagnostic errors. A CI pipeline checking for clippy warnings/errors would still catch this.
