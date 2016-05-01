extern crate websocket;
extern crate argparse;

mod program;
pub mod options;

use argparse::{ArgumentParser, Store, StoreTrue, Print};

use options::Options;

/// Parse command line options and invoke the next method
fn main() {

    let mut options = Options {
        url: String::from(""),
        quiet: false,
    };

    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();

        // TODO NO WORK ap.set_description(env!("CARGO_PKG_DESCRIPTION"));
        ap.set_description("The WebSocket Transfer Agent.");

        ap.refer(&mut options.url)
            .required()
            .add_option(&["-u", "--url"], Store,
                        "URL of the server to connect with");

        ap.refer(&mut options.quiet)
            .add_option(&["-q", "--quiet"], StoreTrue,
                        "only output incoming frames without any decoration");

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

