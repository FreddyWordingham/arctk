//! Display trait implementation

use crate::{report, tree::Cell};
use std::fmt::{Display, Formatter, Result};

impl<'a> Display for Cell<'a> {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let name = match self {
            Self::Root { .. } => "Root",
            Self::Branch { .. } => "Branch",
            Self::Leaf { .. } => "Leaf",
            Self::Empty { .. } => "Empty",
        };
        writeln!(
            fmt,
            "{}",
            report::obj("type", name).expect("Could not format name.")
        )?;

        let num_cells = self.num_cells();
        writeln!(
            fmt,
            "{}",
            report::obj("total cells", num_cells).expect("Could not format field.")
        )?;

        let num_leaf_cells = self.num_leaf_cells();
        let leaf_cells_display = format!(
            "{} ({:.2}%)",
            num_leaf_cells,
            num_leaf_cells as f64 / num_cells as f64 * 100.0
        );
        writeln!(
            fmt,
            "{}",
            report::obj("number of leaf cells", leaf_cells_display)
                .expect("Could not format field.")
        )?;

        let num_tri_refs = self.num_tri_refs();
        writeln!(
            fmt,
            "{}",
            report::obj("number of tri refs", num_tri_refs).expect("Could not format field.")
        )?;

        let ave_leaf_tris = num_tri_refs as f64 / num_leaf_cells as f64;
        writeln!(
            fmt,
            "{}",
            report::obj("ave tri refs per leaf", ave_leaf_tris).expect("Could not format field.")
        )?;

        write!(
            fmt,
            "{}",
            report::obj("depth", self.depth()).expect("Could not format field.")
        )
    }
}
