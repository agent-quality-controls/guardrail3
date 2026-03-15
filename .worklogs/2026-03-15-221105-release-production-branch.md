# Switch release workflow to production branch

**Date:** 2026-03-15 22:11
**Scope:** .github/workflows/release.yml

## Summary
Changed release.yml trigger from `main` to `production` branch. Agents push to main freely. Releases only happen when main is merged to production.

## Context
Agents push to main frequently. Triggering releases on every push would publish unreviewed versions. Production branch acts as a release gate.
