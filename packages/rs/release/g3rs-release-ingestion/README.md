# g3rs-release-ingestion

Facade crate for the `release` family ingestion lane.

This package crawls one pointed Rust workspace and assembles typed inputs for:
- release config checks
- release filetree checks
- release source checks

It owns release-specific normalization of Cargo metadata, release workflow facts,
release-plz/cliff parsing, dependency edge extraction, and README ingestion.
