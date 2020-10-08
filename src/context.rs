use crate::result::Result;
use std::cell::RefCell;
use std::clone::Clone;
use std::collections::HashMap;
use std::env;

#[derive(Debug, Clone)]
pub struct Context<'a> {
    pub parent: Option<&'a Context<'a>>,
    pub vars: RefCell<HashMap<String, Result>>,
}

impl<'a> Context<'a> {
    pub fn new() -> Context<'a> {
        return Context {
            parent: Option::None,
            vars: RefCell::new(HashMap::new()),
        };
    }

    pub fn from_parent(parent: &'a Context) -> Context<'a> {
        return Context {
            parent: Option::Some(parent),
            vars: RefCell::new(HashMap::new()),
        };
    }

    pub fn from_vars(parent: &'a Context, vars: HashMap<String, Result>) -> Context<'a> {
        return Context {
            parent: Option::Some(parent),
            vars: RefCell::new(vars),
        };
    }

    pub fn var(&self, name: &str) -> Result {
        if name.starts_with("$"){
            match env::var(name[1..].to_owned()){
                Ok(val) => {return Result::String(val);}
                Err(e) => {return Result::Error("Variable ".to_string() + name + " not found");}
            }
        }
        if !self.vars.borrow().contains_key(name) {
            if let Option::None = self.parent {
                return Result::None;
            } else {
                return self.parent.unwrap().var(name);
            }
        } else {
            return self.vars.borrow().get(name).unwrap().clone();
        }
    }

    pub fn set_var(&self, name: &str, value: Result) {
        if name.starts_with("$"){
            env::set_var(name[1..].to_owned(), value.to_string());
            return;
        }
        if !self.vars.borrow().contains_key(name) {
            if let Option::None = self.parent {
                self.vars.borrow_mut().insert(name.to_string(), value);
            } else {
                if let Option::Some(notfoundvalue) = self.parent.unwrap().set_var_recursively(name, value) {
                    self.vars.borrow_mut().insert(name.to_string(), notfoundvalue);
                }
            }
        } else {
            self.vars.borrow_mut().insert(name.to_string(), value);
        }
    }

    pub fn set_own_var(&self, name: &str, value: Result) {
        self.vars.borrow_mut().insert(name.to_string(), value);
    }

    pub fn set_var_recursively(&self, name: &str, value: Result) -> Option<Result> {
        if !self.vars.borrow().contains_key(name) {
            if let Option::None = self.parent {
                return Option::Some(value);
            } else {
                return self.parent.unwrap().set_var_recursively(name, value);
            }
        } else {
            self.vars.borrow_mut().insert(name.to_string(), value);
            return Option::None;
        }
    }    
    
    
    pub fn get_all_var_names(&self) -> Vec<String>{
        let mut names = Vec::new();
        let vars = self.vars.borrow();
        let mut keys: Vec<&String> = vars.keys().collect();
        for key in keys{
            names.push(key.clone());
        }
        if let Some(parent) = self.parent{
            names.append(&mut parent.get_all_var_names());
        }
        return names;
    }
}
