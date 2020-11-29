/**
* This file defines different subcommands for starting, terminating, etc
* the dataserver for Rubber Duck
*/
use structopt::StructOpt;
use std::process::Command;

// Local imports
use crate::dataserver;
use crate::cli::environment::{
    write_server_pid_file,
    get_server_pid_file
};

/// Passthrough command for the dataserver subcommand of `rd`
#[derive(Debug, StructOpt)]
#[structopt(name = "dataserver")]
pub struct DataserverCLI {

    #[structopt(subcommand)]
    pub cmd: DataserverCommand
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Start the Rubber Ducks dataserver.")]
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
    port: u16,

    #[structopt(
        default_value = "3",
        help = "Number of threads to spawn on start.",
        short,
        long
    )]
    workers: u16
}

/// Listing all options for the the dataserver subcommand.
#[derive(Debug, StructOpt)]
pub enum DataserverCommand {

    // Start the server as a separate process
    Start(StartCLI),

    // Start the server and wait
    RawStart(StartCLI)
}

/// Run a dataserver command.
/// 
/// # Arguments
/// * `command` - Command to run on the dataserver.
/// 
/// # Examples
/// ```
/// run_dataserver_command(&command);
/// ```
pub fn run_dataserver_command(command: &DataserverCLI) {
    match &command.cmd {

        // Start as separate process
        DataserverCommand::Start(cmd) => {
            info!("Spawning server process at {}:{}...", cmd.host, cmd.port);

            // We only want to spawn a process if there's not already a running process
            if get_server_pid_file().is_ok() {
                panic!("Cannot start server: process already exists.");
            }

            // Spawn the process
            let mut server_process = Command::new("rd")
                                            .arg("dataserver")
                                            .arg("raw-start")
                                            .arg("-h")
                                            .arg(format!("{}", cmd.host))
                                            .arg("-p")
                                            .arg(format!("{}", cmd.port))
                                            .arg("-w")
                                            .arg(format!("{}", cmd.workers))
                                            .spawn()
                                            .expect("Failed to start server process.");
            
            // Write the server process ID
            if write_server_pid_file(server_process.id()).is_err() {
                server_process.kill().expect("Failed to kill process on failure to write new process ID.");
                panic!("Failed to write process ID to file.");
            }

            info!("Spawned server process with PID {}.", server_process.id());
        },

        // Start server and wait
        DataserverCommand::RawStart(cmd) => {
            info!("Starting server at host {}:{}...", cmd.host, cmd.port);
            dataserver::start_dataserver(&cmd.host, cmd.port, cmd.workers);
        }
    }
}
