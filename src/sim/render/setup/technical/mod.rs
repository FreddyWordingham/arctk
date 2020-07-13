//! Technical settings structure.

pub mod order;
pub mod settings;

pub use self::{order::*, settings::*};

use crate::{access, form::Engine};
use attr::load;

/// Loadable technical settings structure.
#[load]
pub struct Technical {
    /// Engine selection.
    engine: Engine,
    /// Rendering order.
    order: Order,
    /// Technical settings.
    sett: Settings,
}

impl Technical {
    access!(engine, Engine);
    access!(order, Order);
    access!(sett, Settings);
}
