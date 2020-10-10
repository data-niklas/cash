use super::*;
use crate::result::Result;

#[path = "doc.rs"]
mod doc;
use doc::*;

#[path = "math.rs"]
mod math;
pub use math::*;

#[path = "type.rs"]
mod typefunctions;
pub use typefunctions::*;


pub fn print_help(args: &Vec<Node>, runtime: &Runtime, ctx: &Context){
    if let Some(pair) = args.get(1){
        let stringresult = eval(pair, runtime, ctx);
        if let Result::String(txt) = stringresult{
            for (key,val) in FUNCTIONS.iter(){
                if key.contains(txt.as_str()){
                    println!("\x1b[1m{}\x1b[0m\t{}",key,val);
                }
            }
            return;
        }
    }
    for (key,val) in FUNCTIONS.iter(){
        println!("\x1b[1m{}\x1b[0m\t{}",key,val);
    }
}


pub fn exec_func(mut iter: std::slice::Iter<Result>, block: &Node, parameters: &Vec<Parameter>, vars: &HashMap<String, Result>, runtime: &Runtime, ctx: &Context) -> Result{
    let newctx = Context::from_vars(ctx, vars.clone());
    for param in parameters{
        if let Some(expr) = iter.next(){
            newctx.set_own_var(&param.name, expr.clone());
        }
        else{
            if let Some(defaultvalue) = param.defaultvalue.clone(){
                newctx.set_own_var(&param.name, defaultvalue);
            }
            else{
                return Result::Error("Function parameter needs to be passed, or default value needs to be deckared".to_string());
            }
        }
    }
    return eval(&block, runtime, &newctx);
}
