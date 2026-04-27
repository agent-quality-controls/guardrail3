# g3rs-fmt-filetree-checks

Filetree checks for the `fmt` family.

Current scope:

- `g3rs-fmt/rustfmt-config-exists`: root rustfmt config exists
- `g3rs-fmt/per-crate-override`: nested rustfmt configs are forbidden
- `g3rs-fmt/dual-file-conflict`: `rustfmt.toml` and `.rustfmt.toml` conflict in the same directory
