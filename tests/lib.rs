extern crate beam_file;

use std::path::PathBuf;
use beam_file::RawBeamFile;
use beam_file::StandardBeamFile;
use beam_file::chunk::Chunk;

#[test]
fn raw_chunks() {
    let beam = RawBeamFile::from_file(test_file("test.beam")).unwrap();

    // Chunk List
    assert_eq!(vec!["Atom", "Code", "StrT", "ImpT", "ExpT", "LitT", "LocT", "Attr", "CInf",
                    "Abst", "Line"],
               collect_id(&beam.chunks));
}

#[test]
fn standard_chunks() {
    use beam_file::chunk::StandardChunk::*;
    macro_rules! find_chunk{
        ($beam:expr, $chunk:ident) => (
            $beam
                .chunks.iter()
                .filter_map(|c| if let $chunk(ref x) = *c { Some(x) } else { None })
                .nth(0)
                .unwrap()
        )
    }

    let beam = StandardBeamFile::from_file(test_file("test.beam")).unwrap();

    // Chunk List
    assert_eq!(vec!["Atom", "Code", "StrT", "ImpT", "ExpT", "LitT", "LocT", "Attr", "CInf",
                    "Abst", "Line"],
               collect_id(&beam.chunks));

    // Atom Chunk
    assert_eq!(vec!["test",
                    "hello",
                    "io",
                    "format",
                    "ok",
                    "module_info",
                    "erlang",
                    "get_module_info"],
               find_chunk!(beam, Atom).atoms.iter().map(|a| &a.name).collect::<Vec<_>>());
}

fn test_file(name: &str) -> PathBuf {
    let mut path = PathBuf::from("tests/testdata/");
    path.push(name);
    path
}

fn collect_id<C: Chunk>(chunks: &Vec<C>) -> Vec<String> {
    chunks.iter().map(|c| std::str::from_utf8(&c.id()).unwrap().to_string()).collect()
}
