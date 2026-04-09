# g3rs-test-ast-checks

Runs the `test` family AST checks on one root-scoped source bundle.

Current rules:

- `RS-TEST-01` inline `#[cfg(test)] mod ... { ... }`
- `RS-TEST-04` `#[ignore]` reason quality
- `RS-TEST-05` `#[should_panic(expected = ...)]`
- `RS-TEST-06` tautological assertions
- `RS-TEST-07` real proof site
- `RS-TEST-08` weak wildcard `matches!`
- `RS-TEST-10` AST parse/input failures
- `RS-TEST-16` assertions modules prove runtime
- `RS-TEST-17` external harnesses use owned assertions
