extern crate websocket;
extern crate argparse;

use std::io;
use std::io::Write;
use std::thread;
use std::time::Duration;

use argparse::{ArgumentParser, Store, StoreTrue, Print};

use websocket::{Client, Message, Sender, Receiver};
use websocket::client::request::Url;

fn main() {

    let mut url = String::new();
    let mut quiet = false;

    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();

        // TODO NO WORK ap.set_description(env!("CARGO_PKG_DESCRIPTION"));
        ap.set_description("The WebSocket Transfer Agent.");

        ap.refer(&mut url)
            .required()
            .add_option(&["-u", "--url"], Store,
                        "URL of the server to connect with");

        ap.refer(&mut quiet)
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

    // Get the URL
    let url = Url::parse(&url).unwrap();

    // Connect to the server
    let request = Client::connect(url).unwrap();

    // Send the request
    let response = request.send().unwrap();

    // Ensure the response is valid
    // TODO Show error if invalid
    response.validate().unwrap();

    // Get a Client
    let client = response.begin();

    // Send message
    let (mut sender, mut receiver) = client.split();

    // Read incoming messages in separate thread
    thread::spawn(move || {
        for message in receiver.incoming_messages() {
            if !quiet {
                print!("< ");
            }

            println!("{}", message_to_string(message.unwrap()));
        }
    });

    // Main loop on stdin
    loop {
        let mut stdin = String::new();

        if !quiet {
            print!("> ");
        }

        io::stdout().flush().unwrap();

        // Will block until a stdin-line is read
        match io::stdin().read_line(&mut stdin) {
            Ok(_) => {

                // If stdin is not empty
                if !stdin.trim().is_empty() {
                    let message = Message::text(stdin.trim());
                    sender.send_message(&message).unwrap();
                }
            },
            Err(error) => println!("error: {}", error)
        }

        // When looping noninteractively, ensure we don't eat the processor
        // Sleep for 0.5 sec
        thread::sleep(Duration::new(0, 500000000));
    }
}

fn message_to_string<'a>(message: Message) -> String {
    let owned = message.payload.into_owned();

    return String::from_utf8(owned).unwrap();
}

