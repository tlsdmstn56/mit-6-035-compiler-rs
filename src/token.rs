#[derive(Debug)]
pub struct FieldDecl {
    pub type_: Type,
    pub loc: Vec<FieldDecl0>,
}

#[derive(Debug)]
pub struct VarDecl {
    pub type_: Type,
    pub identifiers: Vec<Identifier>,
}

#[derive(Debug)]
pub struct Block {
    pub var_decls: Vec<VarDecl>,
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct Assign {
    pub dst: Location,
    pub op: AssignOp,
    pub val: Expr,
}
#[derive(Debug)]
pub struct IfElse {
    pub cond: Expr,
    pub true_block: Block,
    pub false_block: Option<Block>,
}
#[derive(Debug)]
pub struct Loop {
    pub index_var: Identifier,
    pub start: Expr,
    pub end: Expr,
    pub block: Block,
}

#[derive(Debug)]
pub struct Return {
    pub val: Option<Expr>,
}

#[derive(Debug)]
pub enum Statement {
    Assign(Assign),
    MethodCall(MethodCall),
    IfElse(IfElse),
    Loop(Loop),
    Return(Return),
    Break,
    Continue,
    Block(Block),
}

#[derive(Debug)]
pub struct Location {
    pub name: String,
    pub arr_size: Option<Expr>,
}

#[derive(Debug)]
pub struct FieldDecl0 {
    pub name: String,
    pub arr_size: Option<i32>,
}

#[derive(Debug)]
pub struct MethodArg {
    pub type_: Type,
    pub name: Identifier,
}

#[derive(Debug)]
pub struct MethodDecl {
    pub return_type: Type,
    pub name: Identifier,
    pub args: Vec<MethodArg>,
    pub block: Block,
}

#[derive(Debug)]
pub struct Program {
    pub field_decls: Vec<FieldDecl>,
    pub method_decls: Vec<MethodDecl>,
}

pub type MethodName = String;
pub type Identifier = String;

#[derive(Debug)]
pub enum Literal {
    Int(IntLiteral),
    Bool(BoolLiteral),
    Char(CharLiteral),
}

#[derive(Debug)]
pub enum BinaryOp {
    Arith(ArithOp),
    Compare(CompareOp),
    Eq(EqOp),
    Cond(CondOp),
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum ArithOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Clone)]
pub enum CompareOp {
    GT,
    GE,
    LT,
    LE,
}

#[derive(Debug, Clone)]
pub enum EqOp {
    EQ,
    NE,
}

#[derive(Debug, Clone)]
pub enum CondOp {
    Or,
    And,
}

pub type IntLiteral = i32;
pub type DecimalLiteral = i32;
pub type HexLiteral = i32;

#[derive(Debug, Clone)]
pub enum BoolLiteral {
    True,
    False,
}
pub type CharLiteral = char;
pub type StringLiteral = String;

#[derive(Debug)]
pub struct Binary {
    pub lhs: Expr,
    pub rhs: Expr,
    pub op: BinaryOp,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    NegInt,
    NegBool,
}

#[derive(Debug)]
pub struct Unary {
    pub expr: Expr,
    pub op: UnaryOp,
}

pub type Expr = Box<Expr0>;

#[derive(Debug)]
pub enum Expr0 {
    Location(Location),
    MethodCall(MethodCall),
    Literal(Literal),
    Unary(Unary),
    Binary(Binary),
}

#[derive(Debug)]
pub enum CalloutArg {
    Expr(Expr),
    StringLiteral(StringLiteral),
}

#[derive(Debug)]
pub struct Method {
    pub name: MethodName,
    pub args: Vec<Expr>,
}

#[derive(Debug)]
pub struct Callout {
    pub name: StringLiteral,
    pub args: Vec<CalloutArg>,
}


#[derive(Debug)]
pub enum MethodCall {
    Method (Method),
    Callout (Callout),

}
