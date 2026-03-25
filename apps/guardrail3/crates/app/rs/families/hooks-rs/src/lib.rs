#![recursion_limit = "1024"]

use glob as _;
use guardrail3_domain_project_tree as _;
use guardrail3_outbound_traits as _;
use proc_macro2 as _;
use quote as _;
use semver as _;
use serde_yaml as _;
use syn as _;
use toml as _;

pub mod domain {
    pub use guardrail3_domain_config as config;
    pub use guardrail3_domain_modules as modules;
    pub use guardrail3_domain_report as report;
}

#[doc(hidden)]
#[path = "../../../checks/hooks/shell.rs"]
pub mod hook_shell;

pub mod app {
    pub use guardrail3_app_core as core;

    pub mod rs {
        pub mod checks {
            pub mod hooks {
                pub use crate::hook_shell as shell;
            }
        }
    }
}

#[path = "../../../checks/hooks/rs/mod.rs"]
mod inner;

pub use inner::*;
