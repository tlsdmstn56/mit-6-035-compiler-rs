use super::ir::{For, IfElse, MethodDecl, VarDecl};
use std::cell::RefCell;
use std::collections::HashMap;
use std::mem::discriminant;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum EnvType {
    Global,
    Anon,
    Method(MethodDecl),
    For(For),
    If(IfElse),
    Else(IfElse),
    NoEnv,
}

pub enum EnvError {
    DuplicatedMethod(MethodDecl),
    DuplicatedVar(VarDecl),
}

struct Env {
    pub type_: EnvType,
    pub table: HashMap<String, VarDecl>,
}

impl Env {
    pub fn new(t: EnvType) -> Self {
        Self {
            type_: t,
            table: HashMap::new(),
        }
    }
}

/// Env RAII
pub struct EnvContext {
    envs: Rc<RefCell<EnvStack>>,
    t: EnvType,
}

impl Drop for EnvContext {
    fn drop(&mut self) {
        let desc = discriminant(&self.t);
        let noenv_desc = discriminant(&EnvType::NoEnv);
        if desc != noenv_desc {
            self.envs.borrow_mut().pop();
        }
    }
}

impl EnvContext {
    pub fn new(envs: Rc<RefCell<EnvStack>>, t: EnvType) -> Self {
        let desc = discriminant(&t);
        let noenv_desc = discriminant(&EnvType::NoEnv);
        if desc != noenv_desc {
            envs.borrow_mut().push(t.clone());
        }
        Self { envs, t }
    }

    /// Add a new var declation in current env
    pub fn add_var(&self, f: &VarDecl) -> Result<(), EnvError> {
        self.envs.borrow_mut().add_var(f)
    }

    /// Add a new method
    pub fn add_method(&self, m: &MethodDecl) -> Result<(), EnvError> {
        self.envs.borrow_mut().add_method(m)
    }
    pub fn find_var_decl(&self, name: &String) -> Option<VarDecl> {
        self.envs.borrow().find_var_decl(name)
    }

    pub fn get_current_scope_method_decl(&self) -> Option<MethodDecl> {
        self.envs.borrow().get_current_scope_method_decl()
    }
    pub fn find_method_decl(&self, name: &String) -> Option<MethodDecl> {
        self.envs.borrow().find_method_decl(name)
    }
    pub fn find_for(&self) -> Option<For> {
        self.envs.borrow().find_for()
    }
}

pub struct EnvStack {
    methods: HashMap<String, MethodDecl>,
    envs: Vec<Env>,
}

impl EnvStack {
    pub fn new() -> Self {
        Self {
            methods: HashMap::new(),
            envs: Vec::new(),
        }
    }

    /// Push Env
    ///
    /// This usually means being into a new scope
    pub fn push(&mut self, t: EnvType) {
        self.envs.push(Env::new(t));
    }
    /// Pop Env
    ///
    /// This usually means getting out of a scope
    pub fn pop(&mut self) /* -> Env*/
    {
        self.envs.pop(); // .unwrap()
    }

    /// Add a new var declation in current env
    pub fn add_var(&mut self, f: &VarDecl) -> Result<(), EnvError> {
        let val = f.clone();
        let res = self
            .envs
            .last_mut()
            .unwrap()
            .table
            .insert(f.borrow().name.clone(), val);
        match res {
            Some(d) => Err(EnvError::DuplicatedVar(d)),
            None => Ok(()),
        }
    }

    /// Add a new method
    pub fn add_method(&mut self, m: &MethodDecl) -> Result<(), EnvError> {
        let res = self.methods.insert(m.borrow().name.clone(), m.clone());
        match res {
            Some(dup_decl) => Err(EnvError::DuplicatedMethod(dup_decl)),
            None => Ok(()),
        }
    }

    /// Find variable declation with given name in current scope
    pub fn find_var_decl(&self, name: &String) -> Option<VarDecl> {
        for env in self.envs.iter().rev() {
            match env.table.get(name) {
                Some(d) => return Some(d.clone()),
                None => (),
            }
        }
        None
    }

    /// Find method declation in current scope
    pub fn get_current_scope_method_decl(&self) -> Option<MethodDecl> {
        for env in self.envs.iter().rev() {
            if let EnvType::Method(m) = &env.type_ {
                return Some(m.clone());
            }
        }
        None
    }

    /// Find method declation by method name
    pub fn find_method_decl(&self, name: &String) -> Option<MethodDecl> {
        self.methods.get(name).cloned()
    }
    /// Find method declation in current scope
    pub fn find_for(&self) -> Option<For> {
        for env in self.envs.iter().rev() {
            if let EnvType::For(m) = &env.type_ {
                return Some(m.clone());
            }
        }
        None
    }
}
