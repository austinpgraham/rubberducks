/**
* The primary entry point into the CLI of Rubber Ducks, where
* all other aspects can be started.
*/

// External crates
extern crate structopt;

pub mod cli;

use structopt::StructOpt;
use cli::Command;

fn main() {
    let opts = cli::RD::from_args();
    
    // Let's fire off the command!!
    match opts.cmd {

        // For our dataserver...
        Command::Dataserver(cmd) => cli::dataserver::run_dataserver_command(&cmd),
        _ => {
            println!("Unknown command. Exiting.")
        }
    }
}
