

use app::cmd::Command;
use app::application::{ApplicationError};

#[derive(Copy, Clone,Ord, PartialOrd, Eq, PartialEq,Debug,Hash)]
/// Processor for [app::cmd::Command]
pub struct Processor;

impl Processor{

    /// executes the given Command, if something bad is happening then it returns an ApplicationError
    /// (in case of on IO Error or invalid arguments etc)
    pub fn execute<'a,'b>(cmd: Command<'a,'b>) -> Result<(),ApplicationError>{
        match cmd {
          Command::Add(args) => {Err(ApplicationError::UnknownSubCommand)},
            _ => {Err(ApplicationError::UnknownSubCommand)}
        }
    }

}