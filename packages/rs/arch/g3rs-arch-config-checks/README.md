# g3rs-arch-config-checks

Runs the `arch` family config checks.

Current rules:

- `RS-ARCH-CONFIG-05` no boundary-crossing crate dependencies
- `RS-ARCH-CONFIG-06` non-child dependencies require `shared = true`
- `RS-ARCH-CONFIG-07` dependency-count threshold forces a split
- `RS-ARCH-CONFIG-08` facade-export feature contract is valid
