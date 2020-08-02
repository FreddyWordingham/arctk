//! Mesh form implementation.

use crate::{display_field, display_field_ln, Build, Error, Load, Trans3Builder, Transform};
use attr::load;
use std::{
    fmt::{Display, Formatter},
    path::Path,
};

/// Loadable triangle mesh conglomerate structure.
#[load]
pub struct MeshBuilder(
    /// List of object files.
    Vec<String>,
    /// Optional transformation.
    Option<Trans3Builder>,
);

impl Build for MeshBuilder {
    type Inst = crate::Mesh;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let trans = if let Some(t) = self.1 {
            Some(t.build(in_dir)?)
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

impl Display for MeshBuilder {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        display_field_ln!(fmt, "number of objects", self.0.len())?;
        if let Some(trans) = &self.1 {
            display_field!(fmt, "transform", trans)
        } else {
            display_field!(fmt, "transform", "none")
        }
    }
}
