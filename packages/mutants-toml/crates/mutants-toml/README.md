# mutants-toml

Typed parser for `.cargo/mutants.toml` configuration files used by [cargo-mutants](https://mutants.rs).

Maps all known configuration keys to typed Rust fields. Unknown keys are captured in a catch-all `extra` field for forward compatibility.
