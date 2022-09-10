//! Rendering.

pub mod attribute;
pub mod attribute_builder;
pub mod gradient_builder;
pub mod input;
pub mod output;
pub mod parameters;
pub mod run;
pub mod settings;
pub mod shader;
pub mod shader_builder;

pub use self::{
    attribute::*, attribute_builder::*, gradient_builder::*, input::*, output::*, parameters::*,
    run::*, settings::*, shader::*, shader_builder::*,
};
