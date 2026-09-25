use bon::Builder;

use capnp::Error;

#[cfg(feature = "ownership")]
use ownership::IntoOwned;

use serde::{Deserialize, Serialize};

use jiyu_core::id::Id;

use crate::{
    request_capnp::{candidate, connect, payload, replace, request, rotate, runtime},
    schema::{FromReader, Schema, ToBuilder},
    trigger::Trigger,
    version::Version,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Builder)]
#[cfg_attr(feature = "ownership", derive(IntoOwned))]
pub struct Request {
    #[builder(default, into)]
    pub version: Version,
    #[builder(default)]
    pub payload: Payload,
}

impl FromReader for Request {
    type Reader<'r>
        = request::Reader<'r>
    where
        Self: 'r;

    fn from_reader(reader: Self::Reader<'_>) -> Result<Self, Error> {
        let version = reader.get_version();

        let payload_reader = reader.get_payload()?;

        let payload = Payload::from_reader(payload_reader)?;

        let request = Self::builder().version(version).payload(payload).build();

        Ok(request)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "ownership", derive(IntoOwned))]
#[serde(rename_all = "snake_case")]
pub enum Payload {
    #[default]
    Ping,
    Shutdown,
    Runtime(Runtime),
    Rotate(Rotate),
}

impl FromReader for Payload {
    type Reader<'r>
        = payload::Reader<'r>
    where
        Self: 'r;

    fn from_reader(reader: Self::Reader<'_>) -> Result<Self, Error> {
        let which = reader.which()?;

        let payload = match which {
            payload::Ping(()) => Self::Ping,
            payload::Shutdown(()) => Self::Shutdown,
            payload::Runtime(result) => {
                let runtime_reader = result?;

                let runtime = Runtime::from_reader(runtime_reader)?;

                Self::Runtime(runtime)
            }
            payload::Rotate(result) => {
                let rotate_reader = result?;

                let rotate = Rotate::from_reader(rotate_reader)?;

                Self::Rotate(rotate)
            }
        };

        Ok(payload)
    }
}

impl ToBuilder for Payload {
    type Builder<'b>
        = payload::Builder<'b>
    where
        Self: 'b;

    fn to_builder(&self, mut builder: Self::Builder<'_>) -> Result<(), Error> {
        match self {
            Self::Ping => {
                builder.set_ping(());
            }
            Self::Shutdown => {
                builder.set_shutdown(());
            }
            Self::Runtime(runtime) => {
                let runtime_builder = builder.init_runtime();

                runtime.to_builder(runtime_builder)?;
            }
            Self::Rotate(rotate) => {
                let rotate_builder = builder.init_rotate();

                rotate.to_builder(rotate_builder)?;
            }
        }

        Ok(())
    }
}

impl Schema for Payload {
    type Owned<'s>
        = payload::Owned
    where
        Self: 's;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "ownership", derive(IntoOwned))]
#[serde(rename_all = "snake_case")]
pub enum Runtime {
    #[default]
    Status,
    Disconnect,
    Connect(Connect),
    Replace(Replace),
}

impl FromReader for Runtime {
    type Reader<'r>
        = runtime::Reader<'r>
    where
        Self: 'r;

    fn from_reader(reader: Self::Reader<'_>) -> Result<Self, Error> {
        let which = reader.which()?;

        let runtime = match which {
            runtime::Status(()) => Self::Status,
            runtime::Disconnect(()) => Self::Disconnect,
            runtime::Connect(result) => {
                let connect_reader = result?;

                let connect = Connect::from_reader(connect_reader)?;

                Self::Connect(connect)
            }
            runtime::Replace(result) => {
                let replace_reader = result?;

                let replace = Replace::from_reader(replace_reader)?;

                Self::Replace(replace)
            }
        };

        Ok(runtime)
    }
}

impl ToBuilder for Runtime {
    type Builder<'b>
        = runtime::Builder<'b>
    where
        Self: 'b;

    fn to_builder(&self, mut builder: Self::Builder<'_>) -> Result<(), Error> {
        match self {
            Self::Status => {
                builder.set_status(());
            }
            Self::Disconnect => {
                builder.set_disconnect(());
            }
            Self::Connect(connect) => {
                let connect_builder = builder.init_connect();

                connect.to_builder(connect_builder)?;
            }
            Self::Replace(replace) => {
                let replace_builder = builder.init_replace();

                replace.to_builder(replace_builder)?;
            }
        }

        Ok(())
    }
}

impl Schema for Runtime {
    type Owned<'s>
        = runtime::Owned
    where
        Self: 's;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Builder)]
#[cfg_attr(feature = "ownership", derive(IntoOwned))]
pub struct Connect {
    #[builder(into)]
    pub id: Id,
}

impl Connect {
    pub const fn id(&self) -> Id {
        self.id
    }
}

impl FromReader for Connect {
    type Reader<'r>
        = connect::Reader<'r>
    where
        Self: 'r;

    fn from_reader(reader: Self::Reader<'_>) -> Result<Self, Error> {
        let id = Id::new(reader.get_id());

        let connect = Self::builder().id(id).build();

        Ok(connect)
    }
}

impl ToBuilder for Connect {
    type Builder<'b>
        = connect::Builder<'b>
    where
        Self: 'b;

    fn to_builder(&self, mut builder: Self::Builder<'_>) -> Result<(), Error> {
        builder.set_id(self.id().get());

        Ok(())
    }
}

impl Schema for Connect {
    type Owned<'s>
        = connect::Owned
    where
        Self: 's;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Builder)]
#[cfg_attr(feature = "ownership", derive(IntoOwned))]
pub struct Replace {
    #[builder(default)]
    pub trigger: Trigger,
    #[builder(default, into)]
    pub candidate: Candidate,
}

impl Replace {
    pub const fn trigger(&self) -> Trigger {
        self.trigger
    }

    pub const fn candidate(&self) -> Candidate {
        self.candidate
    }
}

impl FromReader for Replace {
    type Reader<'r>
        = replace::Reader<'r>
    where
        Self: 'r;

    fn from_reader(reader: Self::Reader<'_>) -> Result<Self, Error> {
        let trigger_schema = reader.get_trigger()?;
        let candidate_reader = reader.get_candidate()?;

        let trigger = Trigger::from_schema(trigger_schema);

        let candidate = Candidate::from_reader(candidate_reader)?;

        let replace = Self::builder()
            .trigger(trigger)
            .candidate(candidate)
            .build();

        Ok(replace)
    }
}

impl ToBuilder for Replace {
    type Builder<'b>
        = replace::Builder<'b>
    where
        Self: 'b;

    fn to_builder(&self, mut builder: Self::Builder<'_>) -> Result<(), Error> {
        builder.set_trigger(self.trigger().into_schema());

        let candidate_builder = builder.init_candidate();

        self.candidate().to_builder(candidate_builder)?;

        Ok(())
    }
}

impl Schema for Replace {
    type Owned<'s>
        = replace::Owned
    where
        Self: 's;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "ownership", derive(IntoOwned))]
pub enum Candidate {
    #[default]
    Any,
    Id(Id),
}

impl From<Id> for Candidate {
    fn from(id: Id) -> Self {
        Self::Id(id)
    }
}

impl From<Option<Id>> for Candidate {
    fn from(option: Option<Id>) -> Self {
        Self::optional(option)
    }
}

impl Candidate {
    pub const fn is_any(self) -> bool {
        matches!(self, Self::Any)
    }

    pub const fn optional(option: Option<Id>) -> Self {
        match option {
            Some(id) => Self::Id(id),
            None => Self::Any,
        }
    }

    pub const fn id(self) -> Option<Id> {
        match self {
            Self::Any => None,
            Self::Id(id) => Some(id),
        }
    }
}

impl FromReader for Candidate {
    type Reader<'r>
        = candidate::Reader<'r>
    where
        Self: 'r;

    fn from_reader(reader: Self::Reader<'_>) -> Result<Self, Error> {
        let which = reader.which()?;

        let candidate = match which {
            candidate::Any(()) => Self::Any,
            candidate::Id(value) => Self::Id(Id::new(value)),
        };

        Ok(candidate)
    }
}

impl ToBuilder for Candidate {
    type Builder<'b> = candidate::Builder<'b>;

    fn to_builder(&self, mut builder: Self::Builder<'_>) -> Result<(), Error> {
        match self.id() {
            None => {
                builder.set_any(());
            }
            Some(id) => {
                builder.set_id(id.get());
            }
        }

        Ok(())
    }
}

impl Schema for Candidate {
    type Owned<'s>
        = candidate::Owned
    where
        Self: 's;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "ownership", derive(IntoOwned))]
#[serde(rename_all = "snake_case")]
pub enum Rotate {
    #[default]
    Status,
    Disable,
    Enable,
}

impl FromReader for Rotate {
    type Reader<'r>
        = rotate::Reader<'r>
    where
        Self: 'r;

    fn from_reader(reader: Self::Reader<'_>) -> Result<Self, Error> {
        let which = reader.which()?;

        let rotate = match which {
            rotate::Status(()) => Self::Status,
            rotate::Disable(()) => Self::Disable,
            rotate::Enable(()) => Self::Enable,
        };

        Ok(rotate)
    }
}

impl ToBuilder for Rotate {
    type Builder<'b>
        = rotate::Builder<'b>
    where
        Self: 'b;

    fn to_builder(&self, mut builder: Self::Builder<'_>) -> Result<(), Error> {
        match self {
            Self::Status => {
                builder.set_status(());
            }
            Self::Disable => {
                builder.set_disable(());
            }
            Self::Enable => {
                builder.set_enable(());
            }
        }

        Ok(())
    }
}

impl Schema for Rotate {
    type Owned<'s>
        = rotate::Owned
    where
        Self: 's;
}
