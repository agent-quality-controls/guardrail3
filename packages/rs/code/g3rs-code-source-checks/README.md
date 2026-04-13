# g3rs-code-source-checks

Extracted single-file Rust source checks for the `code` family.

Initial rules in this package:

- `RS-CODE-SOURCE-13`
- `RS-CODE-SOURCE-15`
- `RS-CODE-SOURCE-16`

This package validates one Rust source file at a time.

Intentional divergence from the retired old app runtime:

- `RS-CODE-SOURCE-24` stays in `code` as the documented `#[path]` policy.
- `arch` also owns the stricter blanket ban `RS-ARCH-SOURCE-09`.

It does not own:

- workspace/root discovery
- config checks
- root/workspace structural checks
- cross-file or repo-global legality
