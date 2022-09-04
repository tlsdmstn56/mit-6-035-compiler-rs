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
    pub true_block: Option<Block>,
    pub false_block: Option<Block>,
}
pub type IfElse = Rc<RefCell<IfElse0>>;

#[derive(Debug)]
pub struct For0 {
    pub index_decl: VarDecl,
    pub start: Expr,
    pub end: Expr,
    pub block: Option<Block>,
}
pub type For = Rc<RefCell<For0>>;

#[derive(Debug)]
pub struct Return {
    pub func: MethodDecl,
    pub val: Option<Expr>,
}

#[derive(Debug)]
pub struct Break {
    pub for_: For,
}

#[derive(Debug)]
pub struct Continue {
    pub for_: For,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

#[derive(Debug, Copy, PartialEq, Eq, Clone)]
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

impl BinaryOp {
    pub fn get_return_type(&self) -> Type {
        match self {
            Self::Or  => Type::Bool,  // logical or
            Self::And => Type::Bool, // logical and
            Self::EQ  => Type::Bool,  // ==
            Self::NE  => Type::Bool,  // !=
            Self::GT  => Type::Bool,  // >
            Self::LT  => Type::Bool,  // <
            Self::GE  => Type::Bool,  // >=
            Self::LE  => Type::Bool,  // <=
            Self::Add => Type::Int, // +
            Self::Sub => Type::Int, // -
            Self::Mul => Type::Int, // *
            Self::Div => Type::Int, // /
            Self::Mod => Type::Int, // %
        } 
    }
}

impl BinaryOp {
    pub fn from(t: &token::BinaryOp) -> Self {
        match t {
            token::BinaryOp::Arith(a) => match a {
                token::ArithOp::Add => BinaryOp::Add,
                token::ArithOp::Sub => BinaryOp::Sub,
                token::ArithOp::Mul => BinaryOp::Mul,
                token::ArithOp::Div => BinaryOp::Div,
                token::ArithOp::Mod => BinaryOp::Mod,
            },
            token::BinaryOp::Compare(a) => match a {
                token::CompareOp::GT => BinaryOp::GT,
                token::CompareOp::GE => BinaryOp::GE,
                token::CompareOp::LT => BinaryOp::LT,
                token::CompareOp::LE => BinaryOp::LE,
            },
            token::BinaryOp::Eq(a) => match a {
                token::EqOp::EQ => BinaryOp::EQ,
                token::EqOp::NE => BinaryOp::NE,
            },
            token::BinaryOp::Cond(a) => match a {
                token::CondOp::Or => BinaryOp::Or,
                token::CondOp::And => BinaryOp::And,
            },
        }
    }
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
pub struct MethodDecl0 {
    pub return_type: Type,
    pub name: Identifier,
    pub args: Vec<VarDecl>,
    pub block: Option<Block>,
}

pub type VarDecl = Rc<RefCell<VarDecl0>>;
#[derive(Debug)]
pub struct VarDecl0 {
    pub type_: Type,
    pub name: Identifier,
    pub arr_size: Option<i32>,
}

impl VarDecl0 {
    pub fn is_array(&self) -> bool {
        self.arr_size.is_some()
    }
}

#[derive(Debug)]
pub struct ProgramClassDecl {
    pub field_decls: Vec<VarDecl>,
    pub method_decls: Vec<MethodDecl>,
}



#[derive(Debug)]
pub struct IRRoot {
    pub root: ProgramClassDecl,
}

pub type StringLiteral = String;
pub type Identifier = String;
