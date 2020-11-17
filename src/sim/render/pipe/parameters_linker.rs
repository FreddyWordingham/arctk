//! Parameters setup file.

use crate::{
    geom::{Camera, SurfaceLinker, TreeSettings},
    img::Gradient,
    ord::Set,
    sim::render::{AttributeLinker, Engine, SettingsLinker},
};

/// Parameter linker structure.
/// Holds setup links to data loaded in memory.
pub struct ParametersLinker {
    /// Colour gradients.
    pub grads: Set<Gradient>,
    /// Attributes.
    pub attrs: Set<AttributeLinker>,
    /// Surfaces.
    pub surfs: Set<SurfaceLinker>,
    /// Main camera.
    pub cam: Camera,
    /// Tree settings.
    pub tree: TreeSettings,
    /// Rendering specific settings.
    pub sett: SettingsLinker,
    /// Engine function.
    pub engine: Engine,
}

impl ParametersLinker {
    /// Construct a new instance.
    #[allow(clippy::too_many_arguments)]
    #[inline]
    #[must_use]
    pub fn new(
        grads: Set<Gradient>,
        attrs: Set<AttributeLinker>,
        surfs: Set<SurfaceLinker>,
        cam: Camera,
        tree: TreeSettings,
        sett: SettingsLinker,
        engine: Engine,
    ) -> Self {
        Self {
            grads,
            attrs,
            surfs,
            cam,
            tree,
            sett,
            engine,
        }
    }
}
