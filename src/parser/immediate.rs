use super::label::Label;

#[derive(Debug, PartialEq)]
pub enum Immediate {
    Value(i32),
    Label(Label),
}
