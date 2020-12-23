//! Rendering module.

pub mod attribute;
pub mod attribute_linker;
// pub mod input;
// pub mod output;
// pub mod parameters;
// pub mod parameters_builder;
// pub mod parameters_builder_loader;
// pub mod run;
pub mod settings;
pub mod shader;
pub mod shader_linker;

pub use self::{attribute::*, attribute_linker::*, settings::*, shader::*, shader_linker::*};
