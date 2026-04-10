# g3rs-hexarch-source-checks

Extracted `hexarch` source checks for the Rust family.

Initial rules in this package:

- `RS-HEXARCH-22`
- `RS-HEXARCH-23`

This package validates one source crate surface at a time.

It does not own:

- workspace/member discovery
- config checks
- file-tree structural checks
- repo-global legality
