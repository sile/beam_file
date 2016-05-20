extern crate byteorder;
extern crate flate2;

pub mod chunk;
pub mod parts;
mod beam_file;

pub use beam_file::BeamFile;
pub type RawBeamFile = BeamFile<chunk::RawChunk>;
pub type StandardBeamFile = BeamFile<chunk::StandardChunk>;
