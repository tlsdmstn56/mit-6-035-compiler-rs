// #[derive(Debug)]
// pub struct FieldDecl {
//     pub type_: Type,
//     pub loc: Vec<FieldDecl0>,
// }
// 
// #[derive(Debug)]
// pub struct VarDecl {
//     pub type_: Type,
//     pub identifiers: Vec<Identifier>,
// }
// 
// #[derive(Debug)]
// pub struct Block {
//     pub var_decls: Vec<VarDecl>,
//     pub statements: Vec<Statement>,
// }
// 
// #[derive(Debug)]
// pub enum Statement {
//     Assign{dst:Location, op: AssignOp, val: Expr},
//     MethodCall(MethodCall),
//     IfElse{cond:Expr, true_block: Block, false_block: Option<Block>},
//     Loop{index_var: Identifier, start: Expr, end: Expr, block: Block},
//     Return{ val: Option<Expr> },
//     Break,
//     Continue,
//     Block(Block)
// }
// 
// #[derive(Debug)]
// pub struct Location {
//     pub name: String,
//     pub arr_size: Expr,
// }
// 
// #[derive(Debug)]
// pub struct FieldDecl0 {
//     pub name: String,
//     pub arr_size: i32,
// }
// 
// #[derive(Debug)]
// pub struct MethodArg {
//     pub type_: Type,
//     pub name: Identifier,
// }
// 
// #[derive(Debug)]
// pub struct MethodDecl {
//     pub return_type: Type,
//     pub name: Identifier,
//     pub args: Vec<MethodArg>,
//     pub block: Block,
// }
// 
// #[derive(Debug)]
// pub struct Program {
//     pub field_decls: Vec<FieldDecl>,
//     pub method_decls: Vec<MethodDecl>,
// }
// 
// 
// #[derive(Debug)]
// pub enum Literal {
//     Int(IntLiteral),
//     Bool(BoolLiteral),
//     Char(CharLiteral),
// }
// 
// 
// 
// 
// 
// 
// 
// // ------------------------------------------------------
// #[derive(Debug)]
// pub enum Statement {
//     Assign{dst:Location, op: AssignOp, val: Expr},
//     MethodCall(MethodCall),
//     IfElse{cond:Expr, true_block: Block, false_block: Option<Block>},
//     Loop{index_var: Identifier, start: Expr, end: Expr, block: Block},
//     Return{ val: Option<Expr> },
//     Break,
//     Continue,
//     Block(Block)
// }
// #[derive(Debug)]
// pub enum AssignOp {
//     Assign,
//     AddAssign,
//     SubAssign,
//     MulAssign,
//     DivAssign,
// }
// 
// #[derive(Debug)]
// pub enum Type {
//     Int, 
//     Bool,
//     Void,
// }
// 
// #[derive(Debug)]
// pub enum BinaryOp {
//     Or,   // logical or
//     And,  // logical and
//     EQ,   // == 
//     NE,   // !=
//     GT,   // >
//     LT,   // <
//     GE,   // >=
//     LE,   // <=
//     Add,  // +
//     Sub,  // -
//     Mul,  // *
//     Div,  // /
//     Mod,  // %
// }
// 
// #[derive(Debug)]
// pub struct Binary {
//     pub lhs : Expr,
//     pub rhs : Expr,
//     pub op : BinaryOp,
// }
// #[derive(Debug)]
// pub enum UnaryOp {
//     NegInt,
//     NegBool,
// }
// 
// #[derive(Debug)]
// pub struct Unary {
//     pub expr: Expr,
//     pub op: UnaryOp,
// }
// 
// #[derive(Debug)]
// pub enum CalloutArg {
//     Expr(Expr),
//     StringLiteral(StringLiteral),
// }
// 
// #[derive(Debug)]
// pub enum Call {
//     Method{
//         name: MethodName,
//         args: Vec<Expr>,
//     },
//     Callout{
//         name: StringLiteral,
//         args: Vec<CalloutArg>,
//     },
// }
// 
// #[derive(Debug)]
// pub enum Literal {
//     Int(i32),
//     Boolean(bool),
// }
// 
// pub type Expr = Rc<RefCell<Expr0>>;
// 
// #[derive(Debug)]
// pub enum Expr0 {
//     Literal(Literal),
//     Call(Call),
//     Unary(Unary),
//     Binary(Binary),
// }
// 
// pub type Statement = Rc<RefCell<Statement0>>;
// 
// #[derive(Debug)]
// pub enum Statement0 {
//     Assign,
//     Break,
//     Continue,
//     If,
//     For,
//     Return,
//     Invoke(Call),
//     Block,
// }
// 
// #[derive(Debug)]
// pub enum IR {
//     Expr(Expr),
//     Statement(Statement), 
//     ClassDecl {
//     },
//     MemberDecl {
//     },
//     VarDecl {
//     }
//     Type(Type),
// }
// 
// pub type IntLiteral = i32;
// pub type DecimalLiteral = i32;
// pub type HexLiteral = i32;
// pub type CharLiteral = char;
// pub type StringLiteral = String;
// pub type MethodName = String;
// pub type Alphabet = char;
// pub type Digit = char;
// pub type Char = char;
// pub type AlphaNum = char;
// pub type Identifier = String;
// 
// 
// 
// 
