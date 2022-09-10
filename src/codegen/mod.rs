use super::ssagen::SSA;

pub enum ArchType {
    X86_64,
}

pub fn generate_asm(ssa: SSA, t: ArchType) -> String {
    match t {
        ArchType::X86_64 => X86CodeGenenerator::new(ssa).generate(),
    }

}

struct X86CodeGenenerator {}

impl X86CodeGenenerator {
    pub fn new(ssa:SSA) -> Self {
        Self{}
    }

    pub fn generate(&self) -> String {
        String::new()
    }
}
