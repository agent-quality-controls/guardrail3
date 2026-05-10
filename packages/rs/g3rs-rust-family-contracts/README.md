# g3rs-rust-family-contracts

Aggregator facade that exposes each Rust family's hook contract through a single
`family_hook_contract(family)` entry point. Centralizing the dependency on every
per-family hook-contract crate keeps callers (such as the validate runtime) from
linking each family contract directly.
