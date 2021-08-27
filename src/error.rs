use std::io::Error as IoError;
use std::str::Utf8Error;

use crate::chunk::Id as ChunkId;

#[derive(Debug, ::thiserror::Error)]
pub enum Error {
    #[error("Error::Io")]
    Io(#[source] IoError),

    #[error("Error::InvalidString")]
    InvalidString(#[source] Utf8Error),

    #[error("Error::UnexpectedMagicNumber: magic_number - {:?}", magic_number)]
    UnexpectedMagicNumber { magic_number: [u8; 4] },

    #[error("Error::UnexpectedFormType: form_type - {:?}", form_type)]
    UnexpectedFormType { form_type: [u8; 4] },

    #[error("Error::UnexpectedChunk: id - {:?}, expected - {:?}", id, expected)]
    UnexpectedChunk { id: ChunkId, expected: ChunkId },
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Self::Io(e)
    }
}
impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Self {
        Self::InvalidString(e)
    }
}
