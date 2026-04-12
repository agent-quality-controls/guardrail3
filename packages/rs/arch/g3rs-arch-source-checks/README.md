# g3rs-arch-source-checks

Runs the `arch` family source checks.

Current rules:

- `RS-ARCH-SOURCE-02` lib.rs must be facade-only
- `RS-ARCH-SOURCE-04` mod.rs must be facade-only
- `RS-ARCH-SOURCE-08` facade exports must be feature-gated
- `RS-ARCH-SOURCE-09` no `#[path = ...]`
