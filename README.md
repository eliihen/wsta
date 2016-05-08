# wsta

The WebSocket Transfer Agent

## Development setup

[Install the rust language and
tools](https://doc.rust-lang.org/book/getting-started.html#installing-rust).
We use some cargo environment variables that are beta features (cargo 0.10).

    curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- --channel=beta

Run the program

    cargo run -- -vvv -I -e ws://echo.websocket.org

In order to generate the man page, `groff` is needed

    make man

