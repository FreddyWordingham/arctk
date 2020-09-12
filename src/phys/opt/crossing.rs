//! Crossing implementation.

use crate::{access, clone};

// TODO: Replace with math module.
use nalgebra::{Unit, Vector3};
pub type Dir3 = Unit<Vector3<f64>>;

/// Optical interface crossing information.
/// Calculates trajectory information.
pub struct Crossing {
    /// Probability of reflection.
    ref_prob: f64,
    /// Reflection direction.
    ref_dir: Dir3,
    /// Transmission (refraction) direction.
    trans_dir: Option<Dir3>,
}

impl Crossing {
    clone!(ref_prob, f64);
    access!(ref_dir, Dir3);
    access!(trans_dir, Option<Dir3>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(inc: &Dir3, norm: &Dir3, n_curr: f64, n_next: f64) -> Self {
        debug_assert!(n_curr >= 1.0);
        debug_assert!(n_next >= 1.0);
        debug_assert!(inc.dot(norm) < 0.0);

        let ci = -inc.dot(norm);
        let n = n_curr / n_next;

        let crit_ang = if n_curr <= n_next {
            None
        } else {
            Some((n_next / n_curr).asin())
        };

        let (ref_prob, trans_dir) = if crit_ang.is_some() && (ci.acos() >= crit_ang.unwrap()) {
            (1.0, None)
        } else {
            let s2t = n.powi(2) * (1.0 - ci.powi(2));
            let ct = (1.0 - s2t).sqrt();

            (
                Self::init_ref_prob(n_curr, n_next, ci, ct),
                Some(Self::init_trans_dir(inc, norm, n, ci, ct)),
            )
        };

        Self {
            ref_prob,
            ref_dir: Self::init_ref_dir(inc, norm, ci),
            trans_dir,
        }
    }

    /// Calculate the reflection probability.
    #[inline]
    #[must_use]
    fn init_ref_prob(n1: f64, n2: f64, ci: f64, ct: f64) -> f64 {
        debug_assert!(n1 >= 0.0);
        debug_assert!(n2 >= 0.0);

        let n1_c_i = n1 * ci;
        let n2_c_t = n2 * ct;
        let rn = ((n1_c_i - n2_c_t) / (n1_c_i + n2_c_t)).powi(2);

        let n2_c_i = n2 * ci;
        let n1_c_t = n1 * ct;
        let rt = ((n2_c_i - n1_c_t) / (n2_c_i + n1_c_t)).powi(2);

        (rn + rt) / 2.0
    }

    /// Calculate the reflection direction.
    #[inline]
    #[must_use]
    fn init_ref_dir(inc: &Dir3, norm: &Dir3, ci: f64) -> Dir3 {
        Dir3::new_unchecked(inc.into_inner() + (2.0 * ci * norm.into_inner()))
    }

    /// Calculate the reflection direction and the required constant.
    #[inline]
    #[must_use]
    pub fn calc_ref_dir(inc: &Dir3, norm: &Dir3) -> Dir3 {
        Self::init_ref_dir(inc, norm, -inc.dot(norm))
    }

    /// Calculate the transmission direction.
    #[inline]
    #[must_use]
    fn init_trans_dir(inc: &Dir3, norm: &Dir3, n: f64, ci: f64, ct: f64) -> Dir3 {
        Dir3::new_unchecked((n * inc.into_inner()) + ((n * ci) - ct) * norm.into_inner())
    }

    /// Get the transmission probability.
    #[inline]
    #[must_use]
    pub fn trans_prob(&self) -> f64 {
        1.0 - self.ref_prob
    }
}
