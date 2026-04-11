# g3rs-hooks-source-checks

Extracted hook script source checks for the `hooks` family.

This package runs:

- Rust hook command rules from the old `HOOK-RS-*` slice
- Hook script safety and dispatcher rules from the old `HOOK-SHARED-*` slice

It validates one hook script at a time.

It does not own:

- hook discovery
- config checks
- file-tree checks
