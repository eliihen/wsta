use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;

use config::reader::from_file;
use config::types::Config;
use config::error::ConfigErrorKind::{IoError,ParseError};

#[cfg(unix)]
use xdg::BaseDirectories;

#[cfg(test)]
use std::fs;
#[cfg(test)]
use std::fs::{File,DirBuilder};
#[cfg(test)]
use std::env;
#[cfg(test)]
use std::path::Path;
#[cfg(test)]
use std::thread::sleep;
#[cfg(test)]
use std::time::Duration;

/// Reads the configuration file and copies the values into a temporary
/// options object. The object is the overridden with the parameters
/// specified on the command line.
pub fn read_conf_file() -> Option<Config> {

    // Lookup config_folder based on OS
    let conf_path = get_config_path(None);

    if conf_path.is_none() {
        return None;
    }

    // Read the configuration path and handle errors
    match from_file(&conf_path.unwrap()) {
        Ok(config) => Some(config),
        Err(config_err) => {
            match config_err.kind {

                // User has not made a config file, which is fine
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

#[cfg(windows)]
fn get_config_path(profile: Option<String>) -> Option<PathBuf> {
    // TODO Support profiles on windows
    Some(PathBuf::from("%APPDATA%\\wsta\\wsta.conf"))
}

/// Determine the conf file location using the XDG basedir spec, which defaults
/// to $HOME/.config/wsta/wsta.conf
#[cfg(unix)]
fn get_config_path(profile: Option<String>) -> Option<PathBuf> {

    // TODO Support profiles
    let xdg_dirs_option = match profile {
        Some(_) => unimplemented!(),
        None => BaseDirectories::with_prefix(env!("CARGO_PKG_NAME"))
    };

    let xdg_dirs = match xdg_dirs_option {
        Ok(val) => val,
        Err(_) => {
            println!("Not present");

            log!(3, "XDG application data not present");
            return None
        }
    };

    xdg_dirs.find_config_file("wsta.conf")
}



///////////////////////// TESTS ///////////////////////////
// Tests need to support both windows and unix, and will //
// run CI in both these systems                          //

#[test]
fn config_path_is_read() {
    backup_user_config();
    create_dummy_conf();

    let conf_file = get_config_path(None)
        .expect("Could not find a config file");

    assert!(conf_file.is_file());

    // Assert it is in the correct directory
    assert_eq!(conf_file, get_test_conf_path());

    replace_backup();
}

#[test]
fn configfile_is_read() {

    // Workaround for tests failing when they are run at the same time
    sleep(Duration::from_millis(1000));

    backup_user_config();
    create_dummy_conf();

    let conf = read_conf_file().expect("Could not read config file");

    // Assert is parsed properly
    let result: &str = conf.lookup_str("login_cookie_name")
      .expect("Could not find login_cookie_name in the config file");
    assert_eq!(result, "testing");

    replace_backup();
}

#[cfg(test)]
fn create_dummy_conf() {

    let conf_file = get_test_conf_path();
    let directory = conf_file.clone();
    let directory = directory.parent().unwrap();

    println!("Does config dir {} exist? {}",
             directory.display(),
             directory.exists());

    if !directory.exists() {
        println!("Creating directory {}", directory.display());
        DirBuilder::new()
            .recursive(true)
            .create(directory)
            .expect("Could not create config directory for testing");
    }

    // Create dummy conf
    let mut f = File::create(&conf_file)
        .expect("Could not create dummy conf file");
    f.write_all(b"login_cookie_name = \"testing\";")
        .expect("Could not write dummy config file");
    f.sync_all().expect("Could not sync writes to system");

    println!("Created dummy file {} ", conf_file.display());
}

#[cfg(test)]
fn backup_user_config() {
    let conf_file = get_config_path(None);

    if let Some(conf_file) = conf_file {

        // Backup if file exists
        let backup_file = format!("{}.bak", conf_file.display());
        let backup_file = Path::new(&backup_file);

        println!("Moving {} to {}", conf_file.display(), backup_file.display());
        if conf_file.exists() && !backup_file.exists() {
            fs::rename(&conf_file, &backup_file)
                .expect("Failed to backup config file");

        }
    }
}

#[cfg(test)]
fn replace_backup() {
    let conf_file = get_config_path(None);

    if let Some(conf_file) = conf_file {

        // Move back if exists
        let backup_file = format!("{}.bak", conf_file.display());
        let backup_file = Path::new(&backup_file);

        println!("Moving {} to {}", backup_file.display(), conf_file.display());
        if conf_file.exists() && backup_file.exists() {
            fs::rename(&backup_file, &conf_file)
                .expect("Failed to place backup file back");
        }
    }
}

/// Return a mock path that makes sense for the current OS
#[cfg(test)]
fn get_test_conf_path() -> PathBuf {

    if cfg!(windows) {
        PathBuf::from("%APPDATA%\\wsta\\wsta.conf")
    } else {
        PathBuf::from(format!("{}/.config/wsta/wsta.conf",
            env::home_dir()
              .expect("Could not fetch home dir for test!")
              .display()
        ))
    }
}
