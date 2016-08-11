use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use std::option::Option;

use config::reader::from_file;
use config::types::{ScalarValue,Value,Config};
use config::error::ConfigErrorKind::{IoError,ParseError};

#[cfg(unix)] use xdg::BaseDirectories;

#[cfg(test)] use std::fs;
#[cfg(test)] use std::fs::{File,DirBuilder};
#[cfg(test)] use std::env;
#[cfg(test)] use std::path::Path;
#[cfg(test)] use std::thread::sleep;
#[cfg(test)] use std::time::Duration;

/// Reads the configuration file and copies the values into a temporary
/// options object. The object is the overridden with the parameters
/// specified on the command line.
pub fn read_conf_file(profile: Option<String>) -> Option<Config> {

    // Lookup config_folder based on OS
    let conf_path = get_config_path(profile);

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

/// Utility method for fetching config as String
pub fn get_str(config: &Config, key: &str) -> String {
   config.lookup_str_or(key, "").to_string()
}

/// Utility method for fetching config as String
pub fn get_str_or(config: &Config, key: &str, default: &str) -> String {
   config.lookup_str_or(key, default).to_string()
}

/// Utility method for fetching config as boolean
pub fn get_bool(config: &Config, key: &str) -> bool {
   config.lookup_boolean_or(key, false)
}

/// Utility method for fetching config as Vec
pub fn get_vec(config: &Config, key: &str) -> Vec<String> {

    let mut result = Vec::new();

    // CALLBACK HELL, TURN BACK NOW
    // Lookup key in config
    match config.lookup(key) {
        Some(value) => {

            // Lookup array in value in config
            match value {
                &Value::Array(ref array) => {

                    // Iterate over values in array
                    for svalue in array {

                        // Lookup any strings in the array
                        match svalue {
                             &Value::Svalue(ScalarValue::Str(ref header)) => {
                                result.push(format!("{}", header));
                            },
                            val => stderr!(concat!("CONFIG ERROR: A key in ",
                              "the {} array is invalid: {:?}"), key, val)
                        }
                    }
                },
                val => stderr!(concat!("CONFIG ERROR: A key in the {} array ",
                  "is invalid: {:?}"), key, val)
            };
        },

        // Non-present keys are OK
        _ => {}
    };

    result
}

/// Determine the conf file location using the special %APPDATA% directory of
/// windows.
///
/// Supports profiles, i.e. `-p someapp` resolves to
/// %APPDATA%/wsta/someapp/wsta.conf
#[cfg(windows)]
fn get_config_path(profile: Option<String>) -> Option<PathBuf> {

    let mut path = PathBuf::from("%APPDATA%\\wsta");

    if profile.is_some() {
        path.push(profile.unwrap());
    }

    path.push("wsta.conf");

    Some(path)
}

/// Determine the conf file location using the XDG basedir spec, which defaults
/// to $HOME/.config/wsta/wsta.conf
///
/// Supports profiles, i.e. `-p someapp` resolves to
/// XDG_CONF_DIR/wsta/someapp/wsta.conf
#[cfg(unix)]
fn get_config_path(profile: Option<String>) -> Option<PathBuf> {

    let xdg_dirs_option = match profile {
        Some(p) => BaseDirectories::with_profile(env!("CARGO_PKG_NAME"), p),
        None => BaseDirectories::with_prefix(env!("CARGO_PKG_NAME"))
    };

    let xdg_dirs = match xdg_dirs_option {
        Ok(val) => val,
        Err(_) => {
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
    create_dummy_conf(None);

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
    create_dummy_conf(None);

    let conf = read_conf_file(None).expect("Could not read config file");

    // Assert is parsed properly
    let result: &str = conf.lookup_str("login_cookie_name")
      .expect("Could not find login_cookie_name in the config file");
    assert_eq!(result, "testing");

    replace_backup();
}

#[test]
fn get_vec_works() {
    // Workaround for tests failing when they are run at the same time
    sleep(Duration::from_millis(2000));

    backup_user_config();
    create_dummy_conf(None);

    let conf = read_conf_file(None).expect("Could not read config file");

    // Assert is parsed properly
    let key_name = "headers";
    let result = get_vec(&conf, &key_name);
    assert_eq!(result, vec!["Foo:Bar"]);

    let key_name = "ricepudding";
    let result = get_vec(&conf, &key_name);
    assert_eq!(result, Vec::<String>::new());

    replace_backup();
}

#[test]
#[cfg(unix)]
fn config_profile_works() {
    // Workaround for tests failing when they are run at the same time
    sleep(Duration::from_millis(3000));

    let profile_name = String::from("wsta_test_profile");

    let conf_file = format!("{}/.config/wsta/{}/wsta.conf",
        env::home_dir()
          .expect("Could not fetch home dir for test!")
          .display(),
        &profile_name
    );

    create_dummy_conf(Some(conf_file));

    let conf = read_conf_file(Some(profile_name))
        .expect("Could not read config file");

    // Assert is parsed properly
    let result: &str = conf.lookup_str("login_cookie_name")
      .expect("Could not find login_cookie_name in the config file");
    assert_eq!(result, "testing");

    let key_name = "headers";
    let result = get_vec(&conf, &key_name);
    assert_eq!(result, vec!["Foo:Bar"]);

    let key_name = "ricepudding";
    let result = get_vec(&conf, &key_name);
    assert_eq!(result, Vec::<String>::new());

    replace_backup();
}

#[test]
#[cfg(windows)]
fn config_profile_works() {
    // Workaround for tests failing when they are run at the same time
    sleep(Duration::from_millis(3000));

    let profile_name = String::from("wsta_test_profile");

    let conf_file = format!("%APPDATA%\\wsta\\{}\\wsta.conf", &profile_name);

    create_dummy_conf(Some(conf_file));

    let conf = read_conf_file(Some(profile_name))
        .expect("Could not read config file");

    // Assert is parsed properly
    let result: &str = conf.lookup_str("login_cookie_name")
      .expect("Could not find login_cookie_name in the config file");
    assert_eq!(result, "testing");

    let key_name = "headers";
    let result = get_vec(&conf, &key_name);
    assert_eq!(result, vec!["Foo:Bar"]);

    let key_name = "ricepudding";
    let result = get_vec(&conf, &key_name);
    assert_eq!(result, Vec::<String>::new());

    replace_backup();
}

#[cfg(test)]
fn create_dummy_conf(conf_file_override: Option<String>) {

    let conf_file = if conf_file_override.is_some() {
        PathBuf::from(conf_file_override.unwrap())
    } else {
        get_test_conf_path()
    };

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
    f.write_all(b"login_cookie_name = \"testing\";\nheaders=[\"Foo:Bar\"];")
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
