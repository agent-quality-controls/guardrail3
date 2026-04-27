## Goal

Complete the `release` family under the pointed-workspace package model.

End state:
- `packages/rs/release` owns all meaningful old app release checks
- package rule placement is lane-pure:
  - `config` for Cargo, release-plz, cliff, workflow, tool, and dependency semantics
  - `filetree` for root-file existence, README existence, and lane-local fail-closed input surfacing
  - `source` for README content quality
- fake public release source/filetree stubs are removed or replaced with real package lanes

## Approach

1. Fix current package correctness bugs first.
   - Add failing tests for:
     - workspace package inheritance for publish metadata and `publish`
     - grouped `cliff` parser coverage
     - exact `release-plz` baseline branches
   - Expand release types and ingestion so config rules can resolve:
     - local package fields
     - `workspace.package` inherited fields
     - workspace-level `publish`

2. Build the missing release filetree lane.
   - Add `packages/rs/release/g3rs-release-filetree-checks`
   - Migrate:
     - `g3rs-release/license-file` - repo license material exists
     - `g3rs-release/release-plz-exists` - root `release-plz.toml` exists
     - `g3rs-release/readme-exists` - root `cliff.toml` exists
     - `RS-RELEASE-FILETREE-13` - crate README exists
     - filetree-side release input failures
   - Add failing tests before each fix and use exact result assertions.

3. Build the missing release source lane.
   - Add `packages/rs/release/g3rs-release-source-checks`
   - Migrate:
     - README quality rule from old `RS-PUB-05`
     - source-side release input failures for unreadable README content
   - Keep source inputs narrow: one README file per publishable crate.

4. Finish the remaining config migration.
   - Extend `g3rs-release-config-checks` for:
     - workflow checks
     - cargo-semver-checks PATH check
     - root publish/profile inventory
     - publish dry-run result
     - path-dependency and local version consistency checks
     - publishable/non-publishable crate inventory
     - include/exclude inventory
     - config-side input failures
   - Strengthen release-plz and cliff tests to exact outputs.

5. Fix package surface drift.
   - Remove fake placeholder lane types and fake ingestion exports.
   - Add real ingestion for config/source/filetree.
   - Fix broken ingestion `readme` metadata paths in release package manifests.

6. Verify and attack.
   - Run release package tests.
   - Run a fresh adversarial review against:
     - old app release rules
     - new package rules
     - lane distribution
     - implementation-vs-intent
   - Do not stop until the adversarial review returns no concrete migration or behavior bug.

## Key decisions

- Keep `release` as a real three-lane family.
  - Rejected forcing README existence into config because it is file presence.
  - Rejected forcing README quality into config because it reads source content.
- Treat old `RS-RELEASE-12` as lane-local input failure surfacing rather than one mixed package rule.
  - This matches the package architecture already used in other families.
- Fix inheritance at the typed input boundary instead of special-casing individual config rules.
  - The old app resolved `workspace.package` centrally; the package model should do the same.

## Files to modify

- `packages/rs/release/g3rs-release-types/src/lib.rs`
- `packages/rs/release/g3rs-release-config-checks/**`
- `packages/rs/release/g3rs-release-ingestion/**`
- `packages/rs/release/g3rs-release-filetree-checks/**` (new)
- `packages/rs/release/g3rs-release-source-checks/**` (new)
- release package `Cargo.toml` files and READMEs
