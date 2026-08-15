#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BType {
    Beq,
    Bne,
    Blt,
    Bltu,
    Bge,
    Bgeu,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IType {
    // arithmetic
    Addi,
    // bitwise logic
    Andi,
    Ori,
    Xori,
    // shift
    Slli,
    Srli,
    Srai,
    // load
    Lw,
    Lh,
    Lhu,
    Lb,
    Lbu,
    // jump
    Jalr,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JType {
    Jal,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RType {
    // arithmetic
    Add,
    Sub,
    Mul,
    Mulh,
    Mulhu,
    Mulhsu,
    Div,
    Rem,
    // bitwise logic
    And,
    Or,
    Xor,
    // shift
    Sll,
    Srl,
    Sra,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SType {
    Sw,
    Sh,
    Sb,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UType {
    Lui,
    Auipc,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum System {
    Ebreak,
}
