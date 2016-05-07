use std::io;
use std::io::Write;
use std::thread;
use std::process::exit;
use std::time::Duration;

use websocket::{Client, Message, Sender, Receiver};
use websocket::client::Sender as SenderObj;
use websocket::client::request::{Request, Url};
use websocket::stream::WebSocketStream;
use websocket::result::WebSocketError;

use log;
use options::Options;
use http::{fetch_session_cookie, print_headers};

pub fn run_wsta(options: &mut Options) {

    // Get the URL
    log!(2, "About to unwrap: {}", options.url);
    let url = Url::parse(&options.url).unwrap();
    log!(3, "Parsed URL: {}", url);

    // Connect to the server
    let mut request;
    match Client::connect(url) {
        Ok(res) => request = res,
        Err(err) => {
            log!(1, "Error object: {:?}", err);
            panic!("An error occured while connecting to {}: {}",
                           options.url, err);
        }
    }

    // Authenticate if requested
    if !options.login_url.is_empty() {
        let session_cookie = fetch_session_cookie(options);
        log!(2, "Got session cookie: {:?}", session_cookie);

        if session_cookie.is_some() {
            request.headers.set(session_cookie.unwrap());
            log!(3, "Session cookie set on request. Headers are now: {:?}",
                 request.headers);
        } else {
            panic!(concat!("Attempted to fetch session cookie, but no ",
              ".*session.* cookie was found in response. Inspect -I"));
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
    log!(3, "About to send and unwrap request");
    let response = request.send().unwrap();
    log!(3, "Request sent");

    // Dump headers when requested
    if options.print_headers {
        print_headers("WebSocket upgrade response",
                      &response.headers, Some(response.status));
    }

    // Ensure the response is valid and show an error if not
    match response.validate() {
        Err(error) => {
            log!(1, "Invalid reponse: {:?}", error);
            stderr!("{}", error);

            if !options.print_headers {
                stderr!("Try -I for more info");
            }

            exit(1);
        },
        _ => stderr!("Connected to {}", options.url)
    }

    // Get a Client
    let client = response.begin();
    log!(3, "Client created");

    // Send message
    let (mut sender, mut receiver) = client.split();

    // Send pre-provided messages if preesnt
    if !options.messages.is_empty() {
        send_messages(&mut sender, &mut options.messages, options.echo);
    }

    // Read incoming messages in separate thread
    thread::spawn(move || {
        log!(3, "Reader thread spawned");

        for message in receiver.incoming_messages() {
            match message {
                Ok(msg) => {
                    println!("{}", message_to_string(msg));
                },
                Err(err) => {

                    // Handle the different types of possible errors
                    match err {
                        WebSocketError::NoDataAvailable => {
                            println!("\nDisconnected!");
                            log!(1, "Error: {:?}", err);
                            exit(0);
                        },
                        _ => {
                            log!(1, "Error: {:?}", err);
                            panic!("Error in WebSocket reader: {}", err);
                        }
                    }
                }
            }
        }
    });

    // Main loop on stdin
    log!(3, "Entering main loop");
    loop {
        let mut stdin = String::new();

        // Will block until a stdin-line is read
        match io::stdin().read_line(&mut stdin) {
            Ok(_) => {

                // Only send non-empty lines to server
                if !stdin.trim().is_empty() {

                    // Print when ehco is active
                    if options.echo {
                        print!("> {}", stdin.trim());
                    }

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

    log!(2, "Adding headers to request: {:?}", headers);
    for header in headers {

        // Only process the header if it is a valid "key: value" header
        if header.contains(':') {

            // Split by first colon into [key, value]
            let split = header.splitn(2, ':').collect::<Vec<&str>>();
            log!(3, "Split header: {:?}", split);

            let key = split[0];
            log!(3, "Key is: {}", key);

            let val = split[1].to_string().into_bytes();
            log!(3, "Val is: {:?} (bytes)", val);

            // Write raw (untyped) header
            request.headers.set_raw(format!("{}", key), vec![val]);
            log!(2, "Wrote new header. Headers are now: {:?}", request.headers);
        } else {
            stderr!("Invalid header: {}. Must contain a colon (:)", header);
        }
    }
}

fn send_messages(sender: &mut SenderObj<WebSocketStream>,
                 messages: &mut Vec<String>,
                 echo: bool) {

    for message in messages {
        if echo {
            println!("> {}", message);
        }

        let frame = Message::text(message.as_str());
        sender.send_message(&frame).unwrap();
    }
}

fn message_to_string<'a>(message: Message) -> String {
    let owned = message.payload.into_owned();

    return String::from_utf8(owned).unwrap();
}

