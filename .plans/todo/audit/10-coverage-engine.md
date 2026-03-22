# Audit: Coverage Map Engine + 11 Tool Coverage Modules

**Scope:** `engine.rs`, `clippy.rs`, `deny.rs`, `rustfmt.rs`, `rust_toolchain.rs`, `eslint.rs`, `tsconfig.rs`, `stylelint.rs`, `prettier.rs`, `cspell.rs`, `npmrc.rs`, `jscpd.rs`, `crawl.rs`

---

## CRITICAL — String::starts_with in shadow detection (FALSE POSITIVE PATH MATCHING)

**File:** `engine.rs:195`
**Code:** `dir_i.starts_with(dir_j.as_str())`

This is `String::starts_with`, NOT `Path::starts_with`. String prefix matching is NOT safe for path comparisons.

**Example exploit:** Two configs at `apps/web-admin/clippy.toml` and `apps/web/clippy.toml`. The directories are `apps/web-admin` and `apps/web`. String `"apps/web-admin".starts_with("apps/web")` = **true**. This falsely reports `apps/web-admin` as a shadow of `apps/web`, even though they are siblings.

**Fix:** Either convert to `Path` and use `Path::starts_with` (component-based), or append `/` separator before comparing: `dir_i.starts_with(&format!("{dir_j}/"))`. The same comparison on line 109 correctly uses `Path::starts_with`, making this inconsistency even more suspicious.

**Lines also affected:** 269 — `dir.starts_with(a)` — this one is `Path::starts_with` on `PathBuf`, so it's actually safe. But the shadow detection at 195 is a real bug.

---

## CRITICAL — Non-walk-up resolution finds FIRST match, not NEAREST ancestor

**File:** `engine.rs:106-110`
```rust
config_files
    .iter()
    .filter_map(|cf| cf.parent())
    .find(|config_dir| dir.starts_with(config_dir))
    .map(Path::to_path_buf)
```

`.find()` returns the first match in iteration order. Config files are sorted alphabetically (crawl sorts them). For jscpd with configs at both `./` and `apps/web/`, a source dir at `apps/web/src/` would match whichever config directory comes first alphabetically — which is `.` (project root), NOT the nearer `apps/web/`.

The correct algorithm should find the **nearest ancestor** (deepest/longest matching path), not the first match. This should sort candidates by path depth descending, or use `.filter().max_by_key(|d| d.components().count())`.

---

## HIGH — Crawler misses `cspell.json` (bare filename, no prefix/suffix)

**File:** `crawl.rs:192-195`
```rust
// cspell: cspell.json, .cspell.json, cspell.config.*, .cspell.config.*
if name.starts_with("cspell.config.") || name.starts_with(".cspell") {
```

The comment says `cspell.json` is a valid config filename, but the pattern `name.starts_with("cspell.config.")` does NOT match bare `cspell.json`. And `name.starts_with(".cspell")` catches `.cspell.json` and `.cspell.config.*` but not `cspell.json`.

The exact match `"cspell.json"` is never handled — not in the `match name.as_str()` block above, and not in `classify_by_pattern`. A project using `cspell.json` as their config file will have it silently ignored by the crawler.

---

## HIGH — Crawler misses `.cargo/deny.toml` variant

**File:** `crawl.rs:99` only matches `"deny.toml" | ".deny.toml"`
**File:** `deny.rs:5` documents: "Checks `deny.toml`, `.deny.toml`, `.cargo/deny.toml`"

The crawler catches files named `deny.toml` or `.deny.toml`. A file at `.cargo/deny.toml` IS named `deny.toml`, so it WILL be caught by the crawler — but only by accident (the filename matches). However, the walk-up resolution checks `cf.parent()` against the current directory. For `.cargo/deny.toml`, the parent is `.cargo/`, not the project root. So a source dir at `src/` walking up to the project root will never match `.cargo/` as a parent directory.

**Result:** `.cargo/deny.toml` is collected but never resolves as covering any source directory. It appears in the config list with zero coverage. This is semantically wrong — cargo-deny DOES use `.cargo/deny.toml` as if it were at the project root.

---

## HIGH — Crawler misses `rust-toolchain` (without `.toml` extension)

**File:** `crawl.rs:101` — only matches `"rust-toolchain.toml"`

Rustup also supports `rust-toolchain` (no extension, legacy format). The coverage module at `rust_toolchain.rs:31` says "nearest `rust-toolchain.toml`" — but real-world projects may still use the extensionless variant. If present, it will be completely invisible.

---

## HIGH — Prettier `source_dirs` only covers TS, misses CSS

**File:** `prettier.rs:34` — `&crawl.dirs_with_ts`

Prettier formats both TS/JS AND CSS files. The comment on line 33 says "Prettier formats TS/JS and CSS" but then only returns `dirs_with_ts`. Directories that contain ONLY CSS files (no TS) will show as uncovered by Prettier even if a Prettier config exists above them.

Should return a union of `dirs_with_ts` and `dirs_with_css`. Since the trait returns `&BTreeSet<PathBuf>`, this would require either a precomputed union in `CrawlResult` or a change to the trait signature.

---

## HIGH — cspell `source_dirs` only covers TS, misses Rust

**File:** `cspell.rs:34-35` — `&crawl.dirs_with_ts`

Comment on line 34 says "cspell checks both TS and Rust files" but only returns `dirs_with_ts`. Directories with only `.rs` files will show as uncovered by cspell, even if they are within cspell's scope.

Same structural issue as Prettier — needs a union of `dirs_with_ts` and `dirs_with_rs`.

---

## HIGH — jscpd `source_dirs` only covers TS, misses Rust

**File:** `jscpd.rs:34-35` — `&crawl.dirs_with_ts`

Comment says "jscpd checks both TS and Rust" but only returns `dirs_with_ts`. Same gap as cspell.

---

## HIGH — Collapse algorithm never terminates early on deep nesting

**File:** `engine.rs:241-265`

The collapse loop iterates until stable. But it never collapses to project root or one level below (line 248-249 guard). Consider this case: 100 uncovered directories all under `apps/web/src/components/...` at varying depths. The algorithm will:
1. Group by parent, collapse pairs
2. Repeat... many iterations

For deeply nested trees with many sibling directories, this is O(depth * n) where each iteration clones the entire `BTreeSet`. Not a correctness bug, but a performance concern for large monorepos.

---

## MEDIUM — `covers` field is always a single-element vec or empty

**File:** `engine.rs:139` — `covers: if count > 0 { vec![config_dir] } else { vec![] }`

Each `ConfigInstance` always gets either zero or exactly ONE `covers` entry — the config's own parent directory. This means the `covers` field never shows the actual individual directories covered. It just shows the config's location. For a config at project root covering 50 directories, `covers` says `["."]` — not the 50 directories.

This is misleading. The field name suggests it lists covered directories, but it just echoes the config location. The actual per-directory coverage data (`config_covered_dirs`) is discarded after building config instances.

---

## MEDIUM — Shadow detection doesn't find nearest parent for 3+ level nesting

**File:** `engine.rs:185-228`

With 3 configs: root (`./`), mid (`apps/`), deep (`apps/web/`):
- `apps/web/` shadows `apps/` (correct — nearest parent)
- `apps/web/` shadows `./` (also reported — transitive shadow)
- `apps/` shadows `./` (correct)

The deep config gets `is_shadow: true, shadows: <last written value>`. Because `shadow_marks` is iterated sequentially, and the same index can be pushed multiple times with different parent paths, the LAST write to `cfg.shadows` wins. This means the `shadows` field for `apps/web/` could point to `./` instead of `apps/` depending on iteration order.

**Specifically:** line 213-217 iterates `shadow_marks` and overwrites `cfg.shadows` each time. For `apps/web/` shadowing both `apps/` and `./`, whichever appears last in the iteration determines the reported parent. This should report only the nearest parent shadow.

---

## MEDIUM — `tsconfig.rs` parse_details uses string matching, not JSON parsing

**File:** `tsconfig.rs:43-45`
```rust
let has_extends = content.contains("\"extends\"");
let has_strict = content.contains("\"strict\": true") || content.contains("\"strict\":true");
```

This is raw string matching on JSON content, which:
1. Matches `"extends"` in comments (JSON doesn't have comments, but tsconfig.json allows them via JSONC)
2. Misses `"strict" :true` (space before colon)
3. Matches `"extends"` appearing as a value string, not just as a key
4. Misses `"strict": true` when it's inside `"compilerOptions"` nested at any depth (it finds it, but also finds it if it's in the wrong place)

Should use a proper JSON/JSONC parser. At minimum, `serde_json::from_str` with comment stripping.

---

## MEDIUM — Crawler misses `.prettierrc` (no extension) and `package.json` "prettier" key

**File:** `crawl.rs:199` — `name.starts_with(".prettierrc")` catches `.prettierrc.json`, `.prettierrc.yml`, etc.

BUT: Prettier also supports a bare `.prettierrc` file (no extension — YAML or JSON auto-detected). `name.starts_with(".prettierrc")` DOES match `.prettierrc` (the string ".prettierrc" starts with ".prettierrc"). So bare `.prettierrc` IS caught. Good.

However, Prettier config in `package.json` under the `"prettier"` key is NOT detected. This is a real config source that the coverage map misses. Projects using `package.json` as their Prettier config will show as uncovered.

---

## MEDIUM — Crawler misses `.stylelintrc` (no extension, bare file)

**File:** `crawl.rs:187` — `name.starts_with(".stylelintrc")` — this DOES match bare `.stylelintrc`. OK, this is fine.

BUT: `stylelint.config.cjs` and `stylelint.config.mjs` — `name.starts_with("stylelint.config.")` catches these. And `package.json` "stylelint" key is missed (same pattern as Prettier).

---

## MEDIUM — No symlink handling

**File:** `crawl.rs:76-81`

The `ignore::WalkBuilder` by default does NOT follow symlinks. There's no `.follow_links(true)` call. If a project has symlinked config directories or symlinked source directories:
1. Symlinked source files won't be counted in `dirs_with_rs`/`dirs_with_ts`/`dirs_with_css`
2. Symlinked config files won't be discovered
3. Coverage map will silently undercount

This may be intentional (following symlinks risks infinite loops), but it's undocumented.

---

## MEDIUM — walk_up_resolve can walk past project root on non-canonical paths

**File:** `engine.rs:297-315`

The stop condition is `current == project_root`. If `start_dir` was constructed with a different path representation than `project_root` (e.g., one has trailing slash, one doesn't; one is canonicalized, one isn't; one uses `./` prefix), the equality check fails and the walk continues past the project root into parent directories.

The `ignore` crate walker returns paths relative to root, so in practice the paths should be consistent. But if `root` is `/foo/bar` and the walker returns `/foo/bar/./src`, the parent chain goes `/foo/bar/./src` -> `/foo/bar/.` -> `/foo/bar` which equals root. So `./` isn't an issue. But symbolic links in the root path itself could cause divergence.

---

## MEDIUM — Empty `covers` vec means config appears but covers nothing

**File:** `engine.rs:139`

If a config file exists but no source directories resolve to it (e.g., a `clippy.toml` in a directory with no `.rs` files below it), the config gets `covers: []`. This is correct data, but `detect_shadows` at line 189 uses `.covers.first()` — configs with empty covers are filtered out of shadow detection. This means a config that SHOULD shadow (it exists at a nested location) but has no direct source files below it won't be reported as a shadow.

**Example:** `apps/shared/clippy.toml` exists but `apps/shared/` has no `.rs` files (only a re-export crate with just `Cargo.toml`). It won't participate in shadow detection, so source dirs under `apps/shared/sub-crate/src/` will walk up, find the root `clippy.toml`, and never know the intermediate `apps/shared/clippy.toml` exists.

Wait — this is wrong. Walk-up resolution IS correct here (it finds the nearest config by walking up from the source dir). The shadow detection just won't REPORT it. But the actual coverage mapping is correct because walk_up_resolve finds configs regardless of whether they have source files directly below them. The shadow detection output is cosmetically incomplete, not functionally broken.

Revising severity: this is a **reporting inaccuracy**, not a coverage bug.

---

## LOW — `check_thresholds` in clippy.rs treats all integers as potential thresholds

**File:** `clippy.rs:127-131`

```rust
for (key, val) in table.as_table().iter().flat_map(|t| t.iter()) {
    if val.is_integer() && !known_keys.contains(&key.as_str()) {
        user_extra = user_extra.saturating_add(1);
    }
}
```

Any integer value in the TOML file (including inside arrays of tables like `[[disallowed-methods]]`) that isn't a known threshold key gets counted as `user_extra`. But `disallowed-methods` entries don't have integer values at the top level, so this is unlikely to fire incorrectly in practice. Still, it's conceptually wrong — it counts ALL top-level integer keys, not just threshold-like ones.

---

## LOW — `diff_bans` in deny.rs only checks `[bans].deny[].name`

**File:** `deny.rs:57-71`

The TOML structure for deny.toml bans is `[bans]` -> `deny` -> array of tables with `name` key. But `table.get("bans")` gets the top-level `[bans]` section. In TOML, `[bans]` is a table and `deny` is an array of tables within it. The parsing `table.get("bans").and_then(|b| b.get("deny"))` is correct for the TOML structure. No issue here.

---

## LOW — Crawler `track_source_dir` misses `.cts` files

**File:** `crawl.rs:267` — TypeScript extensions: `"ts" | "tsx" | "mts" | "js" | "jsx" | "mjs"`

Missing: `.cts` and `.cjs` extensions. While rare, these are valid TypeScript/JavaScript file types that ESLint, Prettier, and other tools process. Directories containing only `.cts` files would not appear in `dirs_with_ts`.

---

## LOW — Crawler misses CSS preprocessor files for Stylelint

**File:** `crawl.rs:271` — Only tracks `.css` extension.

Stylelint can lint `.scss`, `.sass`, `.less`, and `.vue` (with appropriate plugins). If a project has SCSS files but no plain CSS, `dirs_with_css` will be empty and Stylelint coverage will report zero source directories — making the entire coverage map vacuous.

---

## LOW — No race condition protection during crawl

**File:** `crawl.rs:73-176`

The crawler reads the filesystem without any locking or snapshot mechanism. If files are created, moved, or deleted during the crawl (e.g., a parallel build process, an editor saving), the results could be inconsistent — a config file could be collected but then fail to parse in `parse_details`, or a source directory could be recorded but its config removed between crawl and coverage calculation.

This is inherent to any filesystem crawler without snapshotting. The impact is low because guardrail3 is typically run in CI or pre-commit where the tree is stable. But the coverage engine silently swallows parse errors (returning `{"error": "unreadable"}` or `{}`), so the user might not notice the inconsistency.

---

## LOW — `npmrc.rs` walks_up is true but npm doesn't do traditional walk-up

**File:** `npmrc.rs:49` — `walks_up: true`

The module's own documentation says npm finds the project root (first dir with `package.json`) and loads `.npmrc` from there. This is NOT the same as traditional walk-up (check every parent directory). But the coverage engine treats it as walk-up, meaning it will resolve `.npmrc` files at arbitrary parent directories, not just at `package.json` locations.

This is technically correct for coverage mapping purposes (an `.npmrc` above the project root WOULD apply), but it's a semantic mismatch that could produce misleading coverage reports for monorepos where `.npmrc` files exist at non-`package.json` directories.

---

## Summary

| Severity | Count | Key Issues |
|----------|-------|------------|
| CRITICAL | 2 | String::starts_with in shadow detection; non-walk-up finds first not nearest |
| HIGH | 5 | Missing cspell.json; .cargo/deny.toml never resolves; missing rust-toolchain (no ext); Prettier/cspell/jscpd wrong source_dirs |
| MEDIUM | 5 | Shadow nearest-parent overwrite; tsconfig string matching; no symlink handling; walk-up path comparison; shadow reporting for empty-covers configs |
| LOW | 5 | Integer counting in clippy thresholds; missing .cts extension; missing CSS preprocessors; race conditions; npmrc semantic mismatch |
