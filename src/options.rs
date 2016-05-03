/// The command line options provided to the program
pub struct Options {

    /// Flag the application to be more pipe-friendly.
    /// Only print the incoming websocket frames.
    pub quiet: bool,

    /// The verbosity level of the application. Should be a number
    /// between 0 and 3.
    ///   0: NO LOGGING
    ///   1: EVENT LOGGING
    ///   2: DEBUG LOGGING
    /// >=3: TRACE LOGGING
    pub verbosity: u8,

    /// The WebSocket URL to connect to.
    pub url: String,

    /// Optional: A GET URL to authenticate with before connecting
    /// to the main url.
    pub login_url: String,

    /// When passed, this flag will cause the program to follow
    /// HTTP GET redirection encountered when calling login_url.
    pub follow_redirect: bool,

    /// Print the headers of any HTTP request when true.
    pub print_headers: bool,

    /// Headers
    pub headers: Vec<String>
}

