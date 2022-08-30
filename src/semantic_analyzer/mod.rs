mod passes;
mod ir;

use crate::token::Program;
use passes::*;
use ir::{IRRoot, ProgramClassDecl};

pub struct SemanticAnalyzer {
    
}

impl SemanticAnalyzer {    
    pub fn new() -> Self {
        SemanticAnalyzer{}
    }
    pub fn create_ir(&self, p: &Program) -> Result<IRRoot, Vec<SemanticCheckError>> {
        if let Err(errors) = self.pre_ir_check(p) {
            return Err(errors);
        }
        let ir = match self.construct_ir(p) {
            Err(errors) => return Err(errors),
            Ok(ir) => ir,
        };
        if let Err(errors) = self.post_ir_check(&ir) {
            return Err(errors);
        }
        Ok(ir)
    }
    fn construct_ir(&self, p: &Program) -> Result<IRRoot, Vec<SemanticCheckError>>  {
        Ok(IRRoot{root: ProgramClassDecl{field_decls: Vec::new(), method_decls: Vec::new()}})
    }
    fn pre_ir_check(&self, p: &Program) -> Result<(), Vec<SemanticCheckError>> {
        let passes = Vec::from([
            /* pass 3 */ has_main, 
            /* pass 4 */ is_array_size_positive,
        ]);
        let errors: Vec<SemanticCheckError> = passes.iter()
                .map(|&pass| pass(p))
                .filter(|res| res.is_err())
                .map(|res| res.err().unwrap())
                .collect();
        if errors.len() == 0 {
            Ok(())
        } else {
            Err(errors)
        }
    }
    fn post_ir_check(&self, p: &IRRoot) -> Result<(), Vec<SemanticCheckError>> {
        let passes = Vec::from([
            |p: &IRRoot| -> Result<(), SemanticCheckError> { Ok(()) }
        ]);
        let errors: Vec<SemanticCheckError> = passes.iter()
                .map(|&pass| pass(p))
                .filter(|res| res.is_err())
                .map(|res| res.err().unwrap())
                .collect();
        if errors.len() == 0 { Ok(()) } else { Err(errors) }
    }
}


#[cfg(test)]
mod tests{
    use std::fs::read_to_string;
    use std::path::PathBuf;
    use std::env;
    use crate::parser::DecafParser;
    use super::*;

macro_rules! test_sa_illegal {
    ( $testname:ident, $filename:expr ) => {
        #[test]
        fn $testname()
        {
            let path = env::var("CARGO_MANIFEST_DIR").unwrap();
            let path: PathBuf = [&path, "src", "semantic_analyzer", "testcases", $filename].iter()
                    .collect();
            let s = read_to_string(&path).unwrap();
            let program = DecafParser::new().parse(&s).unwrap();
            let res = SemanticAnalyzer::new().create_ir(&program);
            assert!(res.is_err());
            
        }
    };
}

macro_rules! test_sa_legal {
    ( $testname:ident, $filename:expr ) => {
        #[test]
        fn $testname()
        {
            let path = env::var("CARGO_MANIFEST_DIR").unwrap();
            let path: PathBuf = [&path, "src", "semantic_analyzer", "testcases", $filename].iter()
                    .collect();
            let s = read_to_string(&path).unwrap();
            let program = DecafParser::new().parse(&s).unwrap();
            let res = SemanticAnalyzer::new().create_ir(&program);
            assert!(res.is_ok());
        }
    };
}    


test_sa_illegal!(test_sa_illegal_01, "illegal-01.dcf");
test_sa_illegal!(test_sa_illegal_02, "illegal-02.dcf");
test_sa_illegal!(test_sa_illegal_03, "illegal-03.dcf");
test_sa_illegal!(test_sa_illegal_04, "illegal-04.dcf");
test_sa_illegal!(test_sa_illegal_05, "illegal-05.dcf");
test_sa_illegal!(test_sa_illegal_06, "illegal-06.dcf");
test_sa_illegal!(test_sa_illegal_07, "illegal-07.dcf");
test_sa_illegal!(test_sa_illegal_08, "illegal-08.dcf");
test_sa_illegal!(test_sa_illegal_09, "illegal-09.dcf");
test_sa_illegal!(test_sa_illegal_10, "illegal-10.dcf");
test_sa_illegal!(test_sa_illegal_11, "illegal-11.dcf");
test_sa_illegal!(test_sa_illegal_12, "illegal-12.dcf");
test_sa_illegal!(test_sa_illegal_13, "illegal-13.dcf");
test_sa_illegal!(test_sa_illegal_14, "illegal-14.dcf");
test_sa_illegal!(test_sa_illegal_15, "illegal-15.dcf");
test_sa_illegal!(test_sa_illegal_16, "illegal-16.dcf");
test_sa_illegal!(test_sa_illegal_17, "illegal-17.dcf");
test_sa_legal!(test_sa_legal_01, "legal-01.dcf");
}

















