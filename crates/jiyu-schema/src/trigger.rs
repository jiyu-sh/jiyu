#[cfg(feature = "ownership")]
use ownership::IntoOwned;

use serde::{Deserialize, Serialize};

use crate::{schema::Primitive, trigger_capnp::Trigger as Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "ownership", derive(IntoOwned))]
#[serde(rename_all = "snake_case")]
pub enum Trigger {
    #[default]
    Manual,
    Timer,
    Fail,
}

impl From<Value> for Trigger {
    fn from(value: Value) -> Self {
        Self::from_value(value)
    }
}

impl From<Trigger> for Value {
    fn from(trigger: Trigger) -> Value {
        trigger.into_value()
    }
}

impl Primitive for Trigger {
    type Value = Value;
}

impl Trigger {
    pub const fn from_value(value: Value) -> Self {
        match value {
            Value::Manual => Self::Manual,
            Value::Timer => Self::Timer,
            Value::Fail => Self::Fail,
        }
    }

    pub const fn into_value(self) -> Value {
        match self {
            Self::Manual => Value::Manual,
            Self::Timer => Value::Timer,
            Self::Fail => Value::Fail,
        }
    }
}
