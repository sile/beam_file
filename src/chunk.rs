use std::str;
use std::io;
use std::io::Result;
use std::io::Read;
use std::io::Write;
use std::io::Cursor;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use byteorder::BigEndian;
use parts;

pub type Id = [u8; 4];

struct Header {
    chunk_id: Id,
    data_size: u32,
}
impl Header {
    fn new(chunk_id: Id, data_size: u32) -> Self {
        Header {
            chunk_id: chunk_id,
            data_size: data_size,
        }
    }
    fn decode<R: Read>(mut reader: R) -> Result<Self> {
        let mut id = [0; 4];
        try!(reader.read_exact(&mut id));
        let size = try!(reader.read_u32::<BigEndian>());
        Ok(Header::new(id, size))
    }
    fn encode<W: Write>(&self, mut writer: W) -> Result<()> {
        try!(writer.write_all(&self.chunk_id));
        try!(writer.write_u32::<BigEndian>(self.data_size));
        Ok(())
    }
}

pub trait Chunk {
    fn id(&self) -> Id;

    fn decode<R: Read>(mut reader: R) -> Result<Self>
        where Self: Sized
    {
        let header = try!(Header::decode(&mut reader));
        let mut buf = vec![0; header.data_size as usize];
        try!(reader.read_exact(&mut buf));
        for _ in 0..padding_size(header.data_size) {
            try!(reader.read_u8());
        }

        Self::decode_data(header.chunk_id, Cursor::new(&buf))
    }
    fn decode_data<R: Read>(id: Id, reader: R) -> Result<Self> where Self: Sized;

    fn encode<W: Write>(&self, mut writer: W) -> Result<()> {
        let mut buf = Vec::new();
        try!(self.encode_data(&mut buf));
        try!(Header::new(self.id(), buf.len() as u32).encode(&mut writer));
        try!(writer.write_all(&buf));
        for _ in 0..padding_size(buf.len() as u32) {
            try!(writer.write_u8(0));
        }
        Ok(())
    }
    fn encode_data<W: Write>(&self, writer: W) -> Result<()>;
}

fn padding_size(data_size: u32) -> u32 {
    (4 - data_size % 4) % 4
}

fn invalid_data_error<T>(description: String) -> io::Result<T> {
    Err(io::Error::new(io::ErrorKind::InvalidData, description))
}

#[derive(Debug, PartialEq, Eq)]
pub struct RawChunk {
    pub id: Id,
    pub data: Vec<u8>,
}
impl Chunk for RawChunk {
    fn id(&self) -> Id {
        self.id
    }
    fn decode_data<R: Read>(id: Id, mut reader: R) -> Result<Self>
        where Self: Sized
    {
        let mut buf = Vec::new();
        try!(reader.read_to_end(&mut buf));
        Ok(RawChunk {
            id: id,
            data: buf,
        })
    }
    fn encode_data<W: Write>(&self, mut writer: W) -> Result<()> {
        try!(writer.write_all(&self.data));
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct AtomChunk {
    pub atoms: Vec<parts::Atom>,
}
impl Chunk for AtomChunk {
    fn id(&self) -> Id {
        *b"Atom"
    }
    fn decode_data<R: Read>(_id: Id, mut reader: R) -> Result<Self>
        where Self: Sized
    {
        let count = try!(reader.read_u32::<BigEndian>()) as usize;
        let mut atoms = Vec::with_capacity(count);
        for _ in 0..count {
            let len = try!(reader.read_u8()) as usize;
            let mut buf = vec![0; len];
            try!(reader.read_exact(&mut buf));

            let name = try!(str::from_utf8(&buf).or_else(|e| invalid_data_error(e.to_string())));
            atoms.push(parts::Atom::new(name.to_string()));
        }
        Ok(AtomChunk { atoms: atoms })
    }
    fn encode_data<W: Write>(&self, mut writer: W) -> Result<()> {
        for atom in &self.atoms {
            assert!(atom.name.len() < 0x100);
            try!(writer.write_u8(atom.name.len() as u8));
            try!(writer.write_all(atom.name.as_bytes()));
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CodeChunk {
    pub info_size: u32,
    pub version: u32,
    pub opcode_max: u32,
    pub label_count: u32,
    pub function_count: u32,
    pub bytecode: Vec<u8>,
}
impl Chunk for CodeChunk {
    fn id(&self) -> Id {
        *b"Code"
    }
    fn decode_data<R: Read>(_id: Id, mut reader: R) -> Result<Self>
        where Self: Sized
    {
        let mut code = CodeChunk {
            info_size: try!(reader.read_u32::<BigEndian>()),
            version: try!(reader.read_u32::<BigEndian>()),
            opcode_max: try!(reader.read_u32::<BigEndian>()),
            label_count: try!(reader.read_u32::<BigEndian>()),
            function_count: try!(reader.read_u32::<BigEndian>()),
            bytecode: Vec::new(),
        };
        try!(reader.read_to_end(&mut code.bytecode));
        Ok(code)
    }
    fn encode_data<W: Write>(&self, mut writer: W) -> Result<()> {
        try!(writer.write_u32::<BigEndian>(self.info_size));
        try!(writer.write_u32::<BigEndian>(self.version));
        try!(writer.write_u32::<BigEndian>(self.opcode_max));
        try!(writer.write_u32::<BigEndian>(self.label_count));
        try!(writer.write_u32::<BigEndian>(self.function_count));
        try!(writer.write_all(&self.bytecode));
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct StrTChunk {
    pub strings: Vec<u8>
}
impl Chunk for StrTChunk {
    fn id(&self) -> Id {
        *b"StrT"
    }
    fn decode_data<R: Read>(_id: Id, mut reader: R) -> Result<Self>
        where Self: Sized
    {
        let mut buf = Vec::new();
        try!(reader.read_to_end(&mut buf));
        Ok(StrTChunk{strings: buf})
    }
    fn encode_data<W: Write>(&self, mut writer: W) -> Result<()> {
        try!(writer.write_all(&self.strings));
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum StandardChunk {
    Atom(AtomChunk),
    Code(CodeChunk),
    StrT(StrTChunk),
    Unknown(RawChunk),
}
impl Chunk for StandardChunk {
    fn id(&self) -> Id {
        use self::StandardChunk::*;
        match *self {
            Atom(ref c) => c.id(),
            Code(ref c) => c.id(),
            StrT(ref c) => c.id(),
            Unknown(ref c) => c.id(),
        }
    }
    fn decode_data<R: Read>(id: Id, reader: R) -> Result<Self>
        where Self: Sized
    {
        use self::StandardChunk::*;
        match &id {
            b"Atom" => Ok(Atom(try!(AtomChunk::decode_data(id, reader)))),
            b"Code" => Ok(Code(try!(CodeChunk::decode_data(id, reader)))),
            b"StrT" => Ok(StrT(try!(StrTChunk::decode_data(id, reader)))),
            _ => Ok(Unknown(try!(RawChunk::decode_data(id, reader)))),
        }
    }
    fn encode_data<W: Write>(&self, writer: W) -> Result<()> {
        use self::StandardChunk::*;
        match *self {
            Atom(ref c) => c.encode_data(writer),
            Code(ref c) => c.encode_data(writer),
            StrT(ref c) => c.encode_data(writer),            
            Unknown(ref c) => c.encode_data(writer),
        }
    }
}
