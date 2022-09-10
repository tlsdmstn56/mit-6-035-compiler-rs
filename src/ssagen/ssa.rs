use super::cfg::CFG;
use crate::semantic_analyzer::ir::*;
use super::llir::LLIR;
use std::collections::HashMap;
use super::label_gen::LabelGenerator;


struct Method {
    insts: Vec<LLIR>,
}

struct GlobalVar {
    name: String, 
    size: u32,
}

pub struct SSA {
    methods: Vec<Method>,
    global_vars: Vec<GlobalVar>,
}

pub enum SSAGenError {
}

impl SSA {
    pub fn new(ir: IRRoot) -> Self {
        SSA{
            methods: Vec::new(),
            global_vars: Vec::new(),
        }
    }
}

struct ForMeta {
    label_loop_begin: String,
    label_loop_end: String,
}

struct MethodLLIRGenerator<'a, 'b> {
    ir: &'a IRRoot,
    label_gen: &'b LabelGenerator,
    meta_for: HashMap<For, ForMeta>,    
}

impl<'a, 'b> MethodLLIRGenerator<'a, 'b> {
    pub fn new(ir: &'a IRRoot, label_gen: &'b LabelGenerator) -> Self {
        Self {
            ir,
            label_gen,
            meta_for: HashMap::new(),
        }
    }
}
