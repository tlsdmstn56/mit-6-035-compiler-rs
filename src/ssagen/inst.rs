use super::VarDef;

#[derive(Clone)]
pub enum Inst {
    Load(Load),
    Store(Store),
    Label(String),
    IAdd(Binary),
    ISub(Binary),
    IMul(Binary),
    IDiv(Binary),
    IMod(Binary),
}

#[derive(Clone)]
pub struct Load {
    pub dst: SSAVar,
    pub src: Memory,
}
#[derive(Clone)]
pub struct Store {
    pub src: SSAVar,
    pub dst: Memory,
}

#[derive(Clone)]
pub struct Binary {
    pub dst: Location,
    pub lhs: Operand,
    pub rhs: Operand,
}

#[derive(Clone)]
pub enum Operand {
    Literal(i32),
    Location(Location),
}
