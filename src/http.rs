use cookie::Cookie as CookiePair;

use hyper::Client;
use hyper::Url;
use hyper::net::{HttpsConnector, Openssl};
use hyper::header::{Headers, SetCookie, Cookie};
use hyper::status::StatusCode;
use hyper::client::RedirectPolicy;

use options::Options;

pub fn fetch_session_cookie(options: &Options) -> Option<Cookie> {

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

    // Only redirect if requested - otherwise it is really confusing
    if !options.follow_redirect {
        client.set_redirect_policy(RedirectPolicy::FollowNone);
    }

    // Create RequestBuilder
    let request = client.get(url);

    // Create and send an outgoing request.
    match request.send() {
        Ok(res) => {
            if options.print_headers {
                print_headers("Authenticate response", &res.headers, None);
            }

            return extract_cookie(&res.headers);
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
fn extract_cookie(headers: &Headers) -> Option<Cookie> {
    let set_cookie_header = headers.get::<SetCookie>();

    match set_cookie_header {
        Some(header) => {
            for cookie in header.as_slice() {

                if cookie.name.to_lowercase().contains("session") {
                    let pair = CookiePair::new(
                        format!("{}", cookie.name),
                        format!("{}", cookie.value)
                    );

                    return Some(Cookie(vec![pair]));
                }
            }

            None
        },
        _ => None
    }
}

