//! Render input.

use crate::{
    fmt_report,
    geom::{Camera, Tree},
    img::Gradient,
    ord::Set,
    sim::render::{Attribute, Settings, Shader},
    util::gradient::to_string,
};
use std::fmt::{Display, Error, Formatter};

/// Rendering simulation resources conglomerate.
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
    pub sett: &'a Settings,
    /// Shader settings.
    pub shader: &'a Shader<'a>,
    /// Image number.
    pub img_id: usize,
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
        shader: &'a Shader,
        img_id: usize,
    ) -> Self {
        Self {
            grads,
            attrs,
            cam,
            tree,
            sett,
            shader,
            img_id,
        }
    }
}

impl Display for Input<'_> {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, "...", "gradients");
        for (key, val) in self.grads.map() {
            fmt_report!(fmt, to_string(val, 32), &format!("{}", key));
        }
        fmt_report!(fmt, self.attrs, "attributes");
        fmt_report!(fmt, self.cam, "camera");
        fmt_report!(fmt, self.tree, "hit-scan tree");
        fmt_report!(fmt, self.sett, "settings");
        fmt_report!(fmt, self.shader, "shader");
        fmt_report!(fmt, self.img_id, "image id");
        Ok(())
    }
}
