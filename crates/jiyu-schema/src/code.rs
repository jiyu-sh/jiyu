#[cfg(feature = "ownership")]
use ownership::IntoOwned;

use serde::{Deserialize, Serialize};

use crate::code_capnp::Code as CodeSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "ownership", derive(IntoOwned))]
#[serde(rename_all = "snake_case")]
pub enum Code {
    #[default]
    Ok,
    Invalid,
    Error,
}

impl Code {
    pub const fn from_schema(schema: CodeSchema) -> Self {
        match schema {
            CodeSchema::Ok => Self::Ok,
            CodeSchema::Invalid => Self::Invalid,
            CodeSchema::Error => Self::Error,
        }
    }

    pub const fn into_schema(self) -> CodeSchema {
        match self {
            Self::Ok => CodeSchema::Ok,
            Self::Invalid => CodeSchema::Invalid,
            Self::Error => CodeSchema::Error,
        }
    }
}

impl From<CodeSchema> for Code {
    fn from(schema: CodeSchema) -> Self {
        Self::from_schema(schema)
    }
}

impl From<Code> for CodeSchema {
    fn from(code: Code) -> Self {
        code.into_schema()
    }
}
