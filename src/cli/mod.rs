/**
* This file contains code for directing subcommands of the Rubber
* Duck CLI.
*/
use structopt::StructOpt;

pub mod dataserver;

#[derive(Debug, StructOpt)]
#[structopt(name = "rd")]
pub struct RD {
    #[structopt(subcommand)]
    cmd: Command
}

#[derive(Debug, StructOpt)]
enum Command {
    Dataserver(dataserver::DataserverCLI)
}
