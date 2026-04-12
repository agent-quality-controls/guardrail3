# g3rs-arch-config-checks

Runs the `arch` family config checks.

Current rules:

- `RS-ARCH-05` no boundary-crossing crate dependencies
- `RS-ARCH-06` non-child dependencies require `shared = true`
- `RS-ARCH-07B` dependency-count threshold forces a split
- `RS-ARCH-08B` facade-export feature contract is valid
