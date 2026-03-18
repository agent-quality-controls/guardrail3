# release-plz.toml

## Location

**Where release-plz looks:** Same directory as root `Cargo.toml`. Files: `release-plz.toml` or `.release-plz.toml`. No walk-up. No per-crate configs.

**In steady-parent:** DOES NOT EXIST.

**Scoping:** One per repo root. Not per-workspace — release-plz operates on the entire repository. Per-package customization is done WITHIN the single file via `[[package]]` blocks.

## Contents

64 config fields (verified from https://release-plz.dev/docs/config):
- `[workspace]` section: 31 fields (defaults for all packages)
- `[[package]]` section: 24 fields (per-crate overrides) — requires `name` field
- `[changelog]` section: 10 fields (commit parsing, templates)

**guardrail3 baseline (what we'd scaffold):**
- `changelog_update = true`
- `git_release_enable = true`
- `git_tag_enable = true`
- `publish = true`
- `semver_check = true`

**User-owned (project-specific):**
- `repo_url` — project's GitHub URL
- `git_tag_name`, `git_release_name`, `pr_name`, `pr_body` — Tera templates
- `pr_labels` — project-specific labels
- `release_commits` — regex for project's commit convention
- `publish_features` — project-specific features
- ALL `[[package]]` blocks — which crates to release, version groups, per-crate changelog paths
- ALL `[changelog]` config — commit parsers, templates, preprocessors

## Category: Scaffold-once (opt-in)

- Only generate when `[rust.apps.X.checks] release = true` for at least one app
- Generate baseline on first run with guardrail3 defaults
- NEVER overwrite — too much project-specific config (repo_url, package blocks, changelog templates)
- Detection: if file exists, skip. No merge needed — the baseline keys are non-controversial defaults that users rarely change.

## Algorithm

```
1. If file exists: do nothing (scaffold-once)
2. If file doesn't exist AND release checks enabled: create with baseline
3. validate: check that file exists (if release checks enabled), warn if missing
```

## No merge, no overrides

The file is either scaffolded or left alone. guardrail3 validates existence but doesn't manage content after creation.
