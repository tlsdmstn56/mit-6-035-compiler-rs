//! checks before IR construction

use super::SemanticCheckError;
use crate::token::*;


/// Check class program has a main function (pass 3)
///
/// The program contains a definition for a method called 
/// main that has no parameters (note that since execution 
/// starts at method main, any methods defined after main 
/// will never be executed).
pub fn has_main(p: &Program) -> Result<(), SemanticCheckError> {
    let has_main = p
        .method_decls
        .iter()
        .filter(|&m| m.name == "main" && m.args.is_empty())
        .count()
        == 1;
    if has_main {
        Ok(())
    } else {
        Err(SemanticCheckError::NoMainMethod)
    }
}

/// Check all declared array size is positive (pass 4)
///
/// The <int literal> in an array declaration must be greater than 0.
pub fn is_array_size_positive(p: &Program) -> Result<(), SemanticCheckError> {
    let is_valid = p
        .field_decls
        .iter()
        .map(|d| {
            d.loc
                .iter()
                .filter(|a| a.arr_size.is_some())
                .map(|a| a.arr_size.unwrap() > 0)
                .all(|b|b)
        }).all(|b|b);
    if is_valid {
        Ok(())
    } else {
        Err(SemanticCheckError::NonPositiveArraySize)
    }
}
