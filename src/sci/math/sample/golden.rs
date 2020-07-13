//! Golden ratio sampling functions.

use lazy_static::lazy_static;
use std::f64::consts::{FRAC_PI_2, PI};

lazy_static! {
    /// Golden ratio constant.
    static ref GOLDEN_RATIO: f64 = (1.0 + 5.0_f64.sqrt()) / 2.0;
}

/// Sample points within a circle using the golden ratio.
#[inline]
#[must_use]
pub fn circle(n: i32, max: i32) -> (f64, f64) {
    debug_assert!(n >= 0);
    debug_assert!(n < max);

    let r = f64::from(n) / f64::from(max - 1);
    let theta = f64::from(n) * *GOLDEN_RATIO;

    (r, theta)
}

/// Sample points on a sphere's surface using the golden ratio.
#[inline]
#[must_use]
pub fn sphere(n: i32, max: i32) -> (f64, f64) {
    debug_assert!(n >= 0);
    debug_assert!(n < max);

    let d = f64::from(1 - max).mul_add(0.5, f64::from(n));
    let phi = ((2.0 * d) / f64::from(max)).asin() + FRAC_PI_2;
    let theta = ((2.0 * PI) / *GOLDEN_RATIO) * (d % *GOLDEN_RATIO);

    (phi, theta)
}

/// Sample points on a hemisphere's surface using the golden ratio.
#[inline]
#[must_use]
pub fn hemisphere(n: i32, max: i32) -> (f64, f64) {
    debug_assert!(n >= 0);
    debug_assert!(n < max);

    sphere(n, max * 2)
}
