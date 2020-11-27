/**
* This file contains code for directing subcommands of the Rubber
* Duck CLI.
*/
use structopt::StructOpt;

pub mod dataserver;
pub mod environment;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "rd",
    about = "Welcome to Rubber Duck! This CLI assists you in completing all your necessary tasks related to the Rubber Duck environment.")]
pub struct RD {
    #[structopt(subcommand)]
    pub cmd: Command
}

#[derive(Debug, StructOpt)]
pub enum Command {
    Dataserver(dataserver::DataserverCLI),
    Environment(environment::EnvironmentCLI)
}
