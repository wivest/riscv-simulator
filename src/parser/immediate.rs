use super::label::Label;

enum Immediate {
    Value(usize),
    Label(Label),
}
