use std::io;
use std::io::Write;
use std::thread;
use std::process::exit;
use std::time::Duration;

use websocket::{Client, Message, Sender, Receiver};
use websocket::client::request::{Request, Url};
use websocket::stream::WebSocketStream;

use options::Options;
use http::{fetch_session_cookie, print_headers};

pub fn run_wsta(options: &mut Options) {

    // Get the URL
    let url = Url::parse(&options.url).unwrap();

    // Connect to the server
    let mut request = Client::connect(url).unwrap();

    // Authenticate if requested
    if !options.login_url.is_empty() {
        let session_cookie = fetch_session_cookie(options);

        if session_cookie.is_some() {
            request.headers.set(session_cookie.unwrap());
        }
    }

    // Add the headers passed from command line arguments
    if !options.headers.is_empty() {
        add_headers_to_request(&mut request, &mut options.headers);
    }

    // Print request
    if options.print_headers {
        print_headers("WebSocket upgrade request", &request.headers, None);
    }

    // Send the request
    let response = request.send().unwrap();

    // Dump headers when requested
    if options.print_headers {
        print_headers("WebSocket upgrade response",
                      &response.headers, Some(response.status));
    }

    // Ensure the response is valid and show an error if not
    match response.validate() {
        Err(error) => {
            write!(io::stderr(), "{}\n", error).unwrap();

            if !options.print_headers {
                write!(io::stderr(), "Try -I for more info\n").unwrap();
            }

            exit(1);
        },
        _ => {}
    }

    // Get a Client
    let client = response.begin();

    // Send message
    let (mut sender, mut receiver) = client.split();

    // Clone so thread can own this instance
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

                // Only send non-empty lines to server
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

fn add_headers_to_request(request: &mut Request<WebSocketStream, WebSocketStream>,
                          headers: &mut Vec<String>) {
    for header in headers {

        // Only process the header if it is a valid "key: value" header
        if header.contains(':') {

            // Split by first colon into [key, value]
            let split = header.splitn(2, ':').collect::<Vec<&str>>();
            let key = split[0];
            let val = split[1].to_string().into_bytes();

            // Write raw (untyped) header
            request.headers.set_raw(format!("{}", key), vec![val]);
        } else {
            write!(io::stderr(),
            "Invalid header: {}. Must contain a colon (:)\n", header).unwrap();
        }
    }
}

fn message_to_string<'a>(message: Message) -> String {
    let owned = message.payload.into_owned();

    return String::from_utf8(owned).unwrap();
}

