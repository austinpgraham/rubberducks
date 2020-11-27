/**
* This file contains code that deals with the environment
* in which the Rubber Duck project is running.
*/
use std::{
    env,
    fs::{
        File,
        create_dir
    },
    io::{
        BufReader,
        BufRead,
        Write
    },
    path::PathBuf,
    collections::HashMap
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
pub struct SetEnvCLI {
    #[structopt(
        short,
        long,
        about = "The variable key for the environment."
    )]
    key: String,

    #[structopt(
        short,
        long,
        about = "The variable value for the environment."
    )]
    value: String
}

#[derive(Debug, StructOpt)]
pub struct RemoveEnvCLI {
    #[structopt(
        short,
        long,
        about = "The variable key to remove from environment."
    )]
    key: String
}

#[derive(Debug, StructOpt)]
pub enum EnvironmentCommand {

    #[structopt(about = "Lists all environment variables known to the CLI.")]
    GetEnv,

    #[structopt(about = "Sets an environment variable.")]
    SetEnv(SetEnvCLI),

    #[structopt(about = "Removes an item from the environment.")]
    RemoveEnv(RemoveEnvCLI)
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
        match create_dir::<&PathBuf>(&home_directory) {
            Ok(()) => Ok(String::from(home_directory.to_str().unwrap())),
            Err(err) => Err(err.to_string())
        }
    } else {
        Ok(String::from(home_directory.to_str().unwrap()))
    }
}

pub fn get_or_create_env_file() -> Result<String, String> {
    let home_dir = get_or_create_rd_home()?;
    let mut env_path = PathBuf::from(&home_dir);
    env_path.push("base.env");

    // If it does not exist, then create it.
    if !env_path.exists() {
        match File::create::<&PathBuf>(&env_path) {
            Ok(mut file) => {
                // If we've created the file, we need to write RD_HOME
                match file.write_all(format!("RD_HOME={}\n", home_dir).as_bytes()) {
                    Ok(()) => {
                        if file.sync_data().is_err() {
                            error!("There was an error syncing environment data with disk.")
                        }
                        Ok(String::from(env_path.to_str().unwrap()))
                    },
                    Err(err) => Err(err.to_string())
                }
            },
            Err(err) => Err(err.to_string())
        }
    } else {
        Ok(String::from(env_path.to_str().unwrap()))
    }
}

pub fn get_env() -> Result<HashMap<String, String>, String> {
    let mut var_map: HashMap<String, String> = HashMap::new();
    let home_directory = match get_or_create_env_file() {
        Ok(dir) => dir,
        Err(msg) => return Err(msg)
    };

    // Read each line of the file and dump the environment variables
    let file = File::open::<String>(home_directory).unwrap();
    for line in BufReader::new(file).lines() {
        let env_line = match line {
            Ok(l) => l,
            Err(err) => return Err(err.to_string())
        };

        // Split out the the environment information and put into the HashMap
        let env_content = env_line.split("=").map(|s| String::from(s)).collect::<Vec<String>>();
        if env_content.len() < 2 {
            // Skip bad lines
            continue;
        }

        let variable = env_content[0].trim().to_string();
        let value = env_content[1].trim().to_string();
        var_map.insert(variable, value);
    }
    Ok(var_map)
}

pub fn write_to_env(vars: &HashMap<String, String>) -> Result<(), String> {
    let mut env_file = File::with_options().write(true).open(get_or_create_env_file()?).unwrap();

    vars.iter().for_each(|(key, value)| {
        env_file.write_all(format!("{}={}\n", key, value).as_bytes()).unwrap();
    });

    // Make sure it all gets to disk
    match env_file.sync_data() {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string())
    }
}

pub fn run_environment_command(command: &EnvironmentCLI) {
    match &command.cmd {
        EnvironmentCommand::GetEnv => {
            let variables = match get_env() {
                Ok(v) => v,
                Err(msg) => {
                    error!("{}", msg);
                    HashMap::new()
                }
            };

            // Write them all out to standard out
            variables.iter().for_each(|(k, v)| info!("{}: {}", k, v));
        },
        EnvironmentCommand::SetEnv(new_var) => {
            if let Ok(mut variables) = get_env() {
                // Add the new variable
                variables.insert(new_var.key.clone(), new_var.value.clone());

                match write_to_env(&variables) {
                    Ok(()) => info!("Successfully set environment variable"),
                    Err(msg) => error!("{}", msg)
                }

            } else {
                error!("There was an error retrieving environment configuration.");
            }
        },
        EnvironmentCommand::RemoveEnv(remove_var) => {
            if let Ok(mut variables) = get_env() {
                // Add the new variable
                variables.remove(&remove_var.key);

                match write_to_env(&variables) {
                    Ok(()) => info!("Successfully removed environment variable"),
                    Err(msg) => error!("{}", msg)
                }

            } else {
                error!("There was an error retrieving environment configuration.");
            }
        }
    }
}
