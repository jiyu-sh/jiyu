#[cfg(feature = "ownership")]
use ownership::IntoOwned;

use serde::{Deserialize, Serialize};

use crate::code_capnp::Code as Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "ownership", derive(IntoOwned))]
#[serde(rename_all = "snake_case")]
pub enum Code {
    #[default]
    Ok,
    Invalid,
    Error,
}

impl From<Value> for Code {
    fn from(value: Value) -> Self {
        Self::from_value(value)
    }
}

impl From<Code> for Value {
    fn from(code: Code) -> Value {
        code.into_value()
    }
}

impl Code {
    pub const fn from_value(value: Value) -> Self {
        match value {
            Value::Ok => Self::Ok,
            Value::Invalid => Self::Invalid,
            Value::Error => Self::Error,
        }
    }

    pub const fn into_value(self) -> Value {
        match self {
            Self::Ok => Value::Ok,
            Self::Invalid => Value::Invalid,
            Self::Error => Value::Error,
        }
    }
}
