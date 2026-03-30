# Project Walker

`ProjectTree` is a repository snapshot, not a family-specific filtered view.

The walker uses a staged approach:

1. `ignore` walk for the fast baseline
2. tracked-file recovery via `git ls-files`
3. ignored-file recovery for files that still matter to validation
4. immediate-child preservation for loose ignored files and broken symlinks in discovered dirs

## Recovery Rule

We do **not** try to enumerate all junk directories that may exist in a repo.
That does not scale.

Instead:

- the baseline walk uses `.gitignore` to skip noise
- then we patch back files that are relevant to validation

Today, the ignored-file recovery pass restores:

- cached config/manifests handled by `should_cache(...)`
- Rust/TypeScript/JavaScript source files

That means ignored-but-relevant files such as:

- `Cargo.toml`
- `guardrail3.toml`
- `.gitignore`
- `package.json`
- `tsconfig.json`
- `*.rs`
- `*.ts`
- `*.tsx`

still appear in `ProjectTree`.

## Why This Exists

Families and placers should decide what to check.
The walker should decide only what exists and what content is available.

If an ignored file is still relevant to validation, hiding it in the walker is a
modeling bug. The recovery pass exists so Rust families now, and TypeScript
families later, can work from the same repository snapshot contract.

## Future Changes

When a new family needs additional ignored file types restored, extend the
recovery predicate in `project_walker.rs` instead of adding family-specific
filesystem crawling.
