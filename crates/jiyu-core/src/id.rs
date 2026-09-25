use std::fmt;

#[cfg(feature = "ownership")]
use ownership::IntoOwned;

use serde::{Deserialize, Serialize};

pub type Value = u64;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
#[cfg_attr(feature = "ownership", derive(IntoOwned))]
#[serde(transparent)]
#[repr(transparent)]
pub struct Id {
    value: Value,
}

impl From<Value> for Id {
    fn from(value: Value) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for Id {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{value:x}", value = self.get())
    }
}

impl Id {
    pub const fn new(value: Value) -> Self {
        Self { value }
    }

    pub const fn get(self) -> Value {
        self.value
    }
}
