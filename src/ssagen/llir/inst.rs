use super::VarDef;

#[derive(Clone)]
pub enum Inst {
    Alloca,
    Load(Load),
    Store,
    Label(String),
    IAdd(Binary),
    ISub(Binary),
    IMul(Binary),
    IDiv(Binary),
}

#[derive(Clone)]
pub struct Register {
    pub id: u32,
}

#[derive(Clone)]
pub struct Load {
    pub dst: Register,
    pub src: Memory,
}

#[derive(Clone)]
pub struct Binary {
    pub dst: Location,
    pub lhs: Operand,
    pub rhs: Operand,
}

#[derive(Clone)]
pub struct Memory {
    pub decl: VarDef,
    pub offset: Option<Box<Location>>,
}

#[derive(Clone)]
pub enum Location {
    Memory(Memory),
    Register(Register),
}

#[derive(Clone)]
pub enum Operand {
    Literal(i32),
    Location(Location),
}
