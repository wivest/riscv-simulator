use super::label::Label;

pub enum Immediate {
    Value(i32),
    Label(Label),
}
