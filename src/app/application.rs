
use clap::{App,SubCommand,ArgMatches};
use app::cmd::Command;

/// constant for versioning commands and sub-commands
const VERSION: &'static str = "Version 0.1";

/// root struct of the application, it parses the arguments and handle them
pub struct Application<'a,'b> where 'a: 'b,{
    inner: App<'a,'b>
}

impl <'a,'b>Application<'a, 'b>{

    /// Creates a new Application
    pub fn new() -> Application<'a,'b> {
        let app = Application::create();
        Application{inner: app}
    }

    /// initialise clap, which is used for parsing the arguments
    fn create() -> App<'a,'b>{
        let mut app = App::new("Kvasir")
            .about("Kvasir is an command line application for interacting with confluence")
            .author("D.Stu")
            .version(VERSION)
            .help("Kvasir Version 0.1\n\
                Interacting with confluence from cli\n\
                (C) no-mail@no-mail.com\n\n\

                USAGE: kavsir <command> <options>\n\n\

                Options:\n\
                -h, --help      Display this message\n\
                -V, --version   Display version info\n\
                -id --page-id   use page-id instead of title\n\n\

                Commands:\n\
                add              Adds new wiki-page\n\
                edit             Edit an wiki-page \n\
                read             Open wiki-page read only\n\
                delete           Deletes an wiki-page\n\
                config           do stuff with the configuration\n\
                search           search in confluence");

        let add = Application::init_add();
        let edit = Application::init_edit();
        let read = Application::init_read();
        let delete = Application::init_delete();
        let config = Application::init_config();
        let search = Application::init_search();

        app.subcommand(add)
            .subcommand(edit)
            .subcommand(read)
            .subcommand(delete)
            .subcommand(config)
            .subcommand(search)
    }

    fn init_add() -> App<'a,'b> {
        SubCommand::with_name("add")
            .about("about")
            .author("")
            .version(VERSION)
            .help("")
    }

    fn init_edit() -> App<'a,'b>{
        SubCommand::with_name("edit")
    }

    fn init_read() -> App<'a,'b>{
        SubCommand::with_name("read")
    }

    fn init_delete() -> App<'a,'b>{
        SubCommand::with_name("delete")
    }

    fn init_config() -> App<'a,'b> {
        SubCommand::with_name("config")
    }

    fn init_search() -> App<'a,'b>{
        SubCommand::with_name("search")
    }

    /// takes the arguments and try to execute them, if something went wrong
    /// (like IO-Error or missing arguments)
    /// it returns an ApplicationError
    pub fn run(self) -> Result<(),ApplicationError>{
        let matches = self.inner.get_matches();
        let subcommand = matches.subcommand();
        let command = Command::match_from_str(subcommand.0, subcommand.1);
        Ok(())
    }

}

/// Enum for representing error cases for the whole application.
/// This errors cant normally handle and contains an message for the user what was going wrong
#[derive(Copy, Clone,Ord, PartialOrd, Eq, PartialEq,Hash,Debug)]
pub enum ApplicationError{
    UnknownSubCommand,
}