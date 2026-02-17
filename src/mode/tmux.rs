use std::{env, process};

pub fn in_tmux() -> bool {
    match env::var("TMUX") {
       Ok(_) => return true,
       Err(_) => return false,
    }
}

pub fn has_tmux_server() -> bool {
    let res = process::Command::new("tmux")
        .arg("has-session")
        .output()
        .expect("Could not solve if tmux server is running");
    
    if res.status.success() {
        return true;
    }
    false
}
