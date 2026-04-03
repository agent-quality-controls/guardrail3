# Rule message clarity — RELEASE family

Audited all 29 release rules (RS-RELEASE-*, RS-PUB-*, RS-BIN-*).

## Changes
- **RELEASE-01**: Added fix action (create LICENSE)
- **RELEASE-02**: Added fix action (create release-plz.toml)
- **RELEASE-03**: Added fix action for missing crate coverage
- **RELEASE-04**: Added fix action for missing cliff.toml
- **RELEASE-05**: Added fix action (add release-plz workflow step)
- **RELEASE-06**: Added fix action (add publish dry-run step)
- **RELEASE-07**: Added fix action (add CARGO_REGISTRY_TOKEN secret)
- **RELEASE-08**: Added install command (cargo install cargo-semver-checks)
- **RELEASE-11**: Changed severity from Warn to Error (accidental publish is hard to undo)
- **RELEASE-12**: Replaced jargon title "Release-family input failure" → "failed to read release input"
- **PUB-01**: Added fix action (add description)
- **PUB-02**: Added fix action (add license)
- **PUB-03**: Added fix action (add repository)
- **PUB-04**: Changed severity from Warn to Error; added fix action (create README)
- **PUB-05**: Changed severity from Warn to Error; added fix actions (both variants)
- **PUB-06**: Changed severity from Warn to Error; added fix action (add keywords)
- **PUB-07**: Changed severity from Warn to Error; added fix action (add categories)
- **PUB-10**: Added fix action (make target publishable or use version req)
- **PUB-11**: Added fix action (update version requirement)
- **PUB-13**: Changed severity from Info to Warn; added fix action (add docs.rs metadata)
- **PUB-14**: Changed severity from Info to Warn; added fix action with example include pattern
- **BIN-03**: Added fix action (add binstall metadata)

## Severity fixes
- RELEASE-11: Warn → Error (accidentally publishable internal crates)
- PUB-04/05/06/07: Warn → Error (publish metadata is required, not optional)
- PUB-13/14: Info → Warn (soft recommendations should be visible)

## Noted in splitting doc
- Release family should unify ID prefixes: RS-PUB-* and RS-BIN-* → RS-RELEASE-*

## Unchanged
RELEASE-03 (config warnings), RELEASE-09/10 (inventory), PUB-08/09 (already good), PUB-12 (inventory), BIN-01/02 (inventory)
