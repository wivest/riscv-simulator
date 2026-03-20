#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BType {
    Beq,
    Bne,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IType {
    Addi,
    Lb,
    Lh,
    Lw,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JType {
    Jal,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RType {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SType {
    Sb,
    Sh,
    Sw,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UType {
    Lui,
    Auipc,
}
