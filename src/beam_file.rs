use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io::Cursor;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use byteorder::BigEndian;
use chunk::Chunk;
use Result;
use Error;

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
        let f = try!(File::open(path));
        Self::from_reader(f)
    }
    pub fn from_reader<R: Read>(mut reader: R) -> Result<Self> {
        let expected = Header::new(0);
        let header = try!(Header::from_reader(&mut reader));
        if header.magic_number != expected.magic_number {
            return Err(Error::UnexpectedMagicNumber {
                magic_number: header.magic_number,
            });

        }
        if header.type_id != expected.type_id {
            return Err(Error::UnexpectedFormType { form_type: header.type_id });
        }

        let mut buf = vec![0; (header.payload_size - 4) as usize];
        try!(reader.read_exact(&mut buf));

        let mut chunks = Vec::new();
        let mut cursor = Cursor::new(&buf);
        while cursor.position() < buf.len() as u64 {
            chunks.push(try!(C::decode(&mut cursor)));
        }
        Ok(BeamFile { chunks: chunks })
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let f = try!(File::create(path));
        self.to_writer(f)
    }
    pub fn to_writer<W: Write>(&self, mut writer: W) -> Result<()> {
        let mut buf = Vec::new();
        for chunk in &self.chunks {
            try!(chunk.encode(&mut buf));
        }

        let header = Header::new(buf.len() as u32 + 4);
        try!(header.to_writer(&mut writer));
        try!(writer.write_all(&buf));
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
            payload_size: payload_size,
            type_id: *b"BEAM",
        }
    }
    fn from_reader<R: Read>(mut reader: R) -> Result<Self> {
        let mut header = Self::new(0);
        try!(reader.read_exact(&mut header.magic_number));
        header.payload_size = try!(reader.read_u32::<BigEndian>());
        try!(reader.read_exact(&mut header.type_id));
        Ok(header)
    }
    fn to_writer<W: Write>(&self, mut writer: W) -> Result<()> {
        try!(writer.write_all(&self.magic_number));
        try!(writer.write_u32::<BigEndian>(self.payload_size));
        try!(writer.write_all(&self.type_id));
        Ok(())
    }
}
