#[cfg(feature = "ownership")]
use ownership::IntoOwned;

use serde::{Deserialize, Serialize};

pub type Value = u8;

pub const CURRENT: Value = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "ownership", derive(IntoOwned))]
#[serde(transparent)]
#[repr(transparent)]
pub struct Version {
    value: Value,
}

impl Default for Version {
    fn default() -> Self {
        Self::CURRENT
    }
}

impl From<Value> for Version {
    fn from(value: Value) -> Self {
        Self::new(value)
    }
}

impl Version {
    pub const fn new(value: Value) -> Self {
        Self { value }
    }

    pub const fn get(self) -> Value {
        self.value
    }

    pub const CURRENT: Self = Self::new(CURRENT);
}
