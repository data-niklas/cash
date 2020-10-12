use crate::context::Context;
use crate::interpreter::Rule;
use crate::result::Result;
use crate::runtime::Runtime;
use crate::ast::Node;
use std::collections::HashMap;
use std::sync::Arc;

#[path = "functions.rs"]
mod functions;
#[path = "system.rs"]
mod system;
#[path = "block.rs"]
mod block;
#[path = "literal.rs"]
mod literal;
#[path = "util.rs"]
mod util;
#[path = "expr.rs"]
mod expr;
#[path = "pipe.rs"]
mod pipe;

use anyhow;
use dirs;
use std::{thread, time};
use functions::*;
use system::*;
use block::*;
use literal::*;
use expr::*;
use pipe::*;


pub fn eval_runtime_function(name: &str, runtime: Arc<Runtime>) -> Result{
    return call_function(name, Vec::new().iter(), runtime.clone(), runtime.basectx.clone());
}

pub fn eval(rule: &Node, runtime: Arc<Runtime>, ctx: Arc<Context>) -> Result {
    let val = rule.content();
    match rule.rule {
        Rule::Ident => {
            return Result::String(rule.content().to_owned());
        }
        Rule::Literal | Rule::Term => {
            return eval(rule.inner().first().unwrap(), runtime, ctx);
        }
        Rule::Pipe => {
            return eval_pipe(rule.inner(), runtime, ctx);
        }
        Rule::Function => {
            return eval_function(rule.inner(), runtime, ctx);
        }
        Rule::String => {
            return eval_string(rule.inner(), runtime, ctx);
        }
        Rule::Home => {
            return get_home();
        }
        Rule::Int => {
            return Result::Int(val.parse::<i64>().unwrap());
        }
        Rule::Bool => {
            return Result::Bool(val.parse::<bool>().unwrap());
        }
        Rule::Float => {
            return Result::Float(val.parse::<f64>().unwrap());
        }
        Rule::None => {
            return Result::None;
        }
        Rule::Array => {
            return eval_array(rule.inner(), runtime, ctx);
        }
        Rule::Dict => {
            return eval_dict(rule.inner(), runtime, ctx);
        }
        Rule::UnaryExpr => {
            return eval_unary(rule.inner(), runtime, ctx);
        }
        Rule::Expr => {
            return eval_expr(rule.inner(), runtime, ctx);
        }
        Rule::Var => {
            return ctx.var(rule.inner().first().unwrap().content());
        }
        Rule::Block => {
            return eval_block(rule.inner(), runtime, ctx);
        }
        Rule::Statement => {
            return eval_statement(rule.inner(), runtime, ctx);
        }
        Rule::Range => {
            return eval_range(rule.inner(), runtime, ctx);
        }
        Rule::ForLoop => {
            return eval_forloop(rule.inner(), runtime, ctx);
        }
        Rule::WhileLoop => {
            return eval_whileloop(rule.inner(), runtime, ctx);
        }
        Rule::Conditional => {
            return eval_conditional(rule.inner(), runtime, ctx);
        }
        Rule::Assignment => {
            return eval_assignment(rule.inner(), runtime, ctx);
        }
        Rule::Call => {
            return eval_call(rule.inner(), runtime, ctx);
        }
        Rule::LineComment => {
            return Result::None;
        }
        _ => {
            return Result::Error("Not implemented yet: ".to_string() + rule.content());
        }
    }
}


pub fn eval_assignment(inner: &Vec<Node>, runtime: Arc<Runtime>, ctx: Arc<Context>) -> Result {
    let mut iter = inner.iter();
    let var = iter.next().unwrap().content();
    let next = iter.next().unwrap();
    if let Rule::GetIndex = next.rule() {
        return eval_assignindex(
            var,
            eval(next.inner().first().unwrap(), runtime.clone(), ctx.clone()),
            iter.next().unwrap().content().as_str(),
            iter.next(),
            runtime.clone(),
            ctx.clone(),
        );
    }
    let op = next.content().as_str();
    let expr = iter.next();
    let val: Result;
    match op {
        "=" => {
            val = eval(expr.unwrap(), runtime, ctx.clone());
        }
        "*=" => {
            val = multiply(ctx.var(var), eval(expr.unwrap(), runtime, ctx.clone()));
        }
        "/=" => {
            val = divide(ctx.var(var), eval(expr.unwrap(), runtime, ctx.clone()));
        }
        "+=" => {
            val = add(ctx.var(var), eval(expr.unwrap(), runtime, ctx.clone()));
        }
        "-=" => {
            val = subtract(ctx.var(var), eval(expr.unwrap(), runtime, ctx.clone()));
        }
        "++" => {
            val = add(ctx.var(var), Result::Int(1));
        }
        "--" => {
            val = subtract(ctx.var(var), Result::Int(1));
        }
        _ => {
            return Result::Error("Unknown assignment operator ".to_string() + op);
        }
    }
    ctx.set_var(var, val);
    return Result::None;
}


pub fn eval_assignindex(
    var: &str,
    index: Result,
    op: &str,
    expr: Option<&Node>,
    runtime: Arc<Runtime>,
    ctx: Arc<Context>,
) -> Result {
    let val = ctx.var(var);
    if let Result::Error(e) = val {
        return Result::Error(e);
    }
    let indexval = get_index(&val, index.clone());
    if let Result::Error(e) = indexval {
        return Result::Error(e);
    }
    let newval: Result;
    match op {
        "=" => {
            newval = eval(expr.unwrap(), runtime, ctx.clone());
        }
        "*=" => {
            newval = multiply(indexval, eval(expr.unwrap(), runtime, ctx.clone()));
        }
        "/=" => {
            newval = divide(indexval, eval(expr.unwrap(), runtime, ctx.clone()));
        }
        "+=" => {
            newval = add(indexval, eval(expr.unwrap(), runtime, ctx.clone()));
        }
        "-=" => {
            newval = subtract(indexval, eval(expr.unwrap(), runtime, ctx.clone()));
        }
        "++" => {
            newval = add(indexval, Result::Int(1));
        }
        "--" => {
            newval = subtract(indexval, Result::Int(1));
        }
        _ => {
            return Result::Error("Unknown assignment operator ".to_string() + op);
        }
    }
    ctx.set_var(var, set_index(val, index, newval));
    return Result::None;
}

pub fn eval_statement(inner: &Vec<Node>, runtime: Arc<Runtime>, ctx: Arc<Context>) -> Result {
    let pair: &Node;
    let mut isasync: bool = false;
    let first = inner.get(0).unwrap();
    if let Rule::Async = first.rule() {
        isasync = true;
        pair = inner.get(1).unwrap();
    } else {
        pair = first;
    }
    if isasync {
        let node = pair.clone();
        let newruntime = Arc::clone(&runtime);
        let newctx = Arc::new(Context::from_parent(&*ctx, ctx.node));
        return Result::Error("Threads are not implemented yet".to_string());
    } else {
        return eval_statementitem(pair, runtime, ctx);
    }
}





fn get_call(unary_expr: &Node) -> anyhow::Result<&Node> {
    if let Some(term) = unary_expr.inner().first() {
        if let Some(call) = term.inner().first() {
            if let Rule::Call = call.rule() {
                return Ok(call);
            }
        }
    }
    return Err(anyhow::anyhow!(""));
}
