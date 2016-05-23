# wsta

The WebSocket Transfer Agent

`wsta` is a cli tool written in rust for interfacing with WebSockets. `wsta` has
the philosophy of being an easy tool to learn and thus gets out of your way to
let you work your UNIX magic directly on the WebSocket traffic.
The way `wsta` does this is to be as pipe-friendly as possible, letting you
chain it into complex pipelines or bash scripts as you see fit, or just keep it
simple and use it as is.

See the [manual](wsta.md) or type `man wsta` for details.

## Cool things you can do

Since `wsta` is really pipe-friendly, you can easily work with your output in
a way that suits you. If you have a websocket-service that returns JSON, you
might want to have your data printed in a nice, readable format.
[jq](https://stedolan.github.io/jq/) is perfect for that.

```bash
$ wsta ws://echo.websocket.org '{"values":{"test": "what?"}}' | jq .values
Connected to ws://echo.websocket.org
{
  "test": "what?"
}
```

Because `wsta` reads from stdin, it can also be used as an interactive prompt
if you wish to send messages to the server interactively.

```bash
$ wsta ws://echo.websocket.org
Connected to ws://echo.websocket.org
ping
ping
hello
hello
```

If you're debugging some nasty problem with your stream, you are probably only
interested in frames related to your problem. Good news, `grep` is here to save
the day!

```bash
$ while true; do echo  $(( RANDOM %= 200 )); sleep 0.2; done | wsta ws://echo.websocket.org | grep '147'
147
147
147
147
147
147
```

Use `wsta` to monitor your websocket uptime. Use the `--ping` option to keep
the connection alive, and check the exit code for issues. You could also
potentially send the last few messages with POST data.

```bash
#!/bin/bash

while true; do

  # Start persistent connection, pinging evey 10 seconds to stay alive
  wsta --ping 10 ws://echo.websocket.org

  if [ $? -gt 0 ]; then
    curl -X POST https://SOUNDTHEALARM.yourcompany.com
  fi

  sleep 30
done
```

## Installation

### Requirements

Currently the only requirement to run `wsta` is rust-openssl. If you get an error
about a missing `ssllib.so` or similar, try installing OpenSSL runtime libraries
and headers. Have a look at [this link](https://github.com/sfackler/rust-openssl#building)
for instructions on how to do so.

### 64-bit Linux
I've set up a download page here that you can get `wsta`

https://software.opensuse.org/download.html?project=home%3Aesphen&package=wsta

I'm working on getting more distributions, as well as 32-bit into the Open Build
Service pipeline, which is what creates the releases on that page. For now, you
need a 64-bit system to use that page. If you don't use a 64-bit system, have a
look below at compiling it yourself.

### Mac OS X
To install on Max OS X, ensure you have [homebrew](http://brew.sh) installed,
then run the following commands. It's going to take a while, please be patient.

    brew tap esphen/wsta https://github.com/esphen/wsta.git
    brew install wsta

### Other 64-bit Linux distributions
I only have 64-bit Linux machines available to create binaries with, but if you
have a 64-bit Linux machine, I do provide the binary in every release here on
GitHub. Have a look at the most recent release, and place the attached binary
into your `$PATH`.

https://github.com/esphen/wsta/releases

### Windows

See "Compile it yourself".

### Compile it yourself

DON'T PANIC. It's really easy to compile and install `wsta` yourself! Rust
provides solid tools like `cargo` for automating the compilation. If you compile
`wsta` yourself, it should run on all of
[rust's supported platforms](https://doc.rust-lang.org/book/getting-started.html#platform-support).
I have only tested Linux, however, so YMMV.

    # Install the rust language and tools
    curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- --channel=beta

    # Install gcc and OpenSSL on your OS
    dnf install -y gcc openssl-devel

    # Install wsta to `$CARGO_HOME` if set or `$HOME/.cargo`
    # To change the install path, try setting --root to a directory like /usr/local
    cargo install --git https://github.com/esphen/wsta.git

## Development setup

[Install the rust language and
tools](https://doc.rust-lang.org/book/getting-started.html#installing-rust).
We use some cargo environment variables that are beta features (cargo 0.10).

    curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- --channel=beta

Run the program

    cargo run -- -vvv -I -e ws://echo.websocket.org

In order to generate the man page, `groff` is needed

    make man

If updates to the man page are done, remember to generate the markdown manual
afterwards

    make wsta.md

