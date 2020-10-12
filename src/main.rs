use anyhow::Result;
use std::io::{self, Write};
use std::sync::Arc;
mod args;
mod constants;
mod context;
mod result;
mod runtime;
use linefeed::ReadResult;
use linefeed::Signal;
#[path = "lang/ast.rs"]
mod ast;
mod completer;
#[path = "lang/interpreter.rs"]
mod interpreter;

fn main() -> Result<()> {
    let matches = args::parse_args();
    let runtime = runtime::Runtime::new();
    if let Some(path) = matches.value_of("INPUT") {
        run_file(runtime, path)?;
    } else {
        main_loop(runtime)?;
    }
    Ok(())
}

fn run_file(runtime: Arc<runtime::Runtime>, path: &str) -> Result<()> {
    let path = std::fs::canonicalize(std::path::Path::new(path))?;
    std::env::set_current_dir(path.parent().unwrap())?;
    runtime.exec_file(path.as_path())?;
    Ok(())
}

fn main_loop(runtime: Arc<runtime::Runtime>) -> Result<()> {
    runtime.print_start()?;
    io::stdout().flush()?;
    let interface = &runtime.interface;
    loop {
        let lineresult = interface.read_line()?;
        match lineresult {
            ReadResult::Input(line) => {
                let line = line.trim().to_string();
                if !line.is_empty() {
                    interface.add_history_unique(line.clone());
                    runtime.clone().exec(line)?;
                }
                runtime.print_end()?;
                runtime.print_start()?;
                io::stdout().flush()?;
            }
            ReadResult::Signal(signal) => {
                if signal == Signal::Interrupt {
                    runtime.clear_line();
                    runtime.print_start()?;
                    interface.cancel_read_line()?;
                    io::stdout().flush()?;
                } else {
                    println!("OTHER SIGNAL {:?}", signal);
                    break;
                }
            }
            _ => break,
        }
    }
    Ok(())
}
