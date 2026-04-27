# g3rs-test-source-checks

Runs the `test` family source checks on one root-scoped source bundle.

Current rules:

- `g3rs-test/inline-test-bodies` inline `#[cfg(test)] mod ... { ... }`
- `g3rs-test/ignore-reason` `#[ignore]` reason quality
- `g3rs-test/should-panic-expected` `#[should_panic(expected = ...)]`
- `g3rs-test/tautological-assertions` tautological assertions
- `g3rs-test/real-proof-site` real proof site
- `g3rs-test/weak-matches-assert` weak wildcard `matches!`
- `g3rs-test/source-input-failures` source parse/input failures
- `g3rs-test/assertions-modules-prove` assertions modules prove runtime
- `g3rs-test/external-harnesses-use-assertions` external harnesses use owned assertions
