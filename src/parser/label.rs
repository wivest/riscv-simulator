#[derive(Debug, PartialEq)]
pub enum Label {
    Reference(String),
    Definition(usize),
}
