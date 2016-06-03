class Wsta < Formula
  desc "A cli tool written in rust for interfacing with WebSocket services."
  homepage "https://github.com/esphen/wsta"
  url "https://github.com/esphen/wsta/archive/0.3.0.tar.gz"
  sha256 "63139d9a1833e237ddcc1ae585c1d3a25cce9505f8c83803ad4c3c70d2c2cdb7"

  depends_on 'gpg' => :build
  depends_on 'multirust' => :build
  depends_on 'openssl'

  def install
    system "multirust", "update", "beta"
    system "multirust", "override", "beta"
    system "cargo", "build", "--release"

    bin.mkpath
    bin.install "target/release/wsta"

    man.mkpath
    man1.install "wsta.1"
  end
end

