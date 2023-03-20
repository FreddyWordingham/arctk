//! Attribute builder.

use palette::{Gradient, LinSrgba};
use serde::Deserialize;
use std::collections::HashMap;

use crate::render::Attribute;

/// Attribute builder.
#[derive(Deserialize)]
#[non_exhaustive]
pub enum AttributeBuilder {
    /// Opaque coloured surface.
    Opaque(String),
    /// Partially reflective mirror, absorption fraction.
    Mirror(String, f64),
    /// Partially transparent, absorption fraction.
    Transparent(String, f64),
    /// Refractive, absorption fraction, inside and outside refractive indices.
    Refractive(String, f64, [f64; 2]),
    /// Luminous surface, brightness multiplier.
    Luminous(String, f64),
    /// Switchable condition, conditional value.
    Switchable([String; 2], f64),
}

impl AttributeBuilder {
    /// Get the names of the `Gradient`s used.
    #[inline]
    #[must_use]
    pub fn used_gradient_names(&self) -> Vec<String> {
        match *self {
            Self::Opaque(ref grad)
            | Self::Mirror(ref grad, ..)
            | Self::Transparent(ref grad, ..)
            | Self::Refractive(ref grad, ..)
            | Self::Luminous(ref grad, ..) => vec![grad.clone()],
            Self::Switchable(ref grads, ..) => grads.clone().to_vec(),
        }
    }
}

impl<'a> AttributeBuilder {
    /// Build the `Attribute`.
    #[inline]
    #[must_use]
    pub fn build(self, grads: &'a HashMap<String, Gradient<LinSrgba>>) -> Attribute<'a> {
        match self {
            Self::Opaque(ref grad) => Attribute::Opaque(
                grads
                    .get(grad)
                    .unwrap_or_else(|| panic!("Failed to link attribute-gradient key: {grad}")),
            ),
            Self::Mirror(ref grad, abs_frac) => Attribute::Mirror(
                grads
                    .get(grad)
                    .unwrap_or_else(|| panic!("Failed to link attribute-gradient key: {grad}")),
                abs_frac,
            ),
            Self::Transparent(ref grad, abs_frac) => Attribute::Transparent(
                grads
                    .get(grad)
                    .unwrap_or_else(|| panic!("Failed to link attribute-gradient key: {grad}")),
                abs_frac,
            ),
            Self::Refractive(ref grad, abs_frac, ref_indices) => Attribute::Refractive(
                grads
                    .get(grad)
                    .unwrap_or_else(|| panic!("Failed to link attribute-gradient key: {grad}")),
                abs_frac,
                ref_indices,
            ),
            Self::Luminous(ref grad, bright_mult) => Attribute::Luminous(
                grads
                    .get(grad)
                    .unwrap_or_else(|| panic!("Failed to link attribute-gradient key: {grad}")),
                bright_mult,
            ),
            Self::Switchable([ref grad_a, ref grad_b], x) => Attribute::Switchable(
                [
                    grads.get(grad_a).unwrap_or_else(|| {
                        panic!("Failed to link attribute-gradient key: {grad_a}")
                    }),
                    grads.get(grad_b).unwrap_or_else(|| {
                        panic!("Failed to link attribute-gradient key: {grad_b}")
                    }),
                ],
                x,
            ),
        }
    }
}
