use crate::result::Result;
use super::super::*;

pub fn print(args: std::slice::Iter<Node>, runtime: &Runtime, ctx: &Context) -> Result{
    let mut res: String = String::new();
    for arg in args {
        let arg = eval(arg, runtime, ctx);
        if let Result::None = arg {
        } else {
            res += arg.to_string().as_str();
        }
    }
    println!("{}",res);
    return Result::None;
}

pub fn println(args: std::slice::Iter<Node>, runtime: &Runtime, ctx: &Context) -> Result{
    let mut res: String = String::new();
    for arg in args {
        let arg = eval(arg, runtime, ctx);
        if let Result::None = arg {
        } else {
            res += arg.to_string().as_str();
            res += "\n";
        }
    }
    if res.len() > 0 {
        res.pop();
    }
    println!("{}",res);
    return Result::None;
}


pub fn vars(args: &Vec<Node>, runtime: &Runtime, ctx: &Context) -> Result{
    let mut filter = "".to_owned();
    if args.len() == 2{
        if let Result::String(text) = eval(args.get(1).unwrap(),runtime,ctx){
            filter = text.clone();
        }
    }
    let mut matched = Vec::new();
    let names = ctx.get_all_var_names();
    for name in names{
        if name.contains(filter.as_str()){
            matched.push(Result::String(name.clone()));
        }
    }

    return Result::Array(matched);
}

pub fn len(input: Result) -> Result{
    if let Result::String(text) = input{
        return Result::Int(text.len() as i64);
    }
    else if let Result::Array(ar) = input{
        return Result::Int(ar.len() as i64);
    }
    else if let Result::Error(e) = input{
        return Result::Error(e);
    }
    else{
        return Result::Error("Len may only be called on arrays or strings".to_string());
    }
}

pub fn each(array: Result, function: Result, runtime: &Runtime, ctx: &Context) -> Result{
    if let Result::Array(ar) = array{
        if let Result::Function{block, parameters, vars} = function{
            let mut index = 0 as i64;
            for item in ar{
                let mut itemar = Vec::new();
                itemar.push(item);
                itemar.push(Result::Int(index));
                exec_func(itemar.iter(), &block, &parameters, &vars, runtime, ctx);
                index+=1;
            }
            return Result::None;
        }
        else if let Result::Error(e) = function{
            return Result::Error(e);
        } 
    }
    else if let Result::Error(e) = array{
        return Result::Error(e);
    }
    return Result::Error("Len may only be called on arrays or strings".to_string());
}

pub fn get_index(input: Result, index: Result, runtime: &Runtime, ctx: &Context) -> Result{
    if let Result::Array(arr) = input{
        if let Result::Int(i) = index{
            return arr[i as usize].clone();
        }
        else if let Result::Error(e) = index{
            return Result::Error(e);
        }
        else{
            return Result::Error("Array may only be index by an int".to_string());
        }
    }
    else if let Result::String(text) = input{
        if let Result::Int(i) = index{
            return Result::String(text.chars().skip(i as usize).take(1).collect());
        }
        else if let Result::Error(e) = index{
            return Result::Error(e);
        }
        else{
            return Result::Error("Array may only be index by an int".to_string());
        }
    }
    return Result::None;
}