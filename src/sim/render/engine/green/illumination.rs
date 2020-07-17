//! Illumination functions.

use crate::{
    // golden,
    render::{
        // Attributes,
        Input,
        Scene,
    },
    Crossing,
    Dir3,
    Hit,
    Ray,
};
use rand::rngs::ThreadRng;
// use std::f64::consts::PI;

// /// Maximum distance tested for ray visibility [m].
// const MAX_VISIBILITY_DIST: f64 = 1.0e9;

/// Calculate the lighting factor.
#[inline]
#[must_use]
pub fn light(scene: &Scene, ray: &Ray, hit: &Hit) -> f64 {
    let light_dir = Dir3::new_normalize(scene.lighting().sky().sun_pos() - ray.pos());
    let view_dir = Dir3::new_normalize(scene.cam().focus().orient().pos() - ray.pos());
    let ref_dir = Crossing::calc_ref_dir(ray.dir(), hit.side().norm());

    let mut ambient = 1.0;
    let mut diffuse = hit.side().norm().dot(&light_dir).max(0.0);
    let mut specular = view_dir
        .dot(&ref_dir)
        .max(0.0)
        .powi(scene.lighting().light().spec_pow());

    ambient *= scene.lighting().light().ambient();
    diffuse *= scene.lighting().light().diffuse();
    specular *= scene.lighting().light().specular();

    ambient + diffuse + specular
}

/// Calculate the shadowing factor.
#[inline]
#[must_use]
pub fn shadow(
    _input: &Input,
    _scene: &Scene,
    _ray: &Ray,
    _hit: &Hit,
    _bump_dist: f64,
    _rng: &mut ThreadRng,
) -> f64 {
    1.0
}
