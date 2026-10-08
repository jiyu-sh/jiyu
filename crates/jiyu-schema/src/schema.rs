use capnp::{
    Result,
    io::{BufRead, Read, Write},
    message::{
        Allocator, Builder as MessageBuilder, HeapAllocator, Reader as MessageReader, ReaderOptions,
    },
    serialize::{OwnedSegments, read_message, write_message},
    serialize_packed::{
        read_message as read_message_packed, write_message as write_message_packed,
    },
    traits::Owned,
};

use trait_aliases::trait_aliases;

use crate::by::By;

pub type Value<P> = <P as Primitive>::Value;

trait_aliases! {
    #[trait_alias(T)]
    pub trait Convertible<U> = From<U> + Into<U>;
}

pub trait Primitive: Convertible<Self::Value> {
    type Value;
}

pub trait Core {
    type Owned: Owned;
}

pub type Reader<'r, C> = <<C as Core>::Owned as Owned>::Reader<'r>;
pub type Builder<'b, C> = <<C as Core>::Owned as Owned>::Builder<'b>;

pub trait FromReader: Core + Sized {
    fn from_reader(reader: Reader<'_, Self>) -> Result<Self>;
}

pub trait ToBuilder: Core {
    fn to_builder(&self, builder: Builder<'_, Self>) -> Result<()>;
}

pub type OwnedMessage = MessageReader<OwnedSegments>;

pub trait Decode: FromReader {
    fn decode<R: Read>(read: R) -> Result<Self> {
        Self::decode_with(read, ReaderOptions::new())
    }

    fn decode_with<R: Read>(read: R, options: ReaderOptions) -> Result<Self> {
        read_message(read, options).and_then(Self::decode_message)
    }

    fn decode_packed<B: BufRead>(buffered: B) -> Result<Self> {
        Self::decode_packed_with(buffered, ReaderOptions::new())
    }

    fn decode_packed_with<B: BufRead>(buffered: B, options: ReaderOptions) -> Result<Self> {
        read_message_packed(buffered, options).and_then(Self::decode_message)
    }

    fn decode_message(message: OwnedMessage) -> Result<Self> {
        let reader = message.get_root()?;

        let decoded = Self::from_reader(reader)?;

        Ok(decoded)
    }
}

pub type MessageWith<A> = MessageBuilder<A>;
pub type Message = MessageWith<HeapAllocator>;

pub trait Encode: ToBuilder {
    fn encode<W: Write>(&self, write: W) -> Result<()> {
        self.encode_with(write, HeapAllocator::new())
    }

    fn encode_with<W: Write, A: Allocator>(&self, write: W, allocator: A) -> Result<()> {
        self.encode_message_with(allocator)
            .and_then(|message| write_message(write, message.by_ref()))
    }

    fn encode_packed<W: Write>(&self, write: W) -> Result<()> {
        self.encode_packed_with(write, HeapAllocator::new())
    }

    fn encode_packed_with<W: Write, A: Allocator>(&self, write: W, allocator: A) -> Result<()> {
        self.encode_message_with(allocator)
            .and_then(|message| write_message_packed(write, message.by_ref()))
    }

    fn encode_message(&self) -> Result<Message> {
        self.encode_message_with(HeapAllocator::new())
    }

    fn encode_message_with<A: Allocator>(&self, allocator: A) -> Result<MessageWith<A>> {
        let mut message = MessageWith::new(allocator);

        let builder = message.init_root();

        self.to_builder(builder)?;

        Ok(message)
    }
}

trait_aliases! {
    #[trait_alias(S)]
    pub trait Schema = FromReader + ToBuilder;

    #[trait_alias(T)]
    pub trait Encoding = Decode + Encode;
}
