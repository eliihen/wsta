# wsta

The WebSocket Transfer Agent

[![Build Status](https://travis-ci.org/esphen/wsta.svg?branch=master)](https://travis-ci.org/esphen/wsta)

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
the connection alive, and check the exit code for issues. You can also send
the last few messages with POST data for a higher quality alert.

```bash
#!/bin/bash

while true; do

  # Start persistent connection, pinging evey 10 seconds to stay alive
  wsta -v --ping 10 ws://echo.websocket.org > messages.txt

  if [ $? -gt 0 ]; then
    tail messages.txt | curl -F "messages=@-" https://SOUNDTHEALARM.yourcompany.com
  fi

  sleep 30
done
```

If you need to load test your server over websocket, it is simple to write
a short bash script to do this. The example below uses a loop to continously
send messages to the server and saturate the connection as much as possible.

```bash
#!/bin/bash

for i in {1..1000}
do
  echo subscribe?giveMeLotsOfData=true
  echo unsubscribe
done | wsta ws://echo.websocket.org
```

`wsta` also supports binary data using the `--binary` argument. When provided,
all data read from stdin is assumed to be in binary format. The following
simplified example records a binary stream from the microphone and sends it
continously to the server, reading the response JSON as it comes in.

For more information on binary mode, see
[the manual](https://github.com/esphen/wsta/blob/master/wsta.md) and
[#5](https://github.com/esphen/wsta/issues/5).

```bash
$ arecord --format=S16_LE --rate=44100 | wsta -b 'wss://example.com' | jq .results
"hello "
"hello this is me "
"hello this is me talking to "
"hello this is me talking to people "
"hello this is me talking to people "
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

You can also find binary releases on the
[releases page](https://github.com/esphen/wsta/releases).

### Other Linux distributions
I only provide so many Linux distros on OBS, and only 64-bit versions. If your
computer does not fit into the distros provided, then have a look at the
download section of the most recent release, and place the attached binary into
your `$PATH`.

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

