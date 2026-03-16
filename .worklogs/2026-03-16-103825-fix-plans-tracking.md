# Remove .plans/ from git tracking

**Date:** 2026-03-16 10:38
**Scope:** .plans/

## Summary
.plans/ files were both committed and in .gitignore, causing release-plz to fail with "uncommitted changes" error. Removed from git tracking, kept locally via .gitignore.
