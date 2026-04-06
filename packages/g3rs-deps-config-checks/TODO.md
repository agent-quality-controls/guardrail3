# g3rs-deps-config-checks TODO

## Deferred Family Split

- Keep `RS-DEPS-01..04`, `RS-DEPS-09..11` in the app family.

Reason:
- `01..04` are environment/tooling checks
- `09..10` are lockfile / `.gitignore` structural checks
- `11` is the structural fail-closed owner for malformed or untrustworthy input

## Boundary Notes

- The current package boundary uses the live legacy workspace policy file:
  parsed `guardrail3.toml`
- It does not yet use `guardrail3-rs.toml`
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

- Keep structural malformed-input ownership in the app family.
- Package input-site collection must not duplicate `RS-DEPS-11`.
