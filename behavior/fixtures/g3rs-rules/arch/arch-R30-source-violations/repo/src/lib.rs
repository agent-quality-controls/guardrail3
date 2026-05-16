pub mod api;

#[path = "hidden.rs"]
mod hidden;

pub fn leaked() {}
