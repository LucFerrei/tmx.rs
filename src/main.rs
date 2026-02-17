use std::{env::args,
    process::{self, Command}};
mod mode;
mod tmx;

use mode::tmux;
use tmx::have_config;
use tmx::outside::create_or_attach;
use inquire::InquireError;

fn main() -> Result<(), InquireError> {
    let args: Vec<String> = args().collect();
    
    let mode_input = args.get(1);

    let mut mode: Option<mode::Command> = None;

    // match mode_input {
    //    Some(m) => {
    //         match m.as_str() {
    //             "delete" => {mode = Some(mode::Command::Delete)},
    //             _ => {mode = Some(mode::Command::CreateOrAttach)},
    //         }
    //    }
    //    None => mode = Some(mode::Command::CreateOrAttach)
    // }

    match mode_input {
       Some(m) => {
            match m.as_str() {
                // "delete" => {mode = Some(mode::Command::Delete)},
                _ => { create_or_attach() },
            }
       }
       None => { create_or_attach() } 
    }
    
    // let in_tmux = tmux::in_tmux();
    // match have_config() {
    //     Ok(_) => println!("Há aquivo de config"),
    //     Err(_) => println!("Não há arquivo de config"),
    // };
    //
    // // tmx::fzf::fzf();
    // match tmux::has_tmux_server() {
    //     true => println!("há server"),
    //     false => println!("não há server")
    // }
    //
    // match tmx::fzf::fzf(Some(tmx::fzf::list_active_sessions()), tmx::fzf::get_inactive_dirs()) {
    //     Ok(choice) => println!("{}", choice),
    //     Err(_) => println!("Error")
    // }

}
