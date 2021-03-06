use super::*;
use crate::ast::Node;
use crate::context::Context;
use crate::interpreter::Rule;
use crate::result::Result;
use crate::runtime::Runtime;

pub fn eval_unary(inner: &Vec<Node>, runtime: Arc<Runtime>, ctx: Arc<Context>) -> Result {
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
                            &eval(first, runtime.clone(), ctx.clone()),
                            eval(second.inner().first().unwrap(), runtime, ctx),
                        );
                    }
                    Rule::ChainedCall => {
                        return eval_chainedcall(
                            &eval(first, runtime.clone(), ctx.clone()),
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

pub fn match_unarylop(first: &Node, second: &Node, runtime: Arc<Runtime>, ctx: Arc<Context>) -> Result {
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

pub fn match_unaryrop(first: &Node, second: &Node, runtime: Arc<Runtime>, ctx: Arc<Context>) -> Result {
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

pub fn operator_precedence(op: &str) -> usize {
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

pub fn eval_expr(rules: &Vec<Node>, runtime: Arc<Runtime>, ctx: Arc<Context>) -> Result {
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
                results.push(eval_unary(pair.inner(), runtime.clone(),ctx.clone()));
            }
            _ => {}
        }
    }

    let mut i = MAXPRECEDENCE - 1;
    loop {
        let vec = &operatorprecedence[i];
        for j in vec.iter() {
            let op = operators.remove(*j);
            let lhs = results.remove(*j);
            let rhs = results.remove(*j);

            results.insert(*j, make_result(op, lhs, rhs));
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }
    let first = results.remove(0);
    return first;
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
