use std::collections::HashMap;
use super::ir::{FieldDecl, VarDecl, MethodDecl};

enum LocationDecl {
    Field(FieldDecl),
    Var(VarDecl),
}

pub struct EnvStack {
    methods: HashMap<String, MethodDecl>,
    envs: Vec<Env>,
}

impl EnvStack {
    pub fn new() -> Self {
        Self {
            methods: HashMap::new(),
            envs: Vec::new()
        }
    }
    pub fn add_env(&mut self, e: Env) {
        self.envs.push(e);
    }
}

pub struct Env {
    decls: HashMap<String, LocationDecl>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            decls: HashMap::new(),
        }
    }

    pub fn add_field(&mut self, f: &FieldDecl) {
        let borrowed = f.borrow();
        let val = LocationDecl::Field(f.clone());
        self.decls.insert(borrowed.name.clone(), val);
    }
}

