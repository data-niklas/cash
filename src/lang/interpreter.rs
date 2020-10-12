use crate::result;
use crate::context::Context;
use crate::runtime::Runtime;
use anyhow::Result;
use pest::Parser;
use pest_derive::*;
mod eval;
use std::sync::Arc;
use crate::ast::*;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct Language;

pub fn interpret(text: String, runtime: Arc<Runtime>, ctx: Arc<Context>) -> Result<result::Result> {
    //Tokenizer
    let mut pairs = Language::parse(Rule::Block, text.as_str())?;
    let ast = build_ast(pairs.next().unwrap());
    //println!("{:?}",ast);
    //ast.to_string();
    return Ok(eval::eval(&ast, runtime, ctx));
}

pub fn interpret_function(name: &str, runtime: Arc<Runtime>) -> result::Result {
    return eval::eval_runtime_function(name, runtime);
}
