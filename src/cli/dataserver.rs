/**
* This file defines different subcommands for starting, terminating, etc
* the dataserver for Rubber Duck
*/
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "dataserver")]
pub struct DataserverCLI {

    #[structopt(subcommand)]
    pub cmd: DataserverCommand
}

#[derive(Debug, StructOpt)]
pub struct StartCLI {
    #[structopt(
        default_value = "0.0.0.0",
        help = "Host on which the server will be exposed.",
        short,
        long
    )]
    host: String,

    #[structopt(
        default_value = "5555",
        help = "Port on which the server will be exposed.",
        short,
        long
    )]
    port: i32
}

#[derive(Debug, StructOpt)]
pub enum DataserverCommand {
    Start(StartCLI)
}

pub fn run_dataserver_command(command: &DataserverCLI) {
    match &command.cmd {
        DataserverCommand::Start(cmd) => {
            println!("Starting server at host {}:{}...", cmd.host, cmd.port);
        }
    }
}
