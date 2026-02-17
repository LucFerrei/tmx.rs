use std::{env::args,
    process::{self, Command}};
mod mode;
mod tmx;

use mode::tmux;
use tmx::have_config;
use tmx::{inside, outside};
use inquire::InquireError;

fn main() -> Result<(), InquireError> {
    let args: Vec<String> = args().collect();
    
    let mode_input = args.get(1);

    let mut mode: Option<mode::Command> = None;

    match tmux::in_tmux() {
       true => {
           match mode_input {
               Some(m) => {
                   match m.as_str() {
                       // "delete" => {mode = Some(mode::Command::Delete)},
                       _ => { inside::create_or_attach() },
                   }
               }
               None => { inside::create_or_attach() } 
           }
       },
       false => {
           match mode_input {
               Some(m) => {
                   match m.as_str() {
                       // "delete" => {mode = Some(mode::Command::Delete)},
                       _ => { outside::create_or_attach() },
                   }
               }
               None => { outside::create_or_attach() } 
           }
        }
    }
}
