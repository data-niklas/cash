use super::super::*;
use crate::result::Result;

pub fn print(args: std::slice::Iter<Node>, runtime: Arc<Runtime>, ctx: Arc<Context>) -> Result {
    for arg in args {
        let arg = eval(arg, runtime.clone(), ctx.clone());
        if let Result::None = arg {
        } else {
            print!("{}", arg.to_string());
        }
    }
    return Result::None;
}

pub fn println(args: std::slice::Iter<Node>, runtime: Arc<Runtime>, ctx: Arc<Context>) -> Result {
    for arg in args {
        let arg = eval(arg, runtime.clone(), ctx.clone());
        if let Result::None = arg {
        } else {
            println!("{}", arg.to_string());
        }
    }
    return Result::None;
}

pub fn vars(args: &Vec<Node>, runtime: Arc<Runtime>, ctx: Arc<Context>) -> Result {
    let mut filter = "".to_owned();
    if args.len() == 2 {
        if let Result::String(text) = eval(args.get(1).unwrap(), runtime, ctx.clone()) {
            filter = text.clone();
        }
    }
    let mut matched = Vec::new();
    let names = ctx.get_all_var_names();
    for name in names {
        if name.contains(filter.as_str()) {
            matched.push(Result::String(name.clone()));
        }
    }

    return Result::Array(matched);
}

pub fn len(input: Result) -> Result {
    if let Result::String(text) = input {
        return Result::Int(text.len() as i64);
    } else if let Result::Array(ar) = input {
        return Result::Int(ar.len() as i64);
    } else if let Result::Dict(map) = input {
        return Result::Int(map.len() as i64);
    } else if let Result::Error(e) = input {
        return Result::Error(e);
    } else {
        return Result::Error("Len may only be called on arrays or strings".to_string());
    }
}

pub fn each(array: Result, function: Result, runtime: Arc<Runtime>, ctx: Arc<Context>) -> Result {
    if let Result::Array(ar) = array {
        let mut index = 0 as i64;
        for item in ar {
            let mut itemar = Vec::new();
            itemar.push(item);
            itemar.push(Result::Int(index));
            exec_func(itemar.iter(), &function, runtime.clone(), ctx.clone());
            index += 1;
        }
        return Result::None;
    } else if let Result::Dict(map) = array {
        let mut index = 0 as i64;
        for (key, value) in map {
            let mut itemar = Vec::new();
            itemar.push(Result::String(key));
            itemar.push(value);
            itemar.push(Result::Int(index));
            exec_func(itemar.iter(), &function, runtime.clone(), ctx.clone());
            index += 1;
        }
        return Result::None;
    } else if let Result::Error(e) = array {
        return Result::Error(e);
    }
    return Result::Error("Each may only be called on arrays or strings".to_string());
}

pub fn map(array: Result, function: Result, runtime: Arc<Runtime>, ctx: Arc<Context>) -> Result {
    if let Result::Array(ar) = array {
        let mut index = 0 as i64;
        let mut newar = Vec::with_capacity(ar.len());
        for item in ar {
            let mut itemar = Vec::new();
            itemar.push(item);
            itemar.push(Result::Int(index));
            newar.push(exec_func(itemar.iter(), &function, runtime.clone(), ctx.clone()));
            index += 1;
        }
        return Result::Array(newar);
    } else if let Result::Dict(map) = array {
        let mut index = 0 as i64;
        let mut newdict = HashMap::with_capacity(map.len());
        for (key, value) in map {
            let mut itemar = Vec::new();
            itemar.push(Result::String(key.clone()));
            itemar.push(value);
            itemar.push(Result::Int(index));
            newdict.insert(key, exec_func(itemar.iter(), &function, runtime.clone(), ctx.clone()));
            index += 1;
        }
        return Result::Dict(newdict);
    } else if let Result::Error(e) = array {
        return Result::Error(e);
    }
    return Result::Error("Map may only be called on arrays or strings".to_string());
}



pub fn join(array: Result, joinstring: Result, runtime: Arc<Runtime>, ctx: Arc<Context>) -> Result {
    if let Result::Array(ar) = array {
        if let Result::String(text) = joinstring{
            let mut res = String::new();
            for item in ar{
                res += text.as_str();
                res += item.to_string().as_str();
            }
            res.replace_range(0..text.len(), "");
            return Result::String(res);
        }
        else{
            return Result::Error("Array may only be joined with a string".to_string());
        }
    } else if let Result::Error(e) = array {
        return Result::Error(e);
    }
    return Result::Error("Join may only be called on an array and a string".to_string());
}

pub fn get_index(input: &Result, index: Result) -> Result {
    if let Result::Array(arr) = input {
        if let Result::Int(i) = index {
            if i >= arr.len() as i64{
                return Result::Error("Index out of bounds: ".to_string() + &i.to_string() + " >= " + &arr.len().to_string());
            }
            return arr[i as usize].clone();
        } else if let Result::Error(e) = index {
            return Result::Error(e);
        } else {
            return Result::Error("Array may only be indexed by an int".to_string());
        }
    } else if let Result::Dict(map) = input {
        if let Result::String(i) = index {
            if !map.contains_key(&i){
                return Result::Error("Dict does not contain key ".to_string() + &i);
            }
            return map[&i].clone();
        } else if let Result::Error(e) = index {
            return Result::Error(e);
        } else {
            return Result::Error("Dict may only be indexed by a string".to_string());
        }
    } else if let Result::String(text) = input {
        if let Result::Int(i) = index {
            return Result::String(text.chars().skip(i as usize).take(1).collect());
        } else if let Result::Error(e) = index {
            return Result::Error(e);
        } else {
            return Result::Error("Array may only be index by an int".to_string());
        }
    }
    return Result::None;
}

pub fn set_index(input: Result, index: Result, value: Result) -> Result {
    if let Result::Array(mut arr) = input {
        if let Result::Int(i) = index {
            arr[i as usize] = value;
            return Result::Array(arr);
        } else if let Result::Error(e) = index {
            return Result::Error(e);
        } else {
            return Result::Error("Array may only be indexed by an int".to_string());
        }
    } else if let Result::Dict(mut map) = input {
        if let Result::String(i) = index {
            map.insert(i, value);
            return Result::Dict(map);
        } else if let Result::Error(e) = index {
            return Result::Error(e);
        } else {
            return Result::Error("Dict may only be indexed by a string".to_string());
        }
    }
    return Result::None;
}

//Casting
pub fn cast_int(input: Result) -> Result {
    match input {
        Result::Int(i) => {
            return Result::Int(i);
        }
        Result::Float(f) => {
            return Result::Int(f as i64);
        }
        Result::String(t) => {
            return Result::Int(t.parse::<i64>().unwrap());
        }
        Result::Bool(b) => {
            return Result::Int(if b { 1 } else { 0 });
        }
        _ => {
            return Result::Error(
                "May not cast type ".to_string() + input.typename().as_str() + " to int",
            );
        }
    }
}

pub fn cast_float(input: Result) -> Result {
    match input {
        Result::Int(i) => {
            return Result::Float(i as f64);
        }
        Result::Float(f) => {
            return Result::Float(f);
        }
        Result::String(t) => {
            return Result::Float(t.parse::<f64>().unwrap());
        }
        Result::Bool(b) => {
            return Result::Float(if b { 1.0 } else { 0.0 });
        }
        _ => {
            return Result::Error(
                "May not cast type ".to_string() + input.typename().as_str() + " to float",
            );
        }
    }
}

pub fn cast_string(input: Result) -> Result {
    return Result::String(input.to_string());
}

pub fn cast_bool(input: Result) -> Result {
    match input {
        Result::String(t) => {
            return Result::Bool(t.parse::<bool>().unwrap());
        }
        Result::Bool(b) => {
            return Result::Bool(b);
        }
        _ => {
            return Result::Error(
                "May not cast type ".to_string() + input.typename().as_str() + " to bool",
            );
        }
    }
}
