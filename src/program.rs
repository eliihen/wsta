use std::io;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use std::process::exit;
use std::time::{SystemTime, Duration};

use websocket::{Client, Message, Sender};
use websocket::client::Sender as SenderObj;
use websocket::client::Receiver as ReceiverObj;
use websocket::client::request::{Request, Url};
use websocket::stream::WebSocketStream;

use log;
use ws;
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
    let (mut sender, receiver) = client.split();

    // Send pre-provided messages if preesnt
    if !options.messages.is_empty() {
        send_messages(&mut sender, &mut options.messages, options.echo);
    }

    ws::spawn_websocket_reader::<ReceiverObj<WebSocketStream>>(receiver);

    // Share mutable data between writer thread and main thread
    // using a lockable Mutex.
    // Mutex will block threads waiting for the lock to become available
    let stdin_buffer = ws::spawn_stdin_reader::<Arc<Mutex<Vec<String>>>>(options.echo);

    // Variables for checking against a ping interval
    let ping_interval = options.ping_interval.map(|i| Duration::from_secs(i));
    let mut last_time = SystemTime::now();

    log!(3, "Entering main loop");
    loop {

        // Read buffer, and send message to server if buffer contains anything
        ws::read_stdin_buffer(&mut sender, stdin_buffer.clone());

        // Check if ping_interval has passed, if so, send a ping frame
        last_time = ws::check_ping_interval(&ping_interval, last_time,
                                            &mut sender, options.echo);

        // Sleep for a second at a time, as this is the smallest possible
        // ping_interval that can be input
        thread::sleep(Duration::from_secs(1));
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

