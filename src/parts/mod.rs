pub type AtomId = u32;
pub type Arity = u32;

#[derive(Debug, PartialEq, Eq)]
pub struct Atom {
    pub name: String,
}
impl Atom {
    pub fn new(name: String) -> Self {
        Atom { name: name }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Import {
    pub module: AtomId,
    pub function: AtomId,
    pub arity: Arity,
}
impl Import {
    pub fn new(module: AtomId, function: AtomId, arity: Arity) -> Self {
        Import {
            module: module,
            function: function,
            arity: arity,
        }
    }
}
