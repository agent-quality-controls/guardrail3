# cliff.toml

## Location

**Where git-cliff looks:** Since v2.8.0, auto-discovers by walking UP from CWD. Before that, required explicit `--config` flag. Can also be embedded in `Cargo.toml` as `[workspace.metadata.git-cliff.changelog]`.

**In steady-parent:** DOES NOT EXIST.

**Scoping:** One per repo root for the main changelog. Per-crate cliff.toml files optional for per-crate changelogs.

## Contents

Project-specific commit parsing config:
- `[changelog]` — header, body template, trim, tag_pattern, sort_commits
- `[git]` — conventional_commits, filter_unconventional, split_commits, protect_breaking_commits
- `commit_preprocessors` — regex transforms on commit messages
- `commit_parsers` — patterns for grouping commits (feat → Features, fix → Bug Fixes, etc.)
- `link_parsers` — extract issue/PR references as URLs
- `filter_commits` — exclude merge commits, etc.
- `tag_pattern` — regex for matching release tags

ALL of this is project-specific. The commit parser groups depend on the project's commit convention. The link parsers depend on the project's issue tracker URL. The tag pattern depends on the project's tagging scheme.

## Category: Scaffold-once (opt-in)

Same as release-plz.toml:
- Only generate when release checks enabled
- Generate a reasonable starting template on first run
- NEVER overwrite
- No merge needed

## Algorithm

Same as release-plz.toml — scaffold if missing and opted in, otherwise leave alone.
