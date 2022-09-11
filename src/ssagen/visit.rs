use super::inst as llir;
use super::LLIRMethodGenContext;
use super::LLIR;
use crate::semantic_analyzer::ir as sir;
use std::rc::Rc;

pub trait Visit {
    fn visit(&self, ctx: &mut LLIRMethodGenContext) -> Option<llir::Location>;
}

impl Visit for sir::MethodDecl {
    fn visit(&self, ctx: &mut LLIRMethodGenContext) -> Option<llir::Location> {
        // stack allocation for args
        for d in &self.borrow().args {
            ctx.add_var(d);
            ctx.push_inst(llir::Inst::Alloca);
        }

        match self.borrow().block.as_ref() {
            Some(b) => b.visit(ctx),
            None => panic!("Method Block is empty"),
        }
    }
}

impl Visit for sir::Block {
    fn visit(&self, ctx: &mut LLIRMethodGenContext) -> Option<llir::Location> {
        // stack allocation
        for d in &self.var_decls {
            ctx.add_var(d);
            ctx.push_inst(llir::Inst::Alloca);
        }
        for stmt in &self.statements {
            stmt.visit(ctx);
        }
        None
    }
}

impl Visit for sir::Statement {
    fn visit(&self, ctx: &mut LLIRMethodGenContext) -> Option<llir::Location> {
        match &*self.borrow() {
            sir::Statement0::Assign(a) => a.visit(ctx),
            sir::Statement0::Call(a) => a.visit(ctx),
            sir::Statement0::For(a) => a.visit(ctx),
            sir::Statement0::Call(a) => a.visit(ctx),
            sir::Statement0::Return(a) => a.visit(ctx),
            sir::Statement0::Break(a) => a.visit(ctx),
            sir::Statement0::Continue(a) => a.visit(ctx),
            sir::Statement0::Block(a) => a.visit(ctx),
            sir::Statement0::IfElse(a) => a.visit(ctx),
        }
    }
}

impl Visit for sir::IfElse {
    fn visit(&self, ctx: &mut LLIRMethodGenContext) -> Option<llir::Location> {
        todo!();
    }
}

impl Visit for sir::Break {
    fn visit(&self, ctx: &mut LLIRMethodGenContext) -> Option<llir::Location> {
        todo!();
    }
}
impl Visit for sir::Continue {
    fn visit(&self, ctx: &mut LLIRMethodGenContext) -> Option<llir::Location> {
        todo!();
    }
}

impl Visit for sir::Assign {
    fn visit(&self, ctx: &mut LLIRMethodGenContext) -> Option<llir::Location> {
        let offset = match &self.dst.arr_size {
            Some(expr) => Some(Box::new(expr.visit(ctx).unwrap())),
            None => None,
        };
        
        let dst = llir::Location::Memory(llir::Memory {
            decl: ctx.get_var_def(&self.dst.decl).clone(),
            offset,
        });
        
        let lhs = if self.op == sir::AssignOp::Assign {
            llir::Operand::Literal(0)
        } else {
            llir::Operand::Location(dst.clone())
        };

        let rhs = llir::Operand::Location(self.val.visit(ctx).unwrap());

        match self.op {
            sir::AssignOp::Assign | sir::AssignOp::AddAssign => {
                ctx.push_inst(llir::Inst::IAdd(llir::Binary {
                    dst: dst.clone(),
                    lhs,
                    rhs,
                }));
            }
            sir::AssignOp::SubAssign => {
                ctx.push_inst(llir::Inst::ISub(llir::Binary {
                    dst: dst.clone(),
                    lhs,
                    rhs,
                }));
            },
            sir::AssignOp::MulAssign => {
                ctx.push_inst(llir::Inst::IMul(llir::Binary {
                    dst: dst.clone(),
                    lhs,
                    rhs,
                }));
            },
            sir::AssignOp::DivAssign => {
                ctx.push_inst(llir::Inst::IDiv(llir::Binary {
                    dst: dst.clone(),
                    lhs,
                    rhs,
                }));
            },
        };

        Some(dst)
    }
}

impl Visit for sir::Expr {
    fn visit(&self, ctx: &mut LLIRMethodGenContext) -> Option<llir::Location> {
        let res = match &self.borrow().expr {
            sir::ExprType::Location(a) => a.visit(ctx),
            sir::ExprType::Literal(a) => a.visit(ctx),
            sir::ExprType::Call(a) => a.visit(ctx),
            sir::ExprType::Unary(a) => a.visit(ctx),
            sir::ExprType::Binary(a) => a.visit(ctx),
        };
        match res {
            Some(e) => match e {
                llir::Location::Memory(_) => panic!("Expr should return on register"),
                r @ llir::Location::Register(_) => Some(r),
            }
            None => panic!("Expr alwyas return"),
        }
    }

}
impl Visit for sir::Location {
    fn visit(&self, ctx: &mut LLIRMethodGenContext) -> Option<llir::Location> {
        let offset = match &self.arr_size {
            Some(expr) => Some(Box::new(expr.visit(ctx).unwrap())),
            None => None,
        };
        let loc = llir::Memory {
            decl: ctx.get_var_def(&self.decl).clone(),
            offset,
        });
        let store = llir::Store {
            dst:,
            src:ctx.get_var_def(&self.decl).clone(), 
        };
        Some()
    }
}
impl Visit for sir::Binary {
    fn visit(&self, ctx: &mut LLIRMethodGenContext) -> Option<llir::Location> {
        todo!();
    }
}
impl Visit for sir::Unary {
    fn visit(&self, ctx: &mut LLIRMethodGenContext) -> Option<llir::Location> {
        todo!();
    }
}
impl Visit for sir::Literal {
    fn visit(&self, ctx: &mut LLIRMethodGenContext) -> Option<llir::Location> {
        todo!();
    }
}
impl Visit for sir::Return {
    fn visit(&self, ctx: &mut LLIRMethodGenContext) -> Option<llir::Location> {
        todo!();
    }
}

impl Visit for sir::Call {
    fn visit(&self, ctx: &mut LLIRMethodGenContext) -> Option<llir::Location> {
        todo!();
    }
}

impl Visit for sir::For {
    fn visit(&self, ctx: &mut LLIRMethodGenContext) -> Option<llir::Location> {
        todo!();
    }
}
