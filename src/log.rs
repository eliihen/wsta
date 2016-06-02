static mut log_level: u8 = 0;

macro_rules! stderr {
    ( $( $msg:tt )* ) => {{
        writeln!(io::stderr(), $($msg)*).unwrap();
    }}
}

macro_rules! log {
    // No format string
    ($loudness:expr, $msg:expr ) => {{
        let log_level = $crate::log::get_log_level();

        if log_level >= $loudness  {
            stderr!("VERB {}: {}", $loudness, $msg);
        }
    }};
    // Format string and args
    ($loudness:expr, $( $msg:tt )* ) => {{

        let log_level = $crate::log::get_log_level();

        if log_level >= $loudness  {
            let pretty_msg = format!($($msg)*);
            stderr!("VERB {}: {}", $loudness, pretty_msg);
        }
    }};
}

/// Sets the log level of the application. See `Options::verbosity` for
/// more info.
pub fn set_log_level(level: u8) {

    // This is considered unsafe because we are mutating a static
    // variable, which has runtime lifetime and a fixed location in memory.
    // Thus one thread could be updating N while another is reading it,
    // causing memory unsafety. This is not an issue, as we only call this
    // at the very beginning of the program, when verbosity has beeen parsed.
    unsafe {
        log_level = level;
    }
}

/// Get the current log level of the application
pub fn get_log_level() -> u8 {

    // Use of mutable static requires unsafe function or block
    // We are only reading it after it has been changed, so it should
    // be fine
    unsafe {
        log_level
    }
}

