use super::ir::*;
use crate::misc::HashableRc;
use std::cell::RefCell;
use std::collections::{HashSet, HashMap};
use std::rc::Rc;

pub struct Branch {
    cond: Option<Expr>,
    dst: BasicBlock,
}

pub enum Inst {
    Call(Call),
    Assign(Assign),
    Return(Return), // leaf node should has this instruction
    Branch(Branch),
    VarDef(VarDecl),
}

#[derive(Clone)]
pub struct BasicBlock(Rc<RefCell<BasicBlock0>>);

impl BasicBlock {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(BasicBlock0 {
            insts: Vec::new(),
            children: Vec::new(),
            parents: Vec::new(),
        })))
    }

    pub fn push_inst(&self, i: Inst) {
        self.0.borrow_mut().insts.push(i);
    }

    pub fn get_leaf_blocks(&self) -> Vec<BasicBlock> {
        if self.0.borrow().children.is_empty() {
            vec![self.clone()]
        } else {
            let mut visited: HashSet<HashableRc<BasicBlock0>> = HashSet::new();
            for c in &self.0.borrow().children {
                let leafs = c.get_leaf_blocks();
                for l in leafs {
                    visited.insert(l.into());
                }
            }
            visited.iter().map(|e| BasicBlock::from(e.get())).collect()
        }
        
    }
}

impl Into<HashableRc<BasicBlock0>> for BasicBlock {
    fn into(self) -> HashableRc<BasicBlock0>{
        HashableRc::new(self.0)
    }
}

impl From<Rc<RefCell<BasicBlock0>>> for BasicBlock{
    fn from(t: Rc<RefCell<BasicBlock0>>) -> Self {
        Self(t)
    }
}

pub struct BasicBlock0 {
    insts: Vec<Inst>,
    children: Vec<BasicBlock>,
    parents: Vec<BasicBlock>,
}

pub struct Root {
    node: BasicBlock,
    args: Vec<VarDecl>,
}

pub struct CFG {
    global_var: Vec<VarDecl>,
    roots: HashMap<HashableRc<MethodDecl0>, Root>,
}

struct BasicBlockMap {
    map: HashMap<u32, BasicBlock>,
    id: u32,
}

impl BasicBlockMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            id: 0,
        }
    }

    pub fn get(&self, id: u32) -> BasicBlock {
        self.map.get(&id).unwrap().clone()
    }

    pub fn create_node(&mut self) -> (BasicBlock, u32) {
        let n = BasicBlock::new();
        let id = self.id;
        self.id += 1;
        self.map.insert(id, n);
        (n, id)
    }
}

impl CFG {
    pub fn new(ir: &IRRoot) -> Self {
        let roots = HashMap::new();
        for d in &ir.root.method_decls {
            let key = HashableRc::new(d.clone());
            let val = Self::get_root(d);
            roots.insert(key, val);
        }
        Self {
            global_var: ir.root.field_decls.clone(),
            roots,
        }
    }

    fn get_root(d: &MethodDecl) -> Root {
        let mut ctx = CFGGenContext::new();
        let block = d.borrow().block.unwrap();
        block.visit(&mut ctx);
    }

}

struct IfElseMeta {
    true_block: BasicBlock,
    false_block: BasicBlock,
}

struct CFGGenContext {
    pub bb_map: BasicBlockMap,
    pub block_stack: Vec<BasicBlock>,
    pub ifelse_map: HashMap<HashableRc<IfElse0>, IfElseMeta>,
}

impl CFGGenContext {
    pub fn new() -> Self {
        let mut bb_map = BasicBlockMap::new();
        let (curr_block, _) = bb_map.create_node();
        let block_stack = vec![curr_block];
        let ifelse_map = HashMap::new();
        Self { bb_map, block_stack, ifelse_map }
    }

    pub fn get_curr_block(&self) -> &BasicBlock {
        self.block_stack.last().unwrap()
    }
}

trait CFGGen {
    fn visit(&self, ctx: &mut CFGGenContext);
}

impl CFGGen for Block {
    fn visit(&self, ctx: &mut CFGGenContext) {
        for d in &self.var_decls {
            let inst = Inst::VarDef(d.clone());
            ctx.get_curr_block().push_inst(inst);
        }

        for s in &self.statements {
            s.visit(ctx);
        }
    }
}

impl CFGGen for Statement {
    fn visit(&self, ctx: &mut CFGGenContext) {
        match *self.borrow() {
            Statement0::Assign(d) => {
                let inst = Inst::Assign(d.clone());
                ctx.get_curr_block().push_inst(inst);
            }
            Statement0::Call(d) => {
                let inst = Inst::Call(d.clone());
                ctx.get_curr_block().push_inst(inst);
            }
            Statement0::Return(d) => {
                let inst = Inst::Return(d.clone());
                ctx.get_curr_block().push_inst(inst);
            }
            Statement0::IfElse(d) => {
                let (true_block, _) = ctx.bb_map.create_node();

                // Add branch
                let branch = Branch {
                    cond: Some(d.borrow().cond),
                    dst: true_block,
                };
                let inst = Inst::Branch(branch);
                ctx.get_curr_block().push_inst(inst);

                // visit true_block
                ctx.block_stack.push(true_block);
                
                
            }
            Statement0::For(d) => {
                todo!();
            }
            Statement0::Break(d) => {
                todo!();
            }
            Statement0::Continue(d) => {
                todo!();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::SemanticAnalyzer;
    use super::*;
    use crate::parser::DecafParser;
    use crate::test_util::get_current_dir;
    use std::fs::read_to_string;
    use std::path::PathBuf;

    #[test]
    fn test_cfg_gen() {
        let path = get_current_dir();
        let path: PathBuf = [
            &path,
            "src",
            "semantic_analyzer",
            "testcases",
            "legal-01.dcf",
        ]
        .iter()
        .collect();
        let s = read_to_string(&path).unwrap();
        let program = DecafParser::new().parse(&s).unwrap();
        let ir = SemanticAnalyzer::new().create_ir(program).unwrap();
        let cfg = CFG::new(&ir);
    }
}
