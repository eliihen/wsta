extern crate websocket;
extern crate argparse;
extern crate hyper;

mod program;
mod http;
mod options;

use std::vec::Vec;

use argparse::{ArgumentParser, Store, StoreTrue, Print, Collect, IncrBy};

use options::Options;

/// Parse command line options and invoke program
fn main() {

    // Implement default options
    let mut options = Options {
        url: String::new(),
        login_url: String::new(),
        quiet: false,
        verbosity: 0,
        print_headers: false,
        headers: Vec::new()
    };

    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();

        // TODO NO WORK ap.set_description(env!("CARGO_PKG_DESCRIPTION"));
        ap.set_description("The WebSocket Transfer Agent");

        ap.refer(&mut options.url)
            .required()
            .add_option(&["-u", "--url"], Store,
                        "URL of the server to connect with");

        ap.refer(&mut options.login_url)
            .add_option(&["-l", "--login"], Store,
                        "URL to authenticate with before connecting to WS");

        ap.refer(&mut options.headers)
            .add_option(&["-h", "--header"], Collect,
                        "add headers to any HTTP request made");

        ap.refer(&mut options.print_headers)
            .add_option(&["-I", "--head"], StoreTrue,
                        "print HTTP headers");

        ap.refer(&mut options.quiet)
            .add_option(&["-q", "--quiet"], StoreTrue,
                        "only output incoming frames without any decoration");

        ap.refer(&mut options.verbosity)
            .add_option(&["-v", "--verbose"], IncrBy(1),
                        "(TODO) increase the verbosity level by one");

        ap.add_option(&["-V", "--version"],
                      Print(format!("{} {}",
                                    "wsta",
                                    // TODO NO WORK! env!("CARGO_PKG_NAME"),
                                    env!("CARGO_PKG_VERSION"))
                            ),
                      "print version number and exit");

        ap.parse_args_or_exit();
    }

    program::run_wsta(&mut options);
}

