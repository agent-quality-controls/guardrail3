#![allow(dead_code)]
#![allow(unused_crate_dependencies)]

use std::fs::*;

include!("generated_fragment.rs");

#[allow(dead_code)]
fn item_allow_without_reason() {}

#[allow(dead_code)] // reason: generated fixture keeps local dead-code policy explicit
fn item_allow_with_reason() {}

#[deny(dead_code)]
fn deny_without_reason() {}

struct GardeSkipWithoutComment {
    #[garde(skip)]
    field: String,
}

struct GardeSkipWithComment {
    #[garde(skip)] // reason: fixture bypass documents external validator gap
    field: String,
}

#[cfg_attr(feature = "cli", allow(dead_code))]
fn conditional_allow_inventory() {}

#[cfg_attr(all(), allow(dead_code))]
fn always_true_cfg_attr_bypass() {}

struct AllowBlastRadius;

#[allow(dead_code)]
impl AllowBlastRadius {
    fn a() {}
    fn b() {}
    fn c() {}
    fn d() {}
}

#[allow(improper_ctypes)]
unsafe extern "C" {
    fn puts(s: *const i8);
}

fn todo_probe() {
    todo!();
}

fn direct_fs_usage_probe() {
    let _content = std::fs::read_to_string("Cargo.toml");
}

fn panic_probe() {
    panic!("fixture panic");
}

pub struct LargeStruct {
    pub field_01: u8,
    pub field_02: u8,
    pub field_03: u8,
    pub field_04: u8,
    pub field_05: u8,
    pub field_06: u8,
    pub field_07: u8,
    pub field_08: u8,
    pub field_09: u8,
    pub field_10: u8,
    pub field_11: u8,
    pub field_12: u8,
    pub field_13: u8,
    pub field_14: u8,
    pub field_15: u8,
    pub field_16: u8,
}

pub enum LargeEnum {
    Variant01,
    Variant02,
    Variant03,
    Variant04,
    Variant05,
    Variant06,
    Variant07,
    Variant08,
    Variant09,
    Variant10,
    Variant11,
    Variant12,
    Variant13,
    Variant14,
    Variant15,
    Variant16,
    Variant17,
    Variant18,
    Variant19,
    Variant20,
    Variant21,
}

pub trait LargeTrait {
    fn method_01(&self);
    fn method_02(&self);
    fn method_03(&self);
    fn method_04(&self);
    fn method_05(&self);
    fn method_06(&self);
    fn method_07(&self);
    fn method_08(&self);
    fn method_09(&self);
    fn method_10(&self);
    fn method_11(&self);
    fn method_12(&self);
    fn method_13(&self);
}

pub struct PublicFields {
    pub field_01: u8,
    pub field_02: u8,
    pub field_03: u8,
    pub field_04: u8,
    pub field_05: u8,
}

pub fn weak_error_form() -> Result<(), String> {
    Ok(())
}

pub fn generic_parameter_cap<A, B, C, D, E, F, G>(
    _a: A,
    _b: B,
    _c: C,
    _d: D,
    _e: E,
    _f: F,
    _g: G,
) {
}

fn string_dispatch_cap(value: &str) -> u8 {
    match value {
        "v00" => 0,
        "v01" => 1,
        "v02" => 2,
        "v03" => 3,
        "v04" => 4,
        "v05" => 5,
        "v06" => 6,
        "v07" => 7,
        "v08" => 8,
        "v09" => 9,
        "v10" => 10,
        _ => 11,
    }
}
