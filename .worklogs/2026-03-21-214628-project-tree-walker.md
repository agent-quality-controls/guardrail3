# ProjectTree + walker implementation

**Date:** 2026-03-21

## What

`ProjectTree` — the parsed project structure + cached config content. Built once by the walker, consumed by all checkers.

- `crates/domain/project_tree.rs` — types: `ProjectTree`, `DirEntry`, query methods, JSON serialization
- `crates/app/core/project_walker.rs` — `walk_project(fs, root) -> ProjectTree`

## Walker design

1. Walk with `ignore` crate (respects .gitignore, skips .git/)
2. Derive dir children from walker entries (not fs.list_dir — that leaks gitignored files)
3. If in a git repo, `git ls-files` patch: adds back tracked-but-gitignored files

Two maps:
- `structure: BTreeMap<String, DirEntry>` — every dir → children (sorted)
- `content: BTreeMap<String, String>` — cached config file content (not source files)

Keys are relative paths. `""` = root.

## Bugs found and fixed by testing on real projects

1. Walker entered `.git/` — `ignore` crate with `hidden(false)` doesn't auto-skip it. Fixed with `filter_entry`.
2. `fs.list_dir()` leaked gitignored files into structure — rewrote to derive children from walker entries.
3. Tracked-but-gitignored files invisible — `ignore` crate skips ALL gitignore matches regardless of tracking. Fixed with `git ls-files` patch.

## Tests: 25 passing + 3 ignored (real-project lossless)

- Lossless roundtrip on golden fixture (dirs, files, per-dir children, content)
- Lossless on steady-parent (12,300 files) — independent verifier: walkdir + git check-ignore
- Lossless on guardrail3 itself (899 files) — same verifier
- Gitignore: root-level and subdirectory .gitignore
- Tracked-but-gitignored: config in ignored dir, mixed tracked/untracked, root-level .env
- Non-git project works without crash
- Deleted-but-tracked file excluded
- Config vs source classification, all config types
- Edge cases: empty dirs, deep nesting, hidden dirs, sorted children, root key

## Next

Orchestrators consume ProjectTree for checks. Start with RS-ARCH-01 rules 07-11 (workspace enforcement).
