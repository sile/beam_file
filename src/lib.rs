extern crate byteorder;

pub mod chunk;
mod beam_file;

pub type BeamFile<C> = beam_file::BeamFile<C>;
pub type RawBeamFile = beam_file::BeamFile<chunk::RawChunk>;
pub type StandardBeamFile = beam_file::BeamFile<chunk::StandardChunk>;
