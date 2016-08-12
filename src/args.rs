use std::env;

/// Fetch the profile from CLI arguments
/// get_profile needs to handle the profile name fetching, because
/// we need the profile name before the main argument parser is
/// invoked (to init options).
/// This method is not perfect - it would be hard to support
/// both `--profile test` and `--profile=test`, for example, so
/// we keep it simple and only support -P
pub fn get_profile() -> Option<String> {

    // Find the position of -P
    let pos = env::args().position(|a| a == "-P");

    // Get the next arg as the value
    if pos.is_some() {
        env::args().nth(pos.unwrap() + 1)
    } else {
        None
    }
}
