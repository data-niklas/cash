use std::process::Command;
use crate::result::Result;
use std::path::PathBuf;
use std::process::Stdio;
use std::io::prelude::*;

pub fn exec(file: PathBuf, args: std::slice::Iter<String>, stdin: &str) -> Result{
    let mut handle = Command::new(file).args(args).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().unwrap();
    
    handle.stdin.take().unwrap().write_all(stdin.as_bytes());
    if let Ok(res) = handle.wait_with_output(){
        return Result::String(String::from_utf8(res.stdout).unwrap_or("".to_string()));
    }
    else{
        return Result::Error("Command failed".to_string());
    }
}