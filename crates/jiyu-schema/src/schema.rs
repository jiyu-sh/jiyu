use capnp::{
    Error,
    io::{BufRead, Read, Write},
    message::{Allocator, Builder, HeapAllocator, ReaderOptions},
    serialize::{read_message, write_message},
    serialize_packed::{
        read_message as read_message_packed, write_message as write_message_packed,
    },
    traits::{FromPointerBuilder, FromPointerReader, Owned},
};

use crate::by::By;

pub trait FromReader: Sized {
    type Reader<'r>: FromPointerReader<'r>
    where
        Self: 'r;

    fn from_reader(reader: Self::Reader<'_>) -> Result<Self, Error>;
}

pub trait FromRead: FromReader {
    fn read<R: Read>(read: R) -> Result<Self, Error> {
        Self::read_with(read, ReaderOptions::new())
    }

    fn read_with<R: Read>(read: R, options: ReaderOptions) -> Result<Self, Error> {
        let owned = read_message(read, options)?;

        let reader = owned.get_root()?;

        let value = Self::from_reader(reader)?;

        Ok(value)
    }

    fn read_packed<B: BufRead>(read: B) -> Result<Self, Error> {
        Self::read_packed_with(read, ReaderOptions::new())
    }

    fn read_packed_with<B: BufRead>(read: B, options: ReaderOptions) -> Result<Self, Error> {
        let owned = read_message_packed(read, options)?;

        let reader = owned.get_root()?;

        let value = Self::from_reader(reader)?;

        Ok(value)
    }
}

impl<T: FromReader> FromRead for T {}

pub trait ToBuilder {
    type Builder<'b>: FromPointerBuilder<'b>
    where
        Self: 'b;

    fn to_builder(&self, builder: Self::Builder<'_>) -> Result<(), Error>;
}

pub type MessageWith<A> = Builder<A>;
pub type Message = MessageWith<HeapAllocator>;

pub trait ToWrite: ToBuilder {
    fn write<W: Write>(&self, write: W) -> Result<(), Error> {
        self.write_with(write, HeapAllocator::new())
    }

    fn write_with<W: Write, A: Allocator>(&self, write: W, allocator: A) -> Result<(), Error> {
        let message = self.message_with(allocator)?;

        write_message(write, message.by_ref())?;

        Ok(())
    }

    fn write_packed<W: Write>(&self, write: W) -> Result<(), Error> {
        self.write_packed_with(write, HeapAllocator::new())
    }

    fn write_packed_with<W: Write, A: Allocator>(
        &self,
        write: W,
        allocator: A,
    ) -> Result<(), Error> {
        let message = self.message_with(allocator)?;

        write_message_packed(write, message.by_ref())?;

        Ok(())
    }

    fn message(&self) -> Result<Message, Error> {
        self.message_with(HeapAllocator::new())
    }

    fn message_with<A: Allocator>(&self, allocator: A) -> Result<MessageWith<A>, Error> {
        let mut message = MessageWith::new(allocator);

        let builder = message.init_root();

        self.to_builder(builder)?;

        Ok(message)
    }
}

impl<T: ToBuilder> ToWrite for T {}

pub trait Schema: FromReader + ToBuilder {
    type Owned<'s>: Owned<Reader<'s> = Self::Reader<'s>, Builder<'s> = Self::Builder<'s>>
    where
        Self: 's;
}
