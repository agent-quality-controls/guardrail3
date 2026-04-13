# g3rs-clippy-config-checks TODO

- No known remaining config-lane rule migrations under the pointed-workspace package model.
- `RS-CLIPPY-06` is package-native policy, not a direct old-app rule ID port.
- Old app `RS-CLIPPY-13` and `RS-CLIPPY-15` do not survive as standalone package rules:
  - `RS-CLIPPY-13` was a repo-routed local-policy-root assertion
  - `RS-CLIPPY-15` was a no-op placeholder
