# g3rs-code-source-checks

Extracted single-file Rust source checks for the `code` family.

Initial rules in this package:

- `g3rs-code/todo-macros`
- `g3rs-code/direct-fs-usage`
- `g3rs-code/panic-macro`

This package validates one Rust source file at a time.

Intentional divergence from the retired old app runtime:

- `g3rs-code/path-attr-with-reason` stays in `code` as the documented `#[path]` policy.
- `arch` also owns the stricter blanket ban `g3rs-arch/no-path-attr`.

It does not own:

- workspace/root discovery
- config checks
- root/workspace structural checks
- cross-file or repo-global legality
