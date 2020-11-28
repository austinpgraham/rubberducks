#![feature(proc_macro_hygiene, decl_macro, with_options)]
#![crate_name = "rd"]
/**
* The primary entry point into the CLI of Rubber Ducks, where
* all other aspects can be started.
*/

// External crates
#[macro_use]
extern crate log;

extern crate structopt;
extern crate fern;
extern crate chrono;
extern crate juniper;
extern crate juniper_rocket;
extern crate rocket;
extern crate dirs;

pub mod cli;
pub mod dataserver;

use structopt::StructOpt;
use cli::Command;

/// Setup the logger.
///
/// By default, this will log to stdout and locally
/// to a file out/output.log for any searching necessary.
fn setup_logger() -> Result<(), fern::InitError> {

    // Generate the new output file path
    let log_file_path = match cli::environment::get_or_create_log_file() {
        Ok(path) => path,
        Err(err) => return Err(fern::InitError::from(std::io::Error::new(std::io::ErrorKind::Other, err)))
    };

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
        .chain(fern::log_file(log_file_path)?)
        .apply()?;
    Ok(())
}


/// Entry point of the CLI
fn main() {
    // Set all known environment variables
    if cli::environment::set_environment().is_err() {
        error!("Failed to set process environment.");
    }

    setup_logger().expect("Could not configure logger.");
    
    // Let's fire off the command!!
    let opts = cli::RD::from_args();
    match opts.cmd {

        // For our dataserver...
        Command::Dataserver(cmd) => cli::dataserver::run_dataserver_command(&cmd),
        Command::Environment(cmd) => cli::environment::run_environment_command(&cmd)
    }
}
