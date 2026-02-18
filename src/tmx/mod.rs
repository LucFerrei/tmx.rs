pub mod inside;
pub mod outside;
pub mod fzf;
pub mod sessionizer;
pub mod config;

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
    let path = std::path::Path::new(path_str);
    
    let last_two: Vec<_> = path
        .components()
        .filter(|c| matches!(c, std::path::Component::Normal(_)))
        .rev()
        .take(2)
        .collect();

    let full_name = last_two.into_iter()
        .rev()
        .collect::<std::path::PathBuf>()
        .to_string_lossy()
        .into_owned();

    full_name
        .replace('.', "_")
        .replace(':', "_")
}
