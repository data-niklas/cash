use super::*;
use crate::ast::Node;
use crate::context::Context;
use crate::interpreter::Rule;
use crate::result::{Parameter, Result};
use crate::runtime::Runtime;
use std::collections::HashMap;

pub fn get_home() -> Result{
    if let Some(path) = dirs::home_dir(){
        return Result::String(path.as_path().to_str().unwrap_or("").to_owned());
    }
    else{
        return Result::Error("Could not get home directory".to_string());
    }
}

pub fn eval_range(inner: &Vec<Node>, runtime: Arc<Runtime>, ctx: Arc<Context>) -> Result {
    let mut iter = inner.iter();
    if let Result::Int(i1) = eval(iter.next().unwrap(), runtime.clone(), ctx.clone()) {
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


pub fn eval_array(inner: &Vec<Node>, runtime: Arc<Runtime>, ctx: Arc<Context>) -> Result {
    let mut vec: Vec<Result> = Vec::new();
    for rule in inner {
        vec.push(eval(&rule, runtime.clone(), ctx.clone()));
    }
    return Result::Array(vec);
}

pub fn eval_dict(inner: &Vec<Node>, runtime: Arc<Runtime>, ctx: Arc<Context>) -> Result {
    let mut map: HashMap<String, Result> = HashMap::new();
    for pair in inner {
        let first = pair.inner().get(0).unwrap();
        let txt = eval(&first, runtime.clone(), ctx.clone());
        if let Result::String(txt) = txt {
            let val = eval(&pair.inner().get(1).unwrap(), runtime.clone(), ctx.clone());
            map.insert(txt, val);
        } else if let Result::Error(e) = txt {
            return Result::Error(e);
        }
    }
    return Result::Dict(map);
}

pub fn eval_string(inner: &Vec<Node>, runtime: Arc<Runtime>, ctx: Arc<Context>) -> Result {
    let mut text = String::new();
    for node in inner {
        match node.rule() {
            Rule::Text => {
                text += node.content();
            }
            Rule::Interpolation => {
                text += eval(node.inner().first().unwrap(), runtime.clone(), ctx.clone())
                    .to_string()
                    .as_str();
            }
            Rule::Escape => match &node.content()[1..] {
                "n" => {
                    text += "\n";
                }
                "r" => {
                    text += "\r";
                }
                "t" => {
                    text += "\t";
                }
                "b" => {
                    text += "\x7f";
                }
                any => {
                    if any.starts_with("x") {
                        if any.len() > 1 {
                            text +=
                                &std::char::from_u32(u32::from_str_radix(&any[1..], 16).unwrap())
                                    .unwrap()
                                    .to_string();
                            continue;
                        }
                    }
                    text += any;
                }
            },
            _ => {}
        }
    }
    return Result::String(text);
}