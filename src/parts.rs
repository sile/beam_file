pub type AtomId = u32;
pub type Arity = u32;
pub type ExternalTermFormatBinary = Vec<u8>;

#[derive(Debug, PartialEq, Eq)]
pub struct Atom {
    pub name: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Import {
    pub module: AtomId,
    pub function: AtomId,
    pub arity: Arity,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Export {
    pub function: AtomId,
    pub arity: Arity,
    pub label: u32,
}
