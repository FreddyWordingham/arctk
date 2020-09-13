//! Constructor methods.

use crate::{
    tree::{Cell, Settings},
    Aabb, Bar, Collide, Grp, Mesh, Pos3, Set, SmoothTriangle,
};

impl<'a> Cell<'a> {
    /// Construct a new tree root cell.
    /// Root cell has a depth of zero.
    #[inline]
    #[must_use]
    pub fn new_root(sett: &Settings, surfs: &'a Set<Mesh>) -> Self {
        let mut boundary = Self::init_boundary(surfs);
        boundary.expand(sett.padding());

        let mut tris = Vec::new();
        for (group, mesh) in surfs.map() {
            tris.reserve(mesh.tris().len());
            for tri in mesh.tris() {
                tris.push((group.as_str(), tri));
            }
        }

        let mut pb = Bar::new("Growing tree", 8_u64.pow(sett.max_depth() as u32));
        let children = Self::init_children(sett, &boundary, 1, tris.as_slice(), &mut pb);
        pb.finish_with_message("Tree grown.");

        Self::Root { boundary, children }
    }

    /// Initialise the boundary encompassing all of the mesh vertices.
    #[inline]
    #[must_use]
    fn init_boundary(surfs: &Set<Mesh>) -> Aabb {
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

        Aabb::new(mins.unwrap(), maxs.unwrap())
    }

    /// Initialise the children of a branching cell.
    #[allow(clippy::similar_names)]
    #[inline]
    #[must_use]
    fn init_children(
        sett: &Settings,
        parent_boundary: &Aabb,
        depth: i32,
        potential_tris: &[(&'a Grp, &'a SmoothTriangle)],
        mut pb: &mut Bar,
    ) -> [Box<Self>; 8] {
        debug_assert!(depth <= sett.max_depth());
        debug_assert!(!potential_tris.is_empty());

        let hws = parent_boundary.half_widths();
        let mut make_child = |min_x: f64, min_y: f64, min_z: f64| {
            let min = Pos3::new(min_x, min_y, min_z);
            Box::new(Self::init_child(
                sett,
                Aabb::new(min, min + hws),
                depth,
                potential_tris,
                &mut pb,
            ))
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
    fn init_child(
        sett: &Settings,
        boundary: Aabb,
        depth: i32,
        potential_tris: &[(&'a Grp, &'a SmoothTriangle)],
        mut pb: &mut Bar,
    ) -> Self {
        debug_assert!(depth <= sett.max_depth());

        let mut detection_vol = boundary.clone();
        detection_vol.expand(sett.padding());

        let mut tris = Vec::new();
        for (group, tri) in potential_tris {
            if tri.overlap(&detection_vol) {
                tris.push((*group, *tri));
            }
        }

        if tris.is_empty() {
            pb.block(8_u64.pow((sett.max_depth() - depth) as u32));
            return Self::Empty { boundary };
        }

        if (tris.len() <= sett.tar_tris()) || (depth >= sett.max_depth()) {
            pb.block(8_u64.pow((sett.max_depth() - depth) as u32));
            return Self::Leaf { boundary, tris };
        }

        let children = Self::init_children(sett, &boundary, depth + 1, &tris, &mut pb);

        Self::Branch { boundary, children }
    }
}
