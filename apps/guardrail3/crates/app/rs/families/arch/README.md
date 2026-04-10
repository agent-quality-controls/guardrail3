# RS-ARCH

Rust crate-architecture checks for split library facades and internal crate
boundaries.

This family owns the generic cross-crate architecture contract that is broader
than app-only `hexarch` and less topology-oriented than `topology`:

- when a flat library must split into an internal multi-crate architecture
- the split root must remain the workspace facade package
- a split root must actually own internal member crates
- external crates must not depend directly on those internal member crates

It does not own:

- Rust root placement or workspace legality
- app-only hexarch structure
- old layered `api/core/infra` shape rules

Those remain in:

- `RS-TOPOLOGY` for placement and workspace legality
- `RS-HEXARCH` for app-local structure
- no separate package-only layered family
