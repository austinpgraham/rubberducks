/**
* The primary entry point into the CLI of Rubber Ducks, where
* all other aspects can be started.
*/

// External crates
extern crate structopt;

use structopt::StructOpt;

pub mod cli;

fn main() {
    let opts = cli::RD::from_args();
    println!("{:?}", opts);
}
