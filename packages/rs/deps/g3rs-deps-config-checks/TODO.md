# g3rs-deps-config-checks TODO

## Current Ownership

- `RS-DEPS-CONFIG-06..09` own workspace-scoped tool presence for:
  - `cargo-deny`
  - `cargo-machete`
  - `cargo-dupes`
  - `gitleaks`

## Boundary Notes

- The package boundary uses the root `guardrail3-rs.toml`.
- It no longer depends on the legacy repo-global `guardrail3.toml`.
- The package also accepts parsed local path Cargo manifests when the app has
  already discovered them, because dependency identity sometimes depends on the
  target crate's real `package.name`

## Known Limitation

### External local Cargo targets outside the selected family view are not observable at the family layer

- Package-level tests cover local path dependency identity when the target
  manifest is explicitly supplied to the package input.
- The app-family bridge cannot assert that same identity behavior for sibling
  manifests that are outside the selected family view, because those files are
  not part of the routed surface.

Follow-up:

- If deps validation later needs to reason across a broader routed scope, decide
  that explicitly in the family view / mapper layer rather than smuggling it
  through content inputs.

## Boundary Guard

- Keep fail-closed ownership in package ingestion.
- Package input-site collection must not duplicate file discovery inside checks.
