//! Technical configuration.

use serde::Deserialize;

/// Technical settings.
#[derive(Clone, Deserialize)]
pub struct Settings {
    /// Number of tiles resolution.
    pub tiles: [usize; 2],
    /// Bump distance (m).
    pub bump_dist: f64,
    /// Loop limit.
    pub loop_limit: u64,
    /// Minimum statistical weight to continue simulating.
    pub min_weight: f64,
    /// Maximum ray travel distance.
    pub max_distance: f64,
}
