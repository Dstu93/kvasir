


pub mod processor;

use clap::ArgMatches;

/// Enum of Kvasir sub-commands with its arguments
#[derive(Debug,Clone)]
pub enum Command<'a: 'b,'b>{
    Add(Option<&'b ArgMatches<'a>>),
    Edit(Option<&'b ArgMatches<'a>>),
    Read(Option<&'b ArgMatches<'a>>),
    Delete(Option<&'b ArgMatches<'a>>),
    Config(Option<&'b ArgMatches<'a>>),
    Search(Option<&'b ArgMatches<'a>>),
}

impl <'b,'a>Command<'b,'a>{

    /// matches the command from &str
    pub fn match_from_str(cmd: &str, args: Option<&'b ArgMatches<'a>>) -> Result<Command<'b,'a>,()> {
        match cmd{
            "add" => {Ok(Command::Add(args))},
            "edit" => {Ok(Command::Edit(args))},
            "read" => {Ok(Command::Read(args))},
            "delete" => {Ok(Command::Delete(args))},
            "config" => {Ok(Command::Config(args))},
            "search" => {Ok(Command::Search(args))},
            _ => {Err(())}
        }
    }

}