# Clippy And Deny Hardening Lane

## Focus

These families are mostly implemented. The hardening work is parity and policy-edge attack coverage.

## Main attack classes

- generator/checker drift
- mixed-profile drift
- shadowing / precedence
- typo-like config drift
- malformed escape-hatch entries
- inventory vs error branch exactness

## Clippy

- add direct generator-vs-checker parity tests
- attack root resolution and mixed profile/layer cases
- verify temporary `RS-CLIPPY-19` behavior is tested honestly, not overclaimed

## Deny

- add direct generator-vs-checker parity tests
- attack mixed workspace profile selection
- attack nested config placement, same-root precedence, malformed exceptions/skips/ignores/wrappers
- resolve and test the `RS-DENY-19` policy decision explicitly

## Success condition

No family-local hardcoded canonical fixture can drift silently from the generator.
