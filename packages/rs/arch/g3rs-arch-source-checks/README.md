# g3rs-arch-source-checks

Runs the `arch` family source checks.

Current rules:

- `g3rs-arch/lib-facade-only` lib.rs must be facade-only
- `g3rs-arch/mod-facade-only` mod.rs must be facade-only
- `g3rs-arch/feature-gated-exports` facade exports must be feature-gated
- `g3rs-arch/no-path-attr` no `#[path = ...]`
