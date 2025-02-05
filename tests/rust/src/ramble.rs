///////////////////////////////////////////////
// This file was generated using Ramble.
///////////////////////////////////////////////
use binread::{BinRead, BinReaderExt};
use binwrite::BinWrite;
use std::{
    convert::TryFrom,
    io::Cursor,
    str::{from_utf8, Utf8Error},
};

type TagValue = u8;

#[derive(thiserror::Error, Debug)]
pub enum SerializeError {
    // #[error("could not serialize `{0}` ")]
    // BadSerialize(String),
    #[error("could not serialize with binread ")]
    Binread(#[from] binread::Error),
    // #[error("unknown")]
    // Unknown,
}

#[derive(thiserror::Error, Debug)]
pub enum DeserializeError {
    // #[error("could not serialize `{0}` ")]
    // BadDeserialize(String),
    #[error("non utfbytes detect: {0}")]
    UtfError(#[from] Utf8Error),
    #[error("could not deserialize with binread ")]
    Binwrite(#[from] binread::Error),
    #[error("unknown type `{0}` ")]
    UnknownType(i32),
    // #[error("unknown")]
    // Unknown,
}

/* Struct Definitions */

#[derive(PartialEq, Debug, BinRead, BinWrite)]
pub struct Hello {
    pub seq: u32,
    pub flags: u8,
}

#[derive(PartialEq, Debug, BinRead, BinWrite)]
pub struct Bye {
    pub seq: u32,
    pub flags: u8,
}

#[derive(PartialEq, Debug, BinRead, BinWrite)]
pub struct NonAligned {
    pub seq: u32,
    pub flags: u8,
    pub extension: u16,
}

#[derive(PartialEq, Debug, BinRead, BinWrite)]
pub struct Primitives {
    pub u8: u8,
    pub u16: u16,
    pub u32: u32,
    pub u64: u64,
    pub i8: i8,
    pub i16: i16,
    pub i32: i32,
    pub i64: i64,
}

#[derive(Debug)]
pub enum MsgTypes {
    Hello = 0,
    Bye = 1,
    NonAligned = 2,
    Primitives = 3,
}

impl MsgTypes {
    fn get_tag_for(v: &Message) -> TagValue {
        match v {
            Message::Hello(_) => MsgTypes::Hello as TagValue,
            Message::Bye(_) => MsgTypes::Bye as TagValue,
            Message::NonAligned(_) => MsgTypes::NonAligned as TagValue,
            Message::Primitives(_) => MsgTypes::Primitives as TagValue,
        }
    }
}

impl TryFrom<TagValue> for MsgTypes {
    type Error = DeserializeError;

    fn try_from(v: TagValue) -> Result<Self, Self::Error> {
        match v {
            x if x == Self::Hello as TagValue => Ok(Self::Hello),
            x if x == Self::Bye as TagValue => Ok(Self::Bye),
            x if x == Self::NonAligned as TagValue => Ok(Self::NonAligned),
            x if x == Self::Primitives as TagValue => Ok(Self::Primitives),
            _ => Err(DeserializeError::UnknownType(v.into())),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Message {
    Hello(Hello),
    Bye(Bye),
    NonAligned(NonAligned),
    Primitives(Primitives),
}

impl Message {
    #![allow(dead_code)]
    pub fn to_bytes(&self) -> Result<Vec<u8>, SerializeError> {
        let mut buf = vec![];
        if let Err(e) = self.write(&mut buf) {
            return Err(SerializeError::Binread(binread::Error::Io(e)));
        }

        Ok(buf)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, DeserializeError> {
        let mut reader = Cursor::new(bytes);
        let m: Message = reader.read_ne()?;

        Ok(m)
    }
}

impl BinRead for Message {
    type Args = ();

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        options: &binread::ReadOptions,
        args: Self::Args,
    ) -> binread::BinResult<Self> {
        let tag = u8::read_options(reader, options, args)?;

        let msg_type =
            MsgTypes::try_from(tag).map_err(|_| binread::Error::NoVariantMatch { pos: 1 })?;

        let m = match msg_type {
            MsgTypes::Hello => Message::from(Hello::read_options(reader, options, args)?),
            MsgTypes::Bye => Message::from(Bye::read_options(reader, options, args)?),
            MsgTypes::NonAligned => Message::from(NonAligned::read_options(reader, options, args)?),
            MsgTypes::Primitives => Message::from(Primitives::read_options(reader, options, args)?),
        };

        Ok(m)
    }
}

impl BinWrite for Message {
    fn write_options<W: std::io::Write>(
        &self,
        writer: &mut W,
        options: &binwrite::WriterOption,
    ) -> std::io::Result<()> {
        let tag = MsgTypes::get_tag_for(self);

        tag.write_options(writer, options)?;

        match self {
            Message::Hello(m) => m.write(writer),
            Message::Bye(m) => m.write(writer),
            Message::NonAligned(m) => m.write(writer),
            Message::Primitives(m) => m.write(writer),
        }?;

        Ok(())
    }

    fn write<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.write_options(writer, &binwrite::WriterOption::default())
    }
}

impl From<Hello> for Message {
    fn from(value: Hello) -> Self {
        Message::Hello(value)
    }
}

impl From<Bye> for Message {
    fn from(value: Bye) -> Self {
        Message::Bye(value)
    }
}

impl From<NonAligned> for Message {
    fn from(value: NonAligned) -> Self {
        Message::NonAligned(value)
    }
}

impl From<Primitives> for Message {
    fn from(value: Primitives) -> Self {
        Message::Primitives(value)
    }
}

// TODO: Parameterize string length to handle variable sized len field
pub type Strlen = u16;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct PString {
    len: Strlen,
    #[br(count = len)]
    contents: Vec<u8>,
}

impl PString {
    pub fn new(input: String) -> Self {
        PString {
            len: input.len() as Strlen,
            contents: input.as_bytes().to_vec(),
        }
    }

    // TODO: Error only thrown when string is accessed, Error should be thrown on serialize when message is deserialized.
    pub fn get_str(&self) -> Result<&str, DeserializeError> {
        std::str::from_utf8(self.contents.as_slice()).map_err(|e| DeserializeError::UtfError(e))
    }

    pub fn set_str(&mut self, input: String) -> Strlen {
        self.len = input.len() as Strlen;

        // TODO: verify minimal copy of string bytes
        self.contents = input.as_bytes().to_vec();

        return self.len;
    }
}
