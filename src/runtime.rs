use super::completer::CashCompleter;
use super::constants::*;
use super::interpreter;
use super::result;
use crate::context::Context;
use anyhow::*;
use linefeed::terminal::DefaultTerminal;
use linefeed::{Interface, Signal};
use std::sync::Arc;
use std::io::prelude::*;
use dirs;

#[derive(Clone)]
pub struct Runtime<'a> {
    pub interface: Arc<Interface<DefaultTerminal>>,
    pub basectx: Context<'a>,
}

impl<'a> Runtime<'a> {
    pub fn new() -> Runtime<'a> {
        let mut runtime = Runtime {
            basectx: Context::new(),
            interface: Runtime::make_interface(),
        };
        runtime.init();
        runtime.load_config();
        runtime.load_history();
        return runtime;
    }

    fn make_interface() -> Arc<Interface<DefaultTerminal>> {
        let interface = Interface::new("cash").unwrap();
        interface.set_report_signal(Signal::Break, true);
        interface.set_report_signal(Signal::Interrupt, true);
        interface.set_report_signal(Signal::Quit, true);
        interface.set_completer(Arc::new(CashCompleter));
        return Arc::new(interface);
    }

    fn load_history(&self){
        if let Some(path) = dirs::home_dir(){
            let path = path.join(std::path::Path::new(".cash_history"));
            self.interface.load_history(path);
        }
    }

    fn save_history(&self){
        if let Some(path) = dirs::home_dir(){
            let path = path.join(std::path::Path::new(".cash_history"));
            self.interface.save_history(path);
        }
    }

    fn init(&mut self) {
        self.basectx.set_var(
            "LINE_PREFIX",
            result::Result::String("\x1b[1m> \x1b[0m".to_string()),
        );
        self.basectx
            .set_var("AFTER_LINE", result::Result::String("\n".to_string()));

        self.basectx
            .set_var("APPNAME", result::Result::String(APPNAME.to_string()));
        self.basectx
            .set_var("VERSION", result::Result::String(VERSION.to_string()));
        self.basectx
            .set_var("AUTHOR", result::Result::String(AUTHOR.to_string()));
        self.basectx
            .set_var("ABOUT", result::Result::String(ABOUT.to_string()));

        self.basectx
            .set_var("PI", result::Result::Float(std::f64::consts::PI));
        self.basectx
            .set_var("E", result::Result::Float(std::f64::consts::E));
        self.basectx
            .set_var("PHI", result::Result::Float(1.61803398874989484820));
    }

    fn load_config(&mut self) {}

    pub fn exec(&self, text: String) -> Result<()> {
        let res = interpreter::interpret(text.clone(), self, &self.basectx)?;
        res.print();
        Ok(())
    }

    pub fn exec_file(&self, path: &std::path::Path) -> Result<()> {
        let mut file = std::fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        return self.exec(contents);
    }

    pub fn print_start(&self) -> Result<()> {
        if let result::Result::String(text) = self.basectx.var(&"LINE_PREFIX") {
            print!("{}", text);
            return Ok(());
        } else {
            return Err(anyhow!("Variable LINE_PREFIX not found"));
        }
    }

    pub fn print_end(&self) -> Result<()> {
        if let result::Result::String(text) = self.basectx.var(&"AFTER_LINE") {
            print!("{}", text);
            return Ok(());
        } else {
            return Err(anyhow!("Variable AFTER_LINE not found"));
        }
    }

    pub fn clear(&self) {
        println!("\x1b[2J\x1b[1;1H");
    }

    pub fn clear_line(&self) {
        println!("\x1b[2K");
    }

    pub fn quit(&self) {
        self.save_history();
        std::process::exit(0);
    }

    pub fn which(name: &str) -> Result<std::path::PathBuf>{
        for mut path in std::env::split_paths(&std::env::var_os("PATH").unwrap()) {
            path.push(name);
            if path.exists(){
                return Ok(path);
            }
        }
        return Err(anyhow!("Could not find file in path"));
    }
}
