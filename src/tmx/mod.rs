pub mod inside;
pub mod outside;
pub mod fzf;

use std::process;
use std::fs::File;
use std::path::{Path, Component};

const CONFIG_PATH: &str = "/home/lucas/.config/tmx/tmx.conf";

pub fn have_config() -> std::io::Result<()> {
   let mut file = File::open(CONFIG_PATH)?;
   Ok(())
}

pub fn has_sessionizer(path: &str) -> bool {
    match File::open(path) {
        Ok(_) => return true,
        Err(_) => return false,
    }
}

pub fn has_tmux_prefix(path: &String) -> bool {
    if path.contains("[TMUX]") {
        return true;
    }
    false
}

pub fn get_active_session(path: &String) -> String {
    let active_session: String = path[7..].to_string();
    active_session
}


pub fn get_last_two_components(path_str: &str) -> String {
    let path = Path::new(path_str);
    
    let last_two: Vec<_> = path
        .components()
        .filter(|c| matches!(c, Component::Normal(_))) // <--- IGNORE "/" OR "C:\"
        .rev()
        .take(2)
        .collect();

    // If we found nothing (e.g. path was just "/"), return a fallback
    if last_two.is_empty() {
        return "default".to_string();
    }

    last_two.into_iter()
        .rev()
        .collect::<std::path::PathBuf>()
        .to_string_lossy()
        .into_owned()
}
// let args: Vec<String> = args().collect();
//
// let mode_input = args.get(1);
//
// let mut mode: Option<mode::Command> = None;
//
// match mode_input {
//    Some(m) => {
//         match m.as_str() {
//             "delete" => {mode = Some(mode::Command::Delete)},
//             _ => {mode = Some(mode::Command::Delete)},
//         }
//    }
//    None => mode = Some(mode::Command::CreateOrAttach)
// }
