//! Observation methods.

use crate::{
    tree::{Cell, Scan},
    Hit, Ray, Trace,
};

impl<'a> Cell<'a> {
    /// Determine what a ray would observe within the cell.
    #[inline]
    #[must_use]
    pub fn observe(&self, mut ray: Ray, bump_dist: f64, max_dist: f64) -> Option<Hit> {
        debug_assert!(bump_dist > 0.0);
        debug_assert!(max_dist > 0.0);

        let mut dist_travelled = 0.0;

        // Move the ray to within the domain of the tree if it isn't already within it.
        if !self.boundary().contains(ray.pos()) {
            if let Some(dist) = self.boundary().dist(&ray) {
                let d = dist + bump_dist;
                ray.travel(d);
                dist_travelled += d;
            } else {
                return None;
            }
        }

        while let Some(cell) = self.find_terminal_cell(ray.pos()) {
            if dist_travelled > max_dist {
                return None;
            }

            match cell.hit_scan(&ray, bump_dist) {
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

    /// Scan for hits within the cell.
    #[inline]
    #[must_use]
    pub fn hit_scan(&self, ray: &Ray, bump_dist: f64) -> Scan {
        debug_assert!(self.boundary().contains(ray.pos()));
        debug_assert!(bump_dist > 0.0);

        match self {
            Self::Leaf { boundary, tris } => {
                let mut nearest: Option<Hit> = None;
                for (group, tri) in tris {
                    if let Some((dist, side)) = tri.dist_side(ray) {
                        if nearest.is_none() || (dist < nearest.as_ref().unwrap().dist()) {
                            nearest = Some(Hit::new(*group, dist, side));
                        }
                    }
                }

                let boundary_dist = boundary.dist(ray).unwrap();
                if let Some(hit) = nearest {
                    if hit.dist() < (boundary_dist + bump_dist) {
                        return Scan::new_surface(hit);
                    }
                }

                Scan::new_boundary(boundary_dist)
            }
            Self::Empty { boundary } => Scan::new_boundary(boundary.dist(ray).unwrap()),
            Self::Root { .. } | Self::Branch { .. } => {
                panic!("Should not be performing hit scans on branching cells!");
            }
        }
    }
}
