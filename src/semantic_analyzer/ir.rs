use std::rc::Rc;
use std::cell::RefCell;
use crate::token;

#[derive(Debug)]
pub struct Block {
    pub var_decls: Vec<VarDecl>,
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum LocationDecl {
    Var(VarDecl),
    Field(FieldDecl),
}

#[derive(Debug)]
pub struct Location {
    pub decl: LocationDecl,
    pub arr_size: Option<Expr>,
}

#[derive(Debug)]
pub struct Assign {dst:Location, op: AssignOp, val: Expr}

#[derive(Debug)]
pub struct IfElse {cond:Expr, true_block: Block, false_block: Option<Block>}

#[derive(Debug)]
pub struct For0{index_var: Identifier, start: Expr, end: Expr, block: Block}
pub type For = Rc<RefCell<For0>>;

#[derive(Debug)]
pub struct Return { func: MethodDecl, val: Option<Expr> }

#[derive(Debug)]
pub struct Break { r#for: For }

#[derive(Debug)]
pub struct Continue { r#for: For }

#[derive(Debug)]
pub enum AssignOp {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
}

#[derive(Debug, Clone)]
pub enum Type {
    Int, 
    Bool,
    Void,
}

impl Type {
    pub fn from(t: &token::Type) -> Self {
        match t {
            token::Type::Int => Self::Int,
            token::Type::Bool => Self::Bool,
            token::Type::Void => Self::Void,
        }
    }
}


#[derive(Debug)]
pub enum BinaryOp {
    Or,   // logical or
    And,  // logical and
    EQ,   // == 
    NE,   // !=
    GT,   // >
    LT,   // <
    GE,   // >=
    LE,   // <=
    Add,  // +
    Sub,  // -
    Mul,  // *
    Div,  // /
    Mod,  // %
}

#[derive(Debug)]
pub struct Binary {
    pub lhs : Expr,
    pub rhs : Expr,
    pub op : BinaryOp,
}
#[derive(Debug)]
pub enum UnaryOp {
    NegInt,
    NegBool,
}

#[derive(Debug)]
pub struct Unary {
    pub expr: Expr,
    pub op: UnaryOp,
}

#[derive(Debug)]
pub enum CalloutArg {
    Expr(Expr),
    StringLiteral(StringLiteral),
}

#[derive(Debug)]
pub enum Call {
    Method{
        name: MethodName,
        args: Vec<Expr>,
    },
    Callout{
        name: StringLiteral,
        args: Vec<CalloutArg>,
    },
}

#[derive(Debug)]
pub enum Literal {
    Int(i32),
    Boolean(bool),
}

pub type Expr = Rc<RefCell<Expr0>>;

#[derive(Debug)]
pub enum Expr0 {
    Literal(Literal),
    Call(Call),
    Unary(Unary),
    Binary(Binary),
}

pub type Statement = Rc<RefCell<Statement0>>;

#[derive(Debug)]
pub enum Statement0 {
    Assign(Assign),
    Call(Call),
    IfElse(IfElse),
    For(For),
    Return(Return),
    Break(Break),
    Continue(Continue),
    Block(Block)
}

pub type FieldDecl = Rc<RefCell<FieldDecl0>>;

#[derive(Debug)]
pub struct FieldDecl0 {
    pub r#type: Type,
    pub name: String,
    pub arr_size: i32,
}

pub type MethodDecl = Rc<RefCell<MethodDecl0>>;
#[derive(Debug)]
pub struct MethodArg {
    pub r#type: Type,
    pub name: Identifier,
}

#[derive(Debug)]
pub struct MethodDecl0 {
    pub return_type: Type,
    pub name: Identifier,
    pub args: Vec<MethodArg>,
    pub block: Block,
}
pub type VarDecl = Rc<RefCell<VarDecl0>>;
#[derive(Debug)]
pub struct VarDecl0 {
    pub r#type: Type,
    pub identifiers: Vec<Identifier>,
}
#[derive(Debug)]
pub enum MemberDecl {
    FieldDecl(FieldDecl),
    MethodDecl(MethodDecl),
}

#[derive(Debug)]
pub struct ProgramClassDecl {
    pub field_decls: Vec<FieldDecl>,
    pub method_decls: Vec<MethodDecl>,
}

#[derive(Debug)]
pub enum IR {
    Expr(Expr),
    Statement(Statement), 
    ProgramClassDecl(ProgramClassDecl),
    MemberDecl(MemberDecl),
    VarDecl(VarDecl),
    Type(Type),
}

#[derive(Debug)]
pub struct IRRoot {
    pub root: ProgramClassDecl,
}

pub type StringLiteral = String;
pub type MethodName = String;
pub type Identifier = String;




