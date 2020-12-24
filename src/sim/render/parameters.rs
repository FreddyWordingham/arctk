//! Loadable parameters.

use crate::{
    fmt_report,
    geom::{Camera, SurfaceLinker, TreeSettings},
    img::Gradient,
    ord::Set,
    sim::render::{AttributeLinker, Engine, Settings, ShaderLinker},
    util::fmt::gradient::to_string,
};
use std::fmt::{Display, Error, Formatter};

/// Loadable runtime parameters.
pub struct Parameters {
    /// Rendering specific settings.
    pub sett: Settings,
    /// Tree settings.
    pub tree: TreeSettings,
    /// Surfaces.
    pub surfs: Set<SurfaceLinker>,
    /// Attributes.
    pub attrs: Set<AttributeLinker>,
    /// Colour gradients.
    pub grads: Set<Gradient>,
    /// Main camera.
    pub cam: Camera,
    /// Shader settings.
    pub shader: ShaderLinker,
    /// Engine selection.
    pub engine: Engine,
}

impl Parameters {
    /// Construct a new instance.
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub fn new(
        sett: Settings,
        tree: TreeSettings,
        surfs: Set<SurfaceLinker>,
        attrs: Set<AttributeLinker>,
        grads: Set<Gradient>,
        cam: Camera,
        shader: ShaderLinker,
        engine: Engine,
    ) -> Self {
        Self {
            sett,
            tree,
            surfs,
            attrs,
            grads,
            cam,
            shader,
            engine,
        }
    }
}

impl Display for Parameters {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.sett, "settings");
        fmt_report!(fmt, self.tree, "tree settings");
        fmt_report!(fmt, self.surfs, "surfaces");
        fmt_report!(fmt, self.attrs, "attributes");
        fmt_report!(fmt, "...", "gradients");
        for (key, val) in self.grads.map() {
            fmt_report!(fmt, to_string(val, 32), &format!("{}", key));
        }
        writeln!(fmt)?;
        fmt_report!(fmt, self.cam, "camera");
        fmt_report!(fmt, self.shader, "shader");
        fmt_report!(fmt, "{* POINTER LOADED *}", "engine");
        Ok(())
    }
}
