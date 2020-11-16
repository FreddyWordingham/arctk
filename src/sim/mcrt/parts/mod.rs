//! Simulation component parts.

pub mod attribute;
pub mod attribute_linker;
pub mod light;
pub mod light_builder;
pub mod local;
pub mod material;
pub mod material_builder;
pub mod photon;
pub mod settings;
pub mod settings_linker;

pub use self::{
    attribute::*, attribute_linker::*, light::*, light_builder::*, local::*, material::*,
    material_builder::*, photon::*, settings::*, settings_linker::*,
};
