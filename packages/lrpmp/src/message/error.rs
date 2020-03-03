use std::convert::Infallible;

use crate::types::{Kind, ParseKnownKindError, UnexpectedType, UriFromBasicError};

#[derive(Debug)]
pub enum MessageError<E> {
    Eof,
    Codec(E),
    Uri(UriFromBasicError),
    UnexpectedKind(Kind),
    UnexpectedType(UnexpectedType),
    Custom(&'static str),
}

impl<E> MessageError<E> {
    pub fn for_codec(err: MessageError<()>) -> Self {
        use MessageError::*;
        match err {
            Eof => Eof,
            Uri(u) => Uri(u),
            Codec(()) => Custom("unspecified codec error"),
            UnexpectedKind(k) => UnexpectedKind(k),
            UnexpectedType(b) => UnexpectedType(b),
            Custom(c) => Custom(c),
        }
    }
}

impl<E> From<ParseKnownKindError> for MessageError<E> {
    fn from(err: ParseKnownKindError) -> Self {
        match err {
            ParseKnownKindError::UnexpectedType(t) => t.into(),
            ParseKnownKindError::UnknownKind(k) => Self::UnexpectedKind(Kind::Unknown(k)),
        }
    }
}

impl<E> From<UriFromBasicError> for MessageError<E> {
    fn from(err: UriFromBasicError) -> Self {
        Self::Uri(err)
    }
}

impl<E> From<UnexpectedType> for MessageError<E> {
    fn from(err: UnexpectedType) -> Self {
        Self::UnexpectedType(err)
    }
}

impl<E> From<Infallible> for MessageError<E> {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}
