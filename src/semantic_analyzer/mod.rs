mod passes;
mod ir;
mod env;

use crate::token;
use passes::*;
use env::{Env, EnvStack};
use std::rc::Rc;
use std::cell::RefCell;

pub struct SemanticAnalyzer {
    pub envs: EnvStack,
}

fn create_rc<T>(x: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(x))
}

impl SemanticAnalyzer {    
    pub fn new() -> Self {
        Self { 
            envs: EnvStack::new(),
        }
    }
    pub fn create_ir(&mut self, p: token::Program) -> Result<ir::IRRoot, Vec<SemanticCheckError>> {
        if let Err(errors) = self.pre_ir_check(&p) {
            return Err(errors);
        }
        let root = match self.construct_ir(p) {
            Err(errors) => return Err(errors),
            Ok(root) => root,
        };
        if let Err(errors) = self.post_ir_check(&root) {
            return Err(errors);
        }
        Ok(root)
    }
    
    fn get_ir_field_decls(&mut self, decls: Vec<token::FieldDecl>) -> Vec<ir::FieldDecl> {
        let mut res: Vec<ir::FieldDecl> = Vec::new();
        let mut global_env = Env::new();
        for field_decls in decls {
            for field_decl in field_decls.loc {
                let t = ir::Type::from(&field_decls.type_);
                let name = field_decl.name;
                let arr_size = field_decl.arr_size.clone();
                let d = ir::FieldDecl0{
                    r#type: t, 
                    name: name, 
                    arr_size: arr_size,
                };
                let d = create_rc(d);
                global_env.add_field(&d);
                res.push(d);
            }
        }
        self.envs.add_env(global_env);
        res
    }
    fn get_ir_method_decls(&mut self, decls: Vec<token::MethodDecl>) -> Vec<ir::MethodDecl> {
        Vec::new()
    }

    fn construct_ir(&mut self, p: token::Program) -> Result<ir::IRRoot, Vec<SemanticCheckError>>  {
        let field_decls = self.get_ir_field_decls(p.field_decls);
        let method_decls = self.get_ir_method_decls(p.method_decls);
        let program_decl = 
            ir::ProgramClassDecl {
                field_decls: field_decls, 
                method_decls: method_decls,
            };
        let root = ir::IRRoot{root: program_decl};
        Ok(root)
    }
    fn pre_ir_check(&self, p: &token::Program) -> Result<(), Vec<SemanticCheckError>> {
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
    fn post_ir_check(&self, p: &ir::IRRoot) -> Result<(), Vec<SemanticCheckError>> {
        let passes = Vec::from([
            |p: &ir::IRRoot| -> Result<(), SemanticCheckError> { Ok(()) }
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
            let res = SemanticAnalyzer::new().create_ir(program);
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
            let res = SemanticAnalyzer::new().create_ir(program);
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

















