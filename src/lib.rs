#[macro_use] extern crate lalrpop_util;

mod token;
mod parser;
mod semantic_analyzer;
lalrpop_mod!(#[allow(clippy::all)] decaf);
