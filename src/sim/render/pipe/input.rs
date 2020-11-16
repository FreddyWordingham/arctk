//! Render input.

use crate::{
    geom::Tree,
    img::Gradient,
    ord::Set,
    sim::render::{Attribute, Camera, Settings},
};

/// MCRT simulation resources conglomerate.
pub struct Input<'a> {
    /// Gradients.
    pub grads: &'a Set<Gradient>,
    /// Attributes.
    pub attrs: &'a Set<Attribute<'a>>,
    /// Capturing camera.
    pub cam: &'a Camera,
    /// Hit-scan tree.
    pub tree: &'a Tree<'a, Attribute<'a>>,
    /// General settings.
    pub sett: &'a Settings<'a>,
}

impl<'a> Input<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(
        grads: &'a Set<Gradient>,
        attrs: &'a Set<Attribute>,
        cam: &'a Camera,
        tree: &'a Tree<Attribute>,
        sett: &'a Settings,
    ) -> Self {
        Self {
            grads,
            attrs,
            cam,
            tree,
            sett,
        }
    }
}
