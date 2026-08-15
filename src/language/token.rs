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
    Lower(Reference<'a>),
}

// label
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Reference<'a>(pub &'a str);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Definition<'a>(pub &'a str);
