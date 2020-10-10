use crate::ast::Node;
use std::collections::HashMap;

#[derive(Clone)]
#[derive(Debug)]
pub struct Parameter{
    pub name: String,
    pub defaultvalue: Option<Result>
}


#[derive(Clone)]
#[derive(Debug)]
pub enum Result{
    None,
    Return(Box<Result>),
    Error(String),
    String(String),
    Array(Vec<Result>),
    Dict(HashMap<String,Result>),
    Function{
        block: Node,
        parameters: Vec<Parameter>,
        vars: HashMap<String, Result>
    },
    Bool(bool),
    Int(i64),
    Float(f64)
}


impl Result{

    pub fn typename(&self) -> String{
        match self{
            Result::Bool(_) => {
                return "bool".to_string();
            }
            Result::String(_) => {
                return "string".to_string();
            }
            Result::Int(_) => {
                return "int".to_string();
            }
            Result::Float(_) => {
                return "float".to_string();
            }
            Result::Error(_) => {
                return "error".to_string();
            }
            Result::Array(_) => {
                return "array".to_string();
            }
            Result::Dict(_) => {
                return "dict".to_string();
            }
            Result::None | _ => {
                return "none".to_string();
            }
        }
    }

    pub fn print(&self){
        match self {
            Result::Bool(txt) => {
                println!("{}", txt);
            }
            Result::String(txt) => {
                println!("{}", txt);
            }
            Result::Int(txt) => {
                println!("{}", txt);
            }
            Result::Float(txt) => {
                println!("{}", txt);
            }
            Result::Array(_) => {
                println!("{}", self.to_string());
            }
            Result::Dict(_) => {
                println!("{}", self.to_string());
            }
            Result::Error(e) => {
                println!("\x1b[1;31mError:\x1b[0;31m {}\x1b[0m", e);
            }
            _ => {}
        }
    }

    pub fn to_string(&self) -> String{
        match self {
            Result::Bool(txt) => {
                return txt.to_string();
            }
            Result::String(txt) => {
                return txt.clone();
            }
            Result::Int(txt) => {
                return txt.to_string();
            }
            Result::Array(vec) => {
                let mut txt = String::new();
                for result in vec{
                    txt += ", ";
                    txt += result.to_string().as_str();
                }
                txt += " ]";
                return "[".to_string() + &txt[1..];
            }
            Result::Dict(map) => {
                let mut txt = String::new();
                for (key,result) in map{
                    txt += ", ";
                    txt = txt + key.to_string().as_str() + ": " + result.to_string().as_str()                    
                }
                txt += " }";
                return "{".to_string() + &txt[1..];
            }
            Result::Float(txt) => {
                return txt.to_string();
            }
            Result::Error(e) => {
                return "\x1b[1;31mError:\x1b[0;31m ".to_string() + e.to_string().as_str() + "\x1b[0m";
            }
            _ => {return "".to_string();}
        }
    }
}

#[derive(Debug)]
pub enum ExprResult<'a>{
    Result(Result),
    Node(&'a Node)
}