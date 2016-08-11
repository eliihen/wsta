use std::io;
use std::io::Write;
use std::process::exit;

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
    log!(3, "Created HTTP client: {:?}", client);

    // Parse string as url and handle ParseErrors
    let url = match Url::parse(&options.login_url) {
        Ok(result) => {
            log!(2, "Parsed URL {:?}", result);

            result
        }
        Err(err) => {
            log!(1, "Error object: {:?}", err);
            stderr!("Failed to parse url '{}': {}", &options.login_url, &err);
            exit(1);
        }
    };

    // Wrap with TLS if needed
    if url.scheme() == "https" {
        log!(3, "Scheme is https");

        let https_connector = HttpsConnector::new(Openssl::default());
        log!(3, "Created https_connector: {:?}", https_connector);

        client = Client::with_connector(https_connector);
        log!(3, "Set client to be TLS wrapped client: {:?}", client);
    }

    // Only redirect if requested - otherwise it is really confusing
    if !options.follow_redirect {
        client.set_redirect_policy(RedirectPolicy::FollowNone);
        log!(3, "Set client to not follow redirects: {:?}", client);
    }

    // Create RequestBuilder
    let request = client.get(url);
    log!(3, "Created RequestBuilder");

    // Create and send an outgoing request.
    match request.send() {
        Ok(res) => {
            log!(2, "Received response: {:?}", res);

            if options.print_headers {
                print_headers("Authenticate response", &res.headers, None);
            }

            return extract_cookie(&res.headers);
        },
        Err(err) => {
            log!(1, "Error: {:?}", err);
            stderr!("Error sending login request: {}", &err);
            exit(1);
        }
    }
}

pub fn print_headers(title: &str, headers: &Headers,
                     status: Option<StatusCode>) {
    stderr!("{}", title);
    stderr!("---");

    if status.is_some() {
        stderr!("{}", status.unwrap());
    }

    stderr!("{}\n", headers);
}

/// Looks up the SetCookie header of the Headers set, and maps
/// those headers into a consumable set of CookiePairs. It will
/// return None of no SetCookie header was found.
fn extract_cookie(headers: &Headers) -> Option<Cookie> {

    let set_cookie_header = headers.get::<SetCookie>();
    log!(3, "Found SetCookie header: {:?}", set_cookie_header);

    match set_cookie_header {
        Some(header) => {

            // The name:value cookies
            let mut cookies = Vec::new();

            // Iterate over cookies in SetCookie header and re-map them
            for cookie in header.as_slice() {

                let pair = CookiePair::new(
                    cookie.name.to_string(),
                    cookie.value.to_string()
                );

                log!(3, "Created CookiePair: {:?}", pair);
                cookies.push(pair);
            }

            Some(Cookie(cookies))
        },
        _ => {
            log!(1, "No SetCookie header found in response");
            None
        }
    }
}

