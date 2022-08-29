use crate::token::*;

pub enum SemanticCheckError {
    
    NoMainMethod,         // pass 3
    NonPositiveArraySize, // pass 4
}

// 1. 	 No identifier is declared twice in the same scope. 
// 2. 	 No identifier is used before it is declared. 
// 3. 	 The program contains a definition for a method called main that has no parameters (note that since execution starts at method main, any methods defined after main will never be  executed). 
pub fn has_main(p: &Program) -> Result<(),SemanticCheckError> { 
    let has_main = p.method_decls.iter()
                                 .filter(|&m| m.name == "main" && m.args.len() == 0)
                                 .count() == 1;
    if has_main {
        Ok(())
    } else {
        Err(SemanticCheckError::NoMainMethod)
    }
}

// 4. 	 The <int literal> in an array declaration must be greater than 0. 
pub fn is_array_size_positive(p: &Program) -> Result<(), SemanticCheckError> {
    // FieldDecl: loop over all fields and check array size
    let is_valid = p.field_decls.iter().map(
            |d| d.loc.iter().map(|a| a.arr_size > 0).fold(true, |a, b| a && b)
        ).fold(true, |a, b| a&& b);
    if is_valid { Ok(()) } else { Err(SemanticCheckError::NonPositiveArraySize)}
}
// 5. 	 The number and types of arguments in a method call must be the same as the number and types of the formals, i.e., the signatures must be identical. 
// 6. 	 If a method call is used as an expression, the method must return a result. 
// 7. 	 A return statement must not have a return value unless it appears in the body of a method that is declared to return a value. 
// 8. 	 The expression in a return statement must have the same type as the declared result type of the enclosing method definition. 
// 9. 	 An <id> used as a <location> must name a declared local/global variable or formal parameter. 
// 10. For all locations of the form <id>[<expr>]
//     (a) <id> must be an array variable, and 
//     (b) the type of <expr> must be int. 
// 11. The <expr> in an if statement must have type boolean. 
// 12. The operands of <arith op>s and <rel op>s must have type int. 
// 13. The operands of <eq op>s must have the same type, either int or boolean. 
// 14. The operands of <cond op>s and the operand of logical not (!) must have type boolean. 
// 15. The <location> and the <expr> in an assignment, <location> = <expr>, must have the same type. 
// 16. The <location> and the <expr> in an incrementing/decrementing assignment, <location> += <expr> and <location> -= <expr>, must be of type int. 
// 17. The initial <expr> and the ending <expr> of for must have type int. 
// 18. All break and continue statements must be contained within the body of a for.
