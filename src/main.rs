#![feature(proc_macro_hygiene, decl_macro)]
#![crate_name = "rd"]
/**
* The primary entry point into the CLI of Rubber Ducks, where
* all other aspects can be started.
*/

// External crates
#[macro_use]
extern crate log;

extern crate structopt;
extern crate dotenv;
extern crate fern;
extern crate chrono;

pub mod cli;

use structopt::StructOpt;
use cli::Command;

/// Setup the logger.
///
/// By default, this will log to stdout and locally
/// to a file out/output.log for any searching necessary.
fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Utc::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}


fn main() {
    dotenv::dotenv().ok();
    setup_logger().expect("Could not configure logger.");
    
    // Let's fire off the command!!
    let opts = cli::RD::from_args();
    match opts.cmd {

        // For our dataserver...
        Command::Dataserver(cmd) => cli::dataserver::run_dataserver_command(&cmd),
        _ => {
            println!("Unknown command. Exiting.")
        }
    }
}
