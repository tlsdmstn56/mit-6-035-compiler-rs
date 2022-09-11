#[derive(Clone)]
pub enum Location {
    Memory(Memory),
    SSAVar(SSAVar),
}

#[derive(Clone)]
pub struct Memory {
    pub decl: VarDef,
    pub offset: Option<Box<Location>>,
}

pub struct SSAVar {
    loc: Option<Memory>,
    id: u32,
}
