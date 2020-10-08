use crate::interpreter::Rule;
use crate::result::{Result, Parameter};
use crate::runtime::Runtime;
use crate::context::Context;
use std::collections::HashMap;
#[path = "functions.rs"]
mod functions;
use functions::*;
use crate::ast::Node;

pub fn eval(rule: &Node, runtime: &Runtime, ctx: &Context) -> Result {
    let val = rule.content();
    match rule.rule {
        Rule::Literal | Rule::Term | Rule::Ident => {
            return eval(rule.inner().first().unwrap(), runtime, ctx);
        }
        Rule::Function => {
            return eval_function(rule.inner(), runtime, ctx);
        }
        Rule::String => {
            return Result::String(val[1..val.len() - 1].to_owned());
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
        Rule::Array => {
            return eval_array(rule.inner(), runtime, ctx);
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
            return eval_statement(rule.inner().first().unwrap(), runtime, ctx);
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
            eval_assignment(rule.inner(), runtime, ctx);
            return Result::None;
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

pub fn eval_block(pairs: &Vec<Node>, runtime: &Runtime, ctx: &Context) -> Result{
    let mut lastres = Result::None;
    for pair in pairs {
        let res = eval(pair, runtime, ctx);
        if let Result::Return(e) = res{
            return *e;
        }
        else{
            lastres = res;
        }
    }
    return lastres;
}

pub fn eval_function(pairs: &Vec<Node>, runtime: &Runtime, ctx: &Context) -> Result{
    let mut iter = pairs.iter();
    let first = iter.next().unwrap();
    let block;
    let mut params: Vec<Parameter> = Vec::new();
    if let Rule::FunctionParams = first.rule(){
        for param in first.inner(){
            let inner = param.inner();
            let ident = inner.first().unwrap().content();
            let mut defaultvalue = None;
            if inner.len() == 2{
                defaultvalue = Some(eval(inner.get(1).unwrap(), runtime, ctx));
            }
            params.push(Parameter{
                defaultvalue: defaultvalue,
                name: ident.clone()
            });
        }
        block = iter.next().unwrap().clone();
    }
    else{
        block = first.clone();
    }
    let mut vars = HashMap::new();
    if let Some(_) = ctx.parent{
        vars = ctx.vars.borrow().clone();
    }

    return Result::Function{
        block: block,
        parameters: params,
        vars: vars
    };
}

pub fn eval_assignment(inner: &Vec<Node>, runtime: &Runtime, ctx: & Context){
    let mut iter = inner.iter();
    let var = iter.next().unwrap().content();
    let op = iter.next().unwrap().content().as_str();
    let expr = iter.next();
    let val: Result;
    match op{
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
            panic!("Unknown assignment operator");
        }
    }
    ctx.set_var(var, val);
}

pub fn eval_statement(pair: &Node, runtime: &Runtime, ctx: &Context) -> Result{
    match pair.rule(){
        Rule::Block => {
            let parent: Context = Context::from_parent(ctx);
            return eval(pair, runtime, &parent);
        }
        _ => {
            return eval(pair, runtime, ctx);
        }
    }
}

pub fn eval_range(inner: &Vec<Node>, runtime: &Runtime, ctx: &Context) -> Result{
    let mut iter = inner.iter();
    if let Result::Int(i1) = eval(iter.next().unwrap(), runtime, ctx){
        if let Result::Int(i2) = eval(iter.next().unwrap(), runtime, ctx){
            let mut vec: Vec<Result> = Vec::new();
            for i in i1..i2{
                vec.push(Result::Int(i));
            }
            return Result::Array(vec);
        }
    }
    return Result::Error("Range may only include ints".to_string());
}

pub fn eval_forloop(inner: &Vec<Node>, runtime: &Runtime, ctx: &Context) -> Result{
    let mut iter = inner.iter();
    let varname = iter.next().unwrap().content().as_str();
    let range = eval(iter.next().unwrap(), runtime, ctx);
    if let Result::Error(e) = range{
        return Result::Error(e);
    }
    else if let Result::Array(vec) = range{
        let block = iter.next().unwrap();
        for i in vec{
            let newctx = Context::from_parent(ctx);
            newctx.set_own_var(varname, i);
            eval(block, runtime, &newctx);
        }
    }
    else {
        return Result::Error("For loop can only loop over arrays".to_string());
    }
    return Result::None;
}

pub fn eval_whileloop(inner: &Vec<Node>, runtime: &Runtime, ctx: &Context) -> Result{
    let mut iter = inner.iter();
    let expr = iter.next().unwrap();
    let block = iter.next().unwrap();
    while let Result::Bool(true) = eval(expr, runtime, ctx){
        let newctx = Context::from_parent(ctx);
        eval(block, runtime, &newctx);
    }
    return Result::None;
}

pub fn eval_conditional(inner: &Vec<Node>, runtime: &Runtime, ctx: &Context) -> Result{
    let mut iter = inner.iter();
    let expr = iter.next().unwrap();
    if let Result::Bool(true) = eval(expr, runtime, ctx){
        let newctx = Context::from_parent(ctx);
        eval(iter.next().unwrap(), runtime, &newctx);
    }
    else{
        while let Some(node) = iter.next(){
            if let Rule::Expr = node.rule(){
                if let Result::Bool(true) = eval(node, runtime, ctx){
                    let newctx = Context::from_parent(ctx);
                    eval(iter.next().unwrap(), runtime, &newctx);
                    break;
                }
            }
            else{
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
            return log(eval(iter.next().unwrap(), runtime, ctx),eval(iter.next().unwrap(), runtime, ctx));
        }
        "lg" => {
            return log(eval(iter.next().unwrap(), runtime, ctx),Result::Float(10.0));
        }
        "ld" => {
            return log(eval(iter.next().unwrap(), runtime, ctx),Result::Float(2.0));
        }
        "ln" => {
            return log(eval(iter.next().unwrap(), runtime, ctx),Result::Float(std::f64::consts::E));
        }
        "rand" => {
            return Result::Float(rand::random::<f64>());
        }

        //Types
        "print" => {
            return print(iter, runtime, ctx);
        }
        "println" => {
            return println(iter, runtime, ctx);
        }
        "type" => {
            return Result::String(eval(iter.next().unwrap(),runtime,ctx).typename());
        }
        "len" => {
            return len(eval(iter.next().unwrap(), runtime, ctx));
        }
        "each" => {
            return each(eval(iter.next().unwrap(), runtime, ctx),eval(iter.next().unwrap(), runtime, ctx),runtime,ctx);
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
            if let Option::Some(e) = iter.next(){
                return Result::Return(Box::new(eval(e, runtime, ctx)));
            }
            else {
                return Result::Return(Box::new(Result::None));
            }
        }
        _ => {
            return call_function(func, iter, runtime, ctx);
        }
    }
}

pub fn call_function(name: &str, iter: std::slice::Iter<Node>, runtime: &Runtime, ctx: &Context) -> Result{
    let res = eval_chainedcall(ctx.var(name), iter, runtime, ctx);
    if let Result::Error(e) = res{
        if e == "Term is not a function".to_string(){
            return Result::Error("Function ".to_string() + name + " was not found");
        }
        return Result::Error(e);
    }
    return res;
}


pub fn eval_chainedcall(input: Result, mut iter: std::slice::Iter<Node>, runtime: &Runtime, ctx: &Context) -> Result{
    if let Result::Function{block, parameters, vars} = input{
        let newctx = Context::from_vars(ctx, vars);
        for param in parameters{
            if let Some(exprnode) = iter.next(){
                let expr = eval(exprnode, runtime, ctx);
                newctx.set_own_var(&param.name, expr);
            }
            else{
                if let Some(defaultvalue) = param.defaultvalue{
                    newctx.set_own_var(&param.name, defaultvalue);
                }
                else{
                    return Result::Error("Function parameter needs to be passed, or default value needs to be deckared".to_string());
                }
            }
        }
        return eval(&block, runtime, &newctx);
    }
    else{
        return Result::Error("Term is not a function".to_string());
    }
}

pub fn eval_array(inner: &Vec<Node>, runtime: &Runtime, ctx: &Context) -> Result{
    let mut vec: Vec<Result> = Vec::new();
    for rule in inner{
        vec.push(eval(&rule, runtime, ctx));
    }
    return Result::Array(vec);
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
                match second.rule(){
                    Rule::UnaryROp => {
                        return match_unaryrop(first, second, runtime, ctx);
                    }
                    Rule::GetIndex => {
                        return get_index(eval(first, runtime, ctx), eval(second.inner().first().unwrap(), runtime, ctx), runtime, ctx);
                    }
                    Rule::ChainedCall => {
                        return eval_chainedcall(eval(first, runtime, ctx), second.inner().iter(), runtime, ctx);
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

fn match_unarylop(first: &Node, second: &Node, runtime: &Runtime, ctx: &Context) -> Result{
    match first.content().as_str() {
        "+" => {
            return eval(second, runtime, ctx);
        }
        "-" => {
            return multiply(eval(second,runtime, ctx), Result::Int(-1));
        }
        "!" => {
            return negate(eval(second, runtime, ctx));
        }
        _ => {
            return eval(second, runtime, ctx);
        }
    }
}

fn match_unaryrop(first: &Node, second: &Node, runtime: &Runtime, ctx: &Context) -> Result{
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
        "<=" | ">=" | "<" | ">" => {
            return 1;
        }
        "+" | "-" => {
            return 2;
        }
        "*" | "/" | "%" => {
            return 3;
        }
        "**" | "//" | "^" => {
            return 4;
        }
        _ => {
            return 4;
        }
    }
}

fn eval_expr(rules: &Vec<Node>, runtime: &Runtime, ctx: &Context) -> Result {
    const MAXPRECEDENCE: usize = 5;
    let vec: Vec<usize> = Vec::new();
    let mut operatorprecedence: [Vec<usize>; MAXPRECEDENCE] = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let mut operators = Vec::<&str>::new();
    let mut results: Vec<Result> = Vec::new();

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
                results.push(eval_unary(pair.inner(), runtime, ctx));
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
            results.insert(*j, make_result(op, lhs, rhs));
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }
    return results.remove(0);
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