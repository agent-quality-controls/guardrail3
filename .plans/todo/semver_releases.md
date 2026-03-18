# Automated Semver Releases — Template for All Rust Projects

## Tool: release-plz

[release-plz](https://release-plz.dev/) is the Rust ecosystem standard for automated versioning and publishing.

## What it does

1. **Reads conventional commits** — `feat:` → minor, `fix:` → patch
2. **Runs cargo-semver-checks** — detects API breaking changes at the CODE level (changed function signatures, removed public types, etc.) regardless of commit message. Forces major bump automatically.
3. **Opens a Release PR** — version bumps in Cargo.toml, generated CHANGELOG.md (via git-cliff), updated Cargo.lock
4. **On PR merge** — auto-publishes to crates.io, creates GitHub release, tags version

**Key insight:** cargo-semver-checks catches breaking changes even if the agent/developer writes `fix:` in the commit. The version bump is derived from actual code analysis, not trust.

## Files needed (template)

### release-plz.toml (repo root)
```toml
[workspace]
changelog_config = "cliff.toml"
git_release_enable = true
release_always = false

# One entry per publishable package
[[package]]
name = "your-crate-name"
publish = true

# Add more [[package]] entries for monorepos
```

### cliff.toml (repo root — changelog config)
```toml
[changelog]
header = """
# Changelog\n
"""
body = """
{% for group, commits in commits | group_by(attribute="group") %}
    ### {{ group | upper_first }}
    {% for commit in commits %}
        - {{ commit.message | upper_first }}\
    {% endfor %}
{% endfor %}\n
"""
trim = true

[git]
conventional_commits = true
filter_unconventional = true
commit_parsers = [
    { message = "^feat", group = "Features" },
    { message = "^fix", group = "Bug Fixes" },
    { message = "^doc", group = "Documentation" },
    { message = "^perf", group = "Performance" },
    { message = "^refactor", group = "Refactoring" },
    { message = "^style", group = "Styling" },
    { message = "^test", group = "Testing" },
    { message = "^chore", group = "Miscellaneous" },
]
```

### .github/workflows/release.yml
```yaml
name: Release

permissions:
  pull-requests: write
  contents: write

on:
  push:
    branches:
      - main

jobs:
  release-plz:
    name: Release-plz
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install cargo-semver-checks
        run: cargo install cargo-semver-checks

      - name: Release-plz release-pr
        uses: release-plz/action@v0.5
        with:
          command: release-pr
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}

      - name: Release-plz release
        uses: release-plz/action@v0.5
        with:
          command: release
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
```

## Setup checklist for new repos

1. Copy `release-plz.toml`, `cliff.toml`, `.github/workflows/release.yml` from template
2. Update `[[package]]` entries in `release-plz.toml` for each publishable crate
3. Set `CARGO_REGISTRY_TOKEN` secret on GitHub repo (crates.io API token with `publish-new` + `publish-update` scopes)
4. `GITHUB_TOKEN` is provided automatically by GitHub Actions
5. Ensure commits follow conventional commits format (`feat:`, `fix:`, `docs:`, etc.)

## Monorepo considerations

- release-plz handles monorepos natively — each `[[package]]` entry is versioned independently
- Publishing order matters when packages depend on each other. release-plz handles this automatically if path dependencies are declared.
- For workspaces with `exclude` (like shedul3r app), only list publishable packages in `release-plz.toml`

## What NOT to publish

- Apps/binaries that are deployed (not imported as libraries) — shedul3r, dev-process
- Internal tools that are repo-specific

## First deployment used

Implemented in [pipelin3r](https://github.com/websmasher/pipelin3r) monorepo with three publishable packages:
- limit3r (resilience patterns)
- shedul3r-rs-sdk (task server client)
- pipelin3r (pipeline orchestration)
