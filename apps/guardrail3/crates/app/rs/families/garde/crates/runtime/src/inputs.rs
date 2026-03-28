use super::facts::{
    BoundaryFieldFacts, DerivedBoundaryTypeFacts, GardeInputFailureFacts, GardeRootFacts,
    ManualDeserializeImplFacts, QueryAsMacroFacts,
};

pub struct GardeRootInput<'a> {
    pub root: &'a GardeRootFacts,
}

pub struct DerivedBoundaryTypeInput<'a> {
    pub target: &'a DerivedBoundaryTypeFacts,
}

pub struct ManualDeserializeImplInput<'a> {
    pub target: &'a ManualDeserializeImplFacts,
}

pub struct QueryAsMacroInput<'a> {
    pub macro_use: &'a QueryAsMacroFacts,
}

pub struct BoundaryFieldInput<'a> {
    pub field: &'a BoundaryFieldFacts,
}

pub struct GardeInputFailureInput<'a> {
    pub failure: &'a GardeInputFailureFacts,
}

impl<'a> GardeRootInput<'a> {
    pub const fn new(root: &'a GardeRootFacts) -> Self {
        Self { root }
    }
}

impl<'a> DerivedBoundaryTypeInput<'a> {
    pub const fn new(target: &'a DerivedBoundaryTypeFacts) -> Self {
        Self { target }
    }
}

impl<'a> ManualDeserializeImplInput<'a> {
    pub const fn new(target: &'a ManualDeserializeImplFacts) -> Self {
        Self { target }
    }
}

impl<'a> QueryAsMacroInput<'a> {
    pub const fn new(macro_use: &'a QueryAsMacroFacts) -> Self {
        Self { macro_use }
    }
}

impl<'a> BoundaryFieldInput<'a> {
    pub const fn new(field: &'a BoundaryFieldFacts) -> Self {
        Self { field }
    }
}

impl<'a> GardeInputFailureInput<'a> {
    pub const fn new(failure: &'a GardeInputFailureFacts) -> Self {
        Self { failure }
    }
}
