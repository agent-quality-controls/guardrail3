# g3rs-test-source-checks

Runs the `test` family source checks on one root-scoped source bundle.

Current rules:

- `RS-TEST-SOURCE-01` inline `#[cfg(test)] mod ... { ... }`
- `RS-TEST-SOURCE-04` `#[ignore]` reason quality
- `RS-TEST-SOURCE-05` `#[should_panic(expected = ...)]`
- `RS-TEST-SOURCE-06` tautological assertions
- `RS-TEST-SOURCE-07` real proof site
- `RS-TEST-SOURCE-08` weak wildcard `matches!`
- `RS-TEST-SOURCE-10` source parse/input failures
- `RS-TEST-SOURCE-16` assertions modules prove runtime
- `RS-TEST-SOURCE-17` external harnesses use owned assertions
