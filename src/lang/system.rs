use std::process::{Child, Command};
use crate::result::Result;
use std::path::PathBuf;
use std::process::Stdio;
use std::io::prelude::*;

pub fn exec(file: PathBuf, args: std::slice::Iter<String>) -> Result{
    Command::new(file).args(args).stdin(Stdio::inherit()).stdout(Stdio::inherit()).spawn().unwrap().wait();
    return Result::None;
}