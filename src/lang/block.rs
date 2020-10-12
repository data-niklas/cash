use super::*;
use crate::ast::Node;
use crate::context::Context;
use crate::interpreter::Rule;
use crate::result::{Parameter, Result};
use crate::runtime::Runtime;
use std::collections::HashMap;

pub fn eval_block(pairs: &Vec<Node>, runtime: &Runtime, ctx: &Context) -> Result {
    let mut lastres = Result::None;
    for pair in pairs {
        let res = eval(pair, runtime, ctx);
        if let Result::Return(e) = res {
            return *e;
        } else {
            lastres = res;
        }
    }
    return lastres;
}

pub fn eval_function(pairs: &Vec<Node>, runtime: &Runtime, ctx: &Context) -> Result {
    let mut iter = pairs.iter();
    let first = iter.next().unwrap();
    let block;
    let mut params: Vec<Parameter> = Vec::new();
    if let Rule::FunctionParams = first.rule() {
        for param in first.inner() {
            let inner = param.inner();
            let ident = inner.first().unwrap().content();
            let mut defaultvalue = None;
            if inner.len() == 2 {
                defaultvalue = Some(eval(inner.get(1).unwrap(), runtime, ctx));
            }
            params.push(Parameter {
                defaultvalue: defaultvalue,
                name: ident.clone(),
            });
        }
        block = iter.next().unwrap().clone();
    } else {
        block = first.clone();
    }
    let mut vars = HashMap::new();
    if let Some(_) = *ctx.parent {
        vars = ctx.vars.lock().unwrap().clone();
    }

    return Result::Function {
        block: block,
        parameters: params,
        vars: vars,
    };
}

pub fn eval_forloop(inner: &Vec<Node>, runtime: &Runtime, ctx: &Context) -> Result {
    let mut iter = inner.iter();
    let varname = iter.next().unwrap().content().as_str();
    let range = eval(iter.next().unwrap(), runtime, ctx);
    if let Result::Error(e) = range {
        return Result::Error(e);
    } else if let Result::Array(vec) = range {
        let block = iter.next().unwrap();
        for i in vec {
            let newctx = Context::from_parent(ctx, ctx.me());
            newctx.set_own_var(varname, i);
            eval(block, runtime, &newctx);
        }
    } else {
        return Result::Error("For loop can only loop over arrays".to_string());
    }
    return Result::None;
}

pub fn eval_whileloop(inner: &Vec<Node>, runtime: &Runtime, ctx: &Context) -> Result {
    let mut iter = inner.iter();
    let expr = iter.next().unwrap();
    let block = iter.next().unwrap();
    while let Result::Bool(true) = eval(expr, runtime, ctx) {
        let newctx = Context::from_parent(ctx, ctx.me());
        eval(block, runtime, &newctx);
    }
    return Result::None;
}

pub fn eval_conditional(inner: &Vec<Node>, runtime: &Runtime, ctx: &Context) -> Result {
    let mut iter = inner.iter();
    let expr = iter.next().unwrap();
    if let Result::Bool(true) = eval(expr, runtime, ctx) {
        let newctx = Context::from_parent(ctx, ctx.me());
        return eval(iter.next().unwrap(), runtime, &newctx);
    } else {
        iter.next().unwrap();
        while let Some(node) = iter.next() {
            if let Rule::Expr = node.rule() {
                if let Result::Bool(true) = eval(node, runtime, ctx) {
                    let newctx = Context::from_parent(ctx, ctx.me());
                    return eval(iter.next().unwrap(), runtime, &newctx);
                } else {
                    iter.next().unwrap();
                }
            } else {
                let newctx = Context::from_parent(ctx, ctx.me());
                return eval(node, runtime, &newctx);
            }
        }
    }
    return Result::None;
}

pub fn eval_call(inner: &Vec<Node>, runtime: &Runtime, ctx: &Context) -> Result {
    let mut iter = inner.iter();
    let func = iter.next().unwrap().content().as_str();
    match func {
        //Math
        "abs" => {
            return abs(eval(iter.next().unwrap(), runtime, ctx));
        }
        "ceil" => {
            return ceil(eval(iter.next().unwrap(), runtime, ctx));
        }
        "floor" => {
            return floor(eval(iter.next().unwrap(), runtime, ctx));
        }
        "round" => {
            return round(eval(iter.next().unwrap(), runtime, ctx));
        }
        "signum" => {
            return signum(eval(iter.next().unwrap(), runtime, ctx));
        }
        "sin" => {
            return sin(eval(iter.next().unwrap(), runtime, ctx));
        }
        "cos" => {
            return cos(eval(iter.next().unwrap(), runtime, ctx));
        }
        "tan" => {
            return tan(eval(iter.next().unwrap(), runtime, ctx));
        }
        "asin" => {
            return asin(eval(iter.next().unwrap(), runtime, ctx));
        }
        "acos" => {
            return acos(eval(iter.next().unwrap(), runtime, ctx));
        }
        "atan" => {
            return atan(eval(iter.next().unwrap(), runtime, ctx));
        }
        "sinh" => {
            return sinh(eval(iter.next().unwrap(), runtime, ctx));
        }
        "cosh" => {
            return cosh(eval(iter.next().unwrap(), runtime, ctx));
        }
        "tanh" => {
            return tanh(eval(iter.next().unwrap(), runtime, ctx));
        }
        "asinh" => {
            return asinh(eval(iter.next().unwrap(), runtime, ctx));
        }
        "acosh" => {
            return acosh(eval(iter.next().unwrap(), runtime, ctx));
        }
        "atanh" => {
            return atanh(eval(iter.next().unwrap(), runtime, ctx));
        }
        "log" => {
            return log(
                eval(iter.next().unwrap(), runtime, ctx),
                eval(iter.next().unwrap(), runtime, ctx),
            );
        }
        "lg" => {
            return log(
                eval(iter.next().unwrap(), runtime, ctx),
                Result::Float(10.0),
            );
        }
        "ld" => {
            return log(eval(iter.next().unwrap(), runtime, ctx), Result::Float(2.0));
        }
        "ln" => {
            return log(
                eval(iter.next().unwrap(), runtime, ctx),
                Result::Float(std::f64::consts::E),
            );
        }
        "rand" => {
            return Result::Float(rand::random::<f64>());
        }

        //Types
        "type" => {
            return Result::String(eval(iter.next().unwrap(), runtime, ctx).typename());
        }
        "int" => {
            return cast_int(eval(iter.next().unwrap(), runtime, ctx));
        }
        "float" => {
            return cast_float(eval(iter.next().unwrap(), runtime, ctx));
        }
        "bool" => {
            return cast_bool(eval(iter.next().unwrap(), runtime, ctx));
        }
        "string" => {
            return cast_string(eval(iter.next().unwrap(), runtime, ctx));
        }
        "print" => {
            return print(iter, runtime, ctx);
        }
        "println" => {
            return println(iter, runtime, ctx);
        }
        "len" => {
            return len(eval(iter.next().unwrap(), runtime, ctx));
        }
        "each" => {
            return each(
                eval(iter.next().unwrap(), runtime, ctx),
                eval(iter.next().unwrap(), runtime, ctx),
                runtime,
                ctx,
            );
        }
        "map" => {
            return map(
                eval(iter.next().unwrap(), runtime, ctx),
                eval(iter.next().unwrap(), runtime, ctx),
                runtime,
                ctx,
            );
        }
        "join" => {
            return join(
                eval(iter.next().unwrap(), runtime, ctx),
                eval(iter.next().unwrap(), runtime, ctx),
                runtime,
                ctx,
            );
        }

        //Control
        "quit" | "exit" => {
            runtime.quit();
            return Result::None;
        }
        "cd" => {
            change_dir(iter.next(), runtime, ctx);
            return Result::None;
        }
        "include" => {
            include_file(iter.next(), runtime, ctx);
            return Result::None;
        }
        "clear" | "cls" => {
            runtime.clear();
            return Result::None;
        }
        "help" => {
            print_help(inner, runtime, ctx);
            return Result::None;
        }
        "vars" => {
            return vars(inner, runtime, ctx);
        }
        "return" => {
            if let Option::Some(e) = iter.next() {
                return Result::Return(Box::new(eval(e, runtime, ctx)));
            } else {
                return Result::Return(Box::new(Result::None));
            }
        }
        "me" => {
            if let Some(func) = ctx.me() {
                return eval_chainedcall(func, &mut iter, runtime, ctx);
            } else {
                return Result::Error("Me no t possible".to_string());
            }
        }

        //Time
        "wait" => {
            if let Option::Some(e) = iter.next() {
                let e = eval(e, runtime, ctx);
                if let Result::Int(i) = e {
                    thread::sleep(time::Duration::from_secs(i as u64));
                } else if let Result::Float(f) = e {
                    thread::sleep(time::Duration::from_secs_f64(f));
                }
            }
            return Result::None;
        }
        _ => {
            return call_function(func, iter, runtime, ctx);
        }
    }
}

pub fn call_function(
    name: &str,
    mut iter: std::slice::Iter<Node>,
    runtime: &Runtime,
    ctx: &Context,
) -> Result {
    let res = eval_chainedcall(&ctx.var(name), &mut iter, runtime, ctx);
    if let Result::Error(e) = res {
        if e == "Term is not a function".to_string() {
            return call_sys_function(name, iter, runtime, ctx);
        }
        return Result::Error(e);
    }
    return res;
}

pub fn call_sys_function(
    name: &str,
    iter: std::slice::Iter<Node>,
    runtime: &Runtime,
    ctx: &Context,
) -> Result {
    if let Ok(path) = Runtime::which(name) {
        let mut args = Vec::new();
        for expr in iter {
            args.push(eval(expr, runtime, ctx).to_string());
        }
        return exec(path, args.iter());
    }
    return Result::Error("Function ".to_string() + name + " was not found");
}

pub fn eval_chainedcall(
    input: &Result,
    iter: &mut std::slice::Iter<Node>,
    runtime: &Runtime,
    ctx: &Context,
) -> Result {
    if let Result::Function {
        block,
        parameters,
        vars,
    } = input.clone()
    {
        let newctx = Context::from_vars(ctx, vars, Some(&input));
        for param in parameters {
            if let Some(exprnode) = iter.next() {
                let expr = eval(exprnode, runtime, ctx);
                newctx.set_own_var(&param.name, expr);
            } else {
                if let Some(defaultvalue) = param.defaultvalue {
                    newctx.set_own_var(&param.name, defaultvalue);
                } else {
                    return Result::Error("Function parameter needs to be passed, or default value needs to be deckared".to_string());
                }
            }
        }
        return eval(&block, runtime, &newctx);
    } else {
        return Result::Error("Term is not a function".to_string());
    }
}

pub fn eval_statementitem(pair: &Node, runtime: &Runtime, ctx: &Context) -> Result {
    match pair.rule() {
        Rule::Block => {
            let parent: Context = Context::from_parent(ctx, ctx.me());
            return eval(pair, runtime, &parent);
        }
        _ => {
            return eval(pair, runtime, ctx);
        }
    }
}