#![crate_name = "wsta"]
#![crate_type = "bin"]
//! `wsta` - The WebSocket Transfer Agent
//!
//! Parses command line [options](Options) and connects to a WebSocket server
//!
//! # Examples
//!
//! Connect to a WebSocket server
//!
//! ```bash
//! wsta wss://echo.websocket.org
//! ```
//!
//! # Exit codes
//! | Code | Reason                                      |
//! |------|---------------------------------------------|
//! | 1    | Irrecoverable error during normal operation |
//! | 2    | Websocket stream was closed unexpectedly    |

extern crate websocket;
extern crate argparse;
extern crate hyper;
extern crate cookie;
extern crate config;
extern crate xdg;

// Needs to be imported first because of log! macro
#[macro_use]
mod log;
mod conf;
mod args;
mod frame_data;
mod program;
mod http;
mod ws;
mod options;

use argparse::*;
use std::io;
use std::io::Write;
use std::process::exit;
use std::str::from_utf8;

use args::get_profile;
use options::Options;

/// The main entry point of the app.
/// Parses command line options and starts the main program.
///
/// The main method first tries to load a config file as the
/// default runtime options, then falls back to Options::new().
/// In any case, the user can override the defaults using the
/// CLI arguments.
fn main() {

    // Read the provided configuration profile from args, if any
    let profile = get_profile();

    // Prepare log of profile name until we have parsed verbosity
    let parsed_profile_log = format!("Parsed profile as: {:?}", &profile);

    // Read config file
    let config = conf::read_conf_file(profile);

    // Prepare log of conf until we have parsed verbosity
    let parsed_conf_log = format!("Parsed config file as: {:?}", &config);

    // Get default options from config if config exists,
    // else use global defaults
    let mut options = match config {
        Some(conf) => Options::build_from_config(&conf),
        None => Options::new()
    };

    let mut help_text = Vec::<u8>::new();

    {  // this block limits scope of borrows by ap.refer() method
        let mut dummy = String::new();
        let mut ap = ArgumentParser::new();

        ap.set_description(env!("CARGO_PKG_DESCRIPTION"));

        // Required, but we don't use ArgumentParser's required, as it
        // can be provided from a configuration file
        ap.refer(&mut options.url)
            .add_argument("url", Store,
                        "URL of the server to connect with");

        ap.refer(&mut options.headers)
            .add_option(&["-H", "--header"], Collect,
                        "add headers to any HTTP request made");

        ap.refer(&mut options.print_headers)
            .add_option(&["-I", "--head"], StoreTrue,
                        "print HTTP headers");

        ap.refer(&mut options.ping_interval)
            .metavar("SECONDS")
            .add_option(&["-p", "--ping"], StoreOption,
                        "specify an interval to send `ping` to the server");

        ap.refer(&mut options.login_url)
            .add_option(&["-l", "--login"], Store,
                        "URL to authenticate with before connecting to WS");

        ap.refer(&mut options.binary_mode)
            .add_option(&["-b", "--binary"], StoreTrue,
                        "enable binary mode");

        ap.refer(&mut options.follow_redirect)
            .add_option(&["--follow-redirect"], StoreTrue,
                        "honour HTTP redirection when authenticating");

        ap.refer(&mut options.echo)
            .add_option(&["-e", "--echo"], StoreTrue,
                        "echo outgoing frames");

        // This is a dummy entry used in --help - the actual profile is read
        // before ArgumentParser is invoked
        ap.refer(&mut dummy)
            .metavar("NAME")
            .add_option(&["-p"], Store,
                        "use a different configuration profile");

        ap.refer(&mut options.verbosity)
            .add_option(&["-v", "--verbose"], IncrBy(1),
                        "increase the verbosity level by one");

        ap.add_option(&["-V", "--version"],
                      Print(format!("{} {}",
                                    env!("CARGO_PKG_NAME"),
                                    env!("CARGO_PKG_VERSION"))
                            ),
                      "print version number and exit");

        ap.refer(&mut options.messages)
            .add_argument("messages", Collect,
                          r#"message(s) to send after connecting"#);

        ap.print_help(env!("CARGO_PKG_NAME"), &mut help_text)
            .expect("Could not write help text to buffer! File a bug!");

        ap.parse_args_or_exit();
    }

    // Check if url is empty manually, as the user may enter
    // it either as an argument or via a configuration file
    if options.url.is_empty() {
        stderr!("{}: You need to enter a URL", env!("CARGO_PKG_NAME"));

        let help_text = from_utf8(&help_text[..])
            .expect("Could not read help text from buffer! File a bug!")
            .to_string();
        stderr!("{}", help_text);
        exit(1);
    }


    // Set log level, no logging before this is possible
    log::set_log_level(options.verbosity);
    log!(3, parsed_profile_log);
    log!(3, parsed_conf_log);
    log!(3, "Resulting options: {:?}", options);

    program::run_wsta(&mut options);
}
