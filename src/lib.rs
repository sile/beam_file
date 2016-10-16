//! This crate provides functionality to process Erlang BEAM file.
//!
//! # Examples
//!
//! Collects ID of chunks in a BEAM file:
//!
//! ```
//! use beam_file::StandardBeamFile;
//! use beam_file::chunk::Chunk;
//!
//! let beam = StandardBeamFile::from_file("tests/testdata/test.beam").unwrap();
//!
//! assert_eq!(vec![b"Atom", b"Code", b"StrT", b"ImpT", b"ExpT", b"FunT", b"LitT",
//!                 b"LocT", b"Attr", b"CInf", b"Abst", b"Line"],
//!            beam.chunks.iter().map(|c| c.id()).collect::<Vec<_>>());
//! ```
//!
//! Generates a BEAM file:
//!
//! ```
//! use beam_file::RawBeamFile;
//! use beam_file::chunk::{Chunk, RawChunk};
//!
//! let chunk = RawChunk{id: *b"Atom", data: Vec::new()}; // NOTICE: The chunk is malformed
//! let beam = RawBeamFile{chunks: vec![chunk]};
//! beam.to_file("my.beam").unwrap();
//! ```
extern crate byteorder;
extern crate libflate;

pub mod chunk;
pub mod parts;
mod beam_file;

pub use beam_file::BeamFile;
pub type RawBeamFile = BeamFile<chunk::RawChunk>;
pub type StandardBeamFile = BeamFile<chunk::StandardChunk>;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    InvalidString(std::str::Utf8Error),
    UnexpectedMagicNumber { magic_number: [u8; 4] },
    UnexpectedFormType { form_type: [u8; 4] },
    UnexpectedChunk { id: chunk::Id, expected: chunk::Id },
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::Io(ref x) => x.fmt(f),
            Error::InvalidString(ref x) => x.fmt(f),
            Error::UnexpectedMagicNumber { ref magic_number } => {
                write!(f,
                       r#"Unexpected magic number {} (expected b"FOR1")"#,
                       bytes_to_str(magic_number))
            }
            Error::UnexpectedFormType { ref form_type } => {
                write!(f,
                       r#"Unexpected from type {} (expected b"BEAM")"#,
                       bytes_to_str(form_type))
            }
            Error::UnexpectedChunk { ref id, ref expected } => {
                write!(f,
                       "Unexpected chunk id {} (expected {})",
                       bytes_to_str(id),
                       bytes_to_str(expected))
            }
        }
    }
}
impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref x) => x.description(),
            Error::InvalidString(ref x) => x.description(),
            Error::UnexpectedMagicNumber { .. } => "Unexpected magic number",
            Error::UnexpectedFormType { .. } => "Unexpected form type",
            Error::UnexpectedChunk { .. } => "Unexpected chunk",
        }
    }
    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            Error::Io(ref x) => Some(x),
            Error::InvalidString(ref x) => Some(x),
            _ => None,
        }
    }
}
impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}
impl std::convert::From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Self {
        Error::InvalidString(err)
    }
}

fn bytes_to_str(bytes: &[u8]) -> String {
    std::str::from_utf8(bytes).map(|x| format!("b{:?}", x)).unwrap_or(format!("{:?}", bytes))
}
