//! Name type.

use serde::{Deserialize, Serialize};

/// Human-readable identifier type.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Name(String);
