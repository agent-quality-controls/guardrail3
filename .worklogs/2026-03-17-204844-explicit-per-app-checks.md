# Explicit per-app checks + clean output format

**Date:** 2026-03-17 20:48
**Scope:** commands/init.rs

## Summary
1. Init dry-run output now reads naturally: "Would create guardrail3.toml:" not "file — would create:"
2. No more global [typescript.checks] — each app lists its own checks explicitly
3. landing (content): architecture=false, content=true, tests=true
4. admin (service): architecture=true, content=false, tests=true
