# clippy-toml

Typed parser for `clippy.toml` / `.clippy.toml` configuration files used by [Clippy](https://doc.rust-lang.org/clippy/).

Maps all known configuration keys to typed Rust fields. Ban entries (`disallowed-methods`, `disallowed-types`, `disallowed-macros`) support both simple string and `{path, reason}` table formats. Unknown keys are captured in a catch-all `extra` field for forward compatibility.
