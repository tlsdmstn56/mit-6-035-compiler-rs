use super::SemanticCheckError;
use crate::token::*;

// ----------------------------------------
// checks before IR construction
// ----------------------------------------
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
