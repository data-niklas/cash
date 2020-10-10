use crate::context::Context;
use crate::interpreter::Rule;
use crate::result::{ExprResult, Parameter, Result};
use crate::runtime::Runtime;
use std::collections::HashMap;
use std::sync::Arc;
#[path = "functions.rs"]
mod functions;
#[path = "system.rs"]
mod system;
use crate::ast::Node;
use anyhow;
use functions::*;
use std::{thread, time};
use system::*;

pub fn eval(rule: &Node, runtime: &Runtime, ctx: &Context) -> Result {
    let val = rule.content();
    match rule.rule {
        Rule::Ident => {
            return Result::String(rule.content().to_owned());
        }
        Rule::Literal | Rule::Term => {
            return eval(rule.inner().first().unwrap(), runtime, ctx);
        }
        Rule::Function => {
            return eval_function(rule.inner(), runtime, ctx);
        }
        Rule::String => {
            return eval_string(rule.inner(), runtime, ctx);
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

pub fn eval_assignment(inner: &Vec<Node>, runtime: &Runtime, ctx: &Context) -> Result {
    let mut iter = inner.iter();
    let var = iter.next().unwrap().content();
    let next = iter.next().unwrap();
    if let Rule::GetIndex = next.rule() {
        return eval_assignindex(
            var,
            eval(next.inner().first().unwrap(), runtime, ctx),
            iter.next().unwrap().content().as_str(),
            iter.next(),
            runtime,
            ctx,
        );
    }
    let op = next.content().as_str();
    let expr = iter.next();
    let val: Result;
    match op {
        "=" => {
            val = eval(expr.unwrap(), runtime, ctx);
        }
        "*=" => {
            val = multiply(ctx.var(var), eval(expr.unwrap(), runtime, ctx));
        }
        "/=" => {
            val = divide(ctx.var(var), eval(expr.unwrap(), runtime, ctx));
        }
        "+=" => {
            val = add(ctx.var(var), eval(expr.unwrap(), runtime, ctx));
        }
        "-=" => {
            val = subtract(ctx.var(var), eval(expr.unwrap(), runtime, ctx));
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
    runtime: &Runtime,
    ctx: &Context,
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
            newval = eval(expr.unwrap(), runtime, ctx);
        }
        "*=" => {
            newval = multiply(indexval, eval(expr.unwrap(), runtime, ctx));
        }
        "/=" => {
            newval = divide(indexval, eval(expr.unwrap(), runtime, ctx));
        }
        "+=" => {
            newval = add(indexval, eval(expr.unwrap(), runtime, ctx));
        }
        "-=" => {
            newval = subtract(indexval, eval(expr.unwrap(), runtime, ctx));
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

pub fn eval_statement(inner: &Vec<Node>, runtime: &Runtime, ctx: &Context) -> Result {
    let pair: &Node;
    let mut isasync: bool = false;
    let first = inner.first().unwrap();
    if let Rule::Async = first.rule() {
        isasync = true;
        pair = inner.get(1).unwrap();
    } else {
        pair = first;
    }
    if isasync {
        let a1 = Arc::clone(&ctx.parent);
        let a2 = Arc::clone(&ctx.vars);
        return Result::Error("Threads are not implemented yet".to_string());
    } else {
        return eval_statementitem(pair, runtime, ctx);
    }
}

pub fn eval_statementitem(pair: &Node, runtime: &Runtime, ctx: &Context) -> Result {
    match pair.rule() {
        Rule::Block => {
            let parent: Context = Context::from_parent(ctx);
            return eval(pair, runtime, &parent);
        }
        _ => {
            return eval(pair, runtime, ctx);
        }
    }
}

pub fn eval_range(inner: &Vec<Node>, runtime: &Runtime, ctx: &Context) -> Result {
    let mut iter = inner.iter();
    if let Result::Int(i1) = eval(iter.next().unwrap(), runtime, ctx) {
        if let Result::Int(i2) = eval(iter.next().unwrap(), runtime, ctx) {
            let mut vec: Vec<Result> = Vec::new();
            for i in i1..i2 {
                vec.push(Result::Int(i));
            }
            return Result::Array(vec);
        }
    }
    return Result::Error("Range may only include ints".to_string());
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
            let newctx = Context::from_parent(ctx);
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
        let newctx = Context::from_parent(ctx);
        eval(block, runtime, &newctx);
    }
    return Result::None;
}

pub fn eval_conditional(inner: &Vec<Node>, runtime: &Runtime, ctx: &Context) -> Result {
    let mut iter = inner.iter();
    let expr = iter.next().unwrap();
    if let Result::Bool(true) = eval(expr, runtime, ctx) {
        let newctx = Context::from_parent(ctx);
        eval(iter.next().unwrap(), runtime, &newctx);
    } else {
        while let Some(node) = iter.next() {
            if let Rule::Expr = node.rule() {
                if let Result::Bool(true) = eval(node, runtime, ctx) {
                    let newctx = Context::from_parent(ctx);
                    eval(iter.next().unwrap(), runtime, &newctx);
                    break;
                }
            } else {
                let newctx = Context::from_parent(ctx);
                eval(node, runtime, &newctx);
            }
        }
    }
    return Result::None;
}

fn eval_call(inner: &Vec<Node>, runtime: &Runtime, ctx: &Context) -> Result {
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

        //Control
        "quit" | "exit" => {
            runtime.quit();
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
            return call_function(func, iter, "", runtime, ctx);
        }
    }
}

pub fn call_function(
    name: &str,
    mut iter: std::slice::Iter<Node>,
    stdin: &str,
    runtime: &Runtime,
    ctx: &Context,
) -> Result {
    let res = eval_chainedcall(ctx.var(name), &mut iter, runtime, ctx);
    if let Result::Error(e) = res {
        if e == "Term is not a function".to_string() {
            return call_sys_function(name, iter, stdin, runtime, ctx);
        }
        return Result::Error(e);
    }
    return res;
}

pub fn call_sys_function(
    name: &str,
    iter: std::slice::Iter<Node>,
    stdin: &str,
    runtime: &Runtime,
    ctx: &Context,
) -> Result {
    if let Ok(path) = Runtime::which(name) {
        let mut args = Vec::new();
        for expr in iter {
            args.push(eval(expr, runtime, ctx).to_string());
        }
        return exec(path, args.iter(), stdin);
    }
    return Result::Error("Function ".to_string() + name + " was not found");
}

pub fn eval_chainedcall(
    input: Result,
    iter: &mut std::slice::Iter<Node>,
    runtime: &Runtime,
    ctx: &Context,
) -> Result {
    if let Result::Function {
        block,
        parameters,
        vars,
    } = input
    {
        let newctx = Context::from_vars(ctx, vars);
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

pub fn eval_array(inner: &Vec<Node>, runtime: &Runtime, ctx: &Context) -> Result {
    let mut vec: Vec<Result> = Vec::new();
    for rule in inner {
        vec.push(eval(&rule, runtime, ctx));
    }
    return Result::Array(vec);
}

pub fn eval_dict(inner: &Vec<Node>, runtime: &Runtime, ctx: &Context) -> Result {
    let mut map: HashMap<String, Result> = HashMap::new();
    for pair in inner {
        let first = pair.inner().get(0).unwrap();
        let txt = eval(&first, runtime, ctx);
        if let Result::String(txt) = txt {
            let val = eval(&pair.inner().get(1).unwrap(), runtime, ctx);
            map.insert(txt, val);
        } else if let Result::Error(e) = txt {
            return Result::Error(e);
        }
    }
    return Result::Dict(map);
}

pub fn eval_string(inner: &Vec<Node>, runtime: &Runtime, ctx: &Context) -> Result {
    let mut text = String::new();
    for node in inner {
        match node.rule() {
            Rule::Text => {
                text += node.content();
            }
            Rule::Interpolation => {
                text += eval(node.inner().first().unwrap(), runtime, ctx)
                    .to_string()
                    .as_str();
            }
            Rule::Escape => {
                match &node.content()[1..]{
                    "n" => {
                        text+="\n";
                    }
                    "r" => {
                        text+="\r";
                    }
                    "t" => {
                        text+="\t";
                    }
                    "b" => {
                        text+="\x7f";
                    }
                    any => {
                        if any.starts_with("x"){
                            if any.len() > 1{
                                text += &std::char::from_u32(u32::from_str_radix(&any[1..], 16).unwrap()).unwrap().to_string();
                                continue;
                            }
                        }
                        text += any;
                    }
                }
            }
            _ => {}
        }
    }
    return Result::String(text);
}

fn eval_unary(inner: &Vec<Node>, runtime: &Runtime, ctx: &Context) -> Result {
    let mut iter = inner.iter();
    let first = iter.next().unwrap();
    match first.rule() {
        Rule::UnaryLOp => {
            return match_unarylop(first, iter.next().unwrap(), runtime, ctx);
        }
        Rule::Term => {
            let second = iter.next();
            if let Some(second) = second {
                match second.rule() {
                    Rule::UnaryROp => {
                        return match_unaryrop(first, second, runtime, ctx);
                    }
                    Rule::GetIndex => {
                        return get_index(
                            &eval(first, runtime, ctx),
                            eval(second.inner().first().unwrap(), runtime, ctx),
                        );
                    }
                    Rule::ChainedCall => {
                        return eval_chainedcall(
                            eval(first, runtime, ctx),
                            &mut second.inner().iter(),
                            runtime,
                            ctx,
                        );
                    }
                    _ => {
                        return Result::Error("Rule::Term followed by unknown Rule".to_string());
                    }
                }
            } else {
                return eval(first, runtime, ctx);
            }
        }
        _ => {
            return eval(first, runtime, ctx);
        }
    }
}

fn match_unarylop(first: &Node, second: &Node, runtime: &Runtime, ctx: &Context) -> Result {
    match first.content().as_str() {
        "+" => {
            return eval(second, runtime, ctx);
        }
        "-" => {
            return multiply(eval(second, runtime, ctx), Result::Int(-1));
        }
        "!" => {
            return negate(eval(second, runtime, ctx));
        }
        _ => {
            return eval(second, runtime, ctx);
        }
    }
}

fn match_unaryrop(first: &Node, second: &Node, runtime: &Runtime, ctx: &Context) -> Result {
    match second.content().as_str() {
        "!" => {
            // TODO
            return faculty(eval(first, runtime, ctx));
        }
        _ => {
            return eval(first, runtime, ctx);
        }
    }
}

fn operator_precedence(op: &str) -> usize {
    match op {
        "==" => {
            return 0;
        }
        "|" => {
            return 1;
        }
        "<=" | ">=" | "<" | ">" => {
            return 2;
        }
        "+" | "-" => {
            return 3;
        }
        "*" | "/" | "%" => {
            return 4;
        }
        "**" | "//" | "^" => {
            return 5;
        }
        _ => {
            return 5;
        }
    }
}

fn eval_expr(rules: &Vec<Node>, runtime: &Runtime, ctx: &Context) -> Result {
    const MAXPRECEDENCE: usize = 6;
    let mut operatorprecedence: [Vec<usize>; MAXPRECEDENCE] = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];
    let mut operators = Vec::<&str>::new();
    let mut results: Vec<ExprResult> = Vec::new();

    for pair in rules {
        let rule = pair.rule();
        match rule {
            Rule::Operator => {
                let mut n = operators.len();
                let op = pair.content().as_str();
                let precedence = operator_precedence(op);
                for j in precedence..MAXPRECEDENCE {
                    n -= operatorprecedence[j].len();
                }
                operatorprecedence[precedence].push(n);
                operators.push(op);
            }
            Rule::UnaryExpr => {
                results.push(ExprResult::Node(pair));
            }
            _ => {}
        }
    }

    let mut i = MAXPRECEDENCE - 1;
    loop {
        let vec = &operatorprecedence[i];
        for j in vec.iter() {
            let op = operators[*j];
            let lhs = results.remove(*j);
            let rhs = results.remove(*j);

            if op == "|" {
                let mut inserted = false;
                if let ExprResult::Result(res) = lhs {
                    if let Result::String(txt) = res {
                        if let ExprResult::Node(res) = rhs {
                            if let Rule::Call = res.rule() {
                                let call = get_call(res);
                                if let Ok(call) = call {
                                    let mut iter = call.inner().iter();
                                    let func = iter.next().unwrap().content().as_str();
                                    results.insert(
                                        *j,
                                        ExprResult::Result(call_function(
                                            func, iter, &txt, runtime, ctx,
                                        )),
                                    );
                                    inserted = true;
                                }
                            }
                        }
                    }
                } else if let ExprResult::Node(node) = lhs {
                    if let Ok(node) = get_call(node) {
                        if let Result::String(txt) = eval(node, runtime, ctx) {
                            if let ExprResult::Node(res) = rhs {
                                let call = get_call(res);
                                if let Ok(call) = call {
                                    let mut iter = call.inner().iter();
                                    let func = iter.next().unwrap().content().as_str();
                                    results.insert(
                                        *j,
                                        ExprResult::Result(call_function(
                                            func, iter, &txt, runtime, ctx,
                                        )),
                                    );
                                    inserted = true;
                                }
                            }
                        }
                    }
                }
                if !inserted {
                    return Result::Error(
                        "| operator may only be used with Strings and system calls".to_string(),
                    );
                }
            } else {
                let lhsr: Result;
                let rhsr: Result;
                if let ExprResult::Result(e) = lhs {
                    lhsr = e;
                } else if let ExprResult::Node(n) = lhs {
                    lhsr = eval_unary(n.inner(), runtime, ctx);
                } else {
                    lhsr = Result::Error("Error in evaluating the expression".to_string());
                }

                if let ExprResult::Result(e) = rhs {
                    rhsr = e;
                } else if let ExprResult::Node(n) = rhs {
                    rhsr = eval_unary(n.inner(), runtime, ctx);
                } else {
                    rhsr = Result::Error("Error in evaluating the expression".to_string());
                }
                results.insert(*j, ExprResult::Result(make_result(op, lhsr, rhsr)));
            }
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }
    let first = results.remove(0);
    if let ExprResult::Result(res) = first {
        return res;
    } else if let ExprResult::Node(node) = first {
        return eval_unary(node.inner(), runtime, ctx);
    } else {
        return Result::Error("Expression could not be evaluated".to_string());
    }
}

fn make_result(op: &str, lhs: Result, rhs: Result) -> Result {
    match op {
        "+" => {
            return add(lhs, rhs);
        }
        "-" => {
            return subtract(lhs, rhs);
        }
        "*" => {
            return multiply(lhs, rhs);
        }
        "/" => {
            return divide(lhs, rhs);
        }
        "%" => {
            return modulo(lhs, rhs);
        }
        "**" | "^" => {
            return power(lhs, rhs);
        }
        "//" => {
            return root(lhs, rhs);
        }
        "==" => {
            return equals(&lhs, &rhs);
        }
        "<=" => {
            return smallereq(&lhs, &rhs);
        }
        ">=" => {
            return greatereq(&lhs, &rhs);
        }
        "<" => {
            return smaller(&lhs, &rhs);
        }
        ">" => {
            return greater(&lhs, &rhs);
        }
        _ => {
            return Result::Error("Unknown operator ".to_owned() + op);
        }
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
