use hyper::Client;
use hyper::Url;
use hyper::net::{HttpsConnector, Openssl};
use hyper::header::{Headers, SetCookie, CookieJar};
use hyper::status::StatusCode;
use hyper::client::response::Response;

use options::Options;

pub fn fetch_session_cookie(options: &Options) { // -> Cookie {

    // Create a client.
    let mut client = Client::new();

    // Parse string as url and handle ParseErrors
    let url;
    match Url::parse(&options.login_url) {
        Ok(result) => url = result,
        Err(err) => panic!("Failed to parse url '{}': {}",
                           &options.login_url, &err)
    }

    // Wrap with TLS if needed
    if url.scheme() == "https" {
        let https_connector = HttpsConnector::new(Openssl::default());
        client = Client::with_connector(https_connector);
    }

    // Create RequestBuilder
    // TODO Query string is stripped here
    println!("URL {:?}", &url);
    let request = client.get(url);

    // Create and send an outgoing request.
    match request.send() {
        Ok(res) => {
            println!("{:?}", res);
            if options.print_headers {
                print_headers("Authenticate response", &res.headers, None);
            }

            extract_cookie(&res.headers);
            // return extract_cookie(&res.headers);
        },
        Err(err) => panic!("Error sending login request: {}", &err)
    }
}

pub fn print_headers(title: &str, headers: &Headers,
                     status: Option<StatusCode>) {
    println!("{}", title);
    println!("---");

    if status.is_some() {
        println!("{}", status.unwrap());
    }

    println!("{}\n", headers);
}

/// Finds the cookie with name matching .*session.* and returns it
fn extract_cookie(headers: &Headers) { // -> Cookie {
    let mut cookie_jar = CookieJar::new(b"d1zbqbctvkuthji4rulxiikq4ctvkuthj");
    let cookie_header = headers.get::<SetCookie>();

    println!("{:?}", headers);
    println!("{:?}", cookie_header);

    // SetCookie::apply_to_cookie_jar(cookie_header.unwrap(), &mut cookie_jar);

    // for cookie in cookie_jar.iter() {
    //     println!("{}", cookie);
    // }
}

