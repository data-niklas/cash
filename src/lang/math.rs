use crate::result::Result;
use std::collections::HashMap;

pub fn negate(input: Result) -> Result{
    if let Result::Bool(b) = input {
        return Result::Bool(!b);
    }else if let Result::Int(i) = input {
        return Result::Int(i*-1);
    }else if let Result::Float(f) = input {
        return Result::Float(f*-1.0);
    }  else if let Result::Error(e) = input {
        return Result::Error(e);
    } else {
        return Result::Error("Only Bools, ints and floats may be negated".to_string());
    }
}

pub fn compare(lhs: &Result, rhs: &Result) -> Result{
    if let Result::Int(i1) = lhs {
        if let Result::Int(i2) = rhs {
            return Result::Int(compare_int(i1,i2));
        } else if let Result::Error(e) = rhs {
            return Result::Error(e.to_string());
        } else {
            return Result::Error("Can only compare ints with each other".to_string());
        }
    } else if let Result::Float(f1) = lhs {
        if let Result::Float(f2) = rhs {
            return Result::Int(compare_float(f1,f2));
        } else if let Result::Error(e) = rhs {
            return Result::Error(e.to_string());
        } else {
            return Result::Error("Can only compare floats with each other".to_string());
        }
    } else if let Result::String(s1) = lhs {
        if let Result::String(s2) = rhs {
            return Result::Int(compare_string(s1,s2));
        } else if let Result::Error(e) = rhs {
            return Result::Error(e.to_string());
        } else {
            return Result::Error("Can only compare strings with each other".to_string());
        }
    } else if let Result::Bool(b1) = lhs {
        if let Result::Bool(b2) = rhs {
            return Result::Int(compare_bool(b1,b2));
        } else if let Result::Error(e) = rhs {
            return Result::Error(e.to_string());
        } else {
            return Result::Error("Can only compare bools with each other".to_string());
        }
    } else if let Result::Array(a1) = lhs {
        if let Result::Array(a2) = rhs {
            return compare_array(a1,a2);
        } else if let Result::Error(e) = rhs {
            return Result::Error(e.to_string());
        } else {
            return Result::Error("Can only compare arrays with each other".to_string());
        }
    } else if let Result::Dict(a1) = lhs {
        if let Result::Dict(a2) = rhs {
            return compare_dict(a1,a2);
        } else if let Result::Error(e) = rhs {
            return Result::Error(e.to_string());
        } else {
            return Result::Error("Can only compare dicts with each other".to_string());
        }
    } else {
        return lhs.clone();
    }
}

pub fn compare_int(value1: &i64, value2: &i64) -> i64{
    if value1 == value2 {
        return 0;
    }
    else if value1 < value2{
        return -1;
    }
    else{
        return 1;
    }
}

pub fn compare_float(value1: &f64, value2: &f64) -> i64{
    if value1 == value2 {
        return 0;
    }
    else if value1 < value2{
        return -1;
    }
    else{
        return 1;
    }
}

pub fn compare_bool(value1: &bool, value2: &bool) -> i64{
    if value1 == value2 {
        return 0;
    }
    else if value1 < value2{
        return -1;
    }
    else{
        return 1;
    }
}

pub fn compare_string(value1: &String, value2: &String) -> i64{
    if value1 == value2 {
        return 0;
    }
    else if value1 < value2{
        return -1;
    }
    else{
        return 1;
    }
}

pub fn compare_array(a1: &Vec<Result>, a2: &Vec<Result>) -> Result{
    if a1.len() == a2.len(){
        for i in 0..a1.len(){
            let res = compare(&a1[i], &a2[i]);
            if let Result::Int(int) = res{
                if int == -1{
                    return Result::Int(-1);
                }
                else if int == 1{
                    return Result::Int(1);
                }
            }
            else if let Result::Error(e) = res{
                return Result::Error(e);
            }
            else{
                return Result::Error("Could not compare both arrays".to_string());
            }
        }
        return Result::Int(0);
    }
    else if a1.len() < a2.len(){
        return Result::Int(-1);
    }
    else{
        return Result::Int(1);
    }
}

pub fn compare_dict(m1: &HashMap<String,Result>, m2: &HashMap<String,Result>) -> Result{
    if m1.len() == m2.len(){
        for (key,value) in m1{
            let res = compare(value, &m2[key]);
            if let Result::Int(int) = res{
                if int == -1{
                    return Result::Int(-1);
                }
                else if int == 1{
                    return Result::Int(1);
                }
            }
            else if let Result::Error(e) = res{
                return Result::Error(e);
            }
            else{
                return Result::Error("Could not compare both dicts".to_string());
            }
        }
        return Result::Int(0);
    }
    else if m1.len() < m2.len(){
        return Result::Int(-1);
    }
    else{
        return Result::Int(1);
    }
}

pub fn equals(v1: &Result, v2: &Result) -> Result{
    if let Result::Int(0) = compare(v1, v2){
        return Result::Bool(true);
    }
    else{
        return Result::Bool(false);
    }
}

pub fn smaller(v1: &Result, v2: &Result) -> Result{
    if let Result::Int(-1) = compare(v1, v2){
        return Result::Bool(true);
    }
    else{
        return Result::Bool(false);
    }
}

pub fn greater(v1: &Result, v2: &Result) -> Result{
    if let Result::Int(1) = compare(v1, v2){
        return Result::Bool(true);
    }
    else{
        return Result::Bool(false);
    }
}

pub fn smallereq(v1: &Result, v2: &Result) -> Result{
    if let Result::Int(-1) | Result::Int(0) = compare(v1, v2){
        return Result::Bool(true);
    }
    else{
        return Result::Bool(false);
    }
}

pub fn greatereq(v1: &Result, v2: &Result) -> Result{
    if let Result::Int(1) | Result::Int(0) = compare(v1, v2){
        return Result::Bool(true);
    }
    else{
        return Result::Bool(false);
    }
}


pub fn add(mut lhs: Result, rhs: Result) -> Result{
    if let Result::Int(i1) = lhs {
        if let Result::Int(i2) = rhs {
            return Result::Int(i1 + i2);
        } else if let Result::Float(f2) = rhs {
            return Result::Float(i1 as f64 + f2);
        } else if let Result::String(s2) = rhs {
            return Result::String(i1.to_string() + s2.as_str());
        } else if let Result::Array(mut a2) = rhs {
            a2.insert(0, Result::Int(i1));
            return Result::Array(a2);
        }
    } else if let Result::Float(f1) = lhs {
        if let Result::Int(i2) = rhs {
            return Result::Float(f1 + i2 as f64);
        } else if let Result::Float(f2) = rhs {
            return Result::Float(f1 + f2);
        } else if let Result::String(s2) = rhs {
            return Result::String(f1.to_string() + s2.as_str());
        } else if let Result::Array(mut a2) = rhs {
            a2.insert(0, Result::Float(f1));
            return Result::Array(a2);
        }
    } else if let Result::String(ref s1) = lhs {
        if let Result::String(s2) = rhs {
            return Result::String(s1.to_string() + s2.as_str());
        } else if let Result::Int(i2) = rhs {
            return Result::String(s1.to_string() + i2.to_string().as_str());
        } else if let Result::Float(f2) = rhs {
            return Result::String(s1.to_string() + f2.to_string().as_str());
        } else if let Result::Array(mut a2) = rhs {
            a2.insert(0, Result::String(s1.to_string()));
            return Result::Array(a2);
        }
    } else if let Result::Bool(b1) = lhs {
        if let Result::Array(mut a2) = rhs {
            a2.insert(0, Result::Bool(b1));
            return Result::Array(a2);
        }
    }else if let Result::Array(mut a1) = lhs {
        if let Result::String(s2) = rhs {
            a1.push(Result::String(s2));
            return Result::Array(a1);
        } else if let Result::Int(i2) = rhs {
            a1.push(Result::Int(i2));
            return Result::Array(a1);
        } else if let Result::Float(f2) = rhs {
            a1.push(Result::Float(f2));
            return Result::Array(a1);
        } else if let Result::Bool(b2) = rhs {
            a1.push(Result::Bool(b2));
            return Result::Array(a1);
        } else if let Result::Array(mut a2) = rhs {
            a1.append(&mut a2);
            return Result::Array(a1);
        }
        lhs = Result::Array(a1);
    }
    else if let Result::Dict(mut m1) = lhs{
        if let Result::Dict(m2) = rhs{
            for (key,value) in m2{
                m1.insert(key, value);
            }
            return Result::Dict(m1);
        }
        lhs = Result::Dict(m1);
    }
    if let Result::Error(e) = lhs {
        return Result::Error(e);
    } else if let Result::Error(e) = rhs {
        return Result::Error(e);
    } else {
        return Result::Error("Could not add ".to_string() + lhs.typename().as_str() + " and " + rhs.to_string().as_str());
    }
}

pub fn divide(lhs: Result, rhs: Result) -> Result{
    if let Result::Int(i1) = lhs {
        if let Result::Int(i2) = rhs {
            if i2 == 0 {
                return Result::Error(
                    "Please don't divide by 0. A kitten just died :(".to_string(),
                );
            }
            return Result::Float(i1 as f64 / i2 as f64);
        } else if let Result::Float(f2) = rhs {
            if f2 == 0.0 {
                return Result::Error(
                    "Please don't divide by 0. A kitten just died :(".to_string(),
                );
            }
            return Result::Float(i1 as f64 / f2);
        }
    } else if let Result::Float(f1) = lhs {
        if let Result::Int(i2) = rhs {
            if i2 == 0 {
                return Result::Error(
                    "Please don't divide by 0. A kitten just died :(".to_string(),
                );
            }
            return Result::Float(f1 / i2 as f64);
        } else if let Result::Float(f2) = rhs {
            if f2 == 0.0 {
                return Result::Error(
                    "Please don't divide by 0. A kitten just died :(".to_string(),
                );
            }
            return Result::Float(f1 / f2);
        }
    }
    if let Result::Error(e) = lhs {
        return Result::Error(e);
    } else if let Result::Error(e) = rhs {
        return Result::Error(e);
    } else {
        return Result::Error("May only divide numbers".to_string());
    }
}

pub fn subtract(lhs: Result, rhs: Result) -> Result{
    if let Result::Int(i1) = lhs {
        if let Result::Int(i2) = rhs {
            return Result::Int(i1 - i2);
        } else if let Result::Float(f2) = rhs {
            return Result::Float(i1 as f64 - f2);
        }
    } else if let Result::Float(f1) = lhs {
        if let Result::Int(i2) = rhs {
            return Result::Float(f1 - i2 as f64);
        } else if let Result::Float(f2) = rhs {
            return Result::Float(f1 - f2);
        }
    }
    if let Result::Error(e) = lhs {
        return Result::Error(e);
    } else if let Result::Error(e) = rhs {
        return Result::Error(e);
    } else {
        return Result::Error("May only subtract numbers".to_string());
    }
}

pub fn multiply(lhs: Result, rhs: Result) -> Result{
    if let Result::Int(i1) = lhs {
        if let Result::Int(i2) = rhs {
            return Result::Int(i1 * i2);
        } else if let Result::Float(f2) = rhs {
            return Result::Float(i1 as f64 * f2);
        }
    } else if let Result::Float(f1) = lhs {
        if let Result::Int(i2) = rhs {
            return Result::Float(f1 * i2 as f64);
        } else if let Result::Float(f2) = rhs {
            return Result::Float(f1 * f2);
        }
    } else if let Result::String(s1) = lhs {
        if let Result::Int(i2) = rhs {
            return Result::String(s1.repeat(i2 as usize));
        } else if let Result::Error(e) = rhs {
            return Result::Error(e);
        } else {
            return Result::Error("String may only be multiplied with an int".to_string());
        }
    } else if let Result::Array(mut a1) = lhs {
        if let Result::Int(i2) = rhs {
            let mut newvec = Vec::with_capacity(a1.len() * i2 as usize);
            for _ in 0..i2{
                newvec.append(&mut a1.clone());
            }
            return Result::Array(newvec);
        } else if let Result::Error(e) = rhs {
            return Result::Error(e);
        } else {
            return Result::Error("Arrays may only be multiplied with an int".to_string());
        }
    }

    if let Result::Error(e) = lhs {
        return Result::Error(e);
    } else if let Result::Error(e) = rhs {
        return Result::Error(e);
    } else {
        return Result::Error("May only multiply numbers".to_string());
    }
}

pub fn modulo(lhs: Result, rhs: Result) -> Result{
    if let Result::Int(i1) = lhs {
        if let Result::Int(i2) = rhs {
            return Result::Int(i1 % i2);
        } else if let Result::Float(_f2) = rhs {
            return Result::Error("Cannot apply modulo on Int and Float Numbers".to_string());
        }
    } else if let Result::Float(_f1) = lhs {
        if let Result::Int(_i2) = rhs {
            return Result::Error("Cannot apply modulo on Int and Float Numbers".to_string());
        } else if let Result::Float(_f2) = rhs {
            return Result::Error("Cannot apply modulo on two Float Numbers".to_string());
        }
    }
    if let Result::Error(e) = lhs {
        return Result::Error(e);
    } else if let Result::Error(e) = rhs {
        return Result::Error(e);
    } else {
        return Result::Error("May only use modulo ints".to_string());
    }
}

pub fn power(lhs: Result, rhs: Result) -> Result{
    if let Result::Int(i1) = lhs {
        let i2: f64;
        if let Result::Int(int) = rhs {
            i2 = int as f64;
        } else if let Result::Float(num) = rhs {
            i2 = num;
        } else if let Result::Error(e) = rhs {
            return Result::Error(e);
        } else {
            return Result::Error("May only use power on numbers".to_string());
        }
        if i2 < 0.0 {
            return divide(Result::Int(1), Result::Float((i1 as f64).powf(i2 * -1.0)));
        } else {
            return Result::Float((i1 as f64).powf(i2));
        }
    } else if let Result::Float(f1) = lhs {
        let i2: f64;
        if let Result::Int(int) = rhs {
            i2 = int as f64;
        } else if let Result::Float(num) = rhs {
            i2 = num;
        } else if let Result::Error(e) = rhs {
            return Result::Error(e);
        } else {
            return Result::Error("May only use power on numbers".to_string());
        }
        if i2 < 0.0 {
            return divide(Result::Int(1), Result::Float((f1 as f64).powf(i2 * -1.0)));
        } else {
            return Result::Float((f1 as f64).powf(i2));
        }
    }
    if let Result::Error(e) = lhs {
        return Result::Error(e);
    } else {
        return Result::Error("May only use power on numbers".to_string());
    }
}

pub fn root(lhs: Result, rhs: Result) -> Result{
    if let Result::Int(i1) = lhs {
        let i2: f64;
        if let Result::Int(int) = rhs {
            i2 = int as f64;
        } else if let Result::Float(num) = rhs {
            i2 = num;
        } else if let Result::Error(e) = rhs {
            return Result::Error(e);
        } else {
            return Result::Error("May only use root on numbers".to_string());
        }
        if i2 < 0.0 {
            return divide(
                Result::Int(1),
                Result::Float((i1 as f64).powf(1.0 / (i2 * -1.0))),
            );
        } else {
            return Result::Float((i1 as f64).powf(1.0 / i2));
        }
    } else if let Result::Float(f1) = lhs {
        let i2: f64;
        if let Result::Int(int) = rhs {
            i2 = int as f64;
        } else if let Result::Float(num) = rhs {
            i2 = num;
        } else if let Result::Error(e) = rhs {
            return Result::Error(e);
        } else {
            return Result::Error("May only use root on numbers".to_string());
        }
        if i2 < 0.0 {
            return divide(
                Result::Int(1),
                Result::Float((f1 as f64).powf(1.0 / (i2 * -1.0))),
            );
        } else {
            return Result::Float((f1 as f64).powf(1.0 / i2));
        }
    }

    if let Result::Error(e) = lhs {
        return Result::Error(e);
    } else {
        return Result::Error("May only use root on numbers".to_string());
    }
}

pub fn faculty(num: Result) -> Result{
    if let Result::Int(int) = num {
        let mut res: i64 = 1;
        for i in 2..(int + 1) {
            res *= i;
        }
        return Result::Int(res);
    }

    if let Result::Error(e) = num {
        return Result::Error(e);
    } else {
        return Result::Error("May only use faculty on numbers".to_string());
    }
}

pub fn abs(num: Result) -> Result{
    if let Result::Int(int) = num {
        return Result::Int(int.abs());
    } else if let Result::Float(float) = num {
        return Result::Float(float.abs());
    } else if let Result::Error(e) = num {
        return Result::Error(e);
    } else {
        return Result::Error("Function abs may only be used with a float or int".to_string());
    }
}

pub fn ceil(num: Result) -> Result{
    if let Result::Int(int) = num {
        return Result::Int(int);
    } else if let Result::Float(float) = num {
        return Result::Float(float.ceil());
    } else if let Result::Error(e) = num {
        return Result::Error(e);
    } else {
        return Result::Error("Function ceil may only be used with a float or int".to_string());
    }
}

pub fn floor(num: Result) -> Result{
    if let Result::Int(int) = num {
        return Result::Int(int);
    } else if let Result::Float(float) = num {
        return Result::Float(float.floor());
    } else if let Result::Error(e) = num {
        return Result::Error(e);
    } else {
        return Result::Error("Function floor may only be used with a float or int".to_string());
    }
}

pub fn sin(num: Result) -> Result{
    if let Result::Int(int) = num {
        return Result::Float((int as f64).sin());
    } else if let Result::Float(float) = num {
        return Result::Float(float.sin());
    } else if let Result::Error(e) = num {
        return Result::Error(e);
    } else {
        return Result::Error("Function sin may only be used with a float or int".to_string());
    }
}

pub fn cos(num: Result) -> Result{
    if let Result::Int(int) = num {
        return Result::Float((int as f64).cos());
    } else if let Result::Float(float) = num {
        return Result::Float(float.cos());
    } else if let Result::Error(e) = num {
        return Result::Error(e);
    } else {
        return Result::Error("Function cos may only be used with a float or int".to_string());
    }
}

pub fn tan(num: Result) -> Result{
    if let Result::Int(int) = num {
        return Result::Float((int as f64).tan());
    } else if let Result::Float(float) = num {
        return Result::Float(float.tan());
    } else if let Result::Error(e) = num {
        return Result::Error(e);
    } else {
        return Result::Error("Function tan may only be used with a float or int".to_string());
    }
}

pub fn asin(num: Result) -> Result{
    if let Result::Int(int) = num {
        return Result::Float((int as f64).asin());
    } else if let Result::Float(float) = num {
        return Result::Float(float.asin());
    } else if let Result::Error(e) = num {
        return Result::Error(e);
    } else {
        return Result::Error("Function asin may only be used with a float or int".to_string());
    }
}

pub fn acos(num: Result) -> Result{
    if let Result::Int(int) = num {
        return Result::Float((int as f64).acos());
    } else if let Result::Float(float) = num {
        return Result::Float(float.acos());
    } else if let Result::Error(e) = num {
        return Result::Error(e);
    } else {
        return Result::Error("Function acos may only be used with a float or int".to_string());
    }
}

pub fn atan(num: Result) -> Result{
    if let Result::Int(int) = num {
        return Result::Float((int as f64).atan());
    } else if let Result::Float(float) = num {
        return Result::Float(float.atan());
    } else if let Result::Error(e) = num {
        return Result::Error(e);
    } else {
        return Result::Error("Function atan may only be used with a float or int".to_string());
    }
}

pub fn sinh(num: Result) -> Result{
    if let Result::Int(int) = num {
        return Result::Float((int as f64).sinh());
    } else if let Result::Float(float) = num {
        return Result::Float(float.sinh());
    } else if let Result::Error(e) = num {
        return Result::Error(e);
    } else {
        return Result::Error("Function sinh may only be used with a float or int".to_string());
    }
}

pub fn cosh(num: Result) -> Result{
    if let Result::Int(int) = num {
        return Result::Float((int as f64).cosh());
    } else if let Result::Float(float) = num {
        return Result::Float(float.cosh());
    } else if let Result::Error(e) = num {
        return Result::Error(e);
    } else {
        return Result::Error("Function cosh may only be used with a float or int".to_string());
    }
}

pub fn tanh(num: Result) -> Result{
    if let Result::Int(int) = num {
        return Result::Float((int as f64).tanh());
    } else if let Result::Float(float) = num {
        return Result::Float(float.tanh());
    } else if let Result::Error(e) = num {
        return Result::Error(e);
    } else {
        return Result::Error("Function tanh may only be used with a float or int".to_string());
    }
}

pub fn asinh(num: Result) -> Result{
    if let Result::Int(int) = num {
        return Result::Float((int as f64).asinh());
    } else if let Result::Float(float) = num {
        return Result::Float(float.asinh());
    } else if let Result::Error(e) = num {
        return Result::Error(e);
    } else {
        return Result::Error("Function asinh may only be used with a float or int".to_string());
    }
}

pub fn acosh(num: Result) -> Result{
    if let Result::Int(int) = num {
        return Result::Float((int as f64).acosh());
    } else if let Result::Float(float) = num {
        return Result::Float(float.acosh());
    } else if let Result::Error(e) = num {
        return Result::Error(e);
    } else {
        return Result::Error("Function acosh may only be used with a float or int".to_string());
    }
}

pub fn atanh(num: Result) -> Result{
    if let Result::Int(int) = num {
        return Result::Float((int as f64).atanh());
    } else if let Result::Float(float) = num {
        return Result::Float(float.atanh());
    } else if let Result::Error(e) = num {
        return Result::Error(e);
    } else {
        return Result::Error("Function atanh may only be used with a float or int".to_string());
    }
}

pub fn signum(num: Result) -> Result{
    if let Result::Int(int) = num {
        return Result::Float((int as f64).signum());
    } else if let Result::Float(float) = num {
        return Result::Float(float.signum());
    } else if let Result::Error(e) = num {
        return Result::Error(e);
    } else {
        return Result::Error("Function signum may only be used with a float or int".to_string());
    }
}

pub fn round(num: Result) -> Result{
    if let Result::Int(int) = num {
        return Result::Int(int);
    } else if let Result::Float(float) = num {
        return Result::Float(float.round());
    } else if let Result::Error(e) = num {
        return Result::Error(e);
    } else {
        return Result::Error("Function round may only be used with a float or int".to_string());
    }
}

pub fn log(num: Result, base: Result) -> Result{
    if let Result::Int(int) = num {
        if let Result::Int(intbase) = base {
            return Result::Float((int as f64).log(intbase as f64));
        } else if let Result::Float(floatbase) = base {
            return Result::Float((int as f64).log(floatbase));
        } else if let Result::Error(e) = base {
            return Result::Error(e);
        } else {
            return Result::Error("The base for the log needs to be a float or int".to_string());
        }
    } else if let Result::Float(float) = num {
        if let Result::Int(intbase) = base {
            return Result::Float(float.log(intbase as f64));
        } else if let Result::Float(floatbase) = base {
            return Result::Float(float.log(floatbase));
        } else if let Result::Error(e) = base {
            return Result::Error(e);
        } else {
            return Result::Error("The base for the log needs to be a float or int".to_string());
        }
    } else if let Result::Error(e) = num {
        return Result::Error(e);
    } else {
        return Result::Error("Function log may only be used with a float or int".to_string());
    }
}