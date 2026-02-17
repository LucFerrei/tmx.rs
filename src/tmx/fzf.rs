use std::process;
use inquire::ui::{Color, RenderConfig, Styled};
use inquire::{InquireError, Select};
use walkdir::WalkDir;
use super::get_last_two_components;

use super::super::mode::tmux;


pub fn list_active_sessions() -> Vec<String> {
    let res = process::Command::new("tmux")
        .args(["list-sessions", "-F", "[TMUX] #S"])
        .output()
        .expect("Error in listing sessions");


    String::from_utf8_lossy(&res.stdout)
        .lines()
        .map(|s| s.to_string())
        .collect()
}

pub fn get_inactive_dirs() -> Vec<String> {
    // TODO: read dirs from config or default
    WalkDir::new("/home/lucas/")
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
        .map(|e| e.path().display().to_string())
        .collect()
}

fn sanitize_dir(active_sessions: Vec<String>, inactive_dirs: Vec<String>) -> Vec<String> {
    let mut sanitized: Vec<String> = Vec::new();
    let mut inactive: Vec<String> = Vec::new();

    for element in inactive_dirs {
        let last_two = get_last_two_components(&element);
        let mut real_element = String::from("[TMUX] ");

        real_element.push_str(&last_two);
        // let cmp = real_element.replace('.', "_");

        if active_sessions.contains(&real_element) {continue;}
        else { inactive.push(element); }
    }
    sanitized.extend(active_sessions);
    sanitized.extend(inactive);

    sanitized
}

pub fn fzf(mut active_sessions: Option<Vec<String>>, inactive: Vec<String>) -> Result<String, InquireError> {

    let render_config = RenderConfig::default()
            .with_prompt_prefix(Styled::new("🔍").with_fg(Color::LightBlue))
            .with_highlighted_option_prefix(Styled::new("👉").with_fg(Color::LightYellow));

    let dirs: Vec<String>;

    if let Some(dir) = active_sessions {
        dirs = sanitize_dir(dir, inactive);
        // dirs.extend(dir);
    }else {
        dirs = sanitize_dir(Vec::new(), inactive);
    }

    // dirs.extend(inactive);
    let answer = Select::new("Select a directory:", dirs)
        .with_render_config(render_config)
        .prompt()?;

    Ok(answer)
}

pub fn fzf_dir() -> Result<String, InquireError> {
    let is_running = tmux::has_tmux_server();

    if is_running {
        let active_sessions = list_active_sessions();
        let inactive = get_inactive_dirs();

        let res = fzf(Some(active_sessions), inactive)?;
        Ok(res)
    }else {
        let inactive = get_inactive_dirs();

        let res = fzf(None, inactive)?;
        Ok(res)
    }
}
