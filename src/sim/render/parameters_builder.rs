//! Loadable parameters.

use crate::{
    fmt_report,
    geom::{CameraBuilder, SurfaceLinker, TreeSettings},
    img::GradientBuilder,
    ord::{Build, Set},
    sim::render::{AttributeLinker, EngineBuilder, Parameters, Settings, ShaderLinker},
};
use std::fmt::{Display, Error, Formatter};

/// Loadable runtime parameters.
pub struct ParametersBuilder {
    /// Rendering specific settings.
    sett: Settings,
    /// Tree settings.
    tree: TreeSettings,
    /// Surfaces.
    surfs: Set<SurfaceLinker>,
    /// Attributes.
    attrs: Set<AttributeLinker>,
    /// Colour gradients.
    grads: Set<GradientBuilder>,
    /// Main camera.
    cam: CameraBuilder,
    /// Shader settings.
    shader: ShaderLinker,
    /// Engine selection.
    engine: EngineBuilder,
}

impl ParametersBuilder {
    /// Construct a new instance.
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    #[inline]
    pub const fn new(
        sett: Settings,
        tree: TreeSettings,
        surfs: Set<SurfaceLinker>,
        attrs: Set<AttributeLinker>,
        grads: Set<GradientBuilder>,
        cam: CameraBuilder,
        shader: ShaderLinker,
        engine: EngineBuilder,
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

impl Build for ParametersBuilder {
    type Inst = Parameters;

    #[inline]
    fn build(self) -> Self::Inst {
        let sett = self.sett;
        let tree = self.tree;
        let surfs = self.surfs;
        let attrs = self.attrs;
        let grads = self.grads.build();
        let cam = self.cam.build();
        let shader = self.shader;
        let engine = self.engine.build();

        Self::Inst::new(sett, tree, surfs, attrs, grads, cam, shader, engine)
    }
}

impl Display for ParametersBuilder {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.sett, "settings");
        fmt_report!(fmt, self.tree, "tree settings");
        fmt_report!(fmt, self.surfs, "surface");
        fmt_report!(fmt, self.attrs, "attribute");
        fmt_report!(fmt, self.grads, "gradients");
        fmt_report!(fmt, self.cam, "camera");
        fmt_report!(fmt, self.shader, "shader");
        fmt_report!(fmt, self.engine, "engine");
        Ok(())
    }
}
