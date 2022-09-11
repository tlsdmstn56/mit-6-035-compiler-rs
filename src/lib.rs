#[macro_use] extern crate lalrpop_util;

mod token;
mod parser;
mod semantic_analyzer;
mod assembler;
mod linker;
// TODO: enable
// mod ssagen;
// mod codegen;
mod misc;

lalrpop_mod!(#[allow(clippy::all)] decaf);

#[cfg(test)]
mod test_util;

use parser::DecafParser;
use semantic_analyzer::SemanticAnalyzer;
// TODO: enable
// use ssagen::SSA;
// use codegen::{generate_asm, ArchType};

/// compile decaf source code to x86-64 assembly code
pub fn compile(code: &str) -> String {
    // TODO: add compile error code
    let parsed = DecafParser::new().parse(code).unwrap();
    let ir = SemanticAnalyzer::new().create_ir(parsed);
    // TODO: enable
    // let ssa = SSA::new(ir.unwrap());
    // generate_asm(ssa, ArchType::X86_64)
    String::new()
}

#[cfg(test)]
mod tests{
    use super::*; 
    use crate::test_util::get_current_dir;
    use std::fs::read_to_string;
    use std::path::PathBuf;
    
    #[test]
    fn test_compile() {
        let path = get_current_dir();
        let path: PathBuf = [&path, "src", "semantic_analyzer", "testcases", "legal-01.dcf"]
                    .iter()
                    .collect();
        let decaf_code = read_to_string(&path).unwrap();
        compile(&decaf_code);
    }
}
