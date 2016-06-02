use std::io;
use std::io::{Read, Write, ErrorKind};
use std::sync::{Arc, Mutex};
use std::thread;
use std::process::exit;
use std::time::{SystemTime, Duration};

use websocket::{Message, Sender, Receiver};
use websocket::client::Sender as SenderObj;
use websocket::client::Receiver as ReceiverObj;
use websocket::stream::WebSocketStream;
use websocket::result::WebSocketError;

use frame_data::FrameData;

/// Spawn a thread to read stdin. This must be done in a thread because reading
/// io is a blocking action, and thus the thread reading stdin cannot be the
/// same thread as the one sending WebSocket messages.
///
/// State is shared with the thread sending WebSocket messages using lockable
/// shared mutable state - a Mutex.
///
/// Function has a static lifetime so the thread does not outlive the function
/// that owns it.
// TODO Move to ws_writer.rs
pub fn spawn_stdin_reader<A: 'static>(echo: bool, binary_frame_size: Option<usize>)
    -> Arc<Mutex<Vec<FrameData>>> {

    let arc = Arc::new(Mutex::new(Vec::<FrameData>::new()));
    let stdin_buffer = arc.clone();

    thread::spawn(move || {
        log!(3, "stdin reader thread spawned");

        loop {

            if binary_frame_size.is_some() {
                read_as_binary(&stdin_buffer, binary_frame_size);
            } else {
                read_as_utf8(&stdin_buffer, echo);
            }

            // When looping noninteractively, sleep for a little bit to
            // ensure we don't eat the processor
            thread::sleep(Duration::from_millis(50));
        }
    });

    arc
}

/// Read incoming messages in a separate thread and write them to stdout.
/// Function has a static lifetime and ownership of the Receiver is moved
/// to the spawned thread
// TODO Move to ws_reader.rs
pub fn spawn_websocket_reader<A: 'static>(mut receiver: ReceiverObj<WebSocketStream>) {

    thread::spawn(move || {
        log!(3, "WebSocket reader thread spawned");

        for message in receiver.incoming_messages() {
            match message {
                Ok(msg) => message_to_stdout(msg),
                Err(err) => {

                    // Handle the different types of possible errors
                    match err {
                        WebSocketError::NoDataAvailable => {
                            println!("\nDisconnected!");
                            log!(1, "Error: {:?}", err);
                            exit(2);
                        },
                        _ => {
                            log!(1, "Error: {:?}", err);
                            stderr!("Error in WebSocket reader: {}", err);
                            exit(2);
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
                         stdin_buffer: Arc<Mutex<Vec<FrameData>>>) {

    // Lock and read string vector from buffer
    let mut vec = stdin_buffer.lock().unwrap();

    // Use a draining iterator to read and empty buffer
    for line in vec.drain(..) {

        log!(3, "Read: {:?}", line);

        let message = if line.is_utf8() {
            Message::text(format!("{}", line.utf8.unwrap().trim()))
        } else {
            Message::binary(line.binary.unwrap())
        };

        match sender.send_message(&message) {
            Err(err) => {
                log!(1, "Error object: {:?}", err);
                stderr!("An error occured while sending message {:?}: {}",
                        message, err);
                exit(1);
            },
            _ => {}
        };
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
            match sender.send_message(&frame) {
                Err(err) => {
                    log!(1, "Error object: {:?}", err);
                    stderr!("An error occured while sending message {:?}: {}",
                            frame, err);
                    exit(1);
                },
                _ => {}
            };

            return now
        }
    }

    last_time
}

/// Read binary data from stdin in chunks of binary_frame_size
/// and write it to stdin_buffer
fn read_as_binary(stdin_buffer: &Arc<Mutex<Vec<FrameData>>>,
                  binary_frame_size: Option<usize>) {

    // Buffer will hold binary_frame_size bytes or
    // a global default
    // TODO Move to constants.rs
    let mut buf: Vec<u8> = vec![0; binary_frame_size.unwrap_or(255)];

    let stdin = io::stdin();

    // Read stdin until buffer is full
    match stdin.lock().read_exact(buf.as_mut_slice()) {
        Ok(_) => {},
        Err(error) => {
            match error.kind() {
                ErrorKind::UnexpectedEof => log!(1, "Stdin reader: EOF in stdin"),
                _ => {
                    stderr!("Could not read binary frame from stdin: {}", error);
                    log!(1, "Error: {:?}", error);
                }
            }

            return
        }
    };

    log!(4, "Following binary data was read from stdin: {:?}", buf);

    // Convert to FrameData object
    let frame_data = FrameData::from_binary_buffer(buf);

    // Insert into stdin_buffer
    stdin_buffer.lock().unwrap().push(frame_data);
}

/// Read UTF-8 from stdin and write it to stdin_buffer
fn read_as_utf8(stdin_buffer: &Arc<Mutex<Vec<FrameData>>>,
                echo: bool) {

    let mut string_buf = String::new();

    // Will block until a stdin-line is read
    match io::stdin().read_line(&mut string_buf) {
        Ok(_) => {

            // Only send non-empty lines to server
            if !string_buf.trim().is_empty() {

                // Print when ehco is active
                if echo {
                    println!("> {}", string_buf.trim());
                }

                // Lock and place read string into buffer
                log!(3, "Placing message into stdin_buffer: {}", string_buf);
                let frame_data = FrameData::from_utf8(string_buf);
                stdin_buffer.lock().unwrap().push(frame_data);
            }
        },
        Err(error) => {
            match error.kind() {

                // Frame is not UTF-8, warn user and abort
                ErrorKind::InvalidData => {
                    stderr!("InvalidData. Is input not UTF-8? Use UTF-8 or try binary mode (-b)");
                    log!(1, "error: {:?}", error);
                },
                _ => {
                    println!("error: {}", error);
                    log!(1, "Error: {:?}", error);
                }
            }
        }
    }
}

fn message_to_stdout(message: Message) {
    let owned = message.payload.into_owned();

    match String::from_utf8(owned.clone()) {
        Ok(result) => println!("{}", result),
        Err(error) => {

            // Failed to parse as UTF-8, assume it is binary
            log!(1, "Error: {}", error);
            log!(2, "Error: {:?}", error);

            match io::stdout().write(owned.as_ref()) {
                Err(error) => {
                    stderr!("Failed to write message to stdout: {}", error);
                    log!(2, "Error: {:?}", error);
                    exit(1);
                },
                _ => {}
            }
        }
    }
}

