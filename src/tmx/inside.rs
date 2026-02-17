use std::process;
use inquire::InquireError;
use super::fzf::fzf_dir;
use super::{has_tmux_prefix, get_active_session, get_last_two_components};

pub fn create_or_attach() -> Result<(), InquireError> {
    let path = fzf_dir()?;

    let session: String;
    
    // if has_tmux_prefix(&path) == true { session = get_active_session(&path) }
    // else { session = get_last_two_components(&path) } 

    if has_tmux_prefix(&path) == true {
        session = get_active_session(&path);
        // let mensagem = format!("{}:1",&session.trim());
        // println!("Format {}", mensagem);

        process::Command::new("tmux")
            // .args(["switch-client", "-t", &format!("{}:1",&session.trim())])
            .args(["switch-client", "-t", &session.trim()])
            .status()
            .expect("Error in switch client");

        Ok(())
    }else {
        session = get_last_two_components(&path);

        process::Command::new("tmux")
            .args(["new", "-s", &session, "-c", &path, "-d"])
            .status()
            .expect("Error in create session");

        process::Command::new("tmux")
            .args(["switch-client", "-t", &session.trim()])
            .status()
            .expect("Error in switch client");

        Ok(())
    }
}
