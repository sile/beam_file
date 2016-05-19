extern crate beam_file;

use std::path::PathBuf;
use beam_file::RawBeamFile;
use beam_file::StandardBeamFile;
use beam_file::chunk::Chunk;

#[test]
fn raw_chunks() {
    let beam = RawBeamFile::from_file(test_file("test.beam")).unwrap();
    assert_eq!(vec!["Atom", "Code", "StrT", "ImpT", "ExpT", "LitT", "LocT", "Attr", "CInf",
                    "Abst", "Line"],
               collect_id(&beam.chunks))
}

#[test]
fn standard_chunks() {
    let beam = StandardBeamFile::from_file(test_file("test.beam")).unwrap();
    assert_eq!(vec!["Atom", "Code", "StrT", "ImpT", "ExpT", "LitT", "LocT", "Attr", "CInf",
                    "Abst", "Line"],
               collect_id(&beam.chunks))
}

fn test_file(name: &str) -> PathBuf {
    let mut path = PathBuf::from("tests/testdata/");
    path.push(name);
    path
}

fn collect_id<C: Chunk>(chunks: &Vec<C>) -> Vec<String> {
    chunks.iter().map(|c| std::str::from_utf8(&c.id()).unwrap().to_string()).collect()
}
