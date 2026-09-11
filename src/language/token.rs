use chumsky::span::SimpleSpan;

// immediate
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Offset<'a> {
    Value(i32),
    Label(Reference<'a>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Immediate<'a> {
    Value(i32),
    Upper(Reference<'a>),
    UpperPseudo(Reference<'a>),
    Lower(Reference<'a>),
    PcrelHi(Reference<'a>),
    PcrelLo(Reference<'a>),
    EquUpper(Reference<'a>),
    Equ20(Reference<'a>),
    Equ12(Reference<'a>),
}

// label
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Label<'a>(pub &'a str);

impl<'a> std::fmt::Display for Label<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Reference<'a>(pub Label<'a>, pub SimpleSpan);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Definition<'a>(pub Label<'a>, pub SimpleSpan);
