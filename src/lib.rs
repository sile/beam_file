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

mod beam_file;
pub mod chunk;
pub mod parts;

pub use crate::beam_file::BeamFile;
pub type RawBeamFile = BeamFile<chunk::RawChunk>;
pub type StandardBeamFile = BeamFile<chunk::StandardChunk>;

pub type Result<T> = std::result::Result<T, Error>;

mod error;
pub use error::Error;
