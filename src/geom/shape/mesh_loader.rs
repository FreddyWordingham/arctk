//! Mesh loader.

use crate::{
    err::Error,
    fs::{File, Load},
    geom::{Mesh, Transformable},
    math::Trans3Builder,
    ord::Build,
};
use arctk_attr::file;
use std::path::{Path, PathBuf};

/// Loadable triangle mesh conglomerate structure.
#[file]
pub struct MeshLoader(
    /// List of object files.
    Vec<PathBuf>,
    /// Optional transformation.
    Option<Trans3Builder>,
);

impl Load for MeshLoader {
    type Inst = Mesh;

    #[inline]
    fn load(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let trans = self.1.map(Build::build);

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
