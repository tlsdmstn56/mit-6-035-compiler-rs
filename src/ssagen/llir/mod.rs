mod inst;
mod llir_method_gen_context;
mod visit;

use crate::misc::HashableRc;
use crate::semantic_analyzer::ir as sir;
use inst::Inst;
use llir_method_gen_context::LLIRMethodGenContext;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Copy)]
pub enum Type {
    Int { len: u32 },
    Bool { len: u32 },
    String { len: u32 },
    Void,
}

impl Type {
    pub fn get_bytesize(&self) -> u32 {
        const align: u32 = 4;
        match self {
            Type::Int { len } | Type::Bool { len } => len * align,
            Type::String { len } => len + 1, // null terminated
            Type::Void => 0,
        }
    }
}

pub trait ToLLIRType {
    fn to_llir_type(&self, arr_size: u32) -> Type;
}

impl ToLLIRType for sir::Type {
    fn to_llir_type(&self, arr_size: u32) -> Type {
        match self {
            sir::Type::Int => Type::Int { len: arr_size },
            sir::Type::Bool => Type::Bool { len: arr_size },
            sir::Type::Void => Type::Void,
        }
    }
}

trait ToLLIRVarDef {
    fn to_llir_var_def(&self) -> VarDef;
}

impl ToLLIRVarDef for sir::VarDecl {
    fn to_llir_var_def(&self) -> VarDef {
        let self_ = self.borrow();
        let name = self_.name.clone();
        let arr_size = self_.arr_size.clone().unwrap_or(0) as u32;
        let type_ = self_.type_.to_llir_type(arr_size);
        Rc::new(RefCell::new(VarDef0 { type_, name }))
    }
}

pub type VarDef = Rc<RefCell<VarDef0>>;

pub struct VarDef0 {
    name: String,
    type_: Type,
}

pub type MethodDef = Rc<RefCell<MethodDef0>>;

pub struct MethodDef0 {
    args: Vec<VarDef>,
    return_type: Type,
    insts: Vec<Inst>,
}

pub struct LLIR {
    pub var_defs: Vec<VarDef>,
    pub method_defs: Vec<MethodDef>,
}

pub fn generate_llir(ir: &sir::IRRoot) -> LLIR {
    let mut vardecl_to_vardef = HashMap::new();

    // global variables (class fields) should
    // be visiable from all methods
    let mut var_defs = Vec::new();
    for d in &ir.root.field_decls {
        let var_def = d.to_llir_var_def();
        var_defs.push(var_def.clone());
        let key = HashableRc::new(d.clone());
        vardecl_to_vardef.insert(key, var_def);
    }

    // methods
    let mut ctx = LLIRMethodGenContext::new(&vardecl_to_vardef);
    let mut method_defs = Vec::new();
    for d in &ir.root.method_decls {
        let method_def = ctx.generate_llir_method(d);
        method_defs.push(method_def);
    }

    LLIR {
        var_defs,
        method_defs,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::DecafParser;
    use crate::semantic_analyzer::SemanticAnalyzer;
    use crate::test_util::get_current_dir;
    use std::fs::read_to_string;
    use std::path::PathBuf;

    #[test]
    fn test_generate_llir() {
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
        let decaf_code = read_to_string(&path).unwrap();
        eprintln!("{}", decaf_code);
        let parsed = DecafParser::new().parse(decaf_code.as_str()).unwrap();
        let ir = SemanticAnalyzer::new().create_ir(parsed).unwrap();
        let _llir = generate_llir(&ir);
    }
}
