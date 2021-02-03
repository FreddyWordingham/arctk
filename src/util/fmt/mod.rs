//! Formatting module.

pub mod analyze;
pub mod banner;
pub mod data_cube;
pub mod data_square;
pub mod data_tesseract;
pub mod gradient;
pub mod report;
pub mod term;

pub use self::{analyze::*, data_cube::*, data_square::*, data_tesseract::*};
