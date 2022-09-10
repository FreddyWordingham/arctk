//! Adaptive tree cell scheme.

use nalgebra::Point3;

use crate::{
    dom::{Surface, TreeBuilder},
    geom::{Cube, Triangle},
    rt::{Hit, Ray, Scan},
};

/// Tree cell enumeration.
#[allow(clippy::exhaustive_enums)]
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
        tris: Vec<(&'a Triangle, &'a T)>,
    },
}

impl<'a, T> Tree<'a, T> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(sett: &TreeBuilder, surfs: &'a [Surface<T>]) -> Self {
        let mut boundary = Self::init_boundary(surfs);
        boundary.expand(sett.padding);

        let mut tris = Vec::new();
        for surf in surfs {
            tris.reserve(surf.mesh.tris.len());
            for tri in &surf.mesh.tris {
                tris.push((tri, surf.attr));
            }
        }

        if (sett.max_depth == 0) || (tris.len() <= sett.tar_tris) {
            return Self::Leaf { boundary, tris };
        }

        let children = Box::new(Self::init_children(sett, &boundary, 1, tris.as_slice()));

        Self::Branch { boundary, children }
    }

    /// Initialise the boundary encompassing all of the mesh vertices.
    #[inline]
    #[must_use]
    fn init_boundary(surfs: &[Surface<T>]) -> Cube {
        let mut mins = None;
        let mut maxs = None;

        for surf in surfs {
            let mesh_mins = surf.mesh.boundary.mins;
            let mesh_maxs = surf.mesh.boundary.maxs;

            if mins.is_none() {
                mins = Some(mesh_mins);
            } else {
                for (grid_min, mesh_min) in mins
                    .as_mut()
                    .expect("Failed to initialise Tree boundary.")
                    .iter_mut()
                    .zip(mesh_mins.iter())
                {
                    if mesh_min < grid_min {
                        *grid_min = *mesh_min;
                    }
                }
            }

            if maxs.is_none() {
                maxs = Some(mesh_maxs);
            } else {
                for (grid_max, mesh_max) in maxs
                    .as_mut()
                    .expect("Failed to initialise Tree boundary.")
                    .iter_mut()
                    .zip(mesh_maxs.iter())
                {
                    if mesh_max > grid_max {
                        *grid_max = *mesh_max;
                    }
                }
            }
        }

        Cube::new(
            mins.expect("Failed to initialise Tree boundary."),
            maxs.expect("Failed to initialise Tree boundary."),
        )
    }

    /// Initialise the children of a branching cell.
    #[allow(clippy::similar_names)]
    #[inline]
    #[must_use]
    fn init_children(
        sett: &TreeBuilder,
        parent_boundary: &Cube,
        depth: u32,
        potential_tris: &[(&'a Triangle, &'a T)],
    ) -> [Self; 8] {
        debug_assert!(depth <= sett.max_depth);
        debug_assert!(!potential_tris.is_empty());

        let hws = parent_boundary.half_widths();
        let make_child = |min_x: f64, min_y: f64, min_z: f64| {
            let min = Point3::new(min_x, min_y, min_z);
            Self::init_child(sett, Cube::new(min, min + hws), depth, potential_tris)
        };

        let min = parent_boundary.mins;

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
        sett: &TreeBuilder,
        boundary: Cube,
        depth: u32,
        potential_tris: &[(&'a Triangle, &'a T)],
    ) -> Tree<'a, T> {
        debug_assert!(depth <= sett.max_depth);

        let mut detection_vol = boundary.clone();
        detection_vol.expand(sett.padding);

        let mut tris = Vec::new();
        for &(tri, attr) in potential_tris {
            if tri.collides(&detection_vol) {
                tris.push((tri, attr));
            }
        }

        if (tris.len() <= sett.tar_tris) || (depth >= sett.max_depth) {
            return Tree::Leaf { boundary, tris };
        }

        let children = Box::new(Self::init_children(sett, &boundary, depth + 1, &tris));

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

    /// If a given position is contained within the cell to being with,
    /// determine the terminal leaf cell containing the given position.
    #[inline]
    #[must_use]
    pub fn try_find_leaf(&self, pos: &Point3<f64>) -> Option<&Self> {
        if !self.boundary().contains(pos) {
            return None;
        }

        Some(self.find_leaf(pos))
    }

    /// Determine the terminal leaf cell containing the given position.
    #[must_use]
    #[inline]
    pub fn find_leaf(&self, pos: &Point3<f64>) -> &Self {
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
        debug_assert!(self.boundary().contains(&ray.pos));
        debug_assert!(bump_dist > 0.0);

        match *self {
            Self::Branch { .. } => {
                panic!("Should not be performing hit scans on branching cells!");
            }
            Self::Leaf {
                ref boundary,
                ref tris,
            } => {
                let boundary_dist = boundary
                    .dist(ray)
                    .expect("Failed to perform Ray-Tree intersection.");
                if tris.is_empty() {
                    return Scan::new_boundary(boundary_dist);
                }

                let mut nearest: Option<Hit<T>> = None;
                for &(tri, attr) in tris {
                    if let Some((dist, side)) = tri.dist_side(ray) {
                        if let Some(ref hit) = nearest {
                            if dist < hit.dist {
                                nearest = Some(Hit::new(attr, dist, side));
                            }
                        } else {
                            nearest = Some(Hit::new(attr, dist, side));
                        }
                    }
                }

                if let Some(hit) = nearest {
                    if hit.dist < (boundary_dist + bump_dist) {
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

        let mut dist_traveled = 0.0;

        // Move the ray to within the domain of the cell if it isn't already within it.
        if !self.boundary().contains(&ray.pos) {
            if let Some(dist) = self.boundary().dist(&ray) {
                let d = dist + bump_dist;
                ray.travel(d);
                dist_traveled += d;
            } else {
                return None;
            }
        }

        while let Some(cell) = self.try_find_leaf(&ray.pos) {
            if dist_traveled > max_dist {
                return None;
            }

            match cell.leaf_scan(&ray, bump_dist) {
                Scan::Surface(mut hit) => {
                    hit.dist += dist_traveled;
                    return Some(hit);
                }
                Scan::Boundary(dist) => {
                    let d = dist + bump_dist;
                    ray.travel(d);
                    dist_traveled += d;
                }
            }
        }

        None
    }
}
