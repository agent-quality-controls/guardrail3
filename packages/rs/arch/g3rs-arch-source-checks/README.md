# g3rs-arch-source-checks

Runs the `arch` family source checks.

Current rules:

- `RS-ARCH-02` lib.rs must be facade-only
- `RS-ARCH-04` mod.rs must be facade-only
- `RS-ARCH-08A` facade exports must be feature-gated
- `RS-ARCH-09` no `#[path = ...]`
