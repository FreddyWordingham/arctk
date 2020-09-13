//! Adaptive tree builder.

use crate::{
    clone,
    geom::{Collide, Cube, Mesh, SmoothTriangle, Tree},
    math::Pos3,
    ord::Set,
    tools::ProgressBar,
};
use arctk_attr::load;
use std::fmt::Display;

/// Tree builder.
#[load]
pub struct TreeBuilder {
    /// Target maximum number of triangles per cell.
    tar_tris: usize,
    /// Maximum mesh depth.
    max_depth: u32,
    /// Collision detection padding.
    padding: f64,
}

impl TreeBuilder {
    clone!(tar_tris, usize);
    clone!(max_depth, u32);
    clone!(padding, f64);

    /// Construct a new tree root cell.
    /// Root cell has a depth of zero.
    #[inline]
    #[must_use]
    pub fn build<'a, T: Display + Clone + Ord>(&self, surfs: &'a Set<T, Mesh>) -> Tree<'a, &'a T> {
        let mut boundary = self.init_boundary(surfs);
        boundary.expand(self.padding);

        let mut tris = Vec::new();
        for (key, mesh) in surfs.map() {
            tris.reserve(mesh.tris().len());
            for tri in mesh.tris() {
                tris.push((key, tri));
            }
        }

        let mut pb = ProgressBar::new("Growing tree", 8_u64.pow(self.max_depth));
        let children = self.init_children(&boundary, 1, tris.as_slice(), &mut pb);
        pb.finish_with_message("Tree grown.");

        Tree::Root { boundary, children }
    }

    /// Initialise the boundary encompassing all of the mesh vertices.
    #[inline]
    #[must_use]
    fn init_boundary<T: Display + Clone + Ord>(&self, surfs: &Set<T, Mesh>) -> Cube {
        let mut mins = None;
        let mut maxs = None;

        for mesh in surfs.map().values() {
            let (mesh_mins, mesh_maxs) = mesh.boundary().mins_maxs();

            if mins.is_none() {
                mins = Some(mesh_mins);
            } else {
                for (grid_min, mesh_min) in mins.as_mut().unwrap().iter_mut().zip(mesh_mins.iter())
                {
                    if mesh_min < grid_min {
                        *grid_min = *mesh_min;
                    }
                }
            }

            if maxs.is_none() {
                maxs = Some(mesh_maxs);
            } else {
                for (grid_max, mesh_max) in maxs.as_mut().unwrap().iter_mut().zip(mesh_maxs.iter())
                {
                    if mesh_max > grid_max {
                        *grid_max = *mesh_max;
                    }
                }
            }
        }

        Cube::new(mins.unwrap(), maxs.unwrap())
    }

    /// Initialise the children of a branching cell.
    #[allow(clippy::similar_names)]
    #[inline]
    #[must_use]
    fn init_children<'a, T: Clone>(
        &self,
        parent_boundary: &Cube,
        depth: u32,
        potential_tris: &[(&'a T, &'a SmoothTriangle)],
        mut pb: &mut ProgressBar,
    ) -> [Box<Tree<'a, &'a T>>; 8] {
        debug_assert!(depth <= self.max_depth);
        debug_assert!(!potential_tris.is_empty());

        let hws = parent_boundary.half_widths();
        let mut make_child = |min_x: f64, min_y: f64, min_z: f64| {
            let min = Pos3::new(min_x, min_y, min_z);
            Box::new(self.init_child(Cube::new(min, min + hws), depth, potential_tris, &mut pb))
        };

        let mins = parent_boundary.mins();
        let min_x = mins.x;
        let min_y = mins.y;
        let min_z = mins.z;

        let nnn = make_child(min_x, min_y, min_z);
        let pnn = make_child(min_x + hws.x, min_y, min_z);
        let npn = make_child(min_x, min_y + hws.y, min_z);
        let ppn = make_child(min_x + hws.x, min_y + hws.y, min_z);
        let nnp = make_child(min_x, min_y, min_z + hws.z);
        let pnp = make_child(min_x + hws.x, min_y, min_z + hws.z);
        let npp = make_child(min_x, min_y + hws.y, min_z + hws.z);
        let ppp = make_child(min_x + hws.x, min_y + hws.y, min_z + hws.z);

        [nnn, pnn, npn, ppn, nnp, pnp, npp, ppp]
    }

    /// Initialise a child cell.
    #[inline]
    #[must_use]
    fn init_child<'a, T: Clone>(
        &self,
        boundary: Cube,
        depth: u32,
        potential_tris: &[(&'a T, &'a SmoothTriangle)],
        mut pb: &mut ProgressBar,
    ) -> Tree<'a, &'a T> {
        debug_assert!(depth <= self.max_depth);

        let mut detection_vol = boundary.clone();
        detection_vol.expand(self.padding);

        let mut tris = Vec::new();
        for (key, tri) in potential_tris {
            if tri.overlap(&detection_vol) {
                tris.push((*key, *tri));
            }
        }

        if tris.is_empty() {
            pb.block(8_u64.pow((self.max_depth - depth) as u32));
            return Tree::Empty { boundary };
        }

        if (tris.len() <= self.tar_tris) || (depth >= self.max_depth) {
            pb.block(8_u64.pow((self.max_depth - depth) as u32));
            return Tree::Leaf { boundary, tris };
        }

        let children = self.init_children(&boundary, depth + 1, &tris, &mut pb);

        Tree::Branch { boundary, children }
    }
}
