/**
* This file contains code that deals with the environment
* in which the Rubber Duck project is running.
*/
use std::{
    env,
    fs::{
        File,
        create_dir,
        remove_file,
        read_to_string
    },
    io::{
        BufReader,
        BufRead,
        Write
    },
    path::PathBuf,
    collections::HashMap,
    time::{
        UNIX_EPOCH,
        SystemTime
    }
};
use dirs::home_dir;
use structopt::StructOpt;

/// Passthrough for subcommands of the `environment` command
#[derive(Debug, StructOpt)]
#[structopt(name = "env")]
pub struct EnvironmentCLI {

    /// Command representative object.
    #[structopt(subcommand)]
    pub cmd: EnvironmentCommand
}

/// Command arg options for setting an environment variable.
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

/// Command arg options for removing an environment variable
#[derive(Debug, StructOpt)]
pub struct RemoveEnvCLI {
    #[structopt(
        short,
        long,
        about = "The variable key to remove from environment."
    )]
    key: String
}

/// Enum listing the various subcommands of the `environment` command.
#[derive(Debug, StructOpt)]
pub enum EnvironmentCommand {

    #[structopt(about = "Lists all environment variables known to the CLI.")]
    GetEnv,

    #[structopt(about = "Sets an environment variable.")]
    SetEnv(SetEnvCLI),

    #[structopt(about = "Removes an item from the environment.")]
    RemoveEnv(RemoveEnvCLI)
}

/// Returns the RD_HOME environment variable, creating
/// the directory if it does not already exist.
/// 
/// # Examples
/// ```
/// let home_dir = get_or_create_rd_home().unwrap();
/// ```
#[inline]
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

/// Get or create the current time log file
/// 
/// # Examples
/// ```
/// if get_or_create_log_file().is_ok() {
///     // Do some stuff
/// }
/// ```
#[inline]
pub fn get_or_create_log_file() -> Result<String, String> {
    let mut file_path = PathBuf::from(get_or_create_rd_home()?);
    let current_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("It seems time went backwards...");
    file_path.push("logs");

    // Create the logs directory if it doesn't already exists
    if !file_path.exists() && create_dir::<&PathBuf>(&file_path).is_err() {
        return Err(String::from("Failed to create logs directory."));
    }

    file_path.push(format!("output-{}.log", current_timestamp.as_secs() / 86400).as_str());

    // Create the file and return out the path
    if !file_path.exists() {
        match File::create::<&PathBuf>(&file_path) {
            Ok(_) => Ok(String::from(file_path.to_str().unwrap())),
            Err(err) => Err(err.to_string())
        }
    } else {
        Ok(String::from(file_path.to_str().unwrap()))
    }
}

/// Get's the environment variable file contained in the configuration
/// or creates it if it does not already exist. If it is first created,
/// then the RD_HOME variable is inserted immediately.
/// 
/// # Examples
/// ```
/// let env_file = get_or_create_env_file.unwrap();
/// ```
#[inline]
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

/// Get's a HashMap representing all known environment variables
/// in the configuration. Ownership of this structured is returned
/// to the caller.
/// 
/// # Examples
/// ```
/// let variables = get_env().unwrap();
/// ```
#[inline]
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
        let env_content = env_line.splitn(2, "=").map(|s| String::from(s)).collect::<Vec<String>>();
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

/// Get the server PID file
/// 
/// # Example
/// ```
/// let file_path = get_server_pid_file().expect("Oh dear...");
/// ```
#[inline]
pub fn get_server_pid_file() -> Result<String, String> {
    let mut pid_file = PathBuf::from(get_or_create_rd_home()?);
    pid_file.push("server.pid");

    if pid_file.exists() {
        Ok(String::from(pid_file.to_str().unwrap()))
    } else {
        Err(String::from("Failed to retrieve server PID file."))
    }
}

/// Write the new process PID to the PID file.
/// 
/// # Arguments
/// * `pid` - New process ID for the file
/// 
/// # Example
/// ```
/// let file_path = write_server_pid_file(process_id).expect("Oh dear...");
/// ```
#[inline]
pub fn write_server_pid_file(pid: u32) -> Result<String, String> {
    match get_server_pid_file() {

        // Overwrite the current file
        Ok(path) => {
            let mut pid_file = File::with_options().write(true).open(path.clone()).expect("Failed to open PID file to save new ID.");
            pid_file.write_all(format!("{}", pid).as_bytes()).expect("Failed to write data to PID file.");

            match pid_file.sync_data() {
                Ok(()) => Ok(path.clone()),
                Err(err) => Err(err.to_string())
            }
        },

        // Create and write data to the file
        Err(_) => {
            let mut pid_file = PathBuf::from(get_or_create_rd_home()?);
            pid_file.push("server.pid");
            let mut new_pid_file = File::create(pid_file.clone()).expect("Failed to create PID file.");

            new_pid_file.write_all(format!("{}", pid).as_bytes()).expect("Failed to write data to PID file.");
            match new_pid_file.sync_data() {
                Ok(()) => Ok(String::from(pid_file.clone().to_str().unwrap())),
                Err(err) => Err(err.to_string())
            }
        }
    }
}

/// Delete the PID file and return the PID it contains
/// 
/// # Example
/// ```
/// let pid = remove_pid_file().expect("Oh dear...");
/// ```
#[inline]
pub fn remove_pid_file() -> Result<u32, String> {
    match get_server_pid_file() {
        Ok(path) => {
            // First read the file to get the PID then
            // remove the file
            let pid_string = read_to_string::<&String>(&path).expect("Failed ot read PID into memory.");
            let pid = pid_string.trim().parse::<u32>().expect("Failed to parse PID into integer.");

            match remove_file(path) {
                Ok(()) => Ok(pid),
                Err(err) => Err(err.to_string())
            }
        },
        Err(err) => Err(err.to_string())
    }
}

/// Write a HashMap of assumed variables to the env file.
/// 
/// # Examples
/// ```
/// if write_to_env(&variables).is_ok() {
///     // Code here
/// }
/// ```
pub fn write_to_env(vars: &HashMap<String, String>) -> Result<(), String> {
    let mut env_file = File::with_options().write(true).open(get_or_create_env_file()?).unwrap();

    // Iteratively write all data
    vars.iter().for_each(|(key, value)| {
        env_file.write_all(format!("{}={}\n", key, value).as_bytes()).unwrap();
    });

    // Make sure it all gets to disk
    match env_file.sync_data() {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string())
    }
}

/// Loads all environment variables in configuration into the
/// the current process environment.
/// 
/// # Examples
/// ```
/// if set_environment().is_ok() {
///     // Do stuff here
/// }
/// ```
pub fn set_environment() -> Result<(), String> {
    match get_env() {
        Ok(variables) => {

            // Iteratively set each variable
            variables.iter().for_each(|(key, value)| {
                env::set_var(key, value);
            });

            Ok(())
        },
        Err(msg) => Err(msg)
    }
}

/// Run a given environment command from the CLI
/// 
/// # Arguments
/// * `command` - The command to run
/// 
/// # Examples
/// ```
/// run_environment_command(&command_from_cli);
/// ```
#[inline]
pub fn run_environment_command(command: &EnvironmentCLI) {
    match &command.cmd {

        // Get current environment
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

        // Set a new variable
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

        // Remove an environment variable
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
