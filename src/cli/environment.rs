/**
* This file contains code that deals with the environment
* in which the Rubber Duck project is running.
*/
use std::{
    env,
    fs,
    path::PathBuf
};
use dirs::home_dir;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "env")]
pub struct EnvironmentCLI {

    #[structopt(subcommand)]
    pub cmd: EnvironmentCommand
}

#[derive(Debug, StructOpt)]
pub enum EnvironmentCommand {

    #[structopt(about = "Lists all environment variables known to the CLI.")]
    GetEnv,

    #[structopt(about = "Sets an environment variable.")]
    SetEnv
}

pub fn get_or_create_rd_home() -> Result<String, String> {
    let home_directory = match env::var("RD_HOME") {
        Ok(home) => PathBuf::from(home),
        Err(_) => match home_dir() {
            Some(mut dir) => {
                dir.push(".rd");
                dir
            },
            None => return Err(String::from("Failed to get system home directory. Set RD_HOME to correct this error."))
        }
    };

    // If the directory does not exist, create it.
    if !home_directory.exists() {
        match fs::create_dir::<&PathBuf>(&home_directory) {
            Ok(()) => Ok(String::from(home_directory.to_str().unwrap())),
            Err(err) => Err(err.to_string())
        }
    } else {
        Ok(String::from(home_directory.to_str().unwrap()))
    }
}

pub fn run_environment_command(command: &EnvironmentCLI) {
    match command.cmd {
        EnvironmentCommand::GetEnv => {
            let home_directory = match get_or_create_rd_home() {
                Ok(h) => h,
                Err(msg) => {
                    error!("{}", msg);
                    String::new()
                }
            };

            info!("{}", home_directory);
        },
        _ => info!("Not yet implemented.")
    }
}
