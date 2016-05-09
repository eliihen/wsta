class Wsta < Formula
  desc "A cli tool written in rust for interfacing with WebSocket services."
  homepage "https://github.com/esphen/wsta"
  url "https://github.com/esphen/wsta/archive/0.2.0.tar.gz"
  sha256 "98af296907b371083289a8b35bd6ff24cfd8fafb013033b724aacb5fe774c9b1"

  depends_on 'pointlessone/homebrew-rust-nightly/rust-nightly' => :build
  depends_on 'pointlessone/homebrew-rust-nightly/cargo-nightly' => :build
  depends_on 'openssl'

  def install
    system "cargo", "build", "--release"
    bin.install Dir["target", "release", "wsta"]

    man.mkpath
    man1.install "wsta.1"
  end
end
