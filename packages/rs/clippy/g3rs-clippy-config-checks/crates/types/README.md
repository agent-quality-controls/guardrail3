# g3rs-clippy-config-checks-types

Feature-gated facade re-export for the public `g3rs-clippy` config-check input
types used by `g3rs-clippy-config-checks`.

This crate owns no private runtime logic. It exists so the package facade can
re-export a stable contract without exposing runtime internals.
