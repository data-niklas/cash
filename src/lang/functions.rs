use super::*;
use crate::result::Result;
use std::io::prelude::*;
use crate::interpreter;


#[path = "doc.rs"]
mod doc;
use doc::*;

#[path = "math.rs"]
mod math;
pub use math::*;

#[path = "type.rs"]
mod typefunctions;
pub use typefunctions::*;

pub fn print_help(args: &Vec<Node>, runtime: Arc<Runtime>, ctx: Arc<Context>) {
    if let Some(pair) = args.get(1) {
        let stringresult = eval(pair, runtime, ctx);
        if let Result::String(txt) = stringresult {
            for (key, val) in FUNCTIONS.iter() {
                if key.contains(txt.as_str()) {
                    println!("\x1b[1m{}\x1b[0m\t{}", key, val);
                }
            }
            return;
        }
    }
    for (key, val) in FUNCTIONS.iter() {
        println!("\x1b[1m{}\x1b[0m\t{}", key, val);
    }
}

pub fn change_dir(node: Option<&Node>, runtime: Arc<Runtime>, ctx: Arc<Context>) {
    if let Some(node) = node{
        if let Result::String(path) = eval(&node,runtime,ctx){
            std::env::set_current_dir(&std::path::Path::new(&path));
        }
    }
}

pub fn include_file(node: Option<&Node>, runtime: Arc<Runtime>, ctx: Arc<Context>) -> Result {
    if let Some(node) = node{
        if let Result::String(pathstring) = eval(&node,runtime.clone(),ctx.clone()){
            if let Ok(res) = runtime.include_file(std::path::Path::new(pathstring.as_str())){
                return res;
            }
            else{
                return Result::Error("File ".to_string() + &pathstring + " could not be interpreted");
            }
        }
    }
    return Result::Error("Only files from String file paths may be included".to_string());
}

pub fn exec_func(
    mut iter: std::slice::Iter<Result>,
    function: &Result,
    runtime: Arc<Runtime>,
    ctx: Arc<Context>,
) -> Result {
    if let Result::Function {
        block,
        parameters,
        vars,
    } = function
    {
        let newctx = Context::from_vars(&*ctx, vars.clone(), Some(function));
        for param in parameters {
            if let Some(expr) = iter.next() {
                newctx.set_own_var(&param.name, expr.clone());
            } else {
                if let Some(defaultvalue) = param.defaultvalue.clone() {
                    newctx.set_own_var(&param.name, defaultvalue);
                } else {
                    return Result::Error("Function parameter needs to be passed, or default value needs to be deckared".to_string());
                }
            }
        }
        return eval(&block, runtime, Arc::new(newctx));
    }
    return Result::Error("Is not a function".to_string());
}
