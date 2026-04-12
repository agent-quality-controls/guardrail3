# g3rs-hooks-source-checks

Extracted hook script source checks for the `hooks` family.

This package runs the merged `hooks` source lane.

The rule set comes from the old Rust hook-command slice and the old shared hook-script slice, but the package IDs are now normalized under `RS-HOOKS-SOURCE-*`.

It validates one hook script at a time.

It does not own:

- hook discovery
- config checks
- file-tree checks
