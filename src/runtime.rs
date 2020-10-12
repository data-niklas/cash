use super::completer::CashCompleter;
use super::constants::*;
use super::interpreter;
use super::result;
use crate::context::Context;
use anyhow::*;
use dirs;
use linefeed::terminal::DefaultTerminal;
use linefeed::{Interface, Signal};
use std::io::prelude::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct Runtime<'a> {
    pub interface: Arc<Interface<DefaultTerminal>>,
    pub basectx: Arc<Context<'a>>,
}

impl<'a> Runtime<'a> {
    pub fn new() -> Arc<Runtime<'a>> {
        let runtime = Runtime {
            basectx: Arc::new(Context::new()),
            interface: Runtime::make_interface(),
        };
        let runtime = Arc::new(runtime);
        runtime.clone().init();
        runtime.clone().load_config();
        runtime.clone().load_history();
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

    fn init(self: Arc<Self>) {
        self.basectx.set_var(
            "PREFIX",
            result::Result::String("\x1b[1m> \x1b[0m".to_string()),
        );
        self.basectx
            .set_var("SUFFIX", result::Result::String("\n".to_string()));

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

    fn load_history(self: Arc<Self>) {
        if let Some(path) = dirs::home_dir() {
            let path = path.join(std::path::Path::new(".cash_history"));
            self.interface.load_history(path);
        }
    }

    fn save_history(self: Arc<Self>) {
        if let Some(path) = dirs::home_dir() {
            let path = path.join(std::path::Path::new(".cash_history"));
            self.interface.save_history(path);
        }
    }

    fn load_config(self: Arc<Self>) {
        if let Some(path) = dirs::home_dir() {
            let path = path.join(std::path::Path::new(".cashrc"));
            if path.exists(){
                self.include_file(path.as_path());
            }
        }
    }

    pub fn exec(self: Arc<Self>, text: String) -> Result<result::Result> {
        let res = interpreter::interpret(text.clone(), self.clone(), self.basectx.clone())?;
        res.print();
        Ok(res)
    }

    pub fn exec_file(self: Arc<Self>, path: &std::path::Path) -> Result<result::Result> {
        let mut file = std::fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        return self.exec(contents);
    }

    fn var_string(self: &Arc<Self>, name: &str) -> String {
        let var = self.basectx.var(name);
        if let result::Result::String(text) = var {
            return text;
        } else if let result::Result::Function {
            block: _,
            parameters: _,
            vars: _,
        } = var
        {
            if let result::Result::String(text) =
                interpreter::interpret_function(name, self.clone())
            {
                return text;
            } else {
                println!(
                    "{}",
                    result::Result::Error("Function needs to return a string".to_string())
                        .to_string()
                );
                return " ".to_string();
            }
        } else {
            println!(
                "{}",
                result::Result::Error(name.to_string() + " needs to be a string or a function")
                    .to_string()
            );
            return " ".to_string();
        }
    }

    pub fn print_start(self: &Arc<Self>) -> Result<()> {
        let text = self.var_string("PREFIX");
        print!("{}", text);
        return Ok(());
    }

    pub fn print_end(self: &Arc<Runtime<'a>>) -> Result<()> {
        let text = self.var_string("SUFFIX");
        print!("{}", text);
        return Ok(());
    }

    pub fn include_file(self: Arc<Self>, path: &std::path::Path) -> Result<result::Result>{
        let cd = std::env::current_dir().unwrap_or_default();
        std::env::set_current_dir(std::fs::canonicalize(path.parent().unwrap()).unwrap());
        let res = self.exec_file(path);
        std::env::set_current_dir(cd);
        return res;
    }

    pub fn clear(&self) {
        println!("\x1b[2J\x1b[1;1H");
    }

    pub fn clear_line(&self) {
        println!("\x1b[2K");
    }

    pub fn quit(self: Arc<Self>) {
        self.save_history();
        std::process::exit(0);
    }

    pub fn which(name: &str) -> Result<std::path::PathBuf> {
        for mut path in std::env::split_paths(&std::env::var_os("PATH").unwrap()) {
            path.push(name);
            if path.exists() {
                return Ok(path);
            }
        }
        return Err(anyhow!("Could not find file in path"));
    }
}
