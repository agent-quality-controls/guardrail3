# nextest-toml

Typed parser for `.config/nextest.toml` configuration files used by [cargo-nextest](https://nexte.st/).

Maps profile definitions to typed Rust structs. Timeout configurations support both simple string (`"60s"`) and detailed table (`{ period = "60s", terminate-after = 2 }`) formats via an untagged enum. Unknown keys at both the top level and within profiles are captured in catch-all `extra` fields for forward compatibility.
