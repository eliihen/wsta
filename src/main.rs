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

// Needs to be imported first because of log! macro
#[macro_use]
mod log;
mod conf;
mod frame_data;
mod program;
mod http;
mod ws;
mod options;

use argparse::*;
use std::io;
use std::io::Write;

use options::Options;

/// The main entry point of the app.
/// Parses command line options and starts the main program
fn main() {

    // Fetch XDG_CONFIG_HOME from env
    let xdg_home = conf::read_xdg_home();

    // Read config file
    let config = conf::read_conf_file(&xdg_home);

    // Get default options from config if config exists,
    // else use global defaults
    let mut options = match config {
        Some(conf) => Options::build_from_config(&conf),
        None => Options::new()
    };


    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();

        ap.set_description(env!("CARGO_PKG_DESCRIPTION"));

        ap.refer(&mut options.url)
            // TODO When !url, Print(help)
            .required()
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

        ap.parse_args_or_exit();
    }


    // Set log level, no logging before this is possible
    log::set_log_level(options.verbosity);
    //log!(3, "Parsed config file: {:?}", config);
    log!(3, "Parsed options: {:?}", options);

    program::run_wsta(&mut options);
}
