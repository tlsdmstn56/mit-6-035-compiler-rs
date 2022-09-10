use super::super::label_gen::LabelGenerator;
use super::inst::{Inst};
use super::visit::Visit;
use super::{MethodDef, MethodDef0, VarDef};
use super::{ToLLIRType, ToLLIRVarDef, VarDef0};
use crate::misc::HashableRc;
use crate::semantic_analyzer::ir as sir;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

pub struct LLIRMethodGenContext {
    global_var_defs: HashMap<HashableRc<sir::VarDecl0>, VarDef>,
    local_var_defs: HashMap<HashableRc<sir::VarDecl0>, VarDef>,
    for_labels: HashMap<HashableRc<sir::For0>, ForLabel>,
    insts: RefCell<Option<Vec<Inst>>>,
    label_gen: LabelGenerator,
    method_decl: Option<sir::MethodDecl>,
    vardef_to_reg: HashMap<HashableRc<VarDef0>, u32>,
    reggen: RegisterGenerator,
}

struct RegisterGenerator {
    reg_id: Cell<u32>,
}

impl RegisterGenerator {
    pub fn new() -> Self {
        Self {
            reg_id: Cell::new(0),
        }
    }

    // pub fn generate(&self, decl: Option<VarDef>) -> Register {
    //     let id = self.reg_id.get();
    //     self.reg_id.set(id + 1);
    //     Register { id, decl }
    // }
}

impl LLIRMethodGenContext {
    pub fn new(global_var_defs: &HashMap<HashableRc<sir::VarDecl0>, VarDef>) -> Self {
        Self {
            global_var_defs: global_var_defs.clone(),
            local_var_defs: HashMap::new(),
            for_labels: HashMap::new(),
            insts: RefCell::new(None),
            label_gen: LabelGenerator::new(),
            method_decl: None,
            vardef_to_reg: HashMap::new(),
            reggen: RegisterGenerator::new(),
        }
    }


    // pub fn generate(&mut self, decl: Option<VarDef>) -> Register {
    //     let reg = self.reggen.generate(decl);
    //     reg
    // }
    pub fn add_var(&mut self, d: &sir::VarDecl) {
        let key = HashableRc::new(d.clone());
        let val = d.to_llir_var_def();
        assert!(
            self.local_var_defs.insert(key, val).is_none(),
            "Duplicate var def"
        );
    }

    pub fn get_var_def(&self, d: &sir::VarDecl) -> &VarDef {
        let key = HashableRc::new(d.clone());
        self.local_var_defs.get(&key).unwrap()
    }

    pub fn generate_llir_method(&mut self, d: &sir::MethodDecl) -> MethodDef {
        let return_type = d.borrow().return_type.to_llir_type(1);
        let args = d
            .borrow()
            .args
            .iter()
            .map(|d| d.to_llir_var_def())
            .collect();
        self.prepare_ctx(d);

        d.visit(self);

        let insts = self.insts.replace(None).unwrap();

        Rc::new(RefCell::new(MethodDef0 {
            args,
            return_type,
            insts,
        }))
    }

    pub fn push_inst(&self, inst: Inst) {
        assert!(self.insts.borrow().is_some());
        match self.insts.borrow_mut().as_mut() {
            Some(insts) => {
                insts.push(inst);
            }
            None => (),
        }
    }

    fn prepare_ctx(&mut self, m: &sir::MethodDecl) {
        self.for_labels = HashMap::new();
        self.local_var_defs = self.global_var_defs.clone();
        self.insts.replace(Some(Vec::new()));
        self.method_decl = Some(m.clone());
        self.vardef_to_reg = HashMap::new();
        self.reggen = RegisterGenerator::new();
    }
}

#[derive(Clone)]
struct ForLabel {
    label_loop_begin: String,
    label_loop_end: String,
}
