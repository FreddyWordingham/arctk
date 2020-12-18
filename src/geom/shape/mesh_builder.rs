//! Mesh form implementation.

use crate::{
    err::Error,
    fs::{File, Load},
    geom::{Mesh, Transformable},
    math::Trans3Builder,
    ord::Build,
};
use arctk_attr::load;
use std::path::{Path, PathBuf};

/// Loadable triangle mesh conglomerate structure.
#[load]
pub struct MeshBuilder(
    /// List of object files.
    Vec<PathBuf>,
    /// Optional transformation.
    Option<Trans3Builder>,
);

impl Load for MeshBuilder {
    type Inst = Mesh;

    #[inline]
    fn load(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let trans = if let Some(t) = self.1 {
            Some(t.build())
        } else {
            None
        };

        let mut tris = Vec::new();
        for name in self.0 {
            let mut obj = Self::Inst::load(&in_dir.join(name))?;
            if let Some(t) = trans {
                obj.transform(&t);
            }
            tris.extend(obj.into_tris())
        }

        Ok(Self::Inst::new(tris))
    }
}
