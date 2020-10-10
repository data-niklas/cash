#[path = "lang/doc.rs"]
mod doc;
use doc::*;
use linefeed::{Prompter, Terminal, Completer, Completion, Suffix};

pub struct CashCompleter;
impl<Term: Terminal> Completer<Term> for CashCompleter {
    fn complete(
        &self,
        word: &str,
        _prompter: &Prompter<Term>,
        _start: usize,
        end: usize
    ) -> Option<Vec<Completion>>{
        let chars = word.chars().collect::<Vec<char>>();
        let mut ident = "".to_string();
        let mut identstart: usize = 0;
        if chars.len() > 0{
            identstart = end-1;
            loop{
                let c = chars[identstart];
                if c.is_ascii_alphanumeric() || c.is_ascii_digit() || c == '$' || c == '_' {
                    if identstart == 0{
                        break
                    }
                    else{
                        identstart-=1;
                    }
                }
                else{
                    identstart+=1;
                    break
                }
            }
            if identstart == end{
                return None
            }
            else{
                ident = chars.iter().skip(identstart).collect::<String>();
            }
        }
        let mut results = Vec::new();
        for key in FUNCTIONS.keys(){
            if key.starts_with(ident.as_str()){
                results.push(Completion{
                    completion: chars.iter().take(identstart).collect::<String>() + key + "(",
                    display: Some(key.to_string()),
                    suffix: Suffix::None
                });
            }
        }
        if results.len() == 0{
            return None;
        }
        else {
            return Some(results);
        }
    }
}