use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::path::Path;

use crate::chunk::Chunk;
use crate::{Error, Result};

/// A BEAM File
///
/// ```
/// use beam_file::BeamFile;
/// use beam_file::chunk::{Chunk, RawChunk};
///
/// let beam = BeamFile::<RawChunk>::from_file("tests/testdata/test.beam").unwrap();
/// assert_eq!(b"Atom", beam.chunks.iter().nth(0).map(|c| c.id()).unwrap());
/// ```
#[derive(Debug)]
pub struct BeamFile<C> {
    pub chunks: Vec<C>,
}
impl<C: Chunk> BeamFile<C> {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let f = r#try!(File::open(path));
        Self::from_reader(f)
    }
    pub fn from_reader<R: Read>(mut reader: R) -> Result<Self> {
        let expected = Header::new(0);
        let header = r#try!(Header::from_reader(&mut reader));
        if header.magic_number != expected.magic_number {
            return Err(Error::UnexpectedMagicNumber {
                magic_number: header.magic_number,
            });
        }
        if header.type_id != expected.type_id {
            return Err(Error::UnexpectedFormType {
                form_type: header.type_id,
            });
        }

        let mut buf = vec![0; (header.payload_size - 4) as usize];
        r#try!(reader.read_exact(&mut buf));

        let mut chunks = Vec::new();
        let mut cursor = Cursor::new(&buf);
        while cursor.position() < buf.len() as u64 {
            chunks.push(r#try!(C::decode(&mut cursor)));
        }
        Ok(BeamFile { chunks })
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let f = r#try!(File::create(path));
        self.to_writer(f)
    }
    pub fn to_writer<W: Write>(&self, mut writer: W) -> Result<()> {
        let mut buf = Vec::new();
        for chunk in &self.chunks {
            r#try!(chunk.encode(&mut buf));
        }

        let header = Header::new(buf.len() as u32 + 4);
        r#try!(header.to_writer(&mut writer));
        r#try!(writer.write_all(&buf));
        Ok(())
    }
}

struct Header {
    magic_number: [u8; 4],
    payload_size: u32,
    type_id: [u8; 4],
}
impl Header {
    fn new(payload_size: u32) -> Self {
        Header {
            magic_number: *b"FOR1",
            payload_size,
            type_id: *b"BEAM",
        }
    }
    fn from_reader<R: Read>(mut reader: R) -> Result<Self> {
        let mut header = Self::new(0);
        r#try!(reader.read_exact(&mut header.magic_number));
        header.payload_size = r#try!(reader.read_u32::<BigEndian>());
        r#try!(reader.read_exact(&mut header.type_id));
        Ok(header)
    }
    fn to_writer<W: Write>(&self, mut writer: W) -> Result<()> {
        r#try!(writer.write_all(&self.magic_number));
        r#try!(writer.write_u32::<BigEndian>(self.payload_size));
        r#try!(writer.write_all(&self.type_id));
        Ok(())
    }
}
