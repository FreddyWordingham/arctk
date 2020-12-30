//! Adaptive tree cell scheme.

use crate::{
    fmt_report,
    geom::{Collide, Cube, Hit, Ray, Scan, SmoothTriangle, Surface, Trace, TreeSettings},
    math::Pos3,
    ord::Set,
    tools::ProgressBar,
};
use std::fmt::{Display, Error, Formatter};

/// Tree cell enumeration.
pub enum Tree<'a, T> {
    /// Branching cell.
    Branch {
        /// Boundary.
        boundary: Cube,
        /// Children.
        children: Box<[Tree<'a, T>; 8]>,
    },
    /// Terminal populated cell.
    Leaf {
        /// Boundary.
        boundary: Cube,
        /// Intersecting triangles and their corresponding mesh index.
        tris: Vec<(&'a SmoothTriangle, &'a T)>,
    },
}

impl<'a, T> Tree<'a, T> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(sett: &TreeSettings, surfs: &'a Set<Surface<T>>) -> Self {
        let mut boundary = Self::init_boundary(surfs);
        boundary.expand(sett.padding());

        let mut tris = Vec::new();
        for surf in surfs.values() {
            tris.reserve(surf.mesh().tris().len());
            for tri in surf.mesh().tris() {
                tris.push((tri, surf.attr()));
            }
        }

        let mut pb = ProgressBar::new("Growing tree", 8_usize.pow(sett.max_depth()));
        if (sett.max_depth() == 0) || (tris.len() <= sett.tar_tris()) {
            pb.finish_with_message("Tree grown.");
            return Self::Leaf { boundary, tris };
        }

        let children = Box::new(Self::init_children(
            &mut pb,
            sett,
            &boundary,
            1,
            tris.as_slice(),
        ));
        pb.finish_with_message("Tree grown.");

        Self::Branch { boundary, children }
    }

    /// Initialise the boundary encompassing all of the mesh vertices.
    #[inline]
    #[must_use]
    fn init_boundary(surfs: &Set<Surface<T>>) -> Cube {
        let mut mins = None;
        let mut maxs = None;

        for surf in surfs.values() {
            let (mesh_mins, mesh_maxs) = surf.mesh().boundary().mins_maxs();

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
    fn init_children(
        mut pb: &mut ProgressBar,
        sett: &TreeSettings,
        parent_boundary: &Cube,
        depth: u32,
        potential_tris: &[(&'a SmoothTriangle, &'a T)],
    ) -> [Self; 8] {
        debug_assert!(depth <= sett.max_depth());
        debug_assert!(!potential_tris.is_empty());

        let hws = parent_boundary.half_widths();
        let mut make_child = |min_x: f64, min_y: f64, min_z: f64| {
            let min = Pos3::new(min_x, min_y, min_z);
            Self::init_child(
                &mut pb,
                sett,
                Cube::new(min, min + hws),
                depth,
                potential_tris,
            )
        };

        let min = parent_boundary.mins();

        let nnn = make_child(min.x, min.y, min.z);
        let pnn = make_child(min.x + hws.x, min.y, min.z);
        let npn = make_child(min.x, min.y + hws.y, min.z);
        let ppn = make_child(min.x + hws.x, min.y + hws.y, min.z);
        let nnp = make_child(min.x, min.y, min.z + hws.z);
        let pnp = make_child(min.x + hws.x, min.y, min.z + hws.z);
        let npp = make_child(min.x, min.y + hws.y, min.z + hws.z);
        let ppp = make_child(min.x + hws.x, min.y + hws.y, min.z + hws.z);

        [nnn, pnn, npn, ppn, nnp, pnp, npp, ppp]
    }

    /// Initialise a child cell.
    #[inline]
    #[must_use]
    fn init_child(
        mut pb: &mut ProgressBar,
        sett: &TreeSettings,
        boundary: Cube,
        depth: u32,
        potential_tris: &[(&'a SmoothTriangle, &'a T)],
    ) -> Tree<'a, T> {
        debug_assert!(depth <= sett.max_depth());

        let mut detection_vol = boundary.clone();
        detection_vol.expand(sett.padding());

        let mut tris = Vec::new();
        for &(tri, attr) in potential_tris {
            if tri.overlap(&detection_vol) {
                tris.push((tri, attr));
            }
        }

        if (tris.len() <= sett.tar_tris()) || (depth >= sett.max_depth()) {
            pb.block(8_usize.pow(sett.max_depth() - depth));
            return Tree::Leaf { boundary, tris };
        }

        let children = Box::new(Self::init_children(
            &mut pb,
            sett,
            &boundary,
            depth + 1,
            &tris,
        ));

        Tree::Branch { boundary, children }
    }

    /// Reference the cell's boundary.
    #[allow(clippy::missing_const_for_fn)]
    #[inline]
    #[must_use]
    pub fn boundary(&self) -> &Cube {
        match *self {
            Self::Branch { ref boundary, .. } | Self::Leaf { ref boundary, .. } => boundary,
        }
    }

    /// Determine the total number of cells used by this cell.
    /// This cell is included in the count.
    #[inline]
    #[must_use]
    pub fn num_cells(&self) -> usize {
        match *self {
            Self::Branch { ref children, .. } => {
                1 + children.iter().map(Self::num_cells).sum::<usize>()
            }
            Self::Leaf { .. } => 1,
        }
    }

    /// Determine the number leaf of cells contained used by this cell.
    /// This cell is potentially included in the count.
    #[inline]
    #[must_use]
    pub fn num_leaves(&self) -> usize {
        match *self {
            Self::Branch { ref children, .. } => {
                children.iter().map(Self::num_leaves).sum::<usize>()
            }
            Self::Leaf { .. } => 1,
        }
    }

    /// Determine the number of triangle collision references used by this cell.
    #[inline]
    #[must_use]
    pub fn num_tris(&self) -> usize {
        match *self {
            Self::Branch { ref children, .. } => children.iter().map(Self::num_tris).sum(),
            Self::Leaf { ref tris, .. } => tris.len(),
        }
    }

    /// Determine the maximum depth from this cell to a terminal cell.
    #[inline]
    #[must_use]
    pub fn depth(&self) -> usize {
        match *self {
            Self::Branch { ref children, .. } => {
                1 + children.iter().map(Self::depth).max().unwrap()
            }
            Self::Leaf { .. } => 1,
        }
    }

    /// If a given position is contained within the cell to being with,
    /// determine the terminal leaf cell containing the given position.
    #[inline]
    #[must_use]
    pub fn try_find_leaf(&self, pos: &Pos3) -> Option<&Self> {
        if !self.boundary().contains(pos) {
            return None;
        }

        Some(self.find_leaf(pos))
    }

    /// Determine the terminal leaf cell containing the given position.
    #[must_use]
    #[inline]
    pub fn find_leaf(&self, pos: &Pos3) -> &Self {
        debug_assert!(self.boundary().contains(pos));

        match *self {
            Self::Leaf { .. } => self,
            Self::Branch {
                ref boundary,
                ref children,
            } => {
                let mut index = 0;
                let c = boundary.centre();

                if pos.x >= c.x {
                    index += 1;
                }
                if pos.y >= c.y {
                    index += 2;
                }
                if pos.z >= c.z {
                    index += 4;
                }
                children[index].find_leaf(pos)
            }
        }
    }

    /// Scan for what a given Ray, known to be within the cell, would observe.
    #[inline]
    #[must_use]
    fn leaf_scan(&self, ray: &Ray, bump_dist: f64) -> Scan<T> {
        debug_assert!(self.boundary().contains(ray.pos()));
        debug_assert!(bump_dist > 0.0);

        match *self {
            Self::Branch { .. } => {
                panic!("Should not be performing hit scans on branching cells!");
            }
            Self::Leaf {
                ref boundary,
                ref tris,
            } => {
                let boundary_dist = boundary.dist(ray).unwrap();
                if tris.is_empty() {
                    return Scan::new_boundary(boundary_dist);
                }

                let mut nearest: Option<Hit<T>> = None;
                for &(tri, attr) in tris {
                    if let Some((dist, side)) = tri.dist_side(ray) {
                        if let Some(ref hit) = nearest {
                            if dist < hit.dist() {
                                nearest = Some(Hit::new(attr, dist, side));
                            }
                        } else {
                            nearest = Some(Hit::new(attr, dist, side));
                        }
                    }
                }

                if let Some(hit) = nearest {
                    if hit.dist() < (boundary_dist + bump_dist) {
                        return Scan::new_surface(hit);
                    }
                }

                Scan::new_boundary(boundary_dist)
            }
        }
    }

    /// Determine what a given Ray would observe.
    /// The maximum distance provided does not guarantee that any hit retrieved is less than the given distance.
    #[inline]
    #[must_use]
    pub fn scan(&self, mut ray: Ray, bump_dist: f64, max_dist: f64) -> Option<Hit<T>> {
        debug_assert!(bump_dist > 0.0);
        debug_assert!(max_dist > 0.0);

        let mut dist_travelled = 0.0;

        // Move the ray to within the domain of the cell if it isn't already within it.
        if !self.boundary().contains(ray.pos()) {
            if let Some(dist) = self.boundary().dist(&ray) {
                let d = dist + bump_dist;
                ray.travel(d);
                dist_travelled += d;
            } else {
                return None;
            }
        }

        while let Some(cell) = self.try_find_leaf(ray.pos()) {
            if dist_travelled > max_dist {
                return None;
            }

            match cell.leaf_scan(&ray, bump_dist) {
                Scan::Surface(mut hit) => {
                    *hit.dist_mut() += dist_travelled;
                    return Some(hit);
                }
                Scan::Boundary(dist) => {
                    let d = dist + bump_dist;
                    ray.travel(d);
                    dist_travelled += d;
                }
            }
        }

        None
    }
}

impl<T> Display for Tree<'_, T> {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.boundary(), "boundary");
        fmt_report!(fmt, self.num_cells(), "total cells");
        fmt_report!(
            fmt,
            &format!(
                "{} ({:.2}%)",
                self.num_leaves(),
                self.num_leaves() as f64 / self.num_cells() as f64 * 100.0
            ),
            "leaf cells"
        );
        fmt_report!(fmt, self.num_tris(), "Triangle references");
        fmt_report!(fmt, self.depth(), "maximum depth");
        Ok(())
    }
}
