#[derive(Clone)]
pub enum Executable {
    Goto(u32),
    Show(u32),
    Step(u32),
    Run,
    Output,
    Memory,
    Registers,
    Instructions,
}
