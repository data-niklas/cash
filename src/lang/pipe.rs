use super::*;
use crate::ast::Node;
use crate::context::Context;
use crate::interpreter::Rule;
use crate::result::Result;
use crate::runtime::Runtime;
use std::process::{Stdio,Command,Child};
use std::os::unix::io::{FromRawFd, AsRawFd};
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

pub struct Pipe {
    child: std::io::Result<Child>,
}

impl Pipe {

    pub fn new(file: PathBuf, args: std::slice::Iter<String>) -> Pipe {
        Pipe {
            child: Command::new(file)
                    .args(args.as_slice())
                    .stdout(Stdio::piped())
                    .spawn(),
        }
    }

    pub fn then(self, file: PathBuf, args: std::slice::Iter<String>, last: bool) -> Pipe {
        let stdout = match self.child {
            Ok(child) => match child.stdout {
                Some(stdout) => stdout,
                None => return Pipe::pipe_new_error("No stdout for a command"),
            },
            Err(e) => return Pipe::pipe_error(Err(e)),
        };

        let stdio = unsafe{ Stdio::from_raw_fd(stdout.as_raw_fd()) };

        Pipe {
            child: Command::new(file)
                    .args(args.as_slice())
                    .stdout(match last{true=>{Stdio::inherit()}false=>{Stdio::piped()}})
                    .stdin(stdio)
                    .spawn(),
        }

    }


    pub fn finally(self) -> std::io::Result<Child> {
        self.child
    }

    fn pipe_new_error(error: &str) -> Pipe {
        Pipe {
            child: Err(Error::new(ErrorKind::Other, error)),
        }
    }
    
    
    fn pipe_error(error: std::io::Result<Child>) -> Pipe {
        Pipe {
            child: error,
        }
    }

}



pub fn eval_pipe(inner: &Vec<Node>, runtime: Arc<Runtime>, ctx: Arc<Context>) -> Result{
    let mut pipe: Option<Pipe> = None;
    let mut i = 0;
    let mut capture = false;
    for call in inner{
        if let Rule::Capture = call.rule(){
            capture = true;
            continue;
        }
        let mut calliter = call.inner().iter();
        let name = calliter.next().unwrap().content().as_str(); 
        if let Ok(path) = Runtime::which(name) {
            let mut args = Vec::new();
            for expr in calliter {
                args.push(eval(expr, runtime.clone(), ctx.clone()).to_string());
            }
            if let Some(innerpipe) = pipe{
                let inherit = (i==inner.len()-1) && !capture;
                pipe = Some(innerpipe.then(path, args.iter(), inherit));
            }
            else{
                pipe = Some(Pipe::new(path, args.iter()));
            }
        }
        else{
            return Result::Error("System function was not found".to_string());
        }
        i+=1;
    }

   if let Ok(handle) = pipe.unwrap().finally(){
       if let Ok(res) = handle.wait_with_output(){
           if !capture{
               return Result::None;
           }
           else{
               let mut text: String = String::from_utf8_lossy(&res.stdout).to_string();
               if text.ends_with("\n"){
                    text.remove(text.len()-1);
               }
            return Result::String(text);
           }
       }
       else{
           return Result::Error("Commands failed".to_string());
       }
   }
   else{
       return Result::Error("Pipe failed".to_string());
   }
}