#[macro_use] extern crate lalrpop_util;

mod token;
mod parser;
mod semantic_analyzer;
mod assembler;
lalrpop_mod!(#[allow(clippy::all)] decaf);

#[cfg(test)]
mod test_util;

use parser::DecafParser;
use semantic_analyzer::SemanticAnalyzer;


/// compile decaf source code to x86-64 assembly code
/// TODO: add compile error code
pub fn compile(code: &str) -> String {
    let parsed = DecafParser::new().parse(code).unwrap();
    let _ir = SemanticAnalyzer::new().create_ir(parsed);
    String::new()
}
