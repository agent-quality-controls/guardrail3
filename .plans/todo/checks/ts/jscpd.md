# TS-JSCPD — Duplication-policy checker

**Input:** `.jscpd.json`, relevant project structure / eslint config
**Parser:** JSON + structured config checks
**Current code:** `app/ts/validate/jscpd_check.rs`
**Owned root:** nearest TS package/app root with a `jscpd` config surface

## Owns

- `.jscpd.json` existence and parseability
- required threshold/config fields
- ignore-pattern policy
- format policy

## Does not own

- general package.json policy
- generic auxiliary tool checks
- content-site structure or content-pipeline checks
  - those belong to `ts/content`

## Contract direction

This should remain its own family because duplication detection has:
- its own config file
- its own threshold policy
- its own ignore semantics

and should not stay buried under generic “config files”.

Content-site checks currently parked in `jscpd_check.rs` are legacy misplacement and should move out.
