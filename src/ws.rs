use std::io;
use std::sync::{Arc, Mutex};
use std::thread;
use std::process::exit;
use std::time::{SystemTime, Duration};

use websocket::{Message, Sender, Receiver};
use websocket::client::Sender as SenderObj;
use websocket::client::Receiver as ReceiverObj;
use websocket::stream::WebSocketStream;
use websocket::result::WebSocketError;

use log;

/// Spawn a thread to read stdin. This must be done in a thread because reading
/// io is a blocking action, and thus the thread reading stdin cannot be the
/// same thread as the one sending WebSocket messages.
///
/// State is shared with the thread sending WebSocket messages using lockable
/// shared mutable state - a Mutex.
///
/// Function has a static lifetime so the thread does not outlive the function
/// that owns it.
pub fn spawn_stdin_reader<A: 'static>(echo: bool) -> Arc<Mutex<Vec<String>>> {

    let arc = Arc::new(Mutex::new(Vec::<String>::new()));
    let stdin_buffer = arc.clone();

    thread::spawn(move || {
        log!(3, "stdin reader thread spawned");

        loop {
            let mut stdin = String::new();

            // Will block until a stdin-line is read
            match io::stdin().read_line(&mut stdin) {
                Ok(_) => {

                    // Only send non-empty lines to server
                    if !stdin.trim().is_empty() {

                        // Print when ehco is active
                        if echo {
                            println!("> {}", stdin.trim());
                        }

                        // Lock and place read string into buffer
                        log!(3, "Placing message into stdin_buffer: {}", stdin);
                        stdin_buffer.lock().unwrap().push(stdin);
                    }
                },
                Err(error) => println!("error: {}", error)
            }

            // When looping noninteractively, sleep for a little bit to
            // ensure we don't eat the processor
            thread::sleep(Duration::new(0, 500000000));
        }
    });

    arc
}

/// Read incoming messages in a separate thread and write them to stdout.
/// Function has a static lifetime and ownership of the Receiver is moved
/// to the spawned thread
pub fn spawn_websocket_reader<A: 'static>(mut receiver: ReceiverObj<WebSocketStream>) {

    thread::spawn(move || {
        log!(3, "WebSocket reader thread spawned");

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
}

/// Reads the `stdin_buffer` and sends the message using the provided
/// `Sender` if any messages are found. It then flushes the buffer.
pub fn read_stdin_buffer(sender: &mut SenderObj<WebSocketStream>,
                     stdin_buffer: Arc<Mutex<Vec<String>>>) {

    // Lock and read string vector from buffer
    let mut vec = stdin_buffer.lock().unwrap();

    // Use a draining iterator to read and empty buffer
    for line in vec.drain(..) {
        log!(3, "Read: {}", line);
        let message = Message::text(line.trim());
        sender.send_message(&message).unwrap();
    }
}

/// Check if the provided interval has passed.
/// If it has passed, send a "ping" frame to the provided sender.
///
/// # Returns
/// Returns last_time if interval has not passed.
/// If the interval has passed, check_ping_interval returns SystemTime::now()
pub fn check_ping_interval(ping_interval: &Option<Duration>,
                           last_time: SystemTime,
                           sender: &mut SenderObj<WebSocketStream>,
                           echo: bool) -> SystemTime {

    if ping_interval.is_some() {
        let now = SystemTime::now();
        let time_passed = now.duration_since(last_time).unwrap().as_secs();

        if time_passed >= ping_interval.unwrap().as_secs() {
            if echo {
                println!("> ping");
            }

            let frame = Message::text("ping");
            sender.send_message(&frame).unwrap();

            return now
        }
    }

    last_time
}

fn message_to_string<'a>(message: Message) -> String {
    let owned = message.payload.into_owned();

    return String::from_utf8(owned).unwrap();
}

