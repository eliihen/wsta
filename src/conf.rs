use std::io;
use std::io::Write;
use std::env;
use std::path::Path;
use std::process::exit;

use config::reader::from_file;
use config::types::Config;
use config::error::ConfigErrorKind::{IoError,ParseError};
use options::Options;

/// Fetch XDG_CONFIG_HOME from env
pub fn read_xdg_home() -> Option<String> {
    match env::var("XDG_CONFIG_HOME") {
        Ok(val) => Some(val),
        Err(_) => None
    }
}

/// Reads the configuration file and copies the values into a temporary
/// options object. The object is the overridden with the parameters
/// specified on the command line.
pub fn read_conf_file(xdg_home: &Option<String>) -> Option<Config> {

    // Lookup config_folder based on OS
    let config_folder = if cfg!(windows) {
        String::from("%APPDATA%")
    } else {
        xdg_home.clone().unwrap_or(String::from("~"))
    };

    // Assume system is $XDG_CONFIG_HOME compliant
    let raw_path = format!("{}/wsta/wsta.conf", config_folder);
    let path = Path::new(&raw_path);

    // Read the configuration path and handle errors
    match from_file(&path) {
        Ok(config) => Some(config),
        Err(config_err) => {
            match config_err.kind {

                // User has not make config file, which is fine
                IoError => None,

                // User has a config file, but it has syntax errors
                ParseError => {
                    stderr!("ERROR: Failed to parse wsta config file: {}",
                            config_err);
                    exit(1);
                }
            }
        }
    }
}

