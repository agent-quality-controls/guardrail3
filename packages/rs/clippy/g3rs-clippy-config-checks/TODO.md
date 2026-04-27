# g3rs-clippy-config-checks TODO

- No known remaining config-lane rule migrations under the pointed-workspace package model.
- `g3rs-clippy/package-native-policy` is package-native policy, not a direct old-app rule ID port.
- Old app `g3rs-clippy/local-policy-root` and `g3rs-clippy/no-op-placeholder` do not survive as standalone package rules:
  - `g3rs-clippy/local-policy-root` was a repo-routed local-policy-root assertion
  - `g3rs-clippy/no-op-placeholder` was a no-op placeholder
