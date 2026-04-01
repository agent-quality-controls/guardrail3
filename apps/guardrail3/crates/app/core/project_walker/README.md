# Project Walker

`ProjectTree` is a repository snapshot, not a family-specific filtered view.

The walker uses a staged approach:

1. `ignore` walk for the fast baseline
2. tracked-file recovery via `git ls-files`
3. ignored-file recovery for files that still matter to discovery and policy
4. immediate-child preservation for loose ignored files and broken symlinks in discovered dirs

## Recovery Rule

We keep a small hard-ban list for roots that are never governed, such as:

- `.git/`
- `.claude/worktrees/`
- `target/`
- `node_modules/`

Instead:

- the baseline walk uses `.gitignore` to skip noise
- then we patch back tracked ignored files
- then we patch back ignored files that define structure or policy outside hard-banned roots

Today, the ignored-file recovery pass restores:

- cached config/manifests handled by `should_cache(...)`

That means ignored-but-relevant files such as:

- `Cargo.toml`
- `guardrail3.toml`
- `.gitignore`
- `package.json`
- `tsconfig.json`

still appear in `ProjectTree`.

## Why This Exists

Families and placers should decide what to check, but they can only do that if
the walker does not let ignored manifests and policy files disappear first.

The recovery pass exists to keep structure/policy discovery fail-closed without
reintroducing arbitrary ignored source files or never-governed scratch trees.

## Future Changes

When a new family needs additional ignored config or policy files restored,
extend the recovery predicate in `project_walker.rs` instead of adding
family-specific filesystem crawling.
