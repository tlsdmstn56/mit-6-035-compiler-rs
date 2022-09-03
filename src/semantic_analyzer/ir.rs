use crate::token;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Block {
    pub var_decls: Vec<VarDecl>,
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct Location {
    pub decl: VarDecl,
    pub arr_size: Option<Expr>,
}

#[derive(Debug)]
pub struct Assign {
    pub dst: Location,
    pub op: AssignOp,
    pub val: Expr,
}

#[derive(Debug)]
pub struct IfElse0 {
    pub cond: Expr,
    pub true_block: Block,
    pub false_block: Option<Block>,
}
pub type IfElse = Rc<RefCell<IfElse0>>;

#[derive(Debug)]
pub struct For0 {
    pub index_var: Identifier,
    pub start: Expr,
    pub end: Expr,
    pub block: Block,
}
pub type For = Rc<RefCell<For0>>;

#[derive(Debug)]
pub struct Return {
    pub func: MethodDecl,
    pub val: Option<Expr>,
}

#[derive(Debug)]
pub struct Break {
    pub r#for: For,
}

#[derive(Debug)]
pub struct Continue {
    pub r#for: For,
}

#[derive(Debug)]
pub enum AssignOp {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
}

impl AssignOp {
    pub fn from(t: token::AssignOp) -> Self {
        match t {
            token::AssignOp::Assign => Self::Assign,
            token::AssignOp::AddAssign => Self::AddAssign,
            token::AssignOp::SubAssign => Self::SubAssign,
            token::AssignOp::MulAssign => Self::MulAssign,
            token::AssignOp::DivAssign => Self::DivAssign,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
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
    Or,  // logical or
    And, // logical and
    EQ,  // ==
    NE,  // !=
    GT,  // >
    LT,  // <
    GE,  // >=
    LE,  // <=
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Mod, // %
}

#[derive(Debug)]
pub struct Binary {
    pub lhs: Expr,
    pub rhs: Expr,
    pub op: BinaryOp,
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
pub struct Method {
    pub decl: MethodDecl,
    pub args: Vec<Expr>,
}

#[derive(Debug)]
pub struct Callout {
    pub name: StringLiteral,
    pub args: Vec<CalloutArg>,
}
#[derive(Debug)]
pub enum Call {
    Method(Method),
    Callout(Callout),
}

#[derive(Debug)]
pub enum Literal {
    Int(i32),
    Boolean(bool),
}

pub type Expr = Rc<RefCell<Expr0>>;

#[derive(Debug)]
pub struct Expr0 {
    pub type_: Type,
    pub expr: ExprType,
}

#[derive(Debug)]
pub enum ExprType {
    Location(Location),
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
    Block(Block),
}

pub type MethodDecl = Rc<RefCell<MethodDecl0>>;
#[derive(Debug)]
pub struct MethodArg {
    pub type_: Type,
    pub name: Identifier,
}

#[derive(Debug)]
pub struct MethodDecl0 {
    pub return_type: Type,
    pub name: Identifier,
    pub args: Vec<MethodArg>,
    pub block: Option<Block>,
}

pub type VarDecl = Rc<RefCell<VarDecl0>>;
#[derive(Debug)]
pub struct VarDecl0 {
    pub type_: Type,
    pub name: Identifier,
    pub arr_size: Option<i32>,
}
#[derive(Debug)]
pub enum MemberDecl {
    FieldDecl(VarDecl),
    MethodDecl(MethodDecl),
}

#[derive(Debug)]
pub struct ProgramClassDecl {
    pub field_decls: Vec<VarDecl>,
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
