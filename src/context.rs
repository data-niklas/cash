use crate::result::Result;
use owning_ref::MutexGuardRefMut;
use std::clone::Clone;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Context<'a> {
    pub parent: Arc<Option<&'a Context<'a>>>,
    pub vars: Arc<Mutex<HashMap<String, Result>>>,
    pub node: Option<&'a Result>,
}

impl<'a> Context<'a> {
    pub fn new() -> Context<'a> {
        return Context {
            parent: Arc::new(Option::None),
            vars: Arc::new(Mutex::new(HashMap::new())),
            node: None,
        };
    }

    pub fn from_parent(parent: &'a Context, node: Option<&'a Result>) -> Context<'a> {
        return Context {
            parent: Arc::new(Option::Some(parent)),
            vars: Arc::new(Mutex::new(HashMap::new())),
            node: node,
        };
    }

    pub fn from_vars(
        parent: &'a Context,
        vars: HashMap<String, Result>,
        node: Option<&'a Result>,
    ) -> Context<'a> {
        return Context {
            parent: Arc::new(Option::Some(parent)),
            vars: Arc::new(Mutex::new(vars)),
            node: node,
        };
    }

    //Cloning the data is slow, a borrow_var variable would be great, especially for assigning values to dicts or arrays
    pub fn var(&self, name: &str) -> Result {
        if name.starts_with("$") {
            match env::var(name[1..].to_owned()) {
                Ok(val) => {
                    return Result::String(val);
                }
                Err(_) => {
                    return Result::Error("Variable ".to_string() + name + " not found");
                }
            }
        } else {
            if name.contains("::") {
                return self.var_from_dict(name);
            } else {
                if let Some(value) = self.var_recursively(name) {
                    return value.clone();
                } else {
                    return Result::None;
                }
            }
        }
    }

    fn var_from_dict(&self, name: &str) -> Result {
        let mut parts = name.split("::");
        let dictname = parts.next().unwrap();
        if let Some(res) = self.var_recursively(dictname) {
            let mut res = &*res;
            for part in parts {
                if let Result::Dict(dict) = res {
                    if dict.contains_key(part) {
                        res = dict.get(part).unwrap();
                    } else {
                        return Result::Error("Dict does not contain key ".to_owned() + part);
                    }
                } else {
                    return Result::Error(":: can only be used with dicts".to_string());
                }
            }
            return res.clone();
        } else {
            return Result::Error("Dict ".to_string() + dictname + " does not exists");
        }
    }

    fn var_recursively(
        &self,
        name: &str,
    ) -> Option<MutexGuardRefMut<HashMap<String, Result>, Result>> {
        if !self.vars.lock().unwrap().contains_key(name) {
            if let Option::None = *self.parent {
                return None;
            } else {
                return self.parent.unwrap().var_recursively(name);
            }
        } else {
            let mgrm = MutexGuardRefMut::new(self.vars.lock().unwrap());
            return Some(mgrm.map_mut(|map| map.get_mut(name).unwrap()));
        }
    }

    pub fn set_var(&self, name: &str, value: Result) {
        if name.starts_with("$") {
            env::set_var(name[1..].to_owned(), value.to_string());
        } else {
            if name.contains("::") {
                self.set_var_in_dict(name, value);
            } else if let Option::Some(notfoundvalue) = self.set_var_recursively(name, value) {
                self.vars
                    .lock()
                    .unwrap()
                    .insert(name.to_string(), notfoundvalue);
            }
        }
    }

    fn set_var_in_dict(&self, name: &str, value: Result) {
        let mut parts = name.split("::").collect::<Vec<&str>>();
        let last = parts.remove(parts.len() - 1);
        if let Some(mut mref) = self.var_recursively(&parts.remove(0)) {
            let mut res = &mut *mref;
            for part in parts.iter() {
                if let Result::Dict(dict) = res {
                    if dict.contains_key(part.to_owned()) {
                        res = dict.get_mut(part.to_owned()).unwrap();
                    } else {
                        return;
                    }
                }
            }
            if let Result::Dict(dict) = res {
                dict.insert(last.to_owned(), value);
            }
        }
    }

    pub fn set_own_var(&self, name: &str, value: Result) {
        self.vars.lock().unwrap().insert(name.to_string(), value);
    }

    pub fn set_var_recursively(&self, name: &str, value: Result) -> Option<Result> {
        if !self.vars.lock().unwrap().contains_key(name) {
            if let Option::None = *self.parent {
                return Option::Some(value);
            } else {
                return self.parent.unwrap().set_var_recursively(name, value);
            }
        } else {
            self.vars.lock().unwrap().insert(name.to_string(), value);
            return Option::None;
        }
    }

    pub fn get_all_var_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        let vars = self.vars.lock().unwrap();
        let keys: Vec<&String> = vars.keys().collect();
        for key in keys {
            names.push(key.clone());
        }
        if let Some(parent) = *self.parent {
            names.append(&mut parent.get_all_var_names());
        }
        return names;
    }

    pub fn me(&self) -> Option<&Result> {
        return self.node;
    }
}
