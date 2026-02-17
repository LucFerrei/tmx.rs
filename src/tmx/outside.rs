use std::process;

use super::fzf::fzf_dir;
use inquire::InquireError;
use super::{has_tmux_prefix, get_active_session, get_last_two_components};

pub fn create_or_attach() -> Result<(), InquireError> {
    let path = fzf_dir()?;
   
    let session: String;
    
    // if has_tmux_prefix(&path) == true { session = get_active_session(&path) }
    // else { session = get_last_two_components(&path) } 

    if has_tmux_prefix(&path) == true {
        session = get_active_session(&path);
        
        process::Command::new("tmux")
            .args(["attach-session", "-t", &session])
            .status()
            .expect("Error in attach session");

        Ok(())

    }else{
        session = get_last_two_components(&path);

        process::Command::new("tmux")
            .args(["new-session", "-d","-s", &session, "-c", &path])
            .status()
            .expect("Error in create session");

        process::Command::new("tmux")
            .args(["attach-session", "-t", &session.trim()])
            .status()
            .expect("Error in attach-session");

        Ok(())
    }
}

#[cfg(test)]
mod tests{
    use super::super::fzf::fzf_dir;
    use super::{has_tmux_prefix, get_active_session, get_last_two_components};
    use crate::tmx::outside::create_or_attach;

    #[test]
    fn create_or_attach_test() {
        let path1 = String::from("/home/lucas/c_lang");
        let path2 = String::from("[TMUX] lucas/rust");
       
        let session1: String;
        let session2: String;
        
        if has_tmux_prefix(&path1) == true { session1 = get_active_session(&path1) }
        else { session1 = get_last_two_components(&path1) } 

        if has_tmux_prefix(&path2) == true { session2 = get_active_session(&path2) }
        else { session2 = get_last_two_components(&path2) } 

        assert_eq!(session1, String::from("lucas/c_lang"));

        assert_eq!(session2, String::from("lucas/rust"));
    }
}
