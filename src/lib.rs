#[macro_use] extern crate lalrpop_util;

pub mod ast;
pub mod parser;
lalrpop_mod!(pub decaf);
