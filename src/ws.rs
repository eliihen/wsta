use std::io;
use std::io::{ErrorKind, Read, Write};
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

use websocket::client::Receiver as ReceiverObj;
use websocket::client::Sender as SenderObj;
use websocket::result::WebSocketError;
use websocket::stream::WebSocketStream;
use websocket::{Message, Receiver, Sender};

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
pub fn spawn_stdin_reader<A: 'static>(
    echo: bool,
    binary_mode: bool,
    frame_size: String,
) -> Arc<Mutex<Vec<FrameData>>> {
    let arc = Arc::new(Mutex::new(Vec::<FrameData>::new()));
    let stdin_buffer = arc.clone();

    thread::spawn(move || {
        log!(3, "stdin reader thread spawned");

        loop {
            if binary_mode {
                read_as_binary(&stdin_buffer, frame_size.clone());
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
                            stderr!("\nDisconnected!");
                            log!(1, "Error: {:?}", err);
                            exit(2);
                        }
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
pub fn read_stdin_buffer(
    sender: &mut SenderObj<WebSocketStream>,
    stdin_buffer: Arc<Mutex<Vec<FrameData>>>,
) {
    // Lock and read string vector from buffer
    let mut vec = stdin_buffer.lock().unwrap();

    // Use a draining iterator to read and empty buffer
    for line in vec.drain(..) {
        log!(4, "Read: {:?}", line);

        let message = if line.is_utf8() {
            Message::text(format!("{}", line.utf8.unwrap().trim()))
        } else {
            Message::binary(line.binary.unwrap())
        };

        match sender.send_message(&message) {
            Err(err) => {
                log!(1, "Error object: {:?}", err);
                stderr!(
                    "An error occured while sending message {:?}: {}",
                    message,
                    err
                );
                exit(1);
            }
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
pub fn check_ping_interval(
    ping_interval: &Option<Duration>,
    last_time: SystemTime,
    sender: &mut SenderObj<WebSocketStream>,
    echo: bool,
    ping_msg: &String,
) -> SystemTime {
    if ping_interval.is_some() {
        let now = SystemTime::now();
        let time_passed = now.duration_since(last_time).unwrap().as_secs();

        if time_passed >= ping_interval.unwrap().as_secs() {
            if echo {
                println!("> {}", ping_msg);
            }

            let frame = Message::ping(format!("{}", ping_msg).into_bytes());
            match sender.send_message(&frame) {
                Err(err) => {
                    log!(1, "Error object: {:?}", err);
                    stderr!(
                        "An error occured while sending message {:?}: {}",
                        frame,
                        err
                    );
                    exit(1);
                }
                _ => {}
            };

            return now;
        }
    }

    last_time
}

/// Read binary data from stdin in chunks of binary_mode
/// and write it to stdin_buffer
fn read_as_binary(stdin_buffer: &Arc<Mutex<Vec<FrameData>>>, frame_size_str: String) {
    // Parse WSTA_BINARY_FRAME_SIZE environment variable
    // as the size of the binary buffer, or use a global
    // default if not present
    let frame_size = match frame_size_str.parse() {
        Ok(result) => result,
        Err(error) => {
            stderr!("Error! WSTA_BINARY_FRAME_SIZE must be a number: {}", error);
            log!(1, "Error: {:?}", error);
            exit(1);
        }
    };

    let mut buf: Vec<u8> = vec![0; frame_size];
    let stdin = io::stdin();

    // Read stdin until buffer is full
    let read_bytes = match stdin.lock().read(buf.as_mut_slice()) {
        Ok(read) => read,
        Err(error) => {
            match error.kind() {
                ErrorKind::UnexpectedEof => log!(1, "Stdin reader: EOF in stdin"),
                _ => {
                    stderr!("Could not read binary frame from stdin: {}", error);
                    log!(1, "Error: {:?}", error);
                }
            }

            // Read no bytes, return 0
            0
        }
    };

    if read_bytes == 0 {
        log!(3, "No bytes were read");
        return;
    }

    log!(3, "Read {} bytes of binary data", read_bytes);

    // If we read less than the buffer size, we need
    // to shrink it. This is so we avoid sending extra
    // zeroes in the frames.
    if read_bytes < frame_size {
        buf.resize(read_bytes, 0);
        log!(3, "Resized buffer");
    }

    if read_bytes > 0 {
        log!(4, "Following binary data was read from stdin: {:?}", buf);

        // Convert to FrameData object
        let frame_data = FrameData::from_binary_buffer(buf);

        // Insert into stdin_buffer
        stdin_buffer.lock().unwrap().push(frame_data);
    }
}

/// Read UTF-8 from stdin and write it to stdin_buffer
fn read_as_utf8(stdin_buffer: &Arc<Mutex<Vec<FrameData>>>, echo: bool) {
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
        }
        Err(error) => {
            match error.kind() {
                // Frame is not UTF-8, warn user and abort
                ErrorKind::InvalidData => {
                    stderr!("InvalidData. Is input not UTF-8? Use UTF-8 or try binary mode (-b)");
                    log!(1, "error: {:?}", error);
                }
                _ => {
                    stderr!("error: {}", error);
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
            log!(2, "Error: {}. Falling back to binary", error);
            log!(4, "Error: {:?}", error);

            match io::stdout().write(owned.as_ref()) {
                Err(error) => {
                    stderr!("Failed to write message to stdout: {}", error);
                    log!(2, "Error: {:?}", error);
                    exit(1);
                }
                _ => log!(3, "Printing binary frame"),
            }
        }
    }
}
