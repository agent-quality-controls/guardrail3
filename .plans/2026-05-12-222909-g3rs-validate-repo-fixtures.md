# Goal

Add behavior replay fixtures for `g3rs validate-repo`.

The fixture split must follow the same rule as the workspace fixtures: bundle as much as possible, but never put two cases in the same fixture when one intentional defect hides the other behavior.

# Verified Branches

`validate-repo` runs this path:

```text
run_command
  -> execute_repo
  -> crawler.crawl_any(repo_root)
  -> Hooks family always
  -> Topology family only when repo_root/Cargo.toml exists
  -> marker_pairs::check_repo(repo_root)
```

Visible branches in current code:

- repo root cannot be crawled
- repo root can be crawled and has no root `Cargo.toml`
- repo root can be crawled and has root `Cargo.toml`
- marker pair absent
- marker pair complete
- marker pair incomplete
- marker pair ignored under `behavior/fixtures`
- default `validate-repo` root resolution

# Fixture Root

Use a separate repo-level fixture stack:

```text
behavior/fixtures/g3rs-validate-repo/
behavior/baselines/g3rs-validate-repo/
```

Do not put repo-level fixtures under `behavior/fixtures/g3rs/L00-L80`.

Reason: L00-L80 are dominance layers for `g3rs validate --path`. `validate-repo` has different gates and different visible behavior.

# Script Changes

Parameterize existing replay scripts by manifest.

Required changes:

- `scripts/behavior/baseline_common.py`
  - keep the current workspace fixture manifest as the default
  - allow `--manifest <path>`
  - keep running the repo-local candidate `g3rs` binary

- `scripts/behavior/generate-baselines.py`
  - accept optional `--manifest <path>`

- `scripts/behavior/verify-baselines.py`
  - accept optional `--manifest <path>`

- `scripts/behavior/verify-fixtures.py`
  - accept optional `--manifest <path>`
  - validate repo fixture metadata with repo-level allowed values

- `scripts/behavior/verify-all.sh`
  - verify the existing workspace fixture manifest
  - verify the new validate-repo fixture manifest

# Manifest

Create:

```text
.plans/2026-05-12-222909-g3rs-validate-repo-fixtures.md.manifest.toml
```

Use:

```toml
[fixture_set]
tool = "g3rs"
root = "behavior/fixtures/g3rs-validate-repo"
baseline_root = "behavior/baselines/g3rs-validate-repo"
```

# Fixture Levels

## R00 Invalid Repo Root

Purpose:

- Prove `validate-repo` fails before repo crawl when the explicit repo root is not crawlable.
- Bundle missing path and file path because both are blocked by the same first gate and neither can expose marker-pair or hook behavior.

Fixture:

```text
behavior/fixtures/g3rs-validate-repo/R00-invalid-repo-root/
  fixture.toml
  repo/
    README.md
```

Commands:

```toml
commands = [
  ["validate-repo", "--repo-root", "missing", "--inventory"],
  ["validate-repo", "--repo-root", "README.md", "--inventory"],
]
```

Expected:

- both commands exit nonzero
- stdout is empty
- stderr captures the exact crawl/root error

## R10 Crawlable Repo Without Adoption

Purpose:

- Prove `validate-repo` can run on a crawlable repo with no Rust adoption markers.
- Prove marker-pair policy does not invent findings when no markers exist.

Fixture:

```text
behavior/fixtures/g3rs-validate-repo/R10-crawlable-repo-no-adoption/
  fixture.toml
  repo/
    README.md
```

Commands:

```toml
commands = [
  ["validate-repo", "--repo-root", ".", "--inventory"],
]
```

Expected:

- baseline owns exact stdout/stderr
- no `g3rs-topology/marker-pair-incomplete` finding
- exit is zero because no repo-level error is present

## R15 Hooks Reachable Without Root Cargo

Purpose:

- Prove Hooks family is reachable from `validate-repo` even when root `Cargo.toml` is absent.
- Prove the no-root-`Cargo.toml` branch skips Topology family.

Fixture:

```text
behavior/fixtures/g3rs-validate-repo/R15-hooks-reachable-no-root-cargo/
  fixture.toml
  repo/
    .githooks/pre-commit
    README.md
```

`fixture.toml` must set `git_init = true`; replay creates the temporary `.git` directory before running the command.

Commands:

```toml
commands = [
  ["validate-repo", "--repo-root", ".", "--inventory"],
]
```

Expected:

- stdout contains `== hooks ==`
- stdout does not contain `g3rs-topology/`
- exit is nonzero because the intentionally broken hook is visible

## R20 Crawlable Repo Marker Pair Policy

Purpose:

- Prove marker-pair absent, complete, both incomplete directions, and ignored-under-`behavior/fixtures` cases in one crawl.
- Keep root `Cargo.toml` absent so Topology family does not add unrelated nested-workspace findings that hide marker-pair behavior.

These do not hide each other:

- complete root pair should not report
- `guardrail3-rs.toml` without workspace `Cargo.toml` should report
- workspace `Cargo.toml` without `guardrail3-rs.toml` should report
- incomplete behavior fixture pair should not report
- absent unrelated directory should not report

Fixture:

```text
behavior/fixtures/g3rs-validate-repo/R20-crawlable-repo-marker-pair-policy/
  fixture.toml
  repo/
    packages/complete/Cargo.toml
    packages/complete/guardrail3-rs.toml
    packages/incomplete/guardrail3-rs.toml
    packages/cargo-only/Cargo.toml
    packages/absent/README.md
    behavior/fixtures/g3rs/demo/repo/guardrail3-rs.toml
```

`Cargo.toml` files that represent adoption must contain:

```toml
[workspace]
members = []
```

Commands:

```toml
commands = [
  ["validate-repo", "--repo-root", ".", "--inventory"],
]
```

Expected:

- stdout contains exactly two `g3rs-topology/marker-pair-incomplete` findings
- one finding is for `packages/incomplete/guardrail3-rs.toml`
- one finding is for `packages/cargo-only/Cargo.toml`
- stdout does not report `.`
- stdout does not report `packages/complete`
- stdout does not report `packages/absent`
- stdout does not report `behavior/fixtures/...`
- stdout does not contain `g3rs-topology/no-nested-workspaces`

## R30 Root Adoption Pair Complete

Purpose:

- Prove root `Cargo.toml` branch makes Topology family reachable.
- Prove complete root adoption pair does not produce marker-pair findings.

This cannot be bundled with R20 because root `Cargo.toml` also activates Topology family checks over nested fixture packages. That adds unrelated topology findings and makes marker-pair policy harder to read.

Fixture:

```text
behavior/fixtures/g3rs-validate-repo/R30-root-adoption-pair-complete/
  fixture.toml
  repo/
    Cargo.toml
    guardrail3-rs.toml
    nested/Cargo.toml
    nested/guardrail3-rs.toml
```

`Cargo.toml` must contain:

```toml
[workspace]
members = []
```

Commands:

```toml
commands = [
  ["validate-repo", "--repo-root", ".", "--inventory"],
]
```

Expected:

- stdout does not contain `g3rs-topology/marker-pair-incomplete`
- stdout contains `g3rs-topology/no-nested-workspaces` for `nested/Cargo.toml`
- baseline owns hook/topology output

## R40 Default Repo Root

Purpose:

- Prove bare `g3rs validate-repo --inventory` resolves to the fixture repo root, not the outer guardrail3 checkout.

Fixture:

```text
behavior/fixtures/g3rs-validate-repo/R40-default-repo-root/
  fixture.toml
  repo/
    README.md
```

Commands:

```toml
commands = [
  ["validate-repo", "--inventory"],
]
```

Expected:

- baseline `cwd` is `repo`
- baseline output is repo-local
- output must not include outer guardrail3 paths
- exit is zero because no repo-level error is present

# Why These Are The Minimum Levels

- Missing repo root and file repo root can be bundled: both fail before crawl.
- Empty crawlable repo cannot be bundled with invalid repo root: invalid root hides hook and marker-pair behavior.
- Hooks reachability cannot be bundled with clean crawlable repo because a hook-visible defect changes the expected exit.
- Marker policy can run without root `Cargo.toml`; bundling it with R10 would hide the marker-pair-positive case in the generic hooks output.
- Root `Cargo.toml` reachability cannot be bundled with marker-pair policy without unrelated nested-topology findings.
- Default root resolution cannot be bundled with explicit `--repo-root .`: it tests a different public command path.

# Metadata

Repo-level `fixture.toml` must use repo-specific levels.

Allowed levels:

```text
repo_root_invalid
repo_root_crawlable_no_adoption
repo_hooks_reachable_no_root_cargo
repo_marker_pair_policy
repo_root_adoption_pair_complete
repo_default_root
```

Allowed valid states:

```text
repo_root_found
repo_root_directory
repo_root_crawlable
repo_markers_absent
repo_marker_pair_complete
repo_marker_pair_incomplete_visible
repo_marker_pair_inverse_incomplete_visible
repo_marker_pair_ignored_under_behavior_fixtures
repo_topology_branch_reachable
repo_hooks_branch_reachable
repo_default_root_resolved
```

Allowed intentionally invalid states:

```text
repo_root_missing
repo_root_file
repo_marker_pair_incomplete
repo_hooks_missing
repo_topology_nested_workspace
```

# Baseline Rules

Every command in every repo-level fixture must have a JSON baseline.

Required metadata stays the same:

- `tool`
- `baseline_commit`
- `fixture_hash`
- `runner_version`
- `normalizer_version`
- `output_schema_version`
- `created_at`

The verifier must still run the repo-local candidate binary, not `g3rs` from `PATH`.

Repo-level baseline verification must also fail when:

- stdout or stderr contains the outer guardrail3 checkout path
- R15 does not contain `== hooks ==`
- R15 contains `g3rs-topology/`
- R20 does not contain exactly two `g3rs-topology/marker-pair-incomplete` findings
- R20 marker-pair findings are not for `packages/incomplete/guardrail3-rs.toml` and `packages/cargo-only/Cargo.toml`
- R20 reports `packages/complete`, `packages/absent`, or `behavior/fixtures` as marker-pair incomplete
- R20 contains `g3rs-topology/no-nested-workspaces`
- R30 reports any `g3rs-topology/marker-pair-incomplete`
- R30 does not report `g3rs-topology/no-nested-workspaces`
- R40 baseline cwd is not `repo`

# Verification

Implementation must run:

```sh
scripts/behavior/verify-all.sh
g3rs validate --path apps/guardrail3-rs
g3rs validate-repo
git diff --check
```

After implementation, send an adversarial verifier with this plan file and the changed scripts/fixtures. The verifier must check:

- repo-level fixtures do not reuse workspace-level semantics
- invalid-root cases are bundled without hiding crawlable-repo behavior
- explicit-root and default-root commands remain separate
- marker-pair policy is covered for absent, complete, incomplete, and ignored-under-behavior-fixtures cases without unrelated nested-topology findings
- root `Cargo.toml` topology reachability is covered separately
- bare `validate-repo` does not escape to the outer repo

# Done

- `scripts/behavior/verify-all.sh` verifies both fixture stacks.
- Repo-level fixture baselines exist for R00-R40.
- Workspace L00-L80 behavior remains unchanged.
- Adversarial verifier finds no blocker.
