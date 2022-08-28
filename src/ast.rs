pub struct FieldDecl {
    pub r#type: Type,
    pub loc: Vec<Location>,
}

pub struct VarDecl {
    pub r#type: Type,
    pub identifiers: Vec<Identifier>,
}

pub struct Block {
    pub var_decls: Vec<VarDecl>,
    pub statements: Vec<Statement>,
}

pub enum Statement {
    Assign{dst:Location, op: AssignOp, val: Expr},
    MethodCall(MethodCall),
    IfElse{cond:Expr, true_block: Block, false_block: Option<Block>},
    Loop{index_var: Identifier, start: Expr, end: Expr, block: Block},
    Return{ val: Option<Expr> },
    Break,
    Continue,
    Block(Block)
}

pub struct Location {
    pub name: String,
    pub arr_size: i32,
}

pub struct MethodArg {
    pub r#type: Type,
    pub name: Identifier,
}

pub struct MethodDecl {
    pub return_type: Type,
    pub name: Identifier,
    pub args: Vec<MethodArg>,
}

pub struct Program {
    pub field_decls: Vec<FieldDecl>,
    pub method_decls: Vec<MethodDecl>,
}


pub type MethodName = String;
pub type Alphabet = char;
pub type Digit = char;
pub type Char = char;
pub type AlphaNum = char;
pub type Identifier = String;

pub enum Literal {
    Int(IntLiteral),
    Bool(BoolLiteral),
    Char(CharLiteral),
}

pub enum BinaryOp {
    Arith(ArithOp),
    Compare(CompareOp),
    Eq(EqOp),
    Cond(CondOp),
}

pub enum AssignOp {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
}

pub enum Type {
    Int, 
    Bool,
    Void,
}

pub enum ArithOp {
    Add, 
    Sub,
    Mul,
    Div,
    Mod,
}

pub enum CompareOp {
   GT,
   GE,
   LT,
   LE,
}

pub enum EqOp {
   EQ,
   NE,
}

pub enum CondOp {
   Or,
   And,
}


pub type IntLiteral = i32;
pub type DecimalLiteral = i32;
pub type HexLiteral = i32;
pub enum BoolLiteral {
    True,
    False, 
}
pub type CharLiteral = char;
pub type StringLiteral = String;

pub struct Binary {
    pub lhs : Expr,
    pub rhs : Expr,
    pub op : BinaryOp,
}

pub enum UnaryOp {
    NegInt,
    NegBool,
}

pub struct Unary {
    pub expr: Expr,
    pub op: UnaryOp,
}

pub type Expr = Box<Expr0>;

pub enum Expr0 {
    Location(Location),
    MethodCall(MethodCall),
    Literal(Literal),
    Unary(Unary),
    Binary(Binary),
}

pub enum CalloutArg {
    Expr(Expr),
    StringLiteral(StringLiteral),
}


pub enum MethodCall {
    Method{
        name: MethodName,
        args: Vec<Expr>,
    },
    Callout{
        name: StringLiteral,
        args: Vec<CalloutArg>,
    },
}





