use crate::result;
use crate::runtime::Runtime;
use anyhow::Result;
use pest::Parser;
use pest_derive::*;
mod eval;

use crate::ast::*;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct Language;

pub fn interpret(text: String, runtime: &Runtime) -> Result<result::Result> {
    //Tokenizer
    let mut pairs = Language::parse(Rule::Block, text.as_str())?;
    let ast = build_ast(pairs.next().unwrap());
    return Ok(eval::eval(&ast, runtime, &runtime.basectx));
}
