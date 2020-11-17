//! Attributes implementation.

use arctk_attr::load;

/// Surface attribute setup.
#[load]
pub enum Attribute {
    /// Material interface, inside material name, outside material name.
    Interface(String, String),
    /// Partially reflective mirror, reflection fraction.
    Mirror(f64),
}
