# Fix EXCEPTION comments in parser crate deny.toml files

**Date:** 2026-04-03 22:25

## Summary
Removed `# EXCEPTION:` prefix from regex ban comment (wrappers are baseline,
not an exception) and yanked comment (baseline value, not an exception) in
both rustfmt-toml and mutants-toml deny.toml files. Eliminated 4 CODE-07
warnings.
