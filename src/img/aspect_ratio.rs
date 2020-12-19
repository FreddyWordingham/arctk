//! Formula implementation.

use arctk_attr::file;
use std::fmt::{Display, Formatter, Result};

/// Aspect-ratio enumeration.
#[file]
#[derive(Clone)]
pub enum AspectRatio {
    /// Square. 1:1.
    Square,
    /// Classic photographic film. 3:2.
    Classic,
    /// Golden ratio. ((1+sqrt(5))/2):1
    Golden,
    /// Silver ratio. (1+sqrt(2)):1
    Silver,
    /// Standard. 16:9
    Standard,
    /// Widescreen. 43:18
    Widescreen,
    /// IPhone XS. (1125 x 2436)
    IPhoneXS,
    /// IPhone 7. (750 x 1334)
    IPhone7,
    /// Custom. (w x h)
    Custom(f64, f64),
}

impl AspectRatio {
    /// Get the pixel ratios.
    #[inline]
    #[must_use]
    pub fn ratio(&self) -> f64 {
        match *self {
            Self::Square => 1.0,
            Self::Classic => 3.0 / 2.0,
            Self::Golden => (1.0 + 5.0_f64.sqrt()) / 2.0,
            Self::Silver => 1.0 + 2.0_f64.sqrt(),
            Self::Standard => 16.0 / 9.0,
            Self::Widescreen => 43.0 / 18.0,
            Self::IPhoneXS => 1125.0 / 2436.0,
            Self::IPhone7 => 750.0 / 1334.0,
            Self::Custom(w, h) => w / h,
        }
    }

    /// Determine a resolution for the target number of pixels.
    /// Resulting resolution target will contain at least this many pixels.
    #[inline]
    #[must_use]
    pub fn resolution(&self, total_target: u64, mult: (u64, u64)) -> (u64, u64) {
        debug_assert!(total_target > 0);

        let fx = (total_target as f64 * self.ratio()).sqrt().ceil() as u64;
        let fy = (total_target as f64 / self.ratio()).sqrt().ceil() as u64;

        // Round up to nearest multiple if required.
        let mx = if fx % mult.0 == 0 {
            fx
        } else {
            fx + (mult.0 - (fx % mult.0))
        };
        let my = if fy % mult.1 == 0 {
            fy
        } else {
            fy + (mult.1 - (fx % mult.1))
        };

        (mx, my)
    }

    /// Calculate the vertical resolution for a given horizontal resolution.
    /// Rounded up to the nearest pixel.
    #[inline]
    #[must_use]
    pub fn vt_res(&self, hr_res: u64) -> u64 {
        debug_assert!(hr_res > 0);

        (hr_res as f64 / self.ratio()).ceil() as u64
    }
}

impl Display for AspectRatio {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        match *self {
            Self::Square { .. } => write!(fmt, "Square"),
            Self::Classic { .. } => write!(fmt, "Classic"),
            Self::Golden { .. } => write!(fmt, "Golden"),
            Self::Silver { .. } => write!(fmt, "Silver"),
            Self::Standard { .. } => write!(fmt, "Standard"),
            Self::Widescreen { .. } => write!(fmt, "Widescreen"),
            Self::IPhoneXS { .. } => write!(fmt, "IPhoneXS"),
            Self::IPhone7 { .. } => write!(fmt, "IPhone7"),
            Self::Custom(w, h) => write!(fmt, "Custom [{}:{}]", w, h),
        }
    }
}
