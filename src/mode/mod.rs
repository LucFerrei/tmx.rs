pub mod tmux;

pub enum Command {
    CreateOrAttach,
    Delete,
}

impl Command {
   pub fn execute(&self, in_tmux: bool) {
       if in_tmux {
           match self {
              Command::CreateOrAttach => {},
              Command::Delete => {},
           }
            
       }else {
           match self {
              Command::CreateOrAttach => {},
              Command::Delete => {},
           }
       }
   }
}
