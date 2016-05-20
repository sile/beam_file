#[derive(Debug, PartialEq, Eq)]
pub struct Atom {
    pub name: String,
}
impl Atom {
    pub fn new(name: String) -> Self {
        Atom { name: name }
    }
}
