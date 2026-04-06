# Fix release config bugs from adversarial review

**Date:** 2026-04-06 22:10
**Scope:** g3rs-release-config-checks, cliff-toml-parser

## Fixes
1. `is_publishable` now treats `publish = []` as unpublishable (was only checking `publish = false`)
2. Check 06 uses `semver::Version::parse()` instead of weak `has_major_minor` check
3. cliff-toml-parser runtime missing `shared = true` metadata — added for consistency with release-plz-toml-parser
