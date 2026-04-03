# Finalize rustfmt-toml guardrail compliance

**Date:** 2026-04-03 21:55

## Summary
Copied full generated clippy.toml and deny.toml from app (with complete
baseline: all method/type/macro bans, thresholds, deny entries). All lint
allows removed — every clippy lint at deny level. Added allowed_deps to
guardrail3.toml packages config.

## Remaining
- CODE-31 (1 error): 80 pub fields — rule needs Deserialize struct exception
- DEPS-08 (1 warning): allowed_deps config path may be wrong for packages
- All other warnings are inventory (ban entries, documented allows, exception comments)
