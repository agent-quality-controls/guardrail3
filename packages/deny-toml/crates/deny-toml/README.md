# deny-toml

Typed parser for `deny.toml` configuration files used by [cargo-deny](https://github.com/EmbarkStudios/cargo-deny).

Maps all five top-level sections (`graph`, `advisories`, `bans`, `licenses`, `sources`) to typed Rust structs. Array entries (ban deny/skip/allow, advisory ignore, license exceptions, ban features) support both simple string and detailed table formats via untagged enums. Unknown keys at every nesting level are captured in catch-all `extra` fields for forward compatibility.
