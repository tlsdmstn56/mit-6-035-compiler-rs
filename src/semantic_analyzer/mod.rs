mod env;
mod ir;
mod passes;

use crate::token;
use env::{EnvContext, EnvStack, EnvType};
use passes::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct SemanticAnalyzer {
    envs: Rc<RefCell<EnvStack>>,
}

fn create_rc<T>(x: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(x))
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            envs: create_rc(EnvStack::new()),
        }
    }
    pub fn create_ir(&self, p: token::Program) -> Result<ir::IRRoot, Vec<SemanticCheckError>> {
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

    fn get_ir_field_decls(
        &self,
        decls: Vec<token::FieldDecl>,
    ) -> Result<Vec<ir::VarDecl>, Vec<SemanticCheckError>> {
        let env_ctx = EnvContext::new(self.envs.clone(), EnvType::NoEnv);
        let mut res: Vec<ir::VarDecl> = Vec::new();
        let mut errors = Vec::new();
        for field_decls in decls {
            for field_decl in field_decls.loc {
                let t = ir::Type::from(&field_decls.type_);
                let name = field_decl.name;
                let arr_size = field_decl.arr_size.clone();
                let d = ir::VarDecl0 {
                    r#type: t,
                    name: name,
                    arr_size: arr_size,
                };
                let d = create_rc(d);
                if let Err(_) = env_ctx.add_var(&d) {
                    let e = SemanticCheckError::DuplicatedSymbol(d.borrow().name.clone());
                    errors.push(e);
                    continue;
                }
                res.push(d);
            }
        }
        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(res)
        }
    }

    fn get_ir_method_arg(&self, t: &token::MethodArg) -> ir::MethodArg {
        ir::MethodArg {
            r#type: ir::Type::from(&t.type_),
            name: t.name.clone(),
        }
    }
    fn get_ir_var_decls(
        &self,
        t: Vec<token::VarDecl>,
    ) -> Result<Vec<ir::VarDecl>, Vec<SemanticCheckError>> {
        let env_ctx = EnvContext::new(self.envs.clone(), EnvType::NoEnv);
        let mut res: Vec<ir::VarDecl> = Vec::new();
        let mut errors = Vec::new();
        for decls in t {
            for name in decls.identifiers {
                let t = ir::Type::from(&decls.type_);
                let d = ir::VarDecl0 {
                    r#type: t,
                    name: name,
                    arr_size: 1,
                };
                let d = create_rc(d);
                if let Err(_) = env_ctx.add_var(&d) {
                    let e = SemanticCheckError::DuplicatedSymbol(d.borrow().name.clone());
                    errors.push(e);
                    continue;
                }
                res.push(d);
            }
        }
        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(res)
        }
    }

    fn get_ir_location(&self, t: token::Location) -> Result<ir::Location, Vec<SemanticCheckError>> {
        let mut errors = Vec::new();
        let env_ctx = EnvContext::new(self.envs.clone(), EnvType::NoEnv);
        let var_decl = env_ctx.find_var_decl(&t.name);
        if var_decl.is_none() {
            errors.push(SemanticCheckError::UnknownSymbol(t.name));
        }
        
        let offset = match t.arr_size {
            Some(i) => 
                match self.get_ir_expr(i) {
                    Ok(i) => Some(i),
                    Err(e) => {
                        errors.extend(e);
                        None
                    },
                }
            None => None,
        };
        if errors.is_empty() {
            Ok(ir::Location{
                decl: var_decl.unwrap(), 
                arr_size: offset, 
            })
        } else {
            Err(errors)
        }
    }

    fn get_ir_call(&self, t: token::MethodCall) -> Result<ir::Call, Vec<SemanticCheckError>> {
        todo!("get_ir_call");
    }
    fn get_ir_literal(&self, t: token::Literal) -> Result<ir::Literal, Vec<SemanticCheckError>> {
        match t {
            token::Literal::Int(l) => Ok(ir::Literal::Int(l)),
            token::Literal::Bool(l) => match l {
                token::BoolLiteral::True => Ok(ir::Literal::Boolean(true)),
                token::BoolLiteral::False => Ok(ir::Literal::Boolean(false)),
            }
            token::Literal::Char(l) => {
                if l.is_ascii() {
                    Ok(ir::Literal::Int(u32::from(l) as i32))
                } else {
                    Err(vec![SemanticCheckError::NonAsciiCharLiteral(l)])
                }
            }
        }
    }
    fn get_ir_unary(&self, t: token::Unary) -> Result<ir::Unary, Vec<SemanticCheckError>> {
        todo!("get_ir_unary");
    }
    fn get_ir_binary(&self, t: token::Binary) -> Result<ir::Binary, Vec<SemanticCheckError>> {
        todo!("get_ir_binary");
    }
    fn get_ir_expr(&self, t: token::Expr) -> Result<ir::Expr, Vec<SemanticCheckError>> {
        match *t {
            token::Expr0::Location(t) => match self.get_ir_location(t) {
                Ok(a) => Ok(create_rc(ir::Expr0::Location(a))),
                Err(e) => Err(e),
            },
            token::Expr0::MethodCall(t) => match self.get_ir_call(t) {
                Ok(a) => Ok(create_rc(ir::Expr0::Call(a))),
                Err(e) => Err(e),
            },
            token::Expr0::Literal(t) => match self.get_ir_literal(t) {
                Ok(a) => Ok(create_rc(ir::Expr0::Literal(a))),
                Err(e) => Err(e),
            },
            token::Expr0::Unary(t) => match self.get_ir_unary(t) {
                Ok(a) => Ok(create_rc(ir::Expr0::Unary(a))),
                Err(e) => Err(e),
            },
            token::Expr0::Binary(t) => match self.get_ir_binary(t) {
                Ok(a) => Ok(create_rc(ir::Expr0::Binary(a))),
                Err(e) => Err(e),
            },
        }
    }

    fn get_ir_assign(&self, t: token::Assign) -> Result<ir::Assign, Vec<SemanticCheckError>> {
        let mut errors = Vec::new();
        let dst = match self.get_ir_location(t.dst) {
            Ok(d) => Some(d),
            Err(e) => {
                errors.extend(e);
                None
            }
        };
        let op = ir::AssignOp::from(t.op);
        let val = match self.get_ir_expr(t.val) {
            Ok(d) => Some(d),
            Err(e) => {
                errors.extend(e);
                None
            }
        };
        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(ir::Assign {
                dst: dst.unwrap(),
                op: op,
                val: val.unwrap(),
            })
        }
    }

    fn get_ir_ifelse(
        &self,
        t: token::IfElse,
    ) -> Result<ir::IfElse, Vec<SemanticCheckError>> {
        todo!("get_ir_ifelse")
    }
    fn get_ir_return(
        &self,
        t: token::Return,
    ) -> Result<ir::Return, Vec<SemanticCheckError>> {
        todo!("get_ir_return")
    }
    fn get_ir_break(
        &self,
    ) -> Result<ir::Break, Vec<SemanticCheckError>> {
        todo!("get_ir_break")
    }
    fn get_ir_continue(
        &self,
    ) -> Result<ir::Continue, Vec<SemanticCheckError>> {
        todo!("get_ir_continue")
    }
    fn get_ir_for(
        &self,
        t: token::Loop,
    ) -> Result<ir::For, Vec<SemanticCheckError>> {
        todo!("get_ir_for")
    }
    fn get_ir_statement(
        &self,
        t: token::Statement,
    ) -> Result<ir::Statement, Vec<SemanticCheckError>> {
        match t {
            token::Statement::Assign(a) => match self.get_ir_assign(a) {
                Ok(a) => Ok(create_rc(ir::Statement0::Assign(a))),
                Err(e) => Err(e),
            },
            token::Statement::MethodCall(a) => match self.get_ir_call(a) {
                Ok(a) => Ok(create_rc(ir::Statement0::Call(a))),
                Err(e) => Err(e),
            },
            token::Statement::IfElse(a) => match self.get_ir_ifelse(a) {
                Ok(a) => Ok(create_rc(ir::Statement0::IfElse(a))),
                Err(e) => Err(e),
            },
            token::Statement::Loop(a) => match self.get_ir_for(a) {
                Ok(a) => Ok(create_rc(ir::Statement0::For(a))),
                Err(e) => Err(e),
            },
            token::Statement::Return(a) => match self.get_ir_return(a) {
                Ok(a) => Ok(create_rc(ir::Statement0::Return(a))),
                Err(e) => Err(e),
            },
            token::Statement::Break => match self.get_ir_break() {
                Ok(a) => Ok(create_rc(ir::Statement0::Break(a))),
                Err(e) => Err(e),
            },
            token::Statement::Continue => match self.get_ir_continue() {
                Ok(a) => Ok(create_rc(ir::Statement0::Continue(a))),
                Err(e) => Err(e),
            },
            token::Statement::Block(a) => match self.get_ir_block(a, EnvType::Anon) {
                Ok(a) => Ok(create_rc(ir::Statement0::Block(a))),
                Err(e) => Err(e),
            },
        }
    }

    fn get_ir_statements(
        &self,
        t: Vec<token::Statement>,
    ) -> Result<Vec<ir::Statement>, Vec<SemanticCheckError>> {
        let mut errors: Vec<SemanticCheckError> = Vec::new();
        let mut statements: Vec<ir::Statement> = Vec::new();

        for stmt in t {
            let ir_stmt = self.get_ir_statement(stmt);
            match ir_stmt {
                Ok(s) => statements.push(s),
                Err(e) => errors.extend(e),
            }
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(statements)
        }
    }

    fn get_ir_block(
        &self,
        t: token::Block,
        scope_type: EnvType,
    ) -> Result<ir::Block, Vec<SemanticCheckError>> {
        let env_ctx = EnvContext::new(self.envs.clone(), scope_type);
        let mut errors = Vec::new();

        // variable declations
        let var_decls = self.get_ir_var_decls(t.var_decls);
        let var_decls = match var_decls {
            Ok(v) => Some(v),
            Err(e) => {
                errors.extend(e);
                None
            }
        };

        // statements in this block
        let statements = self.get_ir_statements(t.statements);
        let statements = match statements {
            Ok(s) => Some(s),
            Err(e) => {
                errors.extend(e);
                None
            }
        };
        if !errors.is_empty() {
            Err(errors)
        } else {
            let b = ir::Block {
                var_decls: var_decls.unwrap(),
                statements: statements.unwrap(),
            };
            Ok(b)
        }
    }

    fn get_ir_method_decls(
        &self,
        decls: Vec<token::MethodDecl>,
    ) -> Result<Vec<ir::MethodDecl>, Vec<SemanticCheckError>> {
        let mut methods = Vec::new();
        let mut errors = Vec::new();
        for decl in decls {
            let return_type = ir::Type::from(&decl.return_type);
            let args = decl
                .args
                .iter()
                .map(|a| self.get_ir_method_arg(a))
                .collect();
            let ir_decl = ir::MethodDecl0 {
                return_type: return_type,
                name: decl.name,
                args: args,
                block: None,
            };
            let ir_decl = create_rc(ir_decl);

            // symbol table should have method arguments
            let env_ctx = EnvContext::new(self.envs.clone(), EnvType::Method(ir_decl.clone()));
            let block = self.get_ir_block(decl.block, EnvType::NoEnv);
            if let Err(e) = block {
                errors.extend(e);
                continue;
            }
            ir_decl.borrow_mut().block = Some(block.ok().unwrap());
            if let Err(_) = env_ctx.add_method(&ir_decl) {
                let e = SemanticCheckError::DuplicatedSymbol(ir_decl.borrow().name.clone());
                errors.push(e);
                continue;
            }
            methods.push(ir_decl);
        }
        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(methods)
        }
    }

    fn construct_ir(&self, p: token::Program) -> Result<ir::IRRoot, Vec<SemanticCheckError>> {
        let env_ctx = EnvContext::new(self.envs.clone(), EnvType::Global);
        let mut errors = Vec::new();
        let field_decls = self.get_ir_field_decls(p.field_decls);
        let field_decls = match field_decls {
            Err(e) => {
                errors.extend(e);
                None
            }
            Ok(m) => Some(m),
        };
        let method_decls = self.get_ir_method_decls(p.method_decls);
        let method_decls = match method_decls {
            Err(e) => {
                errors.extend(e);
                None
            }
            Ok(m) => Some(m),
        };
        if !errors.is_empty() {
            return Err(errors);
        }
        let program_decl = ir::ProgramClassDecl {
            field_decls: field_decls.unwrap(),
            method_decls: method_decls.unwrap(),
        };
        let root = ir::IRRoot { root: program_decl };
        Ok(root)
    }
    fn pre_ir_check(&self, p: &token::Program) -> Result<(), Vec<SemanticCheckError>> {
        let passes = Vec::from([
            /* pass 3 */ has_main,
            /* pass 4 */ is_array_size_positive,
        ]);
        let errors: Vec<SemanticCheckError> = passes
            .iter()
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
        let passes = Vec::from([|p: &ir::IRRoot| -> Result<(), SemanticCheckError> { Ok(()) }]);
        let errors: Vec<SemanticCheckError> = passes
            .iter()
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::DecafParser;
    use crate::test_util::get_current_dir;
    use std::env;
    use std::fs::read_to_string;
    use std::path::PathBuf;

    macro_rules! test_sa_illegal {
        ( $testname:ident, $filename:expr ) => {
            #[test]
            fn $testname() {
                let path = get_current_dir();
                let path: PathBuf = [&path, "src", "semantic_analyzer", "testcases", $filename]
                    .iter()
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
            fn $testname() {
                let path = get_current_dir();
                let path: PathBuf = [&path, "src", "semantic_analyzer", "testcases", $filename]
                    .iter()
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
