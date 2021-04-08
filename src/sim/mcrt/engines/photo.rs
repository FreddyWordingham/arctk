// Photo imaging photon-lifetime engine function.

use crate::{
    geom::Trace,
    phys::{Local, Photon},
    sim::mcrt::{scatter::scatter, surface::surface, travel::travel, Event, Input, Output},
};
use rand::{rngs::ThreadRng, Rng};

/// Photograph the life of a single photon.
#[allow(clippy::expect_used)]
#[inline]
pub fn fluorophore(
    input: &Input,
    mut rng: &mut ThreadRng,
    mut phot: Photon,
    mut data: &mut Output,
) {
}
