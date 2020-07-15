//! Illumination functions.

use crate::{render::Scene, Crossing, Dir3, Hit, Ray};

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
