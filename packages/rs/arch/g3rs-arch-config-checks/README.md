# g3rs-arch-config-checks

Runs the `arch` family config checks.

Current rules:

- `g3rs-arch/no-boundary-crossing` no boundary-crossing crate dependencies
- `g3rs-arch/shared-flag-required` non-child dependencies require `shared = true`
- `g3rs-arch/dependency-count-split` dependency-count threshold forces a split
- `g3rs-arch/feature-contract` facade-export feature contract is valid
