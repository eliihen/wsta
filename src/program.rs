use std::io;
use std::io::Write;
use std::thread;
use std::time::Duration;

use websocket::{Client, Message, Sender, Receiver};
use websocket::client::request::Url;

use options::Options;

pub fn run_wsta(options: &mut Options) {

    // Get the URL
    let url = Url::parse(&options.url).unwrap();

    // Connect to the server
    let request = Client::connect(url).unwrap();

    // Send the request
    let response = request.send().unwrap();

    // Dump headers when requested
    if options.print_headers {
        println!("WebSocket upgrade request");
        println!("---");
        println!("{}", response.status);
        println!("{}", response.headers);
    }

    // Ensure the response is valid
    // TODO Show error if invalid
    response.validate().unwrap();

    // Get a Client
    let client = response.begin();

    // Send message
    let (mut sender, mut receiver) = client.split();

    let quiet = options.quiet.clone();

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

        // Create prompt when interactive
        if !options.quiet {
            print!("> ");
            io::stdout().flush().unwrap();
        }

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

