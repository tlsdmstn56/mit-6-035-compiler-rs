mod pre_ir_check;
pub use pre_ir_check::*;

#[derive(Debug)]
pub enum SemanticCheckError {
    NonAsciiCharLiteral(char),
    TypeMismatch(String),               // pass 12, 13, 14
    DuplicatedSymbol(String),           // pass 1 
    UnknownSymbol(String),              // pass 2, 9 
    NoMainMethod,                       // pass 3
    NonPositiveArraySize,               // pass 4
    ExprCallNoReturn,                   // pass 6
    ReturnTypeMismatch,                 // pass 7,8
    ArrayLocationOnNonArrayVar,         // pass 10.a,
    ArrayLocationOffsetTypeError,       // pass 10.b
    ContinueOutOfForScope,
    BreakOutOfForScope,
    MethodArgumentNotMatch,
}

pub type IRResult<T> = Result<T, Vec<SemanticCheckError>>;

