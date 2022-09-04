mod env;
mod ir;
mod passes;

use crate::token;
use env::{EnvContext, EnvStack, EnvType};
use passes::*;
use std::cell::RefCell;
use std::rc::Rc;

macro_rules! unwrap_or_early_return {
    ($t:expr) => {
        match $t {
            Ok(e) => e,
            Err(e) => return Err(e),
        }
    };
}
macro_rules! check_type_or_early_return {
    ($e:expr, $expected_type:path) => {
        match $e.borrow().type_ {
            $expected_type => (),
            _ => return Err(vec![SemanticCheckError::TypeMismatch(String::from("Early return"))]),
        }
    };
}
macro_rules! return_error {
    ($err:ident) => {
        return Err(vec![SemanticCheckError::$err]);
    };
}

pub struct SemanticAnalyzer {
    envs: Rc<RefCell<EnvStack>>,
}

fn create_rc<T>(x: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(x))
}

fn get_ir_expr_type(e: &ir::ExprType) -> Result<ir::Type, SemanticCheckError> {
    match e {
        ir::ExprType::Location(e) => Ok(e.decl.borrow().type_.clone()),
        ir::ExprType::Literal(e) => match e {
            ir::Literal::Int(_) => Ok(ir::Type::Int),
            ir::Literal::Boolean(_) => Ok(ir::Type::Bool),
        },
        ir::ExprType::Call(e) => match e {
            ir::Call::Method(m) => match m.decl.borrow().return_type {
                ir::Type::Void => Err(SemanticCheckError::ExprCallNoReturn),
                t @ _ => Ok(t),
            },
            ir::Call::Callout(_) => Ok(ir::Type::Int),
        },
        ir::ExprType::Unary(e) => Ok(e.expr.borrow().type_),
        ir::ExprType::Binary(e) => {
            Ok(e.op.get_return_type())
        }
    }
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
                    type_: t,
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

    fn get_ir_method_arg(&self, t: &token::MethodArg) -> ir::VarDecl {
        create_rc(ir::VarDecl0 {
            type_: ir::Type::from(&t.type_),
            name: t.name.clone(),
            arr_size: None,
        })
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
                    type_: t,
                    name: name,
                    arr_size: None,
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
            return Err(errors);
        }
        let var_decl = var_decl.unwrap();
        let offset = match t.arr_size {
            Some(i) => match self.get_ir_expr(i) {
                Ok(i) => {
                    let type_ = i.borrow().type_;
                    let arr_size = &var_decl.borrow().arr_size;
                    if type_ == ir::Type::Int && arr_size.is_some() {
                        Some(i)
                    } else {
                        if arr_size.is_some() {
                            errors.push(SemanticCheckError::ArrayLocationOnNonArrayVar);
                        }
                        if type_ != ir::Type::Int {
                            errors.push(SemanticCheckError::ArrayLocationOffsetTypeError);
                        }
                        None
                    }
                }
                Err(e) => {
                    errors.extend(e);
                    None
                }
            },
            None if !var_decl.borrow().is_array() => None,
            _ => {
                    errors.push(SemanticCheckError::TypeMismatch(String::from("location does not have offset, but delc is an array")));
                    None
            }
        };
        if errors.is_empty() {
            Ok(ir::Location {
                decl: var_decl,
                arr_size: offset,
            })
        } else {
            Err(errors)
        }
    }

    fn get_ir_method(&self, t: token::Method) -> IRResult<ir::Method> {
        let env_ctx = EnvContext::new(self.envs.clone(), EnvType::NoEnv);
        let method_decl = match env_ctx.find_method_decl(&t.name) {
            Some(m) => m,
            None => return Err(vec![SemanticCheckError::UnknownSymbol(t.name)]),
        };

        let mut args = Vec::with_capacity(t.args.len());
        for arg in t.args {
            let expr = unwrap_or_early_return!(self.get_ir_expr(arg));
            args.push(expr);
        }
        
        if args.len() != method_decl.borrow().args.len() {
            return_error!(MethodArgumentNotMatch);
        }
        for (arg, arg_decl) in args.iter().zip(method_decl.borrow().args.iter()) {
            if arg.borrow().type_ != arg_decl.borrow().type_ {
                return_error!(MethodArgumentNotMatch);
            }
        }
        Ok(ir::Method{
            decl: method_decl,
            args: args,
        }) 
    }


    fn get_ir_callout(&self, t: token::Callout) -> IRResult<ir::Callout> {
        let mut args = Vec::new();
        for targ in t.args {
            let arg = match targ{
                token::CalloutArg::Expr(e) => 
                    ir::CalloutArg::Expr(unwrap_or_early_return!(self.get_ir_expr(e))),
                token::CalloutArg::StringLiteral(s) => 
                    ir::CalloutArg::StringLiteral(s),
            };
            args.push(arg);
        }
        Ok(ir::Callout {
            name: t.name,
            args: args,
        })
    }

    fn get_ir_call(&self, t: token::MethodCall) -> IRResult<ir::Call> {
        match t {
            token::MethodCall::Method(t) => match self.get_ir_method(t) {
                Ok(m) => Ok(ir::Call::Method(m)),
                Err(e) => Err(e),
            },
            token::MethodCall::Callout(t) => match self.get_ir_callout(t) {
                Ok(m) => Ok(ir::Call::Callout(m)),
                Err(e) => Err(e),
            },
        }
    }
    fn get_ir_literal(&self, t: token::Literal) -> Result<ir::Literal, Vec<SemanticCheckError>> {
        match t {
            token::Literal::Int(l) => Ok(ir::Literal::Int(l)),
            token::Literal::Bool(l) => match l {
                token::BoolLiteral::True => Ok(ir::Literal::Boolean(true)),
                token::BoolLiteral::False => Ok(ir::Literal::Boolean(false)),
            },
            token::Literal::Char(l) => {
                if l.is_ascii() {
                    Ok(ir::Literal::Int(u32::from(l) as i32))
                } else {
                    Err(vec![SemanticCheckError::NonAsciiCharLiteral(l)])
                }
            }
        }
    }
    fn get_ir_unary(&self, t: token::Unary) -> IRResult<ir::Unary> {
        let expr = unwrap_or_early_return!(self.get_ir_expr(t.expr));
        let type_ = expr.borrow().type_;
        match t.op {
            token::UnaryOp::NegInt if type_ == ir::Type::Int => Ok(ir::Unary {
                expr: expr,
                op: ir::UnaryOp::NegInt,
            }),
            token::UnaryOp::NegBool if type_ == ir::Type::Bool => Ok(ir::Unary {
                expr: expr,
                op: ir::UnaryOp::NegBool,
            }),
            _ => Err(vec![SemanticCheckError::TypeMismatch(String::from("unary op is not supported"))]),
        }
    }
    fn get_ir_binary(&self, t: token::Binary) -> IRResult<ir::Binary> {
        let mut errors = Vec::new();
        let lhs = match self.get_ir_expr(t.lhs) {
            Ok(l) => Some(l),
            Err(e) => {
                errors.extend(e);
                None
            }
        };
        let rhs = match self.get_ir_expr(t.rhs) {
            Ok(l) => Some(l),
            Err(e) => {
                errors.extend(e);
                None
            }
        };

        if !errors.is_empty() {
            return Err(errors);
        }

        let lhs = lhs.unwrap();
        let rhs = rhs.unwrap();

        if lhs.borrow().type_ != rhs.borrow().type_ {
            errors.push(SemanticCheckError::TypeMismatch(String::from("binary lhs rhs type is not matched")));
            return Err(errors);
        }

        let op = ir::BinaryOp::from(&t.op);
        let operand_type = lhs.borrow().type_;
        match op {
            // 12. The operands of <arith op>s and <rel op>s must have type int.
            ir::BinaryOp::Add
            | ir::BinaryOp::Sub
            | ir::BinaryOp::Mul
            | ir::BinaryOp::Div
            | ir::BinaryOp::Mod
            | ir::BinaryOp::GT
            | ir::BinaryOp::GE
            | ir::BinaryOp::LT
            | ir::BinaryOp::LE
                if operand_type == ir::Type::Int =>
            {
                ()
            }
            // 13. The operands of <eq op>s must have the same
            //     type, either int or boolean.
            ir::BinaryOp::EQ | ir::BinaryOp::NE => (),
            // 14. The operands of <cond op>s and the operand of
            //     logical not (!) must have type boolean.
            ir::BinaryOp::Or | ir::BinaryOp::And if operand_type == ir::Type::Bool => (),

            // Otherwise, type mismatch error
            _ => {
                errors.push(SemanticCheckError::TypeMismatch(
                        format!("binary op {:?} is not supported: \nlhs: {:?}\nrhs: {:?}", op, lhs, rhs)));
                return Err(errors);
            }
        }
        Ok(ir::Binary {
            lhs: lhs,
            rhs: rhs,
            op: op,
        })
    }

    fn get_ir_expr(&self, t: token::Expr) -> Result<ir::Expr, Vec<SemanticCheckError>> {
        let expr_type = match *t {
            token::Expr0::Location(t) => match self.get_ir_location(t) {
                Ok(a) => Ok(ir::ExprType::Location(a)),
                Err(e) => Err(e),
            },
            token::Expr0::MethodCall(t) => match self.get_ir_call(t) {
                Ok(a) => Ok(ir::ExprType::Call(a)),
                Err(e) => Err(e),
            },
            token::Expr0::Literal(t) => match self.get_ir_literal(t) {
                Ok(a) => Ok(ir::ExprType::Literal(a)),
                Err(e) => Err(e),
            },
            token::Expr0::Unary(t) => match self.get_ir_unary(t) {
                Ok(a) => Ok(ir::ExprType::Unary(a)),
                Err(e) => Err(e),
            },
            token::Expr0::Binary(t) => match self.get_ir_binary(t) {
                Ok(a) => Ok(ir::ExprType::Binary(a)),
                Err(e) => Err(e),
            },
        };
        let expr_type = match expr_type {
            Ok(e) => e,
            Err(e) => return Err(e),
        };

        let type_ = get_ir_expr_type(&expr_type);
        let type_ = match type_ {
            Ok(t) => t,
            Err(e) => return Err(vec![e]),
        };
        Ok(create_rc(ir::Expr0 {
            type_: type_,
            expr: expr_type,
        }))
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
            return Err(errors);
        }
        let dst = dst.unwrap();
        let val = val.unwrap();
        let dst_type = dst.decl.borrow().type_;
        let val_type = val.borrow().type_;
        if dst_type != val_type {
            errors.push(SemanticCheckError::TypeMismatch(String::from("assign type not match")));
            return Err(errors);
        }

        if op != ir::AssignOp::Assign && dst_type != ir::Type::Int {
            errors.push(SemanticCheckError::TypeMismatch(String::from("non-int assign")));
            return Err(errors);
        }

        Ok(ir::Assign {
            dst: dst,
            op: op,
            val: val,
        })
    }

    fn get_ir_ifelse(&self, t: token::IfElse) -> IRResult<ir::IfElse> {
        let cond = match self.get_ir_expr(t.cond) {
            Ok(c) => c,
            Err(e) => return Err(e),
        };
        check_type_or_early_return!(cond, ir::Type::Bool);
        let ifelse = create_rc(ir::IfElse0 {
            cond: cond,
            true_block: None,
            false_block: None,
        });
        let true_block = self.get_ir_block(t.true_block, EnvType::If(ifelse.clone()));
        let true_block = Some(unwrap_or_early_return!(true_block));
        let false_block = match t.false_block {
            Some(b) => {
                let fb = self.get_ir_block(b, EnvType::Else(ifelse.clone()));
                Some(unwrap_or_early_return!(fb))
            }
            None => None,
        };
        ifelse.borrow_mut().true_block = true_block;
        ifelse.borrow_mut().false_block = false_block;
        assert!(ifelse.borrow().true_block.is_some());
        Ok(ifelse)
    }
    fn get_ir_return(&self, t: token::Return) -> IRResult<ir::Return> {
        let val = match t.val {
            Some(e) => match self.get_ir_expr(e) {
                Ok(e) => Some(e),
                Err(e) => return Err(e),
            },
            None => None,
        };
        let env_ctx = EnvContext::new(self.envs.clone(), EnvType::NoEnv);
        let decl = env_ctx.get_current_scope_method_decl().unwrap();
        let decl_return_type = decl.borrow().return_type;
        match val {
            // non-void return type: declared type and expr type should match
            Some(v)
                if decl_return_type != ir::Type::Void && decl_return_type == v.borrow().type_ =>
            {
                Ok(ir::Return {
                    func: decl,
                    val: Some(v),
                })
            }
            // void return type: declared type is void and expr should none
            None if decl_return_type == ir::Type::Void => Ok(ir::Return {
                func: decl,
                val: None,
            }),
            _ => {
                return Err(vec![SemanticCheckError::ReturnTypeMismatch]);
            }
        }
    }
    fn get_ir_break(&self) -> Result<ir::Break, Vec<SemanticCheckError>> {
        let env_ctx = EnvContext::new(self.envs.clone(), EnvType::NoEnv);
        let for_ = match env_ctx.find_for() {
            Some(f) => f,
            None => return Err(vec![SemanticCheckError::BreakOutOfForScope]),
        };
        Ok(ir::Break{
            for_: for_
        })
    }
    fn get_ir_continue(&self) -> Result<ir::Continue, Vec<SemanticCheckError>> {
        let env_ctx = EnvContext::new(self.envs.clone(), EnvType::NoEnv);
        let for_ = match env_ctx.find_for() {
            Some(f) => f,
            None => return Err(vec![SemanticCheckError::ContinueOutOfForScope]),
        };
        Ok(ir::Continue{
            for_: for_
        })
    }
    fn get_ir_for(&self, t: token::Loop) -> Result<ir::For, Vec<SemanticCheckError>> {
        let start = unwrap_or_early_return!(self.get_ir_expr(t.start));
        check_type_or_early_return!(start, ir::Type::Int);
        let end = unwrap_or_early_return!(self.get_ir_expr(t.end));
        check_type_or_early_return!(end, ir::Type::Int);
        let index_decl = create_rc(ir::VarDecl0 {
            type_: ir::Type::Int,
            name: t.index_var,
            arr_size: None,
        });
        let for_ = create_rc(ir::For0 {
            index_decl: index_decl.clone(),
            start: start,
            end: end,
            block: None,
        });
        let env_ctx = EnvContext::new(self.envs.clone(), EnvType::For(for_.clone()));
        env_ctx.add_var(&index_decl);

        let block = unwrap_or_early_return!(self.get_ir_block(t.block, EnvType::NoEnv));
        for_.borrow_mut().block = Some(block);
        assert!(for_.borrow().block.is_some());
        Ok(for_)
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
        let _env_ctx = EnvContext::new(self.envs.clone(), scope_type);
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

    fn get_ir_method_decl(&self, t: token::MethodDecl) -> IRResult<ir::MethodDecl> {
        let mut errors = Vec::new();
        let return_type = ir::Type::from(&t.return_type);
        let args: Vec<ir::VarDecl> = t 
            .args
            .iter()
            .map(|a| self.get_ir_method_arg(a))
            .collect();
        
        let ir_decl = ir::MethodDecl0 {
            return_type: return_type,
            name: t.name,
            args: args,
            block: None,
        };
        let ir_decl = create_rc(ir_decl);

        let env_ctx = EnvContext::new(self.envs.clone(), EnvType::Method(ir_decl.clone()));
        
        // add method argument to symbol table
        for arg in &ir_decl.borrow().args {
            env_ctx.add_var(&arg);
        }
        
        // add method declaration for recursive call
        if let Err(_) = env_ctx.add_method(&ir_decl) {
            let e = SemanticCheckError::DuplicatedSymbol(ir_decl.borrow().name.clone());
            errors.push(e);
        }

        // symbol table should have method arguments
        let block = self.get_ir_block(t.block, EnvType::NoEnv);
        if let Err(e) = block {
            errors.extend(e);
        }
        else {
            ir_decl.borrow_mut().block = Some(block.ok().unwrap());
        }
        
        if errors.is_empty() {
            Ok(ir_decl)
        } else {
            Err(errors)
        }
    }

    fn get_ir_method_decls(
        &self,
        decls: Vec<token::MethodDecl>,
    ) -> Result<Vec<ir::MethodDecl>, Vec<SemanticCheckError>> {
        let mut methods = Vec::new();
        let mut errors = Vec::new();
        for decl in decls {
            match self.get_ir_method_decl(decl) {
                Ok(d) => methods.push(d),
                Err(e) => errors.extend(e),
            }
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
                // let errors = res.err().unwrap();
                // for err in errors {
                //     println!("{:?}", err);
                // }
                // panic!("no okay")
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
