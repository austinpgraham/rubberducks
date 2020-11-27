/**
* This file defines different subcommands for starting, terminating, etc
* the dataserver for Rubber Duck
*/
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "dataserver")]
pub struct DataserverCLI {

    #[structopt(subcommand)]
    start: DataserverCommand
}

#[derive(Debug, StructOpt)]
enum DataserverCommand {
    Start {

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
}
