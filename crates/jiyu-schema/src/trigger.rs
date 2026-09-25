#[cfg(feature = "ownership")]
use ownership::IntoOwned;

use serde::{Deserialize, Serialize};

use crate::trigger_capnp::Trigger as TriggerSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "ownership", derive(IntoOwned))]
#[serde(rename_all = "snake_case")]
pub enum Trigger {
    #[default]
    Manual,
    Timer,
    Fail,
}

impl Trigger {
    pub const fn from_schema(schema: TriggerSchema) -> Self {
        match schema {
            TriggerSchema::Manual => Self::Manual,
            TriggerSchema::Timer => Self::Timer,
            TriggerSchema::Fail => Self::Fail,
        }
    }

    pub const fn into_schema(self) -> TriggerSchema {
        match self {
            Self::Manual => TriggerSchema::Manual,
            Self::Timer => TriggerSchema::Timer,
            Self::Fail => TriggerSchema::Fail,
        }
    }
}

impl From<TriggerSchema> for Trigger {
    fn from(schema: TriggerSchema) -> Self {
        Self::from_schema(schema)
    }
}

impl From<Trigger> for TriggerSchema {
    fn from(trigger: Trigger) -> Self {
        trigger.into_schema()
    }
}
