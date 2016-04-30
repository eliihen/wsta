extern crate websocket;
extern crate argparse;

use std::io;

use argparse::{ArgumentParser, StoreTrue, Store};

use websocket::{Client, Message};
use websocket::client::request::Url;

fn main() {

    let mut url = String::new();

    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("The Web Socket Transfer Agent.");
        ap.refer(&mut url)
            .add_option(&["-u", "--url"], Store,
                        "Name for the greeting");
        ap.parse_args_or_exit();
    }


    // Read initial message from stdin
    let mut stdin = String::new();
    match io::stdin().read_line(&mut stdin) {
        Ok(_) => {},
        Err(error) => println!("error: {}", error)
    }

    // Get the URL
    let url = Url::parse(&url).unwrap();

    // Connect to the server
    let request = Client::connect(url).unwrap();

    // Send the request
    let response = request.send().unwrap();

    // Ensure the response is valid
    response.validate().unwrap();

    // Get a Client
    let mut client = response.begin();

    let message = Message::text(stdin.trim());

    // Send message
    client.send_message(&message).unwrap();
    println!("> {}", stdin.trim());

    for message in client.incoming_messages() {
        let message: Message = message.unwrap();
        let owned = message.payload.into_owned();
        let payload = String::from_utf8(owned).unwrap();
        println!("< {}", payload);
    }
}

